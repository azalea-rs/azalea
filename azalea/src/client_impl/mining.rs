use azalea_client::mining::{LeftClickMine, StartMiningBlockEvent};
use azalea_core::position::BlockPos;

use crate::Client;

impl Client {
    pub fn start_mining(&self, position: BlockPos) {
        let mut ecs = self.ecs.write();

        ecs.write_message(StartMiningBlockEvent {
            entity: self.entity,
            position,
            force: true,
        });
    }

    /// When enabled, the bot will mine any block that it is looking at if it is
    /// reachable.
    pub fn left_click_mine(&self, enabled: bool) {
        let mut ecs = self.ecs.write();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            entity_mut.insert(LeftClickMine);
        } else {
            entity_mut.remove::<LeftClickMine>();
        }
    }
}
