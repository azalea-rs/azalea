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
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone)]
pub struct Plugin {
    pub swarm_state: SwarmState,
    pub tx: Sender<ChatPacket>,
}

impl crate::Plugin for Plugin {
    type State = State;

    fn build(&self) -> State {
        State {
            chat_index: Arc::new(Mutex::new(0)),
            swarm_state: self.swarm_state.clone(),
            tx: self.tx.clone(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub chat_index: Arc<Mutex<usize>>,
    pub tx: Sender<ChatPacket>,
    pub swarm_state: SwarmState,
}

#[derive(Clone)]
pub struct SwarmState {
    pub chat_queue: Arc<Mutex<VecDeque<ChatPacket>>>,
    pub chat_min_index: Arc<Mutex<usize>>,
    pub rx: Arc<tokio::sync::Mutex<Receiver<ChatPacket>>>,
}

impl State {
    pub fn handle_chat(&self, message: ChatPacket) {
        // When a bot receives a chat messages, it looks into the queue to find the
        // earliest instance of the message content that's after the bot's chat index.
        // If it finds it, then its personal index is simply updated. Otherwise, fire
        // the event and add to the queue.

        let mut chat_queue = self.swarm_state.chat_queue.lock();
        let chat_min_index = self.swarm_state.chat_min_index.lock();
        let mut chat_index = self.chat_index.lock();

        if *chat_min_index > *chat_index {
            // if this happens it's because this bot just logged in, so
            // ignore it and let another bot handle it
            println!("chat_min_index ({chat_min_index}) > chat_index ({chat_index})");
            *chat_index = *chat_min_index;
            return;
        }
        let actual_vec_index = *chat_index - *chat_min_index;

        // go through the queue and find the first message that's after the bot's index
        let mut found = false;
        for (i, past_message) in chat_queue.iter().enumerate().skip(actual_vec_index) {
            if past_message == &message {
                // found the message, update the index
                *chat_index = i + *chat_min_index + 1;
                found = true;
                break;
            }
        }

        if !found {
            // didn't find the message, so fire the swarm event and add to the queue
            self.tx
                .send(message.clone())
                .expect("failed to send chat message to swarm");
            chat_queue.push_back(message);
            *chat_index = chat_queue.len() + *chat_min_index;
        }
    }
}

#[async_trait]
impl crate::PluginState for State {
    async fn handle(self: Box<Self>, event: Event, _bot: Client) {
        // we're allowed to access Plugin::swarm_state since it's shared for every bot
        if let Event::Chat(m) = event {
            self.handle_chat(m);
        }
    }
}

impl SwarmState {
    pub fn new<S>(swarm: Swarm<S>) -> (Self, Sender<ChatPacket>)
    where
        S: Send + Sync + Clone + 'static,
    {
        let (tx, rx) = tokio::sync::broadcast::channel(1);

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
        // it should never be locked unless we reused the same plugin for two swarms
        // (bad)
        let mut rx = self.rx.lock().await;
        while let Ok(m) = rx.recv().await {
            swarm.swarm_tx.send(SwarmEvent::Chat(m)).unwrap();
            let bot_states = swarm
                .bot_datas
                .lock()
                .iter()
                .map(|(bot, _)| {
                    bot.plugins
                        .get::<State>()
                        .expect("Chat plugin not installed")
                        .clone()
                })
                .collect::<Vec<_>>();
            self.handle_new_chat_message(&bot_states);
        }
    }
}

impl SwarmState {
    pub fn handle_new_chat_message(&self, bot_states: &[State]) {
        // To make sure the queue doesn't grow too large, we keep a `chat_min_index`
        // in Swarm that's set to the smallest index of all the bots, and we remove all
        // messages from the queue that are before that index.

        let chat_min_index = *self.chat_min_index.lock();
        let mut new_chat_min_index = usize::MAX;
        for bot_state in bot_states {
            let this_chat_index = *bot_state.chat_index.lock();
            if this_chat_index < new_chat_min_index {
                new_chat_min_index = this_chat_index;
            }
        }

        let mut chat_queue = self.chat_queue.lock();
        if chat_min_index > new_chat_min_index {
            println!(
                "chat_min_index ({chat_min_index}) > new_chat_min_index ({new_chat_min_index})"
            );
            return;
        }
        // remove all messages from the queue that are before the min index
        for _ in 0..(new_chat_min_index - chat_min_index) {
            chat_queue.pop_front();
        }

        // update the min index
        *self.chat_min_index.lock() = new_chat_min_index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_swarm_chat() {
        let (tx, mut rx) = tokio::sync::broadcast::channel(1);
        let swarm_state = SwarmState {
            chat_queue: Arc::new(Mutex::new(VecDeque::new())),
            chat_min_index: Arc::new(Mutex::new(0)),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        };
        let mut bot_states = vec![];
        let bot0 = State {
            swarm_state: swarm_state.clone(),
            chat_index: Arc::new(Mutex::new(0)),
            tx: tx.clone(),
        };
        let bot1 = State {
            swarm_state: swarm_state.clone(),
            chat_index: Arc::new(Mutex::new(0)),
            tx: tx.clone(),
        };
        bot_states.push(bot0.clone());
        bot_states.push(bot1.clone());

        bot0.handle_chat(ChatPacket::new("a"));
        // the swarm should get the event immediately after the bot gets it
        assert_eq!(
            swarm_state.rx.lock().await.try_recv(),
            Ok(ChatPacket::new("a"))
        );
        assert_eq!(*bot0.chat_index.lock(), 1);
        // and a second bot sending the event shouldn't do anything
        bot1.handle_chat(ChatPacket::new("a"));
        assert!(swarm_state.rx.lock().await.try_recv().is_err());
        assert_eq!(*bot1.chat_index.lock(), 1);

        // but if the first one gets it again, it should sent it again
        bot0.handle_chat(ChatPacket::new("a"));
        assert_eq!(
            swarm_state.rx.lock().await.try_recv(),
            Ok(ChatPacket::new("a"))
        );

        // alright and now the second bot got a different chat message and it should be
        // sent
        bot1.handle_chat(ChatPacket::new("b"));
        assert_eq!(
            swarm_state.rx.lock().await.try_recv(),
            Ok(ChatPacket::new("b"))
        );
    }

    #[tokio::test]
    async fn test_new_bot() {
        let (tx, mut rx) = tokio::sync::broadcast::channel(1);
        let swarm_state = SwarmState {
            chat_queue: Arc::new(Mutex::new(VecDeque::new())),
            chat_min_index: Arc::new(Mutex::new(0)),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        };
        let mut bot_states = vec![];
        let bot0 = State {
            swarm_state: swarm_state.clone(),
            chat_index: Arc::new(Mutex::new(0)),
            tx: tx.clone(),
        };
        bot_states.push(bot0.clone());

        // bot0 gets a chat message
        bot0.handle_chat(ChatPacket::new("a"));
        assert_eq!(
            swarm_state.rx.lock().await.try_recv(),
            Ok(ChatPacket::new("a"))
        );
        // now a second bot joined and got a different chat message
        let bot1 = State {
            swarm_state: swarm_state.clone(),
            chat_index: Arc::new(Mutex::new(0)),
            tx: tx.clone(),
        };
        bot_states.push(bot1.clone());
        bot1.handle_chat(ChatPacket::new("b"));
        assert_eq!(
            swarm_state.rx.lock().await.try_recv(),
            Ok(ChatPacket::new("b"))
        );
    }
}
