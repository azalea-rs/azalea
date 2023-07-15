use azalea_client::{
    interact::SwingArmEvent,
    mining::{Mining, StartMiningBlockEvent},
    Client, TickBroadcast,
};
use azalea_core::BlockPos;

pub trait MiningExt {
    /// Start mining a block.
    async fn mine(&mut self, position: BlockPos);
}

impl MiningExt for Client {
    /// Start mining a block. This won't turn the bot's head towards the block,
    /// so you'll have to do that yourself with [`look_at`].
    ///
    /// [`look_at`]: crate::prelude::BotClientExt::look_at
    async fn mine(&mut self, position: BlockPos) {
        self.ecs.lock().send_event(StartMiningBlockEvent {
            entity: self.entity,
            position,
        });
        // vanilla sends an extra swing arm packet when we start mining
        self.ecs.lock().send_event(SwingArmEvent {
            entity: self.entity,
        });

        let mut receiver = {
            let ecs = self.ecs.lock();
            let tick_broadcast = ecs.resource::<TickBroadcast>();
            tick_broadcast.subscribe()
        };
        while receiver.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<Mining>(self.entity).is_none() {
                break;
            }
        }
    }
}
