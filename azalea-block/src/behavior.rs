pub struct BlockBehavior {
    pub has_collision: bool,
    pub friction: f32,
    pub jump_factor: f32,
}

impl Default for BlockBehavior {
    fn default() -> Self {
        Self {
            has_collision: true,
            friction: 0.6,
            jump_factor: 1.0,
        }
    }
}

impl BlockBehavior {
    #[inline]
    pub fn no_collision(mut self) -> Self {
        self.has_collision = false;
        self
    }

    #[inline]
    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    #[inline]
    pub fn jump_factor(mut self, jump_factor: f32) -> Self {
        self.jump_factor = jump_factor;
        self
    }
}
