use std::{mem, ops::Deref, sync::Arc};

use azalea_brigadier::prelude::*;
use bevy_app::App;
use bevy_ecs::{prelude::*, system::RunSystemOnce};
use parking_lot::RwLock;

#[test]
fn bevy_app() {
    let mut app = App::new();

    // Initialize the dispatcher using FromWorld
    app.init_resource::<DispatchStorage>();

    // Process commands from bevy
    if let Err(err) = app
        .world_mut()
        .run_system_once(DispatchStorage::bevy_process_commands)
    {
        panic!("Failed to process commands: {err}");
    }

    // Verify spawned entities exist after processing commands
    if let Err(err) = app
        .world_mut()
        .run_system_once(DispatchStorage::verify_spawned_entities)
    {
        panic!("Failed to verify spawned entities: {err}");
    }
}

#[derive(Resource)]
struct DispatchStorage {
    /// The [`CommandDispatcher`].
    ///
    /// Processes incoming commands.
    dispatch: CommandDispatcher<WorldAccessor>,
    /// The world accessor.
    ///
    /// Allows the dispatcher to query the [`World`].
    world: WorldAccessor,
}

/// Implement [`FromWorld`] to initialize the dispatcher.
///
/// Allows the dispatcher to query the [`World`]
/// for generating commands on startup.
impl FromWorld for DispatchStorage {
    fn from_world(_: &mut World) -> Self {
        let mut dispatch = CommandDispatcher::new();

        // Register dispatcher commands
        {
            // Register the "spawn_entity" command
            dispatch
                .register(literal("spawn_entity").executes(DispatchStorage::command_spawn_entity));

            // Register the "spawn_entity_num" command
            dispatch.register(literal("spawn_entity_num").then(
                argument("entities", integer()).executes(DispatchStorage::command_spawn_entity_num),
            ));
        }

        Self {
            dispatch,
            world: WorldAccessor::empty(),
        }
    }
}

impl DispatchStorage {
    /// A bevy system called to process commands.
    fn bevy_process_commands(world: &mut World) {
        world.resource_scope::<Self, _>(|bevy_world, mut storage| {
            // NOTE: Initial swap to own bevy's `World`
            //
            // This is important, otherwise the dispatcher
            // will only be able to access it's own empty `World`.
            storage.world.swap(bevy_world);

            let source = storage.world.clone();

            // Test "spawn_entity"
            {
                println!("Testing 'spawn_entity' command");
                let result = storage.dispatch.execute("spawn_entity", source.clone());

                // Ensure the command was successful
                assert_eq!(result, Ok(0));

                // Query the World for the spawned entity
                let mut world = source.write();
                let mut query = world.query_filtered::<(), With<SpawnedEntity>>();

                // Ensure only one entity was spawned
                let count = query.iter(&world).count();
                println!("Spawned entities: {count}");
                assert_eq!(count, 1);
            }

            // Test "spawn_entity_num"
            {
                println!("Testing 'spawn_entity_num' command");
                let result = storage
                    .dispatch
                    .execute("spawn_entity_num 3", source.clone());

                // Ensure the command was successful
                assert_eq!(result, Ok(0));

                // Query the World for spawned entities
                let mut world = source.write();
                let mut query = world.query_filtered::<(), With<SpawnedEntity>>();

                // Ensure three additional entities were spawned
                let count = query.iter(&world).count();
                println!("Spawned entities: {count}");
                assert_eq!(count, 4);
            }

            // NOTE: Second swap to give bevy's `World` back
            //
            // It's even more important to give the `World` back
            // after commands are executed, otherwise your app
            // will be stuck with an empty `World`.
            storage.world.swap(bevy_world);
        });
    }

    /// A command called from the dispatcher.
    ///
    /// Spawns an entity with the [`SpawnedEntity`] component.
    fn command_spawn_entity(context: &CommandContext<WorldAccessor>) -> i32 {
        context.source.write().spawn(SpawnedEntity);

        0
    }

    /// A command called from the dispatcher.
    ///
    /// Spawns a number of entities with the [`SpawnedEntity`] component.
    fn command_spawn_entity_num(context: &CommandContext<WorldAccessor>) -> i32 {
        let num = get_integer(context, "entities").unwrap();

        for _ in 0..num {
            context.source.write().spawn(SpawnedEntity);
        }

        0
    }

    /// A bevy system called to verify four total entities was spawned.
    fn verify_spawned_entities(query: Query<(), With<SpawnedEntity>>) {
        assert_eq!(query.iter().count(), 4);
    }
}

/// A wrapper around a [`World`] that allows for
/// access from inside a [`CommandDispatcher`].
#[derive(Clone)]
struct WorldAccessor {
    world: Arc<RwLock<World>>,
}

impl WorldAccessor {
    /// Create a new empty [`WorldAccessor`].
    fn empty() -> Self {
        Self {
            world: Arc::new(RwLock::new(World::new())),
        }
    }

    /// Swap the internal [`World`] with the given one.
    fn swap(&mut self, world: &mut World) {
        mem::swap(&mut *self.write(), world);
    }
}

/// A marker [`Component`] used to test spawning entities from the dispatcher.
#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct SpawnedEntity;

/// Implemented for convenience.
impl Deref for WorldAccessor {
    type Target = Arc<RwLock<World>>;
    fn deref(&self) -> &Self::Target {
        &self.world
    }
}
