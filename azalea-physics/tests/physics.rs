use std::sync::Arc;

use azalea_block::{
    BlockState, block_state::BlockStateIntegerRepr, fluid_state::to_or_from_legacy_fluid_level,
    properties::WaterLevel,
};
use azalea_core::{
    identifier::Identifier,
    position::{BlockPos, ChunkPos, Vec3},
    registry_holder::RegistryHolder,
    tick::GameTick,
};
use azalea_entity::{EntityBundle, EntityPlugin, HasClientLoaded, LocalEntity, Physics, Position};
use azalea_physics::PhysicsPlugin;
use azalea_world::{Chunk, Instance, InstanceContainer, MinecraftEntityId, PartialInstance};
use bevy_app::App;
use parking_lot::RwLock;
use uuid::Uuid;

/// You need an app to spawn entities in the world and do updates.
fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((PhysicsPlugin, EntityPlugin))
        .init_resource::<InstanceContainer>();
    app
}

pub fn insert_overworld(app: &mut App) -> Arc<RwLock<Instance>> {
    app.world_mut()
        .resource_mut::<InstanceContainer>()
        .get_or_insert(
            Identifier::new("minecraft:overworld"),
            384,
            -64,
            &RegistryHolder::default(),
        )
}

#[test]
fn test_gravity() {
    let mut app = make_test_app();
    let world_lock = insert_overworld(&mut app);
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
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
    let world_lock = insert_overworld(&mut app);
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();
    let block_state = partial_world.chunks.set_block_state(
        BlockPos { x: 0, y: 69, z: 0 },
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
    let world_lock = insert_overworld(&mut app);
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();
    let block_state = partial_world.chunks.set_block_state(
        BlockPos { x: 0, y: 69, z: 0 },
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
    let world_lock = insert_overworld(&mut app);
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        BlockPos { x: 0, y: 69, z: 0 },
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
    let world_lock = app
        .world_mut()
        .resource_mut::<InstanceContainer>()
        .get_or_insert(
            Identifier::new("minecraft:overworld"),
            384,
            -64,
            &RegistryHolder::default(),
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        BlockPos { x: 0, y: 69, z: 0 },
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
    let world_lock = app
        .world_mut()
        .resource_mut::<InstanceContainer>()
        .get_or_insert(
            Identifier::new("minecraft:overworld"),
            384,
            -64,
            &RegistryHolder::default(),
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();
    let block_state = world_lock.write().chunks.set_block_state(
        BlockPos {
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
    let world_lock = app
        .world_mut()
        .resource_mut::<InstanceContainer>()
        .get_or_insert(
            Identifier::new("minecraft:overworld"),
            384,
            -64,
            &RegistryHolder::default(),
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
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
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

#[test]
fn test_afk_pool() {
    let mut app = make_test_app();
    let world_lock = insert_overworld(&mut app);
    let mut partial_world = PartialInstance::default();

    partial_world.chunks.set(
        &ChunkPos { x: 0, z: 0 },
        Some(Chunk::default()),
        &mut world_lock.write().chunks,
    );
    let setblock = |x: i32, y: i32, z: i32, b: BlockState| {
        world_lock
            .write()
            .chunks
            .set_block_state(BlockPos { x, y, z }, b);
    };

    let stone = azalea_block::blocks::Stone {}.into();
    let sign = azalea_block::blocks::OakSign {
        rotation: azalea_block::properties::OakSignRotation::_0,
        waterlogged: false,
    }
    .into();
    let water = |level: u8| {
        BlockState::from(azalea_block::blocks::Water {
            level: WaterLevel::from(to_or_from_legacy_fluid_level(level) as BlockStateIntegerRepr),
        })
    };

    let mut y = 69;

    // first layer
    {
        setblock(1, y, 1, stone);
        setblock(2, y, 1, stone);
        setblock(3, y, 1, stone);
        setblock(3, y, 2, stone);
        setblock(3, y, 3, stone);
        setblock(2, y, 3, stone);
        setblock(1, y, 3, stone);
        setblock(1, y, 2, stone);
    }
    // second layer
    y += 1;
    {
        setblock(1, y, 0, stone);
        setblock(2, y, 0, stone);
        setblock(3, y, 0, stone);

        setblock(0, y, 1, stone);
        setblock(0, y, 2, stone);
        setblock(0, y, 3, stone);

        setblock(1, y, 4, stone);
        setblock(2, y, 4, stone);
        setblock(3, y, 4, stone);

        setblock(4, y, 1, stone);
        setblock(4, y, 2, stone);
        setblock(4, y, 3, stone);

        // middle block
        setblock(2, y, 2, stone);

        // sign
        setblock(1, y, 1, sign);

        // water
        setblock(1, y, 2, water(8));
        setblock(1, y, 3, water(7));
        setblock(2, y, 3, water(6));
        setblock(3, y, 3, water(5));
        setblock(3, y, 2, water(4));
        setblock(3, y, 1, water(3));
        setblock(2, y, 1, water(2));
    }
    // third layer
    y += 1;
    {
        setblock(1, y, 1, water(8));
        setblock(2, y, 1, sign);
    }

    let entity = app
        .world_mut()
        .spawn((
            EntityBundle::new(
                Uuid::nil(),
                Vec3 {
                    x: 3.5,
                    y: 70.,
                    z: 1.5,
                },
                azalea_registry::EntityKind::Player,
                Identifier::new("minecraft:overworld"),
            ),
            MinecraftEntityId(0),
            LocalEntity,
            HasClientLoaded,
        ))
        .id();

    let mut blocks_visited = Vec::new();
    let mut loops_done = 0;

    for _ in 0..300 {
        app.world_mut().run_schedule(GameTick);
        app.update();

        let entity_pos = app.world_mut().get::<Position>(entity).unwrap();
        let entity_block_pos = BlockPos::from(entity_pos);

        if !blocks_visited.contains(&entity_block_pos) {
            blocks_visited.push(entity_block_pos);

            if blocks_visited.len() == 8 {
                loops_done += 1;
                blocks_visited.clear();
            }
        }
    }

    assert_eq!(
        blocks_visited.into_iter().collect::<Vec<_>>(),
        vec![
            BlockPos::new(3, 70, 2),
            BlockPos::new(3, 70, 1),
            BlockPos::new(2, 70, 1),
            BlockPos::new(1, 70, 1),
        ]
    );
    assert_eq!(loops_done, 1);
}
