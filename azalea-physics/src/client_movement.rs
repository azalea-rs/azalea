use azalea_core::position::Vec2;
use bevy_ecs::component::Component;

/// Component for entities that can move and sprint.
///
/// Usually only present for [`LocalEntity`]s.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
#[derive(Clone, Component, Default)]
pub struct ClientMovementState {
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
///
/// This can be freely converted to and from [`DirectionStates`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl WalkDirection {
    /// Returns true if the direction is forward, forward-right, or
    /// forward-left.
    pub fn forward(self) -> bool {
        DirectionStates::from(self).forward
    }
    /// Returns true if the direction is backward, backward-right, or
    /// backward-left.
    pub fn backward(self) -> bool {
        DirectionStates::from(self).backward
    }
    /// Returns true if the direction is left, forward-left, or backward-left.
    pub fn left(self) -> bool {
        DirectionStates::from(self).left
    }
    /// Returns true if the direction is right, forward-right, or
    /// backward-right.
    pub fn right(self) -> bool {
        DirectionStates::from(self).right
    }

    pub fn set_forward(&mut self, value: bool) {
        let mut d = DirectionStates::from(*self);
        d.forward = value;
        *self = d.into();
    }
    pub fn set_backward(&mut self, value: bool) {
        let mut d = DirectionStates::from(*self);
        d.backward = value;
        *self = d.into();
    }
    pub fn set_left(&mut self, value: bool) {
        let mut d = DirectionStates::from(*self);
        d.left = value;
        *self = d.into();
    }
    pub fn set_right(&mut self, value: bool) {
        let mut d = DirectionStates::from(*self);
        d.right = value;
        *self = d.into();
    }

    /// Inverts the walk direction.
    ///
    /// ```
    /// # use azalea_physics::client_movement::WalkDirection;
    ///
    /// assert_eq!(WalkDirection::Forward.opposite(), WalkDirection::Backward);
    /// assert_eq!(
    ///     WalkDirection::BackwardRight.opposite(),
    ///     WalkDirection::ForwardLeft
    /// );
    /// assert_eq!(WalkDirection::None.opposite(), WalkDirection::None);
    /// ```
    pub fn opposite(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::ForwardRight => Self::BackwardLeft,
            Self::ForwardLeft => Self::BackwardRight,
            Self::BackwardRight => Self::ForwardLeft,
            Self::BackwardLeft => Self::ForwardRight,
        }
    }
}
/// A struct containing fields for each direction.
///
/// This can be freely converted to and from a [`WalkDirection`], and may
/// simplify certain movement direction checks.
#[derive(Default)]
pub struct DirectionStates {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}
impl From<WalkDirection> for DirectionStates {
    fn from(d: WalkDirection) -> Self {
        let mut s = Self::default();
        match d {
            WalkDirection::None => {}
            WalkDirection::Forward => s.forward = true,
            WalkDirection::Backward => s.backward = true,
            WalkDirection::Left => s.left = true,
            WalkDirection::Right => s.right = true,
            WalkDirection::ForwardRight => {
                s.forward = true;
                s.right = true
            }
            WalkDirection::ForwardLeft => {
                s.forward = true;
                s.left = true
            }
            WalkDirection::BackwardRight => {
                s.backward = true;
                s.right = true
            }
            WalkDirection::BackwardLeft => {
                s.forward = true;
                s.left = true
            }
        };
        s
    }
}
impl From<DirectionStates> for WalkDirection {
    fn from(d: DirectionStates) -> Self {
        let left = d.left && !d.right;
        let right = d.right && !d.left;

        if d.forward && !d.backward {
            if right {
                return Self::ForwardRight;
            } else if left {
                return Self::ForwardLeft;
            }
            return Self::Forward;
        } else if d.backward && !d.forward {
            if right {
                return Self::BackwardRight;
            } else if left {
                return Self::BackwardLeft;
            }
            return Self::Backward;
        }
        if right {
            return Self::Right;
        } else if left {
            return Self::Left;
        }
        Self::None
    }
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
