//! Implements SwarmEvent::Chat

// How the chat event works (to avoid firing the event multiple times):
// ---
// There's a shared queue of all the chat messages
// Each bot contains an index of the farthest message we've seen
// When a bot receives a chat messages, it looks into the queue to find the
// earliest instance of the message content that's after the bot's chat index.
// If it finds it, then its personal index is simply updated. Otherwise, fire
// the event and add to the queue.
//
// To make sure the queue doesn't grow too large, we keep a `chat_min_index`
// in Swarm that's set to the smallest index of all the bots, and we remove all
// messages from the queue that are before that index.

use crate::{Swarm, SwarmEvent};
use async_trait::async_trait;
use azalea_client::{ChatPacket, Client, Event};
use parking_lot::Mutex;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Clone)]
pub struct Plugin {
    pub swarm_state: SwarmState,
    pub tx: UnboundedSender<ChatPacket>,
}

impl crate::Plugin for Plugin {
    type State = State;

    fn build(&self) -> State {
        State {
            farthest_chat_index: Arc::new(Mutex::new(0)),
            swarm_state: self.swarm_state.clone(),
            tx: self.tx.clone(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub farthest_chat_index: Arc<Mutex<usize>>,
    pub tx: UnboundedSender<ChatPacket>,
    pub swarm_state: SwarmState,
}

#[derive(Clone)]
pub struct SwarmState {
    pub chat_queue: Arc<Mutex<VecDeque<ChatPacket>>>,
    pub chat_min_index: Arc<Mutex<usize>>,
    pub rx: Arc<tokio::sync::Mutex<UnboundedReceiver<ChatPacket>>>,
}

#[async_trait]
impl crate::PluginState for State {
    async fn handle(self: Box<Self>, event: Event, _bot: Client) {
        // we're allowed to access Plugin::swarm_state since it's shared for every bot
        if let Event::Chat(m) = event {
            // When a bot receives a chat messages, it looks into the queue to find the
            // earliest instance of the message content that's after the bot's chat index.
            // If it finds it, then its personal index is simply updated. Otherwise, fire
            // the event and add to the queue.

            let mut chat_queue = self.swarm_state.chat_queue.lock();
            let chat_min_index = self.swarm_state.chat_min_index.lock();
            let mut farthest_chat_index = self.farthest_chat_index.lock();

            let actual_vec_index = *farthest_chat_index - *chat_min_index;

            // go through the queue and find the first message that's after the bot's index
            let mut found = false;
            for (i, msg) in chat_queue.iter().enumerate().skip(actual_vec_index) {
                if msg == &m {
                    // found the message, update the index
                    *farthest_chat_index = i + *chat_min_index + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                // didn't find the message, so fire the swarm event and add to the queue
                self.tx
                    .send(m.clone())
                    .expect("failed to send chat message to swarm");
                chat_queue.push_back(m);
                *farthest_chat_index = chat_queue.len() - 1 + *chat_min_index;
            }
        }
    }
}

impl SwarmState {
    pub fn new<S>(swarm: Swarm<S>) -> (Self, UnboundedSender<ChatPacket>)
    where
        S: Send + Sync + Clone + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let swarm_state = SwarmState {
            chat_queue: Arc::new(Mutex::new(VecDeque::new())),
            chat_min_index: Arc::new(Mutex::new(0)),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        };
        tokio::spawn(swarm_state.clone().start(swarm));

        (swarm_state, tx)
    }
    async fn start<S>(self, swarm: Swarm<S>)
    where
        S: Send + Sync + Clone + 'static,
    {
        // it should never be locked unless we reused the same plugin for two swarms (bad)
        let mut rx = self.rx.lock().await;
        while let Some(m) = rx.recv().await {
            swarm.swarm_tx.send(SwarmEvent::Chat(m)).unwrap();

            // To make sure the queue doesn't grow too large, we keep a `chat_min_index`
            // in Swarm that's set to the smallest index of all the bots, and we remove all
            // messages from the queue that are before that index.

            let chat_min_index = *self.chat_min_index.lock();
            let mut new_chat_min_index = usize::MAX;
            for (bot, _) in swarm.bot_datas.lock().iter() {
                let this_farthest_chat_index = *bot
                    .plugins
                    .get::<State>()
                    .expect("Chat plugin not installed")
                    .farthest_chat_index
                    .lock();
                if this_farthest_chat_index < new_chat_min_index {
                    new_chat_min_index = this_farthest_chat_index;
                }
            }

            let mut chat_queue = self.chat_queue.lock();
            // remove all messages from the queue that are before the min index
            for _ in 0..(new_chat_min_index - chat_min_index) {
                chat_queue.pop_front();
            }

            // update the min index
            *self.chat_min_index.lock() = new_chat_min_index;
        }
    }
}
