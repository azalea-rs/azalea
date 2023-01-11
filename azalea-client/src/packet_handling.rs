use std::{io::Cursor, sync::Arc};

use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{ReadConnection, WriteConnection},
    packets::game::{
        serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
        serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
        ClientboundGamePacket, ServerboundGamePacket,
    },
};
use azalea_world::{
    entity::{
        metadata::{apply_metadata, Health, PlayerMetadataBundle},
        set_rotation, Dead, EntityBundle, EntityKind, LastSentPosition, MinecraftEntityId, Physics,
        PlayerBundle, Position,
    },
    EntityInfos, PartialWorld, WorldContainer,
};
use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    prelude::Entity,
    query::Changed,
    schedule::{IntoSystemDescriptor, SystemSet},
    system::{Commands, Query, ResMut},
};
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::{local_player::LocalPlayer, ChatPacket, ClientInformation, Event, PlayerInfo};

pub struct PacketHandlerPlugin;

impl Plugin for PacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(handle_packets.label("packet"))
                .with_system(clear_packets.after("packet")),
        );
    }
}

/// Something that receives packets from the server.
#[derive(Component, Clone)]
pub struct PacketReceiver {
    pub packets: Arc<Mutex<Vec<ClientboundGamePacket>>>,
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

fn handle_packets(
    mut commands: Commands,
    query: Query<(Entity, &PacketReceiver), Changed<PacketReceiver>>,
    local_player_query: Query<&LocalPlayer>,
    entity_kind_query: Query<&EntityKind>,
    mut mut_local_player_query: Query<&mut LocalPlayer>,
    mut mut_health_query: Query<&mut Health>,
    mut mut_position_query: Query<&mut Position>,
    combat_kill_query: Query<(&MinecraftEntityId, Option<&Dead>)>,
    mut position_query: Query<(
        &mut LocalPlayer,
        &mut Physics,
        &mut Position,
        &mut LastSentPosition,
    )>,
    mut world_container: ResMut<WorldContainer>,
    mut entity_infos: ResMut<EntityInfos>,
) {
    for (player_entity, packet_events) in &query {
        let packets = packet_events.packets.lock();

        for packet in packets.iter() {
            match packet {
                ClientboundGamePacket::Login(p) => {
                    debug!("Got login packet");
                    let mut local_player = mut_local_player_query.get_mut(player_entity).unwrap();

                    {
                        // TODO: have registry_holder be a struct because this sucks rn
                        // best way would be to add serde support to azalea-nbt

                        let registry_holder = p
                            .registry_holder
                            .as_compound()
                            .expect("Registry holder is not a compound")
                            .get("")
                            .expect("No \"\" tag")
                            .as_compound()
                            .expect("\"\" tag is not a compound");
                        let dimension_types = registry_holder
                            .get("minecraft:dimension_type")
                            .expect("No dimension_type tag")
                            .as_compound()
                            .expect("dimension_type is not a compound")
                            .get("value")
                            .expect("No dimension_type value")
                            .as_list()
                            .expect("dimension_type value is not a list");
                        let dimension_type = dimension_types
                            .iter()
                            .find(|t| {
                                t.as_compound()
                                    .expect("dimension_type value is not a compound")
                                    .get("name")
                                    .expect("No name tag")
                                    .as_string()
                                    .expect("name is not a string")
                                    == p.dimension_type.to_string()
                            })
                            .unwrap_or_else(|| {
                                panic!("No dimension_type with name {}", p.dimension_type)
                            })
                            .as_compound()
                            .unwrap()
                            .get("element")
                            .expect("No element tag")
                            .as_compound()
                            .expect("element is not a compound");
                        let height = (*dimension_type
                            .get("height")
                            .expect("No height tag")
                            .as_int()
                            .expect("height tag is not an int"))
                        .try_into()
                        .expect("height is not a u32");
                        let min_y = *dimension_type
                            .get("min_y")
                            .expect("No min_y tag")
                            .as_int()
                            .expect("min_y tag is not an int");

                        let world_name = p.dimension.clone();

                        local_player.world_name = Some(world_name.clone());
                        // add this world to the world_container (or don't if it's already
                        // there)
                        let weak_world = world_container.insert(world_name.clone(), height, min_y);
                        // set the partial_world to an empty world
                        // (when we add chunks or entities those will be in the
                        // world_container)

                        *local_player.partial_world.write() = PartialWorld::new(
                            local_player.client_information.view_distance.into(),
                            // this argument makes it so other clients don't update this
                            // player entity
                            // in a shared world
                            Some(player_entity),
                            &mut entity_infos,
                        );
                        local_player.world = weak_world;

                        let player_bundle = PlayerBundle {
                            entity: EntityBundle::new(
                                local_player.profile.uuid,
                                Vec3::default(),
                                azalea_registry::EntityKind::Player,
                                world_name,
                            ),
                            metadata: PlayerMetadataBundle::default(),
                        };
                        // insert our components into the ecs :)
                        commands
                            .entity(player_entity)
                            .insert((MinecraftEntityId(p.player_id), player_bundle));
                    }

                    // send the client information that we have set
                    let client_information_packet: ClientInformation =
                        local_player.client_information.clone();
                    log::debug!(
                        "Sending client information because login: {:?}",
                        client_information_packet
                    );
                    local_player.write_packet(client_information_packet.get());

                    // brand
                    local_player.write_packet(
                        ServerboundCustomPayloadPacket {
                            identifier: ResourceLocation::new("brand").unwrap(),
                            // they don't have to know :)
                            data: "vanilla".into(),
                        }
                        .get(),
                    );

                    local_player.tx.send(Event::Login).unwrap();
                }
                ClientboundGamePacket::SetChunkCacheRadius(p) => {
                    debug!("Got set chunk cache radius packet {:?}", p);
                }
                ClientboundGamePacket::CustomPayload(p) => {
                    debug!("Got custom payload packet {:?}", p);
                }
                ClientboundGamePacket::ChangeDifficulty(p) => {
                    debug!("Got difficulty packet {:?}", p);
                }
                ClientboundGamePacket::Commands(_p) => {
                    debug!("Got declare commands packet");
                }
                ClientboundGamePacket::PlayerAbilities(p) => {
                    debug!("Got player abilities packet {:?}", p);
                }
                ClientboundGamePacket::SetCarriedItem(p) => {
                    debug!("Got set carried item packet {:?}", p);
                }
                ClientboundGamePacket::UpdateTags(_p) => {
                    debug!("Got update tags packet");
                }
                ClientboundGamePacket::Disconnect(p) => {
                    debug!("Got disconnect packet {:?}", p);
                    let local_player = local_player_query.get(player_entity).unwrap();
                    local_player.disconnect();
                }
                ClientboundGamePacket::UpdateRecipes(_p) => {
                    debug!("Got update recipes packet");
                }
                ClientboundGamePacket::EntityEvent(_p) => {
                    // debug!("Got entity event packet {:?}", p);
                }
                ClientboundGamePacket::Recipe(_p) => {
                    debug!("Got recipe packet");
                }
                ClientboundGamePacket::PlayerPosition(p) => {
                    // TODO: reply with teleport confirm
                    debug!("Got player position packet {:?}", p);

                    let (mut local_player, mut physics, mut position, mut last_sent_position) =
                        position_query.get_mut(player_entity).unwrap();

                    let delta_movement = physics.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        last_sent_position.x += p.x;
                        (delta_movement.x, position.x + p.x)
                    } else {
                        last_sent_position.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        last_sent_position.y += p.y;
                        (delta_movement.y, position.y + p.y)
                    } else {
                        last_sent_position.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        last_sent_position.z += p.z;
                        (delta_movement.z, position.z + p.z)
                    } else {
                        last_sent_position.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        x_rot += physics.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        y_rot += physics.y_rot;
                    }

                    physics.delta = Vec3 {
                        x: delta_x,
                        y: delta_y,
                        z: delta_z,
                    };
                    // we call a function instead of setting the fields ourself since the
                    // function makes sure the rotations stay in their
                    // ranges
                    set_rotation(physics.into_inner(), y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };

                    **position = new_pos;

                    local_player
                        .write_packet(ServerboundAcceptTeleportationPacket { id: p.id }.get());
                    local_player.write_packet(
                        ServerboundMovePlayerPosRotPacket {
                            x: new_pos.x,
                            y: new_pos.y,
                            z: new_pos.z,
                            y_rot,
                            x_rot,
                            // this is always false
                            on_ground: false,
                        }
                        .get(),
                    );
                }
                ClientboundGamePacket::PlayerInfoUpdate(p) => {
                    debug!("Got player info packet {:?}", p);

                    let mut local_player = mut_local_player_query.get_mut(player_entity).unwrap();

                    for updated_info in &p.entries {
                        // add the new player maybe
                        if p.actions.add_player {
                            let player_info = PlayerInfo {
                                profile: updated_info.profile.clone(),
                                uuid: updated_info.profile.uuid,
                                gamemode: updated_info.game_mode,
                                latency: updated_info.latency,
                                display_name: updated_info.display_name.clone(),
                            };
                            local_player
                                .players
                                .insert(updated_info.profile.uuid, player_info.clone());
                            local_player.tx.send(Event::AddPlayer(player_info)).unwrap();
                        } else if let Some(info) =
                            local_player.players.get_mut(&updated_info.profile.uuid)
                        {
                            // `else if` because the block for add_player above
                            // already sets all the fields
                            if p.actions.update_game_mode {
                                info.gamemode = updated_info.game_mode;
                            }
                            if p.actions.update_latency {
                                info.latency = updated_info.latency;
                            }
                            if p.actions.update_display_name {
                                info.display_name = updated_info.display_name.clone();
                            }
                            let info = info.clone();
                            local_player.tx.send(Event::UpdatePlayer(info)).unwrap();
                        } else {
                            warn!(
                                "Ignoring PlayerInfoUpdate for unknown player {}",
                                updated_info.profile.uuid
                            );
                        }
                    }
                }
                ClientboundGamePacket::PlayerInfoRemove(p) => {
                    let mut local_player = mut_local_player_query.get_mut(player_entity).unwrap();

                    for uuid in &p.profile_ids {
                        if let Some(info) = local_player.players.remove(uuid) {
                            local_player.tx.send(Event::RemovePlayer(info)).unwrap();
                        }
                    }
                }
                ClientboundGamePacket::SetChunkCacheCenter(p) => {
                    debug!("Got chunk cache center packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    let mut partial_world = local_player.partial_world.write();

                    partial_world.chunks.view_center = ChunkPos::new(p.x, p.z);
                }
                ClientboundGamePacket::LevelChunkWithLight(p) => {
                    // debug!("Got chunk with light packet {} {}", p.x, p.z);
                    let pos = ChunkPos::new(p.x, p.z);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    let world = local_player.world.read();
                    let partial_world = local_player.partial_world.read();

                    // OPTIMIZATION: if we already know about the chunk from the
                    // shared world (and not ourselves), then we don't need to
                    // parse it again. This is only used when we have a shared
                    // world, since we check that the chunk isn't currently owned
                    // by this client.
                    let shared_has_chunk = world.chunks.get(&pos).is_some();
                    let this_client_has_chunk = partial_world.chunks.limited_get(&pos).is_some();
                    if shared_has_chunk && !this_client_has_chunk {
                        trace!(
                            "Skipping parsing chunk {:?} because we already know about it",
                            pos
                        );
                        return;
                    }

                    // ok we're sure we're going to mutate the world, so get exclusive write
                    // access
                    let mut partial_world = local_player.partial_world.write();
                    let mut world = local_player.world.write();

                    if let Err(e) = partial_world.chunks.replace_with_packet_data(
                        &pos,
                        &mut Cursor::new(&p.chunk_data.data),
                        &mut world.chunks,
                    ) {
                        error!("Couldn't set chunk data: {}", e);
                    }
                }
                ClientboundGamePacket::LightUpdate(_p) => {
                    // debug!("Got light update packet {:?}", p);
                }
                ClientboundGamePacket::AddEntity(p) => {
                    debug!("Got add entity packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    if let Some(world_name) = &local_player.world_name {
                        let bundle = p.as_entity_bundle(world_name.clone());
                        let mut entity_commands = commands.spawn((MinecraftEntityId(p.id), bundle));
                        // the bundle doesn't include the default entity metadata so we add that
                        // separately
                        p.apply_metadata(&mut entity_commands);
                    } else {
                        warn!("got add player packet but we haven't gotten a login packet yet");
                    }
                }
                ClientboundGamePacket::SetEntityData(p) => {
                    debug!("Got set entity data packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    let partial_world = local_player.partial_world.write();
                    let entity = partial_world
                        .entity_infos
                        .get_by_id(MinecraftEntityId(p.id));
                    drop(partial_world);

                    if let Some(entity) = entity {
                        let entity_kind = entity_kind_query
                            .get(entity)
                            .expect("EntityKind should always be present");
                        let mut entity_commands = commands.entity(entity);
                        if let Err(e) = apply_metadata(
                            &mut entity_commands,
                            **entity_kind,
                            (*p.packed_items).clone(),
                        ) {
                            warn!("{e}");
                        }
                    } else {
                        warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
                    }
                }
                ClientboundGamePacket::UpdateAttributes(_p) => {
                    // debug!("Got update attributes packet {:?}", p);
                }
                ClientboundGamePacket::SetEntityMotion(_p) => {
                    // debug!("Got entity velocity packet {:?}", p);
                }
                ClientboundGamePacket::SetEntityLink(p) => {
                    debug!("Got set entity link packet {:?}", p);
                }
                ClientboundGamePacket::AddPlayer(p) => {
                    debug!("Got add player packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    if let Some(world_name) = &local_player.world_name {
                        let bundle = p.as_player_bundle(world_name.clone());
                        commands.spawn((MinecraftEntityId(p.id), bundle));
                    } else {
                        warn!("got add player packet but we haven't gotten a login packet yet");
                    }
                }
                ClientboundGamePacket::InitializeBorder(p) => {
                    debug!("Got initialize border packet {:?}", p);
                }
                ClientboundGamePacket::SetTime(p) => {
                    debug!("Got set time packet {:?}", p);
                }
                ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                    debug!("Got set default spawn position packet {:?}", p);
                }
                ClientboundGamePacket::ContainerSetContent(p) => {
                    debug!("Got container set content packet {:?}", p);
                }
                ClientboundGamePacket::SetHealth(p) => {
                    debug!("Got set health packet {:?}", p);

                    let mut health = mut_health_query.get_mut(player_entity).unwrap();
                    **health = p.health;

                    // the `Dead` component is added by `update_dead` in
                    // azalea-world and then the
                    // `dead_event` system fires the Death event.
                }
                ClientboundGamePacket::SetExperience(p) => {
                    debug!("Got set experience packet {:?}", p);
                }
                ClientboundGamePacket::TeleportEntity(p) => {
                    let local_player = local_player_query.get(player_entity).unwrap();
                    let partial_world = local_player.partial_world.read();
                    let partial_entity_infos = &partial_world.entity_infos;
                    let entity = partial_entity_infos.get_by_id(MinecraftEntityId(p.id));
                    drop(partial_world);

                    if let Some(entity) = entity {
                        let mut position = mut_position_query.get_mut(entity).unwrap();
                        **position = p.position;
                    } else {
                        warn!("Got teleport entity packet for unknown entity id {}", p.id);
                    }
                }
                ClientboundGamePacket::UpdateAdvancements(p) => {
                    debug!("Got update advancements packet {:?}", p);
                }
                ClientboundGamePacket::RotateHead(_p) => {
                    // debug!("Got rotate head packet {:?}", p);
                }
                ClientboundGamePacket::MoveEntityPos(p) => {
                    let local_player = local_player_query.get(player_entity).unwrap();
                    let partial_world = local_player.partial_world.read();
                    let partial_entity_infos = &partial_world.entity_infos;
                    let entity = partial_entity_infos.get_by_id(MinecraftEntityId(p.entity_id));
                    drop(partial_world);

                    if let Some(entity) = entity {
                        let mut position = mut_position_query.get_mut(entity).unwrap();
                        **position = position.with_delta(&p.delta);
                    } else {
                        warn!(
                            "Got move entity pos packet for unknown entity id {}",
                            p.entity_id
                        );
                    }
                }
                ClientboundGamePacket::MoveEntityPosRot(p) => {
                    let local_player = local_player_query.get(player_entity).unwrap();
                    let partial_world = local_player.partial_world.read();
                    let partial_entity_infos = &partial_world.entity_infos;
                    let entity = partial_entity_infos.get_by_id(MinecraftEntityId(p.entity_id));
                    drop(partial_world);

                    if let Some(entity) = entity {
                        let mut position = mut_position_query.get_mut(entity).unwrap();
                        **position = position.with_delta(&p.delta);
                    } else {
                        warn!(
                            "Got move entity pos rot packet for unknown entity id {}",
                            p.entity_id
                        );
                    }
                }
                ClientboundGamePacket::MoveEntityRot(_p) => {
                    // debug!("Got move entity rot packet {:?}", p);
                }
                ClientboundGamePacket::KeepAlive(p) => {
                    debug!("Got keep alive packet {:?}", p);

                    let mut local_player = mut_local_player_query.get_mut(player_entity).unwrap();
                    local_player.write_packet(ServerboundKeepAlivePacket { id: p.id }.get());
                }
                ClientboundGamePacket::RemoveEntities(p) => {
                    debug!("Got remove entities packet {:?}", p);
                }
                ClientboundGamePacket::PlayerChat(p) => {
                    debug!("Got player chat packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();

                    local_player
                        .tx
                        .send(Event::Chat(ChatPacket::Player(Arc::new(p.clone()))))
                        .unwrap();
                }
                ClientboundGamePacket::SystemChat(p) => {
                    debug!("Got system chat packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();

                    local_player
                        .tx
                        .send(Event::Chat(ChatPacket::System(Arc::new(p.clone()))))
                        .unwrap();
                }
                ClientboundGamePacket::Sound(_p) => {
                    // debug!("Got sound packet {:?}", p);
                }
                ClientboundGamePacket::LevelEvent(p) => {
                    debug!("Got level event packet {:?}", p);
                }
                ClientboundGamePacket::BlockUpdate(p) => {
                    debug!("Got block update packet {:?}", p);

                    let local_player = local_player_query.get(player_entity).unwrap();
                    let world = local_player.world.write();

                    world.chunks.set_block_state(&p.pos, p.block_state);
                }
                ClientboundGamePacket::Animate(p) => {
                    debug!("Got animate packet {:?}", p);
                }
                ClientboundGamePacket::SectionBlocksUpdate(p) => {
                    debug!("Got section blocks update packet {:?}", p);
                    let local_player = local_player_query.get(player_entity).unwrap();
                    let world = local_player.world.write();

                    for state in &p.states {
                        world
                            .chunks
                            .set_block_state(&(p.section_pos + state.pos.clone()), state.state);
                    }
                }
                ClientboundGamePacket::GameEvent(p) => {
                    debug!("Got game event packet {:?}", p);
                }
                ClientboundGamePacket::LevelParticles(p) => {
                    debug!("Got level particles packet {:?}", p);
                }
                ClientboundGamePacket::ServerData(p) => {
                    debug!("Got server data packet {:?}", p);
                }
                ClientboundGamePacket::SetEquipment(p) => {
                    debug!("Got set equipment packet {:?}", p);
                }
                ClientboundGamePacket::UpdateMobEffect(p) => {
                    debug!("Got update mob effect packet {:?}", p);
                }
                ClientboundGamePacket::AddExperienceOrb(_) => {}
                ClientboundGamePacket::AwardStats(_) => {}
                ClientboundGamePacket::BlockChangedAck(_) => {}
                ClientboundGamePacket::BlockDestruction(_) => {}
                ClientboundGamePacket::BlockEntityData(_) => {}
                ClientboundGamePacket::BlockEvent(_) => {}
                ClientboundGamePacket::BossEvent(_) => {}
                ClientboundGamePacket::CommandSuggestions(_) => {}
                ClientboundGamePacket::ContainerSetData(_) => {}
                ClientboundGamePacket::ContainerSetSlot(_) => {}
                ClientboundGamePacket::Cooldown(_) => {}
                ClientboundGamePacket::CustomChatCompletions(_) => {}
                ClientboundGamePacket::DeleteChat(_) => {}
                ClientboundGamePacket::Explode(_) => {}
                ClientboundGamePacket::ForgetLevelChunk(_) => {}
                ClientboundGamePacket::HorseScreenOpen(_) => {}
                ClientboundGamePacket::MapItemData(_) => {}
                ClientboundGamePacket::MerchantOffers(_) => {}
                ClientboundGamePacket::MoveVehicle(_) => {}
                ClientboundGamePacket::OpenBook(_) => {}
                ClientboundGamePacket::OpenScreen(_) => {}
                ClientboundGamePacket::OpenSignEditor(_) => {}
                ClientboundGamePacket::Ping(_) => {}
                ClientboundGamePacket::PlaceGhostRecipe(_) => {}
                ClientboundGamePacket::PlayerCombatEnd(_) => {}
                ClientboundGamePacket::PlayerCombatEnter(_) => {}
                ClientboundGamePacket::PlayerCombatKill(p) => {
                    debug!("Got player kill packet {:?}", p);
                    let (&entity_id, dead) = combat_kill_query.get(player_entity).unwrap();

                    if *entity_id == p.player_id && dead.is_none() {
                        commands.entity(player_entity).insert(Dead);

                        let local_player = local_player_query.get(player_entity).unwrap();
                        local_player
                            .tx
                            .send(Event::Death(Some(Arc::new(p.clone()))))
                            .unwrap();
                    }
                }
                ClientboundGamePacket::PlayerLookAt(_) => {}
                ClientboundGamePacket::RemoveMobEffect(_) => {}
                ClientboundGamePacket::ResourcePack(_) => {}
                ClientboundGamePacket::Respawn(p) => {
                    debug!("Got respawn packet {:?}", p);
                    // Remove the Dead marker component from the player.
                    commands.entity(player_entity).remove::<Dead>();
                }
                ClientboundGamePacket::SelectAdvancementsTab(_) => {}
                ClientboundGamePacket::SetActionBarText(_) => {}
                ClientboundGamePacket::SetBorderCenter(_) => {}
                ClientboundGamePacket::SetBorderLerpSize(_) => {}
                ClientboundGamePacket::SetBorderSize(_) => {}
                ClientboundGamePacket::SetBorderWarningDelay(_) => {}
                ClientboundGamePacket::SetBorderWarningDistance(_) => {}
                ClientboundGamePacket::SetCamera(_) => {}
                ClientboundGamePacket::SetDisplayObjective(_) => {}
                ClientboundGamePacket::SetObjective(_) => {}
                ClientboundGamePacket::SetPassengers(_) => {}
                ClientboundGamePacket::SetPlayerTeam(_) => {}
                ClientboundGamePacket::SetScore(_) => {}
                ClientboundGamePacket::SetSimulationDistance(_) => {}
                ClientboundGamePacket::SetSubtitleText(_) => {}
                ClientboundGamePacket::SetTitleText(_) => {}
                ClientboundGamePacket::SetTitlesAnimation(_) => {}
                ClientboundGamePacket::SoundEntity(_) => {}
                ClientboundGamePacket::StopSound(_) => {}
                ClientboundGamePacket::TabList(_) => {}
                ClientboundGamePacket::TagQuery(_) => {}
                ClientboundGamePacket::TakeItemEntity(_) => {}
                ClientboundGamePacket::DisguisedChat(_) => {}
                ClientboundGamePacket::UpdateEnabledFeatures(_) => {}
                ClientboundGamePacket::ContainerClose(_) => {}
            }
        }
    }
}

/// A system that clears all packets in the clientbound packet events.
fn clear_packets(mut query: Query<&mut PacketReceiver>) {
    for packets in query.iter_mut() {
        packets.packets.lock().clear();
    }
}

impl PacketReceiver {
    /// Loop that reads from the connection and adds the packets to the queue +
    /// runs the schedule.
    pub async fn read_task(self, mut read_conn: ReadConnection<ClientboundGamePacket>) {
        while let Ok(packet) = read_conn.read().await {
            self.packets.lock().push(packet);
            // tell the client to run all the systems
            self.run_schedule_sender.send(()).unwrap();
        }
    }

    /// Consume the [`ServerboundGamePacket`] queue and actually write the
    /// packets to the server. It's like this so writing packets doesn't need to
    /// be awaited.
    pub async fn write_task(
        self,
        mut write_conn: WriteConnection<ServerboundGamePacket>,
        mut write_receiver: mpsc::UnboundedReceiver<ServerboundGamePacket>,
    ) {
        while let Some(packet) = write_receiver.recv().await {
            if let Err(err) = write_conn.write(packet).await {
                error!("Disconnecting because we couldn't write a packet: {err}.");
                break;
            };
        }
        // receiver is automatically closed when it's dropped
    }
}
