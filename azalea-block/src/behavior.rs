#[derive(Default)]
pub struct BlockBehavior {
    pub has_collision: bool,
    pub friction: f32,
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
}
