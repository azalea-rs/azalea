use std::{collections::HashMap, io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{Connection, ReadConnection, WriteConnection},
    packets::game::{
        serverbound_keep_alive_packet::ServerboundKeepAlivePacket, ClientboundGamePacket,
        ServerboundGamePacket,
    },
};
use azalea_world::{
    entity::{self, metadata::PlayerMetadataBundle, EntityId},
    PartialWorld, WeakWorldContainer,
};
use bevy_ecs::{
    component::Component,
    event::EventReader,
    system::{Query, Res, ResMut},
};
use log::debug;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{ChatPacket, ClientInformation, Event, PlayerInfo, WalkDirection};

/// A player that you control that is currently in a Minecraft server.
#[derive(Component)]
pub struct LocalPlayer {
    pub profile: GameProfile,
    // Arc<tokio::sync::Mutex<
    pub read_conn: ReadConnection<ClientboundGamePacket>,
    pub write_conn: WriteConnection<ServerboundGamePacket>,
    // pub world: Arc<RwLock<PartialWorld>>,
    pub physics_state: PhysicsState,
    pub client_information: ClientInformation,
    pub dead: bool,
    /// A map of player uuids to their information in the tab list
    pub players: HashMap<Uuid, PlayerInfo>,

    pub world: Arc<RwLock<PartialWorld>>,
    pub world_name: Option<ResourceLocation>,

    pub tx: mpsc::Sender<Event>,
}

#[derive(Default)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    pub move_direction: WalkDirection,
    pub forward_impulse: f32,
    pub left_impulse: f32,
}

/// Marks a [`LocalPlayer`] that's in a loaded chunk. This is updated at the
/// beginning of every tick.
#[derive(Component)]
pub struct LocalPlayerInLoadedChunk;

impl LocalPlayer {
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        conn: Connection<ClientboundGamePacket, ServerboundGamePacket>,
        world: Arc<RwLock<PartialWorld>>,
        tx: mpsc::Sender<Event>,
    ) -> Self {
        let (read_conn, write_conn) = conn.into_split();
        let (read_conn, write_conn) = (
            // Arc::new(tokio::sync::Mutex::new(read_conn)),
            // Arc::new(tokio::sync::Mutex::new(write_conn)),
            read_conn, write_conn,
        );

        LocalPlayer {
            profile,
            read_conn,
            write_conn,
            physics_state: PhysicsState::default(),
            client_information: ClientInformation::default(),
            dead: false,
            players: HashMap::new(),
            world,
            tx,
            world_name: None,
        }
    }

    /// Write a packet directly to the server.
    pub async fn write_packet_async(
        &mut self,
        packet: ServerboundGamePacket,
    ) -> Result<(), std::io::Error> {
        self.write_conn.write(packet).await?;
        Ok(())
    }

    /// Spawn a task to write a packet directly to the server.
    pub fn write_packet(&mut self, packet: ServerboundGamePacket) {
        tokio::spawn(self.write_packet_async(packet));
    }

    /// Update the [`LocalPlayerInLoadedChunk`] component for all
    /// [`LocalPlayer`]s.
    fn update_in_loaded_chunk(
        mut commands: bevy_ecs::system::Commands,
        query: Query<(entity::EcsEntityId, &LocalPlayer, &entity::Position)>,
    ) {
        for (ecs_entity_id, local_player, position) in &query {
            let player_chunk_pos = ChunkPos::from(position);
            let in_loaded_chunk = local_player
                .world
                .read()
                .get_chunk(&player_chunk_pos)
                .is_some();
            if in_loaded_chunk {
                commands
                    .entity(ecs_entity_id)
                    .insert(LocalPlayerInLoadedChunk);
            } else {
                commands
                    .entity(ecs_entity_id)
                    .remove::<LocalPlayerInLoadedChunk>();
            }
        }
    }

    pub(crate) fn send_event(event: Event, tx: &mpsc::Sender<Event>) {
        tokio::spawn(tx.send(event));
    }

    fn send_tick_event(query: Query<&LocalPlayer>) {
        for local_player in &query {
            let tx = local_player.tx.clone();
            Self::send_event(Event::Tick, &tx);
        }
    }

    pub fn handle_packet(
        mut event_reader: EventReader<(EntityId, ClientboundGamePacket)>,
        query: Query<(&mut LocalPlayer,)>,
        world_container: ResMut<WeakWorldContainer>,
        // client: &Client,
        // tx: &mpsc::Sender<Event>,
    ) -> Result<(), HandlePacketError> {
        for (player_entity_id, packet) in event_reader.iter() {
            let (mut local_player,) = query.get_mut((*player_entity_id).into()).unwrap();

            match &packet {
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
                        // add this world to the world_container (or don't if it's already there)
                        let weak_world = world_container.insert(world_name, height, min_y);
                        // set the loaded_world to an empty world
                        // (when we add chunks or entities those will be in the world_container)
                        let mut world_lock = local_player.world.write();
                        *world_lock = PartialWorld::new(
                            local_player.client_information.view_distance.into(),
                            weak_world,
                            Some(EntityId(p.player_id)),
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
                        // the first argument makes it so other entities don't update this entity in
                        // a shared world
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
                    client.write_packet(client_information_packet.get()).await?;

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
                        // let mut player_entity = world.entity_mut(player_entity_id).unwrap();
                        let (mut physics, position) =
                            client.query::<(&mut entity::Physics, &mut entity::Position)>();

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
                        // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                        // so investigate that ig
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
                        .write_packet(ServerboundAcceptTeleportationPacket { id: p.id }.get())
                        .await?;
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
                                players_lock.insert(updated_info.profile.uuid, player_info.clone());
                                events.push(Event::AddPlayer(player_info));
                            } else if let Some(info) =
                                players_lock.get_mut(&updated_info.profile.uuid)
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
                                events.push(Event::UpdatePlayer(info.clone()));
                            } else {
                                warn!(
                                    "Ignoring PlayerInfoUpdate for unknown player {}",
                                    updated_info.profile.uuid
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
                    let shared_has_chunk = client.world.read().get_chunk(&pos).is_some();
                    let this_client_has_chunk =
                        client.world.read().chunks.limited_get(&pos).is_some();
                    if shared_has_chunk && !this_client_has_chunk {
                        trace!(
                            "Skipping parsing chunk {:?} because we already know about it",
                            pos
                        );
                        return Ok(());
                    }

                    // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
                    // debug("chunk {:?}")
                    if let Err(e) = client
                        .world
                        .write()
                        .replace_with_packet_data(&pos, &mut Cursor::new(&p.chunk_data.data))
                    {
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
                    // the bundle doesn't include the default entity metadata so we add that
                    // separately
                    let mut entities = world.entity_infos.shared.write();
                    let mut entity = entities.ecs_entity_mut(EntityId(p.id)).unwrap();
                    p.apply_metadata(&mut entity);
                }
                ClientboundGamePacket::SetEntityData(p) => {
                    debug!("Got set entity data packet {:?}", p);
                    let world = client.world.write();
                    let mut entities = world.entity_infos.shared.write();
                    let entity = entities.ecs_entity_mut(EntityId(p.id));
                    if let Some(mut entity) = entity {
                        entity::metadata::apply_metadata(&mut entity, p.packed_items.0.clone());
                    } else {
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
                    let (pos, physics) = self.query::<(&entity::Position, &entity::Physics)>();
                    let _ = world.set_entity_pos(
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
                    let mut world = client.world.write();
                    let _ = world.move_entity_with_delta(EntityId(p.entity_id), &p.delta);
                }
                ClientboundGamePacket::MoveEntityPosRot(p) => {
                    let mut world = client.world.write();
                    let _ = world.move_entity_with_delta(EntityId(p.entity_id), &p.delta);
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
                        world.set_block_state(&(p.section_pos + state.pos.clone()), state.state);
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
                    if client.entity() == EntityId(p.player_id) {
                        // we can't define a variable here with client.dead.lock()
                        // because of https://github.com/rust-lang/rust/issues/57478
                        if !*client.dead.lock() {
                            *client.dead.lock() = true;
                            tx.send(Event::Death(Some(Arc::new(p.clone())))).await?;
                        }
                    }
                }
                ClientboundGamePacket::PlayerLookAt(_) => {}
                ClientboundGamePacket::RemoveMobEffect(_) => {}
                ClientboundGamePacket::ResourcePack(_) => {}
                ClientboundGamePacket::Respawn(p) => {
                    debug!("Got respawn packet {:?}", p);
                    // Sets clients dead state to false.
                    let mut dead_lock = client.dead.lock();
                    *dead_lock = false;
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
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum HandlePacketError {
    #[error("{0}")]
    Poison(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Send(#[from] mpsc::error::SendError<Event>),
}

impl<T> From<std::sync::PoisonError<T>> for HandlePacketError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandlePacketError::Poison(e.to_string())
    }
}
