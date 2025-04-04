pub struct BlockBehavior {
    pub friction: f32,
    pub jump_factor: f32,
    pub destroy_time: f32,
    pub explosion_resistance: f32,
    pub requires_correct_tool_for_drops: bool,

    pub force_solid: Option<bool>,
}

impl Default for BlockBehavior {
    fn default() -> Self {
        Self {
            friction: 0.6,
            jump_factor: 1.0,
            destroy_time: 0.,
            explosion_resistance: 0.,
            requires_correct_tool_for_drops: false,
            force_solid: None,
        }
    }
}

impl BlockBehavior {
    pub fn new() -> Self {
        Self::default()
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

    pub fn explosion_resistance(mut self, explosion_resistance: f32) -> Self {
        self.explosion_resistance = f32::max(0., explosion_resistance);
        self
    }

    pub fn strength(self, destroy_time: f32, explosion_resistance: f32) -> Self {
        self.destroy_time(destroy_time)
            .explosion_resistance(explosion_resistance)
    }

    pub fn requires_correct_tool_for_drops(mut self) -> Self {
        self.requires_correct_tool_for_drops = true;
        self
    }

    pub fn force_solid(mut self, force_solid: bool) -> Self {
        self.force_solid = Some(force_solid);
        self
    }
}
