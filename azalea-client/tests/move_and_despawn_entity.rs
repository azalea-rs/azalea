use azalea_client::test_utils::prelude::*;
use azalea_core::{
    position::{ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::metadata::Cow;
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{ClientboundRemoveEntities, ClientboundTeleportEntity},
    },
};
use azalea_registry::{DataRegistry, DimensionType, EntityKind};
use azalea_world::MinecraftEntityId;
use bevy_ecs::query::With;

#[test]
fn test_move_and_despawn_entity() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("azalea:overworld"),
    ));

    for x in 0..=10 {
        simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(x, 0), (384 + 64) / 16));
    }
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

    // make sure it's despawned
    let mut cow_query = simulation.app.world_mut().query_filtered::<(), With<Cow>>();
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(cow_iter.count(), 0, "cow should be despawned");

    simulation.tick();
}
