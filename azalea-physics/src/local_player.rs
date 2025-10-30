use azalea_core::position::Vec2;
use bevy_ecs::component::Component;

/// Component for entities that can move and sprint.
///
/// Usually only present for [`LocalEntity`]s.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
#[derive(Default, Component, Clone)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    /// Whether our player is currently trying to sneak.
    ///
    /// This is distinct from
    /// [`AbstractEntityShiftKeyDown`](azalea_entity::metadata::AbstractEntityShiftKeyDown),
    /// which is a metadata value that is controlled by the server and affects
    /// how the nametags of other entities are displayed.
    ///
    /// To check whether we're actually sneaking, you can check the
    /// [`Crouching`](azalea_entity::Crouching) or [`Pose`](azalea_entity::Pose)
    /// components.
    pub trying_to_crouch: bool,

    pub move_direction: WalkDirection,
    pub move_vector: Vec2,
}

/// A direction that a player can walk in, including none.
///
/// Superset of [`SprintDirection`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WalkDirection {
    #[default]
    None,
    Forward,
    Backward,
    Left,
    Right,
    ForwardRight,
    ForwardLeft,
    BackwardRight,
    BackwardLeft,
}

/// The directions that a player can sprint in. It's a subset of
/// [`WalkDirection`].
#[derive(Clone, Copy, Debug)]
pub enum SprintDirection {
    Forward,
    ForwardRight,
    ForwardLeft,
}

impl From<SprintDirection> for WalkDirection {
    fn from(d: SprintDirection) -> Self {
        match d {
            SprintDirection::Forward => WalkDirection::Forward,
            SprintDirection::ForwardRight => WalkDirection::ForwardRight,
            SprintDirection::ForwardLeft => WalkDirection::ForwardLeft,
        }
    }
}
