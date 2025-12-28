use azalea_client::{
    PhysicsState, SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection,
};
use azalea_entity::{Jumping, LookDirection};

use crate::Client;

impl Client {
    /// Set whether we're jumping. This acts as if you held space in
    /// vanilla.
    ///
    /// If you want to jump once, use the `jump` function in `azalea`.
    ///
    /// If you're making a realistic client, calling this function every tick is
    /// recommended.
    pub fn set_jumping(&self, jumping: bool) {
        self.query_self::<&mut Jumping, _>(|mut j| **j = jumping);
    }

    /// Returns whether the player will try to jump next tick.
    pub fn jumping(&self) -> bool {
        **self.component::<Jumping>()
    }

    pub fn set_crouching(&self, crouching: bool) {
        self.query_self::<&mut PhysicsState, _>(|mut p| p.trying_to_crouch = crouching);
    }

    /// Whether the client is currently trying to sneak.
    ///
    /// You may want to check the [`Pose`](azalea_entity::Pose) instead.
    pub fn crouching(&self) -> bool {
        self.query_self::<&PhysicsState, _>(|p| p.trying_to_crouch)
    }

    /// Sets the direction the client is looking.
    ///
    /// `y_rot` is yaw (looking to the side, between -180 to 180), and `x_rot`
    /// is pitch (looking up and down, between -90 to 90).
    ///
    /// You can get these numbers from the vanilla f3 screen.
    pub fn set_direction(&self, y_rot: f32, x_rot: f32) {
        self.query_self::<&mut LookDirection, _>(|mut ld| {
            ld.update(LookDirection::new(y_rot, x_rot));
        });
    }

    /// Returns the direction the client is looking.
    ///
    /// See [`Self::set_direction`] for more details.
    pub fn direction(&self) -> LookDirection {
        *self.component::<LookDirection>()
    }

    /// Start walking in the given direction.
    ///
    /// To sprint, use [`Client::sprint`]. To stop walking, call walk with
    /// [`WalkDirection::None`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use azalea::{Client, WalkDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: &Client) {
    /// // walk for one second
    /// bot.walk(WalkDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn walk(&self, direction: WalkDirection) {
        let mut ecs = self.ecs.write();
        ecs.write_message(StartWalkEvent {
            entity: self.entity,
            direction,
        });
    }

    /// Start sprinting in the given direction.
    ///
    /// o stop moving, call [`bot.walk(WalkDirection::None)`](Self::walk)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use azalea::{Client, WalkDirection, SprintDirection};
    /// # use std::time::Duration;
    /// # async fn example(bot: &Client) {
    /// // sprint for one second
    /// bot.sprint(SprintDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn sprint(&self, direction: SprintDirection) {
        let mut ecs = self.ecs.write();
        ecs.write_message(StartSprintEvent {
            entity: self.entity,
            direction,
        });
    }
}
