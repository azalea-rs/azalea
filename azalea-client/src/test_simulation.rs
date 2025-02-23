use std::{fmt::Debug, sync::Arc, time::Duration};

use azalea_auth::game_profile::GameProfile;
use azalea_buf::AzaleaWrite;
use azalea_core::game_type::{GameMode, OptionalGameType};
use azalea_core::position::ChunkPos;
use azalea_core::resource_location::ResourceLocation;
use azalea_core::tick::GameTick;
use azalea_entity::metadata::PlayerMetadataBundle;
use azalea_protocol::packets::common::CommonPlayerSpawnInfo;
use azalea_protocol::packets::game::c_level_chunk_with_light::ClientboundLevelChunkPacketData;
use azalea_protocol::packets::game::c_light_update::ClientboundLightUpdatePacketData;
use azalea_protocol::packets::game::{
    ClientboundLevelChunkWithLight, ClientboundLogin, ClientboundRespawn,
};
use azalea_protocol::packets::{ConnectionProtocol, Packet, ProtocolPacket};
use azalea_registry::DimensionType;
use azalea_world::palette::{PalettedContainer, PalettedContainerKind};
use azalea_world::{Chunk, Instance, MinecraftEntityId, Section};
use bevy_app::App;
use bevy_ecs::{prelude::*, schedule::ExecutorKind};
use parking_lot::{Mutex, RwLock};
use simdnbt::owned::Nbt;
use tokio::{sync::mpsc, time::sleep};
use uuid::Uuid;

use crate::{
    ClientInformation, GameProfileComponent, InConfigState, InstanceHolder, LocalPlayerBundle,
    events::LocalPlayerEvents,
    raw_connection::{RawConnection, RawConnectionReader, RawConnectionWriter},
};

/// A way to simulate a client in a server, used for some internal tests.
pub struct Simulation {
    pub app: App,
    pub entity: Entity,

    // the runtime needs to be kept around for the tasks to be considered alive
    pub rt: tokio::runtime::Runtime,

    pub incoming_packet_queue: Arc<Mutex<Vec<Box<[u8]>>>>,
    pub outgoing_packets_receiver: mpsc::UnboundedReceiver<Box<[u8]>>,
}

impl Simulation {
    pub fn new(initial_connection_protocol: ConnectionProtocol) -> Self {
        let mut app = create_simulation_app();
        let mut entity = app.world_mut().spawn_empty();
        let (player, outgoing_packets_receiver, incoming_packet_queue, rt) =
            create_local_player_bundle(entity.id(), initial_connection_protocol);
        entity.insert(player);

        let entity = entity.id();

        tick_app(&mut app);

        #[allow(clippy::single_match)]
        match initial_connection_protocol {
            ConnectionProtocol::Configuration => {
                app.world_mut().entity_mut(entity).insert(InConfigState);
                tick_app(&mut app);
            }
            _ => {}
        }

        Self {
            app,
            entity,
            rt,
            incoming_packet_queue,
            outgoing_packets_receiver,
        }
    }

    pub fn receive_packet<P: ProtocolPacket + Debug>(&mut self, packet: impl Packet<P>) {
        let buf = azalea_protocol::write::serialize_packet(&packet.into_variant()).unwrap();
        self.incoming_packet_queue.lock().push(buf);
    }

    pub fn tick(&mut self) {
        tick_app(&mut self.app);
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

    pub fn chunk(&self, chunk_pos: ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.component::<InstanceHolder>()
            .instance
            .read()
            .chunks
            .get(&chunk_pos)
    }
}

#[allow(clippy::type_complexity)]
fn create_local_player_bundle(
    entity: Entity,
    connection_protocol: ConnectionProtocol,
) -> (
    LocalPlayerBundle,
    mpsc::UnboundedReceiver<Box<[u8]>>,
    Arc<Mutex<Vec<Box<[u8]>>>>,
    tokio::runtime::Runtime,
) {
    // unused since we'll trigger ticks ourselves
    let (run_schedule_sender, _run_schedule_receiver) = tokio::sync::mpsc::unbounded_channel();

    let (outgoing_packets_sender, outgoing_packets_receiver) = mpsc::unbounded_channel();
    let incoming_packet_queue = Arc::new(Mutex::new(Vec::new()));
    let reader = RawConnectionReader {
        incoming_packet_queue: incoming_packet_queue.clone(),
        run_schedule_sender,
    };
    let writer = RawConnectionWriter {
        outgoing_packets_sender,
    };

    let rt = tokio::runtime::Runtime::new().unwrap();

    // the tasks can't die since that would make us send a DisconnectEvent
    let read_packets_task = rt.spawn(async {
        loop {
            sleep(Duration::from_secs(60)).await;
        }
    });
    let write_packets_task = rt.spawn(async {
        loop {
            sleep(Duration::from_secs(60)).await;
        }
    });

    let raw_connection = RawConnection {
        reader,
        writer,
        read_packets_task,
        write_packets_task,
        connection_protocol,
    };

    let (local_player_events_sender, _local_player_events_receiver) = mpsc::unbounded_channel();

    let instance = Instance::default();
    let instance_holder = InstanceHolder::new(entity, Arc::new(RwLock::new(instance)));

    let local_player_bundle = LocalPlayerBundle {
        raw_connection,
        local_player_events: LocalPlayerEvents(local_player_events_sender),
        game_profile: GameProfileComponent(GameProfile::new(Uuid::nil(), "azalea".to_owned())),
        client_information: ClientInformation::default(),
        instance_holder,
        metadata: PlayerMetadataBundle::default(),
    };

    (
        local_player_bundle,
        outgoing_packets_receiver,
        incoming_packet_queue,
        rt,
    )
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
            states: PalettedContainer::new(PalettedContainerKind::BlockStates),
            biomes: PalettedContainer::new(PalettedContainerKind::Biomes),
        });
    }
    sections.azalea_write(&mut chunk_bytes).unwrap();

    ClientboundLevelChunkWithLight {
        x: pos.x,
        z: pos.z,
        chunk_data: ClientboundLevelChunkPacketData {
            heightmaps: Nbt::None,
            data: chunk_bytes.into(),
            block_entities: vec![],
        },
        light_data: ClientboundLightUpdatePacketData::default(),
    }
}
