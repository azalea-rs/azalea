use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_core::position::Vec3;
use azalea_protocol::packets::{
    ConnectionProtocol,
    game::{ClientboundAddEntity, ClientboundStartConfiguration},
};
use azalea_registry::EntityKind;
use azalea_world::InstanceName;
use uuid::Uuid;

#[test]
fn test_receive_spawn_entity_and_start_config_packet() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(default_login_packet());
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
        movement: Default::default(),
    });
    simulation.receive_packet(ClientboundStartConfiguration);

    simulation.tick();
    assert!(simulation.has_component::<InConfigState>());
}
