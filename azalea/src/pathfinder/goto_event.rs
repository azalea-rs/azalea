use std::{sync::Arc, time::Duration};

use bevy_ecs::{entity::Entity, event::Event};

use crate::pathfinder::{
    astar::PathfinderTimeout,
    goals::Goal,
    moves::{self, SuccessorsFn},
};

/// Send this event to start pathfinding to the given goal.
///
/// Also see [`PathfinderClientExt::goto`].
///
/// This event is read by [`goto_listener`]
///
/// [`goto_listener`]: crate::pathfinder::goto_listener
/// [`PathfinderClientExt::goto`]: crate::pathfinder::PathfinderClientExt::goto
#[derive(Event)]
#[non_exhaustive]
pub struct GotoEvent {
    /// The local bot entity that will do the pathfinding and execute the path.
    pub entity: Entity,
    pub goal: Arc<dyn Goal>,
    /// The function that's used for checking what moves are possible. Usually
    /// [`moves::default_move`].
    pub successors_fn: SuccessorsFn,

    /// Whether the bot is allowed to break blocks while pathfinding.
    pub allow_mining: bool,

    /// Whether we should recalculate the path when the pathfinder timed out and
    /// there's no partial path to try.
    ///
    /// Should usually be set to true.
    pub retry_on_no_path: bool,

    /// The minimum amount of time that should pass before the A* pathfinder
    /// function can return a timeout. It may take up to [`Self::max_timeout`]
    /// if it can't immediately find a usable path.
    ///
    /// A good default value for this is
    /// `PathfinderTimeout::Time(Duration::from_secs(1))`.
    ///
    /// Also see [`PathfinderTimeout::Nodes`]
    pub min_timeout: PathfinderTimeout,
    /// The absolute maximum amount of time that the pathfinder function can
    /// take to find a path. If it takes this long, it means no usable path was
    /// found (so it might be impossible).
    ///
    /// A good default value for this is
    /// `PathfinderTimeout::Time(Duration::from_secs(5))`.
    pub max_timeout: PathfinderTimeout,
}
impl GotoEvent {
    pub fn new(entity: Entity, goal: impl Goal + 'static) -> Self {
        Self {
            entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
            allow_mining: true,
            retry_on_no_path: true,
            min_timeout: PathfinderTimeout::Time(Duration::from_secs(1)),
            max_timeout: PathfinderTimeout::Time(Duration::from_secs(5)),
        }
    }
    pub fn with_successors_fn(mut self, successors_fn: SuccessorsFn) -> Self {
        self.successors_fn = successors_fn;
        self
    }
    pub fn with_allow_mining(mut self, allow_mining: bool) -> Self {
        self.allow_mining = allow_mining;
        self
    }
    pub fn with_retry_on_no_path(mut self, retry_on_no_path: bool) -> Self {
        self.retry_on_no_path = retry_on_no_path;
        self
    }
    pub fn with_min_timeout(mut self, min_timeout: PathfinderTimeout) -> Self {
        self.min_timeout = min_timeout;
        self
    }
    pub fn with_max_timeout(mut self, max_timeout: PathfinderTimeout) -> Self {
        self.max_timeout = max_timeout;
        self
    }
}
