use std::sync::Arc;

use azalea_brigadier::{
    arguments::integer_argument_type::integer,
    builder::{literal_argument_builder::literal, required_argument_builder::argument},
    command_dispatcher::CommandDispatcher,
    context::CommandContext,
};
use bevy_app::App;
use bevy_ecs::{
    component::Component,
    query::With,
    system::{Query, Resource, RunSystemOnce},
    world::{FromWorld, World},
};
use parking_lot::Mutex;

#[test]
fn bevy_app() {
    let mut app = App::new();

    // Initialize the dispatcher using FromWorld
    app.init_resource::<DispatchStorage>();

    // Process commands from bevy
    app.world_mut()
        .run_system_once(DispatchStorage::bevy_process_commands);

    // Verify spawned entities exist after processing commands
    app.world_mut()
        .run_system_once(DispatchStorage::verify_spawned_entities);
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
                let mut world = source.lock();
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
                let mut world = source.lock();
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
        context.source.lock().spawn(SpawnedEntity);

        0
    }

    /// A command called from the dispatcher.
    ///
    /// Spawns a number of entities with the [`SpawnedEntity`] component.
    fn command_spawn_entity_num(context: &CommandContext<WorldAccessor>) -> i32 {
        let num = context.argument("entities").unwrap();
        let num = *num.downcast_ref::<i32>().unwrap();

        for _ in 0..num {
            context.source.lock().spawn(SpawnedEntity);
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
    world: Arc<Mutex<World>>,
}

impl WorldAccessor {
    /// Create a new empty [`WorldAccessor`].
    fn empty() -> Self {
        Self {
            world: Arc::new(Mutex::new(World::new())),
        }
    }

    /// Swap the internal [`World`] with the given one.
    fn swap(&mut self, world: &mut World) {
        std::mem::swap(&mut *self.lock(), world);
    }
}

/// A marker [`Component`] used to test spawning entities from the dispatcher.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct SpawnedEntity;

/// Implemented for convenience.
impl std::ops::Deref for WorldAccessor {
    type Target = Arc<Mutex<World>>;
    fn deref(&self) -> &Self::Target {
        &self.world
    }
}
