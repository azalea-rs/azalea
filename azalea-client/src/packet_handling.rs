use std::sync::Arc;

use azalea_protocol::{
    connect::{ReadConnection, WriteConnection},
    packets::game::{ClientboundGamePacket, ServerboundGamePacket},
};
use azalea_world::{entity::{MinecraftEntityId, Position}, EntityInfos};
use bevy_ecs::{component::Component, prelude::Entity, query::Changed, system::Query};
use log::{error, debug};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::{local_player::{Dead, LocalPlayer}, Event};

/// Something that receives packets from the server.
#[derive(Component, Clone)]
pub struct PacketReceiver {
    pub packets: Arc<Mutex<Vec<ClientboundGamePacket>>>,
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

pub fn handle_packets(
    ecs: &mut bevy_ecs::world::World,
    // ecs: &mut bevy_ecs::world::World,
) {
    let mut query = ecs.query_filtered::<(Entity, &PacketReceiver), Changed<PacketReceiver>>();

    for (entity, packet_events) in query.iter_mut(ecs) {
        for packet in packet_events.packets.lock().iter() {
            handle_packet(ecs, entity, packet);
        }
    }
}

pub fn handle_packet(ecs: &mut bevy_ecs::world::World, entity: Entity, packet: &ClientboundGamePacket) {
    match packet {
        ClientboundGamePacket::Login(p) => {
            debug!("Got login packet");

            {
                // // write p into login.txt
                // std::io::Write::write_all(
                //     &mut std::fs::File::create("login.txt").unwrap(),
                //     format!("{:#?}", p).as_bytes(),
                // )
                // .unwrap();

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
                    .unwrap_or_else(|| panic!("No dimension_type with name {}", p.dimension_type))                 .as_compound()
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
                // add this world to the world_container (or don't if it's already there)
                let weak_world = world_container.insert(world_name, height, min_y);
                             // set the loaded_world to an empty world
                                          // (when we add chunks or entities those will be in the world_container)
                                          let mut world_lock = local_player.world.write();             *world_lock =    PartialWorld::new(
    local_player.client_information.view_distance.into(),
    weak_world,                 Some(EntityId(p.player_id)),
                );

                let player_bundle = entity::PlayerBundle {
                    entity: entity::EntityBundle::new(
                        local_player.profile.uuid,
                        Vec3::default(),
                        azalea_registry::EntityKind::Player,
                    ),
                    metadata: PlayerMetadataBundle::default(),
                };
                // let entity = EntityData::new(
                //     client.profile.uuid,
                //     Vec3::default(),
                //     EntityMetadata::Player(metadata::Player::default()),
                // );
                // the first argument makes it so other entities don't update this entity in a shared world
                world_lock.add_entity(EntityId(p.player_id), player_bundle);

                *client.entity_id.write() = EntityId(p.player_id);
            }

            // send the client information that we have set
            let client_information_packet: ClientInformation =
                client.local_player().client_information.clone();
            log::debug!(
                "Sending client information because login: {:?}",
                client_information_packet
            );
            client.write_packet(client_information_packet.get());

            // brand
            client
                .write_packet(
                    ServerboundCustomPayloadPacket {
                        identifier: ResourceLocation::new("brand").unwrap(),
                        // they don't have to know :)
                        data: "vanilla".into(),
                    }
                    .get(),
                )
                .await?;

            tx.send(Event::Login).await?;
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
            client.disconnect().await?;
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

            let (new_pos, y_rot, x_rot) = {
                let player_entity_id = *client.entity();
                let world = client.world();
                // let mut player_entity =
    world.entity_mut(player_entity_id).unwrap();             let (mut
    physics, position) =                 client.query::<(&mut
    entity::Physics, &mut entity::Position)>();

                let delta_movement = physics.delta;

                let is_x_relative = p.relative_arguments.x;
                let is_y_relative = p.relative_arguments.y;
                let is_z_relative = p.relative_arguments.z;

                let (delta_x, new_pos_x) = if is_x_relative {
                    physics.last_pos.x += p.x;
                    (delta_movement.x, position.x + p.x)
                } else {
                    physics.last_pos.x = p.x;
                    (0.0, p.x)
                };
                let (delta_y, new_pos_y) = if is_y_relative {
                    physics.last_pos.y += p.y;
                    (delta_movement.y, position.y + p.y)
                } else {
                    physics.last_pos.y = p.y;
                    (0.0, p.y)
                };
                let (delta_z, new_pos_z) = if is_z_relative {
                    physics.last_pos.z += p.z;
                    (delta_movement.z, position.z + p.z)
                } else {
                    physics.last_pos.z = p.z;
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
                entity::set_rotation(physics.into_inner(), y_rot, x_rot);
                // TODO: minecraft sets "xo", "yo", and "zo" here but idk
    what that means             // so investigate that ig
                let new_pos = Vec3 {
                    x: new_pos_x,
                    y: new_pos_y,
                    z: new_pos_z,
                };
                world
                    .set_entity_pos(
                        player_entity_id,
                        new_pos,
                        position.into_inner(),
                        physics.into_inner(),
                    )
                    .expect("The player entity should always exist");

                (new_pos, y_rot, x_rot)
            };

            client
                .write_packet(ServerboundAcceptTeleportationPacket { id: p.id
    }.get())             .await?;
            client
                .write_packet(
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
                )
                .await?;
        }
        ClientboundGamePacket::PlayerInfoUpdate(p) => {
            debug!("Got player info packet {:?}", p);
            let mut events = Vec::new();
            {
                let mut players_lock = client.players.write();
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
                        players_lock.insert(updated_info.profile.uuid,
    player_info.clone());
    events.push(Event::AddPlayer(player_info));                 } else if
    let Some(info) = players_lock.get_mut(&updated_info.profile.uuid) {
                        // `else if` because the block for add_player above
                        // already sets all the fields
                        if p.actions.update_game_mode {
                            info.gamemode = updated_info.game_mode;
                        }
                        if p.actions.update_latency {
                            info.latency = updated_info.latency;
                        }
                        if p.actions.update_display_name {
                            info.display_name =
    updated_info.display_name.clone();                     }
                        events.push(Event::UpdatePlayer(info.clone()));
                    } else {
                        warn!(
                            "Ignoring PlayerInfoUpdate for unknown player
    {}",                         updated_info.profile.uuid
                        );
                    }
                }
            }
            for event in events {
                tx.send(event).await?;
            }
        }
        ClientboundGamePacket::PlayerInfoRemove(p) => {
            let mut events = Vec::new();
            {
                let mut players_lock = client.players.write();
                for uuid in &p.profile_ids {
                    if let Some(info) = players_lock.remove(uuid) {
                        events.push(Event::RemovePlayer(info));
                    }
                }
            }
            for event in events {
                tx.send(event).await?;
            }
        }
        ClientboundGamePacket::SetChunkCacheCenter(p) => {
            debug!("Got chunk cache center packet {:?}", p);
            client
                .world
                .write()
                .update_view_center(&ChunkPos::new(p.x, p.z));
        }
        ClientboundGamePacket::LevelChunkWithLight(p) => {
            // debug!("Got chunk with light packet {} {}", p.x, p.z);
            let pos = ChunkPos::new(p.x, p.z);

            // OPTIMIZATION: if we already know about the chunk from the
            // shared world (and not ourselves), then we don't need to
            // parse it again. This is only used when we have a shared
            // world, since we check that the chunk isn't currently owned
            // by this client.
            let shared_has_chunk =
    client.world.read().get_chunk(&pos).is_some();         let
    this_client_has_chunk =
    client.world.read().chunks.limited_get(&pos).is_some();         if
    shared_has_chunk && !this_client_has_chunk {             trace!(
                    "Skipping parsing chunk {:?} because we already know
    about it",                 pos
                );
                return Ok(());
            }

            // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
            // debug("chunk {:?}")
            if let Err(e) = client
                .world
                .write()
                .replace_with_packet_data(&pos, &mut
    Cursor::new(&p.chunk_data.data))         {
                error!("Couldn't set chunk data: {}", e);
            }
        }
        ClientboundGamePacket::LightUpdate(_p) => {
            // debug!("Got light update packet {:?}", p);
        }
        ClientboundGamePacket::AddEntity(p) => {
            debug!("Got add entity packet {:?}", p);
            let bundle = p.as_entity_bundle();
            let mut world = client.world.write();
            world.add_entity(EntityId(p.id), bundle);
            // the bundle doesn't include the default entity metadata so we
    add that         // separately
            let mut entities = world.entity_infos.shared.write();
            let mut entity =
    entities.ecs_entity_mut(EntityId(p.id)).unwrap();
            p.apply_metadata(&mut entity);
        }
        ClientboundGamePacket::SetEntityData(p) => {
            debug!("Got set entity data packet {:?}", p);
            let world = client.world.write();
            let mut entities = world.entity_infos.shared.write();
            let entity = entities.ecs_entity_mut(EntityId(p.id));
            if let Some(mut entity) = entity {
                entity::metadata::apply_metadata(&mut entity,
    p.packed_items.0.clone());         } else {
                // warn!("Server sent an entity data packet for an
                // entity id ({}) that we don't
                // know about", p.id);
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
            let bundle = p.as_player_bundle();
            let mut world = client.world.write();
            world.add_entity(EntityId(p.id), bundle);
            // the default metadata was already included in the bundle
            // for us
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
            if p.health == 0.0 {
                // we can't define a variable here with client.dead.lock()
                // because of https://github.com/rust-lang/rust/issues/57478
                if !*client.dead.lock() {
                    *client.dead.lock() = true;
                    tx.send(Event::Death(None)).await?;
                }
            }
        }
        ClientboundGamePacket::SetExperience(p) => {
            debug!("Got set experience packet {:?}", p);
        }
        ClientboundGamePacket::TeleportEntity(p) => {
            let mut world = client.world.write();
            let (pos, physics) = self.query::<(&entity::Position,
    &entity::Physics)>();         let _ = world.set_entity_pos(
                EntityId(p.id),
                Vec3 {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                },
                pos,
                physics,
            );
        }
        ClientboundGamePacket::UpdateAdvancements(p) => {
            debug!("Got update advancements packet {:?}", p);
        }
        ClientboundGamePacket::RotateHead(_p) => {
            // debug!("Got rotate head packet {:?}", p);
        }
        ClientboundGamePacket::MoveEntityPos(p) => {
            let mut local_player = ecs.query::<&mut LocalPlayer>().get_mut(ecs, entity).unwrap();
            let mut partial_entity_infos = local_player.partial_world.write().entity_infos;
            let entity = partial_entity_infos.entity_by_id(p.entity_id);
            let mut position = ecs.query::<&mut Position>().get_mut(ecs, entity).unwrap();
            **position = position.with_delta(&p.delta);
        },
        ClientboundGamePacket::MoveEntityPosRot(p) => {
            let entity_infos = ecs.resource::<EntityInfos>();
            let entity = entity_infos.entity_by_id(p.entity_id);
            let mut position = ecs.query::<&mut Position>().get_mut(ecs, entity).unwrap();
            **position += p.delta;
        }
        ClientboundGamePacket::MoveEntityRot(_p) => {
            // debug!("Got move entity rot packet {:?}", p);
        }
        ClientboundGamePacket::KeepAlive(p) => {
            debug!("Got keep alive packet {:?}", p);
            client
                .write_packet(ServerboundKeepAlivePacket { id: p.id }.get())
                .await?;
        }
        ClientboundGamePacket::RemoveEntities(p) => {
            debug!("Got remove entities packet {:?}", p);
        }
        ClientboundGamePacket::PlayerChat(p) => {
            debug!("Got player chat packet {:?}", p);
            tx.send(Event::Chat(ChatPacket::Player(Arc::new(p.clone()))))
                .await?;
        }
        ClientboundGamePacket::SystemChat(p) => {
            debug!("Got system chat packet {:?}", p);
            tx.send(Event::Chat(ChatPacket::System(Arc::new(p.clone()))))
                .await?;
        }
        ClientboundGamePacket::Sound(_p) => {
            // debug!("Got sound packet {:?}", p);
        }
        ClientboundGamePacket::LevelEvent(p) => {
            debug!("Got level event packet {:?}", p);
        }
        ClientboundGamePacket::BlockUpdate(p) => {
            debug!("Got block update packet {:?}", p);
            let mut world = client.world.write();
            world.set_block_state(&p.pos, p.block_state);
        }
        ClientboundGamePacket::Animate(p) => {
            debug!("Got animate packet {:?}", p);
        }
        ClientboundGamePacket::SectionBlocksUpdate(p) => {
            debug!("Got section blocks update packet {:?}", p);
            let mut world = client.world.write();
            for state in &p.states {
                world.set_block_state(&(p.section_pos + state.pos.clone()),
    state.state);         }
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
            let (entity_id, mut dead, mut local_player) = ecs.query::<(&MinecraftEntityId, &mut Dead, &mut LocalPlayer)>().get(ecs, entity).unwrap();
            if **entity_id == p.player_id {
                if !**dead {
                    **dead = true;
                    local_player.tx.send(Event::Death(Some(Arc::new(p.clone()))));
                }
            }
        }
        ClientboundGamePacket::PlayerLookAt(_) => {}
        ClientboundGamePacket::RemoveMobEffect(_) => {}
        ClientboundGamePacket::ResourcePack(_) => {}
        ClientboundGamePacket::Respawn(p) => {
            debug!("Got respawn packet {:?}", p);
            // Sets clients dead state to false.
            let mut dead = ecs.query::<&mut Dead>().get(ecs, entity).unwrap();
            **dead = false;
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

/// A system that clears all packets in the clientbound packet events.
pub fn clear_packets(mut query: Query<&mut PacketReceiver>) {
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
