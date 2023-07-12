pub struct BlockBehavior {
    pub has_collision: bool,
    pub friction: f32,
    pub jump_factor: f32,
    pub destroy_time: f32,
    pub requires_correct_tool_for_drops: bool,
}

impl Default for BlockBehavior {
    fn default() -> Self {
        Self {
            has_collision: true,
            friction: 0.6,
            jump_factor: 1.0,
            destroy_time: 0.,
            requires_correct_tool_for_drops: false,
        }
    }
}

impl BlockBehavior {
    pub fn no_collision(mut self) -> Self {
        self.has_collision = false;
        self
    }

    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn jump_factor(mut self, jump_factor: f32) -> Self {
        self.jump_factor = jump_factor;
        self
    }

    pub fn destroy_time(mut self, destroy_time: f32) -> Self {
        self.destroy_time = destroy_time;
        self
    }

    pub fn requires_correct_tool_for_drops(mut self) -> Self {
        self.requires_correct_tool_for_drops = true;
        self
    }
}
