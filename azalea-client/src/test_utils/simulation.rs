use std::{collections::VecDeque, fmt::Debug, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_block::BlockState;
use azalea_buf::AzaleaWrite;
use azalea_core::{
    delta::PositionDelta8,
    game_type::{GameMode, OptionalGameType},
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
    tick::GameTick,
};
use azalea_entity::metadata::PlayerMetadataBundle;
use azalea_protocol::{
    common::client_information::ClientInformation,
    packets::{
        ConnectionProtocol, Packet, ProtocolPacket,
        common::CommonPlayerSpawnInfo,
        config::{ClientboundFinishConfiguration, ClientboundRegistryData},
        game::{
            ClientboundAddEntity, ClientboundLevelChunkWithLight, ClientboundLogin,
            ClientboundRespawn, ServerboundGamePacket,
            c_level_chunk_with_light::ClientboundLevelChunkPacketData,
            c_light_update::ClientboundLightUpdatePacketData,
        },
    },
};
use azalea_registry::{Biome, DataRegistry, DimensionType, EntityKind};
use azalea_world::{Chunk, Instance, MinecraftEntityId, Section, palette::PalettedContainer};
use bevy_app::App;
use bevy_ecs::{component::Mutable, prelude::*, schedule::ExecutorKind};
use parking_lot::{Mutex, RwLock};
use simdnbt::owned::{NbtCompound, NbtTag};
use uuid::Uuid;

use crate::{
    InConfigState, LocalPlayerBundle, connection::RawConnection, disconnect::DisconnectEvent,
    local_player::InstanceHolder, packet::game::SendGamePacketEvent, player::GameProfileComponent,
};

/// A way to simulate a client in a server, used for some internal tests.
pub struct Simulation {
    pub app: App,
    pub entity: Entity,

    // the runtime needs to be kept around for the tasks to be considered alive
    pub rt: tokio::runtime::Runtime,
}

impl Simulation {
    pub fn new(initial_connection_protocol: ConnectionProtocol) -> Self {
        let mut app = create_simulation_app();
        let mut entity = app.world_mut().spawn_empty();
        let (player, rt) =
            create_local_player_bundle(entity.id(), ConnectionProtocol::Configuration);
        entity.insert((player, ClientInformation::default()));

        let entity = entity.id();

        tick_app(&mut app);

        // start in the config state
        app.world_mut().entity_mut(entity).insert((
            InConfigState,
            GameProfileComponent(GameProfile::new(
                Uuid::from_u128(1234),
                "azalea".to_string(),
            )),
        ));
        tick_app(&mut app);

        let mut simulation = Self { app, entity, rt };

        #[allow(clippy::single_match)]
        match initial_connection_protocol {
            ConnectionProtocol::Configuration => {}
            ConnectionProtocol::Game => {
                simulation.receive_packet(ClientboundRegistryData {
                    registry_id: ResourceLocation::new("minecraft:dimension_type"),
                    entries: vec![(
                        ResourceLocation::new("minecraft:overworld"),
                        Some(NbtCompound::from_values(vec![
                            ("height".into(), NbtTag::Int(384)),
                            ("min_y".into(), NbtTag::Int(-64)),
                        ])),
                    )]
                    .into_iter()
                    .collect(),
                });

                simulation.receive_packet(ClientboundFinishConfiguration);
                simulation.tick();
            }
            _ => unimplemented!("unsupported ConnectionProtocol {initial_connection_protocol:?}"),
        }

        simulation
    }

    pub fn receive_packet<P: ProtocolPacket + Debug>(&mut self, packet: impl Packet<P>) {
        let buf = azalea_protocol::write::serialize_packet(&packet.into_variant()).unwrap();
        self.with_component_mut::<RawConnection>(|raw_conn| {
            raw_conn.injected_clientbound_packets.push(buf);
        });
    }
    pub fn write_message(&mut self, message: impl Message) {
        self.app.world_mut().write_message(message);
    }
    pub fn trigger<'a>(&mut self, event: impl Event<Trigger<'a>: Default>) {
        self.app.world_mut().trigger(event);
    }

    pub fn tick(&mut self) {
        tick_app(&mut self.app);
    }
    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn minecraft_entity_id(&self) -> MinecraftEntityId {
        self.component::<MinecraftEntityId>()
    }

    pub fn component<T: Component + Clone>(&self) -> T {
        self.app.world().get::<T>(self.entity).unwrap().clone()
    }
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.app.world().get::<T>(self.entity).cloned()
    }
    pub fn has_component<T: Component>(&self) -> bool {
        self.app.world().get::<T>(self.entity).is_some()
    }
    pub fn with_component<T: Component>(&self, f: impl FnOnce(&T)) {
        f(self.app.world().entity(self.entity).get::<T>().unwrap());
    }
    pub fn with_component_mut<T: Component<Mutability = Mutable>>(
        &mut self,
        f: impl FnOnce(&mut T),
    ) {
        f(&mut self
            .app
            .world_mut()
            .entity_mut(self.entity)
            .get_mut::<T>()
            .unwrap());
    }
    pub fn resource<T: Resource + Clone>(&self) -> T {
        self.app.world().get_resource::<T>().unwrap().clone()
    }
    pub fn with_resource<T: Resource>(&self, f: impl FnOnce(&T)) {
        f(self.app.world().get_resource::<T>().unwrap());
    }
    pub fn with_resource_mut<T: Resource>(&mut self, f: impl FnOnce(Mut<T>)) {
        f(self.app.world_mut().get_resource_mut::<T>().unwrap());
    }

    pub fn chunk(&self, chunk_pos: ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.component::<InstanceHolder>()
            .instance
            .read()
            .chunks
            .get(&chunk_pos)
    }
    pub fn get_block_state(&self, pos: BlockPos) -> Option<BlockState> {
        self.component::<InstanceHolder>()
            .instance
            .read()
            .get_block_state(pos)
    }

    pub fn disconnect(&mut self) {
        // send DisconnectEvent
        self.app.world_mut().write_message(DisconnectEvent {
            entity: self.entity,
            reason: None,
        });
    }
}

#[derive(Clone)]
pub struct SentPackets {
    pub list: Arc<Mutex<VecDeque<ServerboundGamePacket>>>,
}
impl SentPackets {
    pub fn new(simulation: &mut Simulation) -> Self {
        let sent_packets = SentPackets {
            list: Default::default(),
        };

        let simulation_entity = simulation.entity;
        let sent_packets_clone = sent_packets.clone();
        simulation
            .app
            .add_observer(move |send_game_packet: On<SendGamePacketEvent>| {
                if send_game_packet.sent_by == simulation_entity {
                    sent_packets_clone
                        .list
                        .lock()
                        .push_back(send_game_packet.packet.clone())
                }
            });

        sent_packets
    }

    pub fn clear(&self) {
        self.list.lock().clear();
    }

    pub fn expect_tick_end(&self) {
        self.expect("TickEnd", |p| {
            matches!(p, ServerboundGamePacket::ClientTickEnd(_))
        });
    }
    pub fn expect_pong(&self, id: u32) {
        self.expect(
            &format!("Ping {{ id: {id} }}"),
            |p| matches!(p, ServerboundGamePacket::Pong(pong) if pong.id == id),
        );
    }
    pub fn expect_empty(&self) {
        let sent_packet = self.next();
        if sent_packet.is_some() {
            panic!("Expected no packet, got {sent_packet:?}");
        }
    }
    pub fn expect(
        &self,
        expected_formatted: &str,
        check: impl FnOnce(&ServerboundGamePacket) -> bool,
    ) {
        let sent_packet = self.next();
        if let Some(sent_packet) = sent_packet {
            if !check(&sent_packet) {
                panic!("Expected {expected_formatted}, got {sent_packet:?}");
            }
        } else {
            panic!("Expected {expected_formatted}, got nothing");
        }
    }

    pub fn maybe_expect(&self, check: impl FnOnce(&ServerboundGamePacket) -> bool) {
        let sent_packet = self.peek();
        if let Some(sent_packet) = sent_packet
            && check(&sent_packet)
        {
            self.next();
        }
    }

    pub fn next(&self) -> Option<ServerboundGamePacket> {
        self.list.lock().pop_front()
    }
    pub fn peek(&self) -> Option<ServerboundGamePacket> {
        self.list.lock().front().cloned()
    }
}

#[allow(clippy::type_complexity)]
fn create_local_player_bundle(
    entity: Entity,
    connection_protocol: ConnectionProtocol,
) -> (LocalPlayerBundle, tokio::runtime::Runtime) {
    // unused since we'll trigger ticks ourselves

    let rt = tokio::runtime::Runtime::new().unwrap();

    let raw_connection = RawConnection::new_networkless(connection_protocol);

    let instance = Instance::default();
    let instance_holder = InstanceHolder::new(entity, Arc::new(RwLock::new(instance)));

    let local_player_bundle = LocalPlayerBundle {
        raw_connection,
        instance_holder,
        metadata: PlayerMetadataBundle::default(),
    };

    (local_player_bundle, rt)
}

fn create_simulation_app() -> App {
    let mut app = App::new();

    #[cfg(feature = "log")]
    app.add_plugins(
        bevy_app::PluginGroup::build(crate::DefaultPlugins).disable::<bevy_log::LogPlugin>(),
    );

    app.edit_schedule(bevy_app::Main, |schedule| {
        // makes test results more reproducible
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    });
    app
}

fn tick_app(app: &mut App) {
    app.update();
    app.world_mut().run_schedule(GameTick);
}

pub fn default_login_packet() -> ClientboundLogin {
    make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("minecraft:overworld"),
    )
}

pub fn make_basic_login_packet(
    dimension_type: DimensionType,
    dimension: ResourceLocation,
) -> ClientboundLogin {
    ClientboundLogin {
        player_id: MinecraftEntityId(0),
        hardcore: false,
        levels: vec![],
        max_players: 20,
        chunk_radius: 8,
        simulation_distance: 8,
        reduced_debug_info: false,
        show_death_screen: true,
        do_limited_crafting: false,
        common: CommonPlayerSpawnInfo {
            dimension_type,
            dimension,
            seed: 0,
            game_type: GameMode::Survival,
            previous_game_type: OptionalGameType(None),
            is_debug: false,
            is_flat: false,
            last_death_location: None,
            portal_cooldown: 0,
            sea_level: 63,
        },
        enforces_secure_chat: false,
    }
}

pub fn make_basic_respawn_packet(
    dimension_type: DimensionType,
    dimension: ResourceLocation,
) -> ClientboundRespawn {
    ClientboundRespawn {
        common: CommonPlayerSpawnInfo {
            dimension_type,
            dimension,
            seed: 0,
            game_type: GameMode::Survival,
            previous_game_type: OptionalGameType(None),
            is_debug: false,
            is_flat: false,
            last_death_location: None,
            portal_cooldown: 0,
            sea_level: 63,
        },
        data_to_keep: 0,
    }
}

pub fn make_basic_empty_chunk(
    pos: ChunkPos,
    section_count: usize,
) -> ClientboundLevelChunkWithLight {
    let mut chunk_bytes = Vec::new();
    let mut sections = Vec::new();
    for _ in 0..section_count {
        sections.push(Section {
            block_count: 0,
            states: PalettedContainer::<BlockState>::new(),
            biomes: PalettedContainer::<Biome>::new(),
        });
    }
    sections.azalea_write(&mut chunk_bytes).unwrap();

    ClientboundLevelChunkWithLight {
        x: pos.x,
        z: pos.z,
        chunk_data: ClientboundLevelChunkPacketData {
            heightmaps: Default::default(),
            data: Arc::new(chunk_bytes.into()),
            block_entities: vec![],
        },
        light_data: ClientboundLightUpdatePacketData::default(),
    }
}

pub fn make_basic_add_entity(
    entity_type: EntityKind,
    id: i32,
    position: impl Into<Vec3>,
) -> ClientboundAddEntity {
    ClientboundAddEntity {
        id: id.into(),
        uuid: Uuid::from_u128(1234),
        entity_type,
        position: position.into(),
        x_rot: 0,
        y_rot: 0,
        y_head_rot: 0,
        data: 0,
        velocity: PositionDelta8::default(),
    }
}
