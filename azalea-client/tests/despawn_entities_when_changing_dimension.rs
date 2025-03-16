use azalea_client::test_simulation::*;
use azalea_core::{position::ChunkPos, resource_location::ResourceLocation};
use azalea_entity::metadata::Cow;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
};
use azalea_registry::{DimensionType, EntityKind};
use bevy_ecs::query::With;
use bevy_log::tracing_subscriber;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_despawn_entities_when_changing_dimension() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    simulation.receive_packet(ClientboundRegistryData {
        registry_id: ResourceLocation::new("minecraft:dimension_type"),
        entries: vec![
            (
                ResourceLocation::new("minecraft:overworld"),
                Some(NbtCompound::from_values(vec![
                    ("height".into(), NbtTag::Int(384)),
                    ("min_y".into(), NbtTag::Int(-64)),
                ])),
            ),
            (
                ResourceLocation::new("minecraft:nether"),
                Some(NbtCompound::from_values(vec![
                    ("height".into(), NbtTag::Int(256)),
                    ("min_y".into(), NbtTag::Int(0)),
                ])),
            ),
        ]
        .into_iter()
        .collect(),
    });
    simulation.tick();
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    //
    // OVERWORLD
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("azalea:a"),
    ));
    simulation.tick();

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // spawn a cow
    simulation.receive_packet(make_basic_add_entity(EntityKind::Cow, 123, (0.5, 64., 0.5)));
    simulation.tick();
    // make sure it's spawned
    let mut cow_query = simulation.app.world_mut().query_filtered::<(), With<Cow>>();
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(cow_iter.count(), 1, "cow should be spawned");

    //
    // NETHER
    //

    simulation.receive_packet(make_basic_respawn_packet(
        DimensionType::new_raw(1), // nether
        ResourceLocation::new("azalea:b"),
    ));
    simulation.tick();

    // cow should be completely deleted from the ecs
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(
        cow_iter.count(),
        0,
        "cow should be despawned after switching dimensions"
    );
}
