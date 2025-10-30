use azalea_client::test_utils::prelude::*;
use azalea_core::position::{ChunkPos, Vec3};
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{ClientboundRemoveEntities, ClientboundTeleportEntity},
    },
};
use azalea_registry::EntityKind;
use azalea_world::MinecraftEntityId;

#[test]
fn test_move_and_despawn_entity() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(default_login_packet());

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();

    simulation.receive_packet(make_basic_add_entity(EntityKind::Cow, 123, (0.5, 64., 0.5)));
    simulation.tick();

    simulation.receive_packet(ClientboundTeleportEntity {
        id: MinecraftEntityId(123),
        change: PositionMoveRotation {
            pos: Vec3::new(16., 0., 0.),
            delta: Vec3::ZERO,
            look_direction: Default::default(),
        },
        relative: RelativeMovements::all_relative(),
        on_ground: true,
    });
    simulation.receive_packet(ClientboundRemoveEntities {
        entity_ids: vec![MinecraftEntityId(123)],
    });
    simulation.tick();
}
