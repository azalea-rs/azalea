use crate::{movement::MoveDirection, Account, Player};
use azalea_auth::game_profile::GameProfile;
use azalea_block::BlockState;
use azalea_chat::component::Component;
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{Connection, ConnectionError, ReadConnection, WriteConnection},
    packets::{
        game::{
            clientbound_player_chat_packet::ClientboundPlayerChatPacket,
            clientbound_system_chat_packet::ClientboundSystemChatPacket,
            serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
            serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
            serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
            serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
            ClientboundGamePacket, ServerboundGamePacket,
        },
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::{NonceOrSaltSignature, ServerboundKeyPacket},
            ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    read::ReadPacketError,
    resolver, ServerAddress,
};
use azalea_world::entity::EntityData;
use azalea_world::Dimension;
use log::{debug, error, warn};
use std::{
    fmt::Debug,
    io,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::{self},
};

#[derive(Debug, Clone)]
pub enum Event {
    Login,
    Chat(ChatPacket),
    /// A game tick, happens 20 times per second.
    GameTick,
}

#[derive(Debug, Clone)]
pub enum ChatPacket {
    System(ClientboundSystemChatPacket),
    Player(Box<ClientboundPlayerChatPacket>),
}

impl ChatPacket {
    pub fn message(&self) -> Component {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message.message(false),
        }
    }
}

/// A player that you can control that is currently in a Minecraft server.
#[derive(Clone)]
pub struct Client {
    game_profile: GameProfile,
    pub read_conn: Arc<tokio::sync::Mutex<ReadConnection<ClientboundGamePacket>>>,
    pub write_conn: Arc<tokio::sync::Mutex<WriteConnection<ServerboundGamePacket>>>,
    pub player: Arc<Mutex<Player>>,
    pub dimension: Arc<Mutex<Dimension>>,
    pub physics_state: Arc<Mutex<PhysicsState>>,
}

#[derive(Default)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the player moved enough. This is that tick counter.
    pub position_remainder: u32,

    pub move_direction: MoveDirection,
    pub forward_impulse: f32,
    pub left_impulse: f32,
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = !cfg!(debug_assertions);

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] azalea_protocol::read::ReadPacketError),
    #[error("{0}")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("{0}")]
    Poison(String),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Client {
    /// Connect to a Minecraft server with an account.
    pub async fn join(
        account: &Account,
        address: &ServerAddress,
    ) -> Result<(Self, UnboundedReceiver<Event>), JoinError> {
        let resolved_address = resolver::resolve_address(address).await?;

        let mut conn = Connection::new(&resolved_address).await?;

        // handshake
        conn.write(
            ClientIntentionPacket {
                protocol_version: PROTOCOL_VERSION,
                hostname: address.host.clone(),
                port: address.port,
                intention: ConnectionProtocol::Login,
            }
            .get(),
        )
        .await?;
        let mut conn = conn.login();

        // login
        conn.write(
            ServerboundHelloPacket {
                username: account.username.clone(),
                public_key: None,
                profile_id: None,
            }
            .get(),
        )
        .await?;

        let (conn, game_profile) = loop {
            let packet_result = conn.read().await;
            match packet_result {
                Ok(packet) => match packet {
                    ClientboundLoginPacket::Hello(p) => {
                        debug!("Got encryption request");
                        let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();

                        // TODO: authenticate with the server here (authenticateServer)

                        conn.write(
                            ServerboundKeyPacket {
                                nonce_or_salt_signature: NonceOrSaltSignature::Nonce(
                                    e.encrypted_nonce,
                                ),
                                key_bytes: e.encrypted_public_key,
                            }
                            .get(),
                        )
                        .await?;
                        conn.set_encryption_key(e.secret_key);
                    }
                    ClientboundLoginPacket::LoginCompression(p) => {
                        debug!("Got compression request {:?}", p.compression_threshold);
                        conn.set_compression_threshold(p.compression_threshold);
                    }
                    ClientboundLoginPacket::GameProfile(p) => {
                        debug!("Got profile {:?}", p.game_profile);
                        break (conn.game(), p.game_profile);
                    }
                    ClientboundLoginPacket::LoginDisconnect(p) => {
                        debug!("Got disconnect {:?}", p);
                    }
                    ClientboundLoginPacket::CustomQuery(p) => {
                        debug!("Got custom query {:?}", p);
                    }
                },
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }
        };

        let (read_conn, write_conn) = conn.into_split();

        let read_conn = Arc::new(tokio::sync::Mutex::new(read_conn));
        let write_conn = Arc::new(tokio::sync::Mutex::new(write_conn));

        let (tx, rx) = mpsc::unbounded_channel();

        // we got the GameConnection, so the server is now connected :)
        let client = Client {
            game_profile,
            read_conn,
            write_conn,
            player: Arc::new(Mutex::new(Player::default())),
            dimension: Arc::new(Mutex::new(Dimension::default())),
            physics_state: Arc::new(Mutex::new(PhysicsState::default())),
        };

        // just start up the game loop and we're ready!

        // if you get an error right here that means you're doing something with locks wrong
        // read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix
        tokio::spawn(Self::protocol_loop(client.clone(), tx.clone()));
        tokio::spawn(Self::game_tick_loop(client.clone(), tx));

        Ok((client, rx))
    }

    /// Write a packet directly to the server.
    pub async fn write_packet(&self, packet: ServerboundGamePacket) -> Result<(), std::io::Error> {
        self.write_conn.lock().await.write(packet).await
    }

    async fn protocol_loop(client: Client, tx: UnboundedSender<Event>) {
        loop {
            let r = client.read_conn.lock().await.read().await;
            match r {
                Ok(packet) => match Self::handle(&packet, &client, &tx).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error handling packet: {}", e);
                        if IGNORE_ERRORS {
                            continue;
                        } else {
                            panic!("Error handling packet: {}", e);
                        }
                    }
                },
                Err(e) => {
                    if IGNORE_ERRORS {
                        warn!("{}", e);
                        match e {
                            ReadPacketError::FrameSplitter { .. } => panic!("Error: {:?}", e),
                            _ => continue,
                        }
                    } else {
                        panic!("{}", e);
                    }
                }
            };
        }
    }

    async fn handle(
        packet: &ClientboundGamePacket,
        client: &Client,
        tx: &UnboundedSender<Event>,
    ) -> Result<(), HandleError> {
        match packet {
            ClientboundGamePacket::Login(p) => {
                debug!("Got login packet {:?}", p);

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

                    let mut dimension_lock = client.dimension.lock().unwrap();
                    // the 16 here is our render distance
                    // i'll make this an actual setting later
                    *dimension_lock = Dimension::new(16, height, min_y);

                    let entity = EntityData::new(client.game_profile.uuid, Vec3::default());
                    dimension_lock.add_entity(p.player_id, entity);

                    let mut player_lock = client.player.lock().unwrap();

                    player_lock.set_entity_id(p.player_id);
                }

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

                tx.send(Event::Login).unwrap();
            }
            ClientboundGamePacket::UpdateViewDistance(p) => {
                debug!("Got view distance packet {:?}", p);
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
                    let player_entity_id = {
                        let player_lock = client.player.lock().unwrap();
                        player_lock.entity_id
                    };

                    let mut dimension_lock = client.dimension.lock().unwrap();

                    let mut player_entity = dimension_lock
                        .entity_mut(player_entity_id)
                        .expect("Player entity doesn't exist");

                    let delta_movement = player_entity.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        player_entity.last_pos.x += p.x;
                        (delta_movement.x, player_entity.pos().x + p.x)
                    } else {
                        player_entity.last_pos.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        player_entity.last_pos.y += p.y;
                        (delta_movement.y, player_entity.pos().y + p.y)
                    } else {
                        player_entity.last_pos.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        player_entity.last_pos.z += p.z;
                        (delta_movement.z, player_entity.pos().z + p.z)
                    } else {
                        player_entity.last_pos.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        y_rot += player_entity.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        x_rot += player_entity.y_rot;
                    }

                    player_entity.delta = Vec3 {
                        x: delta_x,
                        y: delta_y,
                        z: delta_z,
                    };
                    player_entity.set_rotation(y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };
                    dimension_lock
                        .set_entity_pos(player_entity_id, new_pos)
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
            ClientboundGamePacket::PlayerInfo(p) => {
                debug!("Got player info packet {:?}", p);
            }
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                debug!("Got chunk cache center packet {:?}", p);
                client
                    .dimension
                    .lock()?
                    .update_view_center(&ChunkPos::new(p.x, p.z));
            }
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                debug!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);
                // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
                // debug("chunk {:?}")
                client
                    .dimension
                    .lock()?
                    .replace_with_packet_data(&pos, &mut p.chunk_data.data.as_slice())
                    .unwrap();
            }
            ClientboundGamePacket::LightUpdate(p) => {
                debug!("Got light update packet {:?}", p);
            }
            ClientboundGamePacket::AddEntity(p) => {
                debug!("Got add entity packet {:?}", p);
                let entity = EntityData::from(p);
                client.dimension.lock()?.add_entity(p.id, entity);
            }
            ClientboundGamePacket::SetEntityData(_p) => {
                // debug!("Got set entity data packet {:?}", p);
            }
            ClientboundGamePacket::UpdateAttributes(_p) => {
                // debug!("Got update attributes packet {:?}", p);
            }
            ClientboundGamePacket::EntityVelocity(_p) => {
                // debug!("Got entity velocity packet {:?}", p);
            }
            ClientboundGamePacket::SetEntityLink(p) => {
                debug!("Got set entity link packet {:?}", p);
            }
            ClientboundGamePacket::AddPlayer(p) => {
                debug!("Got add player packet {:?}", p);
                let entity = EntityData::from(p);
                client.dimension.lock()?.add_entity(p.id, entity);
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
            }
            ClientboundGamePacket::SetExperience(p) => {
                debug!("Got set experience packet {:?}", p);
            }
            ClientboundGamePacket::TeleportEntity(p) => {
                let mut dimension_lock = client.dimension.lock()?;

                dimension_lock
                    .set_entity_pos(
                        p.id,
                        Vec3 {
                            x: p.x,
                            y: p.y,
                            z: p.z,
                        },
                    )
                    .map_err(|e| HandleError::Other(e.into()))?;
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {:?}", p);
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {:?}", p);
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut dimension_lock = client.dimension.lock()?;

                dimension_lock
                    .move_entity_with_delta(p.entity_id, &p.delta)
                    .map_err(|e| HandleError::Other(e.into()))?;
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut dimension_lock = client.dimension.lock()?;

                dimension_lock
                    .move_entity_with_delta(p.entity_id, &p.delta)
                    .map_err(|e| HandleError::Other(e.into()))?;
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
                // debug!("Got player chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::Player(Box::new(p.clone()))))
                    .unwrap();
            }
            ClientboundGamePacket::SystemChat(p) => {
                debug!("Got system chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::System(p.clone()))).unwrap();
            }
            ClientboundGamePacket::Sound(p) => {
                debug!("Got sound packet {:?}", p);
            }
            ClientboundGamePacket::LevelEvent(p) => {
                debug!("Got level event packet {:?}", p);
            }
            ClientboundGamePacket::BlockUpdate(p) => {
                debug!("Got block update packet {:?}", p);
                // TODO: update world
                let mut dimension = client.dimension.lock()?;
                // dimension.get_block_state(pos)
                if let Ok(block_state) = BlockState::try_from(p.block_state) {
                    dimension.set_block_state(&p.pos, block_state);
                } else {
                    warn!(
                        "Non-existent block state for block update packet {:?}",
                        p.block_state
                    );
                }
            }
            ClientboundGamePacket::Animate(p) => {
                debug!("Got animate packet {:?}", p);
            }
            ClientboundGamePacket::SectionBlocksUpdate(p) => {
                debug!("Got section blocks update packet {:?}", p);
                // TODO: update world
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
            ClientboundGamePacket::ChatPreview(_) => {}
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetData(_) => {}
            ClientboundGamePacket::ContainerSetSlot(_) => {}
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
            ClientboundGamePacket::CustomSound(_) => {}
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
            ClientboundGamePacket::PlayerChatHeader(_) => {}
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(_) => {}
            ClientboundGamePacket::PlayerLookAt(_) => {}
            ClientboundGamePacket::RemoveMobEffect(_) => {}
            ClientboundGamePacket::ResourcePack(_) => {}
            ClientboundGamePacket::Respawn(_) => {}
            ClientboundGamePacket::SelectAdvancementsTab(_) => {}
            ClientboundGamePacket::SetActionBarText(_) => {}
            ClientboundGamePacket::SetBorderCenter(_) => {}
            ClientboundGamePacket::SetBorderLerpSize(_) => {}
            ClientboundGamePacket::SetBorderSize(_) => {}
            ClientboundGamePacket::SetBorderWarningDelay(_) => {}
            ClientboundGamePacket::SetBorderWarningDistance(_) => {}
            ClientboundGamePacket::SetCamera(_) => {}
            ClientboundGamePacket::SetChunkCacheRadius(_) => {}
            ClientboundGamePacket::SetDisplayChatPreview(_) => {}
            ClientboundGamePacket::SetDisplayObjective(_) => {}
            ClientboundGamePacket::SetEntityMotion(_) => {}
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
        }

        Ok(())
    }

    /// Runs game_tick every 50 milliseconds.
    async fn game_tick_loop(mut client: Client, tx: UnboundedSender<Event>) {
        let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
        loop {
            game_tick_interval.tick().await;
            Self::game_tick(&mut client, &tx).await;
        }
    }

    /// Runs every 50 milliseconds.
    async fn game_tick(client: &mut Client, tx: &UnboundedSender<Event>) {
        // return if there's no chunk at the player's position
        {
            let dimension_lock = client.dimension.lock().unwrap();
            let player_lock = client.player.lock().unwrap();
            let player_entity = player_lock.entity(&dimension_lock);
            let player_entity = if let Some(player_entity) = player_entity {
                player_entity
            } else {
                return;
            };
            let player_chunk_pos: ChunkPos = player_entity.pos().into();
            if dimension_lock[&player_chunk_pos].is_none() {
                return;
            }
        }

        // TODO: if we're a passenger, send the required packets

        if let Err(e) = client.send_position().await {
            warn!("Error sending position: {:?}", e);
        }
        client.ai_step();

        // TODO: minecraft does ambient sounds here

        tx.send(Event::GameTick).unwrap();
    }
}

impl<T> From<std::sync::PoisonError<T>> for HandleError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandleError::Poison(e.to_string())
    }
}
