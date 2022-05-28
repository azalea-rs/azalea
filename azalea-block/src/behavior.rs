#[derive(Default)]
pub struct BlockBehavior {
    pub has_collision: bool,
}

impl BlockBehavior {
    #[inline]
    pub fn no_collision(mut self) -> Self {
        self.has_collision = false;
        self
    }
}
