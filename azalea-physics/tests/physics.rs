use azalea_core::{
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
    tick::GameTick,
};
use azalea_entity::{EntityBundle, EntityPlugin, LocalEntity, Physics, Position};
use azalea_physics::PhysicsPlugin;
use azalea_world::{Chunk, InstanceContainer, MinecraftEntityId, PartialInstance};
use bevy_app::App;
use uuid::Uuid;

/// You need an app to spawn entities in the world and do updates.
fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((PhysicsPlugin, EntityPlugin))
        .init_resource::<InstanceContainer>();
    app
}

#[test]
fn test_gravity() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();
    // the entity has to be in a loaded chunk for physics to work
    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );

    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.,
                    y: 70.,
                    z: 0.,
                },
                azalea_registry::EntityKind::Zombie,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    {
        let entity_pos = *app.world_mut().get::<Position>(entity).unwrap();
        // y should start at 70
        assert_eq!(entity_pos.y, 70.);
    }
    app.update();
    app.world_mut().run_schedule(GameTick);
    app.update();
    {
        let entity_pos = *app.world_mut().get::<Position>(entity).unwrap();
        // delta is applied before gravity, so the first tick only sets the delta
        assert_eq!(entity_pos.y, 70.);
        let entity_physics = app.world_mut().get::<Physics>(entity).unwrap();
        assert!(entity_physics.velocity.y < 0.);
    }
    app.world_mut().run_schedule(GameTick);
    app.update();
    {
        let entity_pos = *app.world_mut().get::<Position>(entity).unwrap();
        // the second tick applies the delta to the position, so now it should go down
        assert!(
            entity_pos.y < 70.,
            "Entity y ({}) didn't go down after physics steps",
            entity_pos.y
        );
    }
}
#[test]
fn test_collision() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 70.,
                    z: 0.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    let block_state = partial_world.chunks.set_block_state(
        &BlockPos { x: 0, y: 69, z: 0 },
        azalea_registry::Block::Stone.into(),
        &world_lock.write().chunks,
    );
    assert!(
        block_state.is_some(),
        "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
    );
    app.update();
    app.world_mut().run_schedule(GameTick);
    app.update();
    {
        let entity_pos = *app.world_mut().get::<Position>(entity).unwrap();
        // delta will change, but it won't move until next tick
        assert_eq!(entity_pos.y, 70.);
        let entity_physics = app.world_mut().get::<Physics>(entity).unwrap();
        assert!(entity_physics.velocity.y < 0.);
    }
    app.world_mut().run_schedule(GameTick);
    app.update();
    {
        let entity_pos = *app.world_mut().get::<Position>(entity).unwrap();
        // the second tick applies the delta to the position, but it also does collision
        assert_eq!(entity_pos.y, 70.);
    }
}

#[test]
fn test_slab_collision() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 71.,
                    z: 0.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    let block_state = partial_world.chunks.set_block_state(
        &BlockPos { x: 0, y: 69, z: 0 },
        azalea_block::blocks::StoneSlab {
            kind: azalea_block::properties::Type::Bottom,
            waterlogged: false,
        }
        .into(),
        &world_lock.write().chunks,
    );
    assert!(
        block_state.is_some(),
        "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
    );
    // do a few steps so we fall on the slab
    for _ in 0..20 {
        app.world_mut().run_schedule(GameTick);
        app.update();
    }
    let entity_pos = app.world_mut().get::<Position>(entity).unwrap();
    assert_eq!(entity_pos.y, 69.5);
}

#[test]
fn test_top_slab_collision() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 71.,
                    z: 0.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        &BlockPos { x: 0, y: 69, z: 0 },
        azalea_block::blocks::StoneSlab {
            kind: azalea_block::properties::Type::Top,
            waterlogged: false,
        }
        .into(),
    );
    assert!(
        block_state.is_some(),
        "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
    );
    // do a few steps so we fall on the slab
    for _ in 0..20 {
        app.world_mut().run_schedule(GameTick);
        app.update();
    }
    let entity_pos = app.world_mut().get::<Position>(entity).unwrap();
    assert_eq!(entity_pos.y, 70.);
}

#[test]
fn test_weird_wall_collision() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 73.,
                    z: 0.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        &BlockPos { x: 0, y: 69, z: 0 },
        azalea_block::blocks::CobblestoneWall {
            east: azalea_block::properties::WallEast::Low,
            north: azalea_block::properties::WallNorth::Low,
            south: azalea_block::properties::WallSouth::Low,
            west: azalea_block::properties::WallWest::Low,
            up: false,
            waterlogged: false,
        }
        .into(),
    );
    assert!(
        block_state.is_some(),
        "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
    );
    // do a few steps so we fall on the wall
    for _ in 0..20 {
        app.world_mut().run_schedule(GameTick);
        app.update();
    }

    let entity_pos = app.world_mut().get::<Position>(entity).unwrap();
    assert_eq!(entity_pos.y, 70.5);
}

#[test]
fn test_negative_coordinates_weird_wall_collision() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: -1, z: -1 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: -7.5,
                    y: 73.,
                    z: -7.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        &BlockPos {
            x: -8,
            y: 69,
            z: -8,
        },
        azalea_block::blocks::CobblestoneWall {
            east: azalea_block::properties::WallEast::Low,
            north: azalea_block::properties::WallNorth::Low,
            south: azalea_block::properties::WallSouth::Low,
            west: azalea_block::properties::WallWest::Low,
            up: false,
            waterlogged: false,
        }
        .into(),
    );
    assert!(
        block_state.is_some(),
        "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
    );
    // do a few steps so we fall on the wall
    for _ in 0..20 {
        app.world_mut().run_schedule(GameTick);
        app.update();
    }

    let entity_pos = app.world_mut().get::<Position>(entity).unwrap();
    assert_eq!(entity_pos.y, 70.5);
}

#[test]
fn spawn_and_unload_world() {
    let mut app = make_test_app();
    let world_lock = app.world_mut().resource_mut::<InstanceContainer>().insert(
        ResourceLocation::new("minecraft:overworld"),
        384,
        -64,
    );
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: -1, z: -1 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let _entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: -7.5,
                    y: 73.,
                    z: -7.5,
                },
                azalea_registry::EntityKind::Player,
                ResourceLocation::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
        ))
        .id();

    // do a tick
    app.world_mut().run_schedule(GameTick);
    app.update();

    // now unload the partial_world and world_lock
    drop(partial_world);
    drop(world_lock);

    // do another tick
    app.world_mut().run_schedule(GameTick);
    app.update();
}
