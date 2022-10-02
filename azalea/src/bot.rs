pub struct BotState {
    jumping_once: bool,
}

pub trait BotTrait {
    fn jump(&mut self);
}

impl BotTrait for azalea_client::Client {
    fn jump(&mut self) {
        let mut physics_state = self.physics_state.lock().unwrap();
        physics_state.jumping_once = true;
    }
}
