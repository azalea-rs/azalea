use std::{fmt::Debug, sync::Arc, time::Duration};

use azalea_auth::game_profile::GameProfile;
use azalea_client::{
    events::LocalPlayerEvents,
    raw_connection::{RawConnection, RawConnectionReader, RawConnectionWriter},
    ClientInformation, GameProfileComponent, InConfigState, InstanceHolder, LocalPlayerBundle,
};
use azalea_core::{
    game_type::{GameMode, OptionalGameType},
    position::Vec3,
    resource_location::ResourceLocation,
    tick::GameTick,
};
use azalea_entity::{metadata::Health, LocalEntity, Position};
use azalea_protocol::packets::{
    common::CommonPlayerSpawnInfo,
    config::ClientboundFinishConfiguration,
    game::{ClientboundLogin, ClientboundSetHealth},
    ConnectionProtocol, Packet, ProtocolPacket,
};
use azalea_registry::DimensionType;
use azalea_world::Instance;
use bevy_app::App;
use bevy_app::PluginGroup;
use bevy_ecs::{prelude::*, schedule::ExecutorKind};
use bevy_log::{tracing_subscriber, LogPlugin};
use parking_lot::{Mutex, RwLock};
use tokio::{sync::mpsc, time::sleep};
use uuid::Uuid;

#[test]
fn test_set_health_before_login() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());

    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    assert!(!simulation.has_component::<InConfigState>());
    assert!(simulation.has_component::<LocalEntity>());

    simulation.receive_packet(ClientboundSetHealth {
        health: 15.,
        food: 20,
        saturation: 20.,
    });
    simulation.tick();
    assert_eq!(*simulation.component::<Health>(), 15.);

    simulation.receive_packet(ClientboundLogin {
        player_id: 0,
        hardcore: false,
        levels: vec![],
        max_players: 20,
        chunk_radius: 8,
        simulation_distance: 8,
        reduced_debug_info: false,
        show_death_screen: true,
        do_limited_crafting: false,
        common: CommonPlayerSpawnInfo {
            dimension_type: DimensionType::Overworld,
            dimension: ResourceLocation::new("overworld"),
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
    });
    simulation.tick();

    // health should stay the same
    assert_eq!(*simulation.component::<Health>(), 15.);
}

pub fn create_local_player_bundle(
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

    let (local_player_events_sender, local_player_events_receiver) = mpsc::unbounded_channel();

    let instance = Instance::default();
    let instance_holder = InstanceHolder::new(entity, Arc::new(RwLock::new(instance)));

    let local_player_bundle = LocalPlayerBundle {
        raw_connection,
        local_player_events: LocalPlayerEvents(local_player_events_sender),
        game_profile: GameProfileComponent(GameProfile::new(Uuid::nil(), "azalea".to_owned())),
        client_information: ClientInformation::default(),
        instance_holder,
    };
    (
        local_player_bundle,
        outgoing_packets_receiver,
        incoming_packet_queue,
        rt,
    )
}

fn simulation_instance_name() -> ResourceLocation {
    ResourceLocation::new("azalea:simulation")
}

fn create_simulation_app() -> App {
    let mut app = App::new();
    app.add_plugins(azalea_client::DefaultPlugins.build().disable::<LogPlugin>());
    app.edit_schedule(bevy_app::Main, |schedule| {
        // makes test results more reproducible
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    });
    app
}

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
        self.incoming_packet_queue.lock().push(buf.into());
        println!("added to incoming_packet_queue");
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
    pub fn position(&self) -> Vec3 {
        *self.component::<Position>()
    }
}

fn tick_app(app: &mut App) {
    app.update();
    app.world_mut().run_schedule(GameTick);
}
