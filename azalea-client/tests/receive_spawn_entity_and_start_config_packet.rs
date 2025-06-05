use azalea_client::{InConfigState, test_simulation::*};
use azalea_core::{position::Vec3, resource_location::ResourceLocation};
use azalea_protocol::packets::{
    ConnectionProtocol,
    game::{ClientboundAddEntity, ClientboundStartConfiguration},
};
use azalea_registry::{DataRegistry, DimensionType, EntityKind};
use azalea_world::InstanceName;
use bevy_log::tracing_subscriber;
use uuid::Uuid;

#[test]
fn test_receive_spawn_entity_and_start_config_packet() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();
    assert!(simulation.has_component::<InstanceName>());
    simulation.tick();

    simulation.receive_packet(ClientboundAddEntity {
        id: 123.into(),
        uuid: Uuid::new_v4(),
        entity_type: EntityKind::ArmorStand,
        position: Vec3::ZERO,
        x_rot: 0,
        y_rot: 0,
        y_head_rot: 0,
        data: 0,
        velocity: Default::default(),
    });
    simulation.receive_packet(ClientboundStartConfiguration);

    simulation.tick();
    assert!(simulation.has_component::<InConfigState>());

    // make sure that the entity is despawned
}
