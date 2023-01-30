//! Implements `SwarmEvent::Chat`.

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

use azalea_client::{packet_handling::ChatReceivedEvent, ChatPacket, LocalPlayer};
use azalea_ecs::app::{App, Plugin};
use azalea_ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::{Added, Without},
    system::{Commands, Query, Res, ResMut, Resource},
};
use std::collections::VecDeque;

use crate::{Swarm, SwarmEvent};

#[derive(Clone)]
pub struct SwarmChatPlugin;
impl Plugin for SwarmChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewChatMessageEvent>()
            .add_system(chat_listener)
            .add_system(add_default_client_state)
            .add_system(update_min_index_and_shrink_queue)
            .insert_resource(GlobalChatState {
                chat_queue: VecDeque::new(),
                chat_min_index: 0,
            });
    }
}

/// Add a `ClientChatState` when a new client is added to the world.
fn add_default_client_state(
    mut commands: Commands,
    query: Query<Entity, (Added<LocalPlayer>, Without<ClientChatState>)>,
    global_chat_state: Res<GlobalChatState>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ClientChatState {
            chat_index: global_chat_state.chat_min_index,
        });
    }
}

#[derive(Component)]
pub struct ClientChatState {
    pub chat_index: usize,
}

/// A chat message that no other bots have seen yet was received by a bot.
pub struct NewChatMessageEvent(ChatPacket);

#[derive(Resource)]
pub struct GlobalChatState {
    pub chat_queue: VecDeque<ChatPacket>,
    pub chat_min_index: usize,
}

fn chat_listener(
    mut query: Query<&mut ClientChatState>,
    mut events: EventReader<ChatReceivedEvent>,
    mut global_chat_state: ResMut<GlobalChatState>,
    mut new_chat_messages_events: EventWriter<NewChatMessageEvent>,
) {
    for event in events.iter() {
        if let Ok(mut client_chat_state) = query.get_mut(event.entity) {
            // When a bot receives a chat messages, it looks into the queue to find the
            // earliest instance of the message content that's after the bot's chat index.
            // If it finds it, then its personal index is simply updated. Otherwise, fire
            // the event and add to the queue.

            if global_chat_state.chat_min_index > client_chat_state.chat_index {
                // if this happens it's because this bot just logged in, so
                // ignore it and let another bot handle it
                println!(
                    "chat_min_index ({}) > chat_index ({})",
                    global_chat_state.chat_min_index, client_chat_state.chat_index
                );
                client_chat_state.chat_index = global_chat_state.chat_min_index;
                return;
            }
            let actual_vec_index = client_chat_state.chat_index - global_chat_state.chat_min_index;

            // go through the queue and find the first message that's after the bot's index
            let mut found = false;
            for (i, past_message) in global_chat_state
                .chat_queue
                .iter()
                .enumerate()
                .skip(actual_vec_index)
            {
                if past_message == &event.packet {
                    // found the message, update the index
                    client_chat_state.chat_index = i + global_chat_state.chat_min_index + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                // didn't find the message, so fire the swarm event and add to the queue
                new_chat_messages_events.send(NewChatMessageEvent(event.packet.clone()));
                global_chat_state.chat_queue.push_back(event.packet.clone());
                client_chat_state.chat_index =
                    global_chat_state.chat_queue.len() + global_chat_state.chat_min_index;
            }
        }
    }
}

fn update_min_index_and_shrink_queue(
    query: Query<&ClientChatState>,
    mut global_chat_state: ResMut<GlobalChatState>,
    mut events: EventReader<NewChatMessageEvent>,
    swarm: Res<Swarm>,
) {
    for event in events.iter() {
        swarm
            .swarm_tx
            .send(SwarmEvent::Chat(event.0.clone()))
            .unwrap();
        // To make sure the queue doesn't grow too large, we keep a `chat_min_index`
        // in Swarm that's set to the smallest index of all the bots, and we remove all
        // messages from the queue that are before that index.

        let mut new_chat_min_index = usize::MAX;
        for client_chat_state in query.iter() {
            let this_chat_index = client_chat_state.chat_index;
            if this_chat_index < new_chat_min_index {
                new_chat_min_index = this_chat_index;
            }
        }

        if global_chat_state.chat_min_index > new_chat_min_index {
            println!(
                "chat_min_index ({chat_min_index}) > new_chat_min_index ({new_chat_min_index})",
                chat_min_index = global_chat_state.chat_min_index,
            );
            return;
        }
        // remove all messages from the queue that are before the min index
        for _ in 0..(new_chat_min_index - global_chat_state.chat_min_index) {
            global_chat_state.chat_queue.pop_front();
        }

        // update the min index
        global_chat_state.chat_min_index = new_chat_min_index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_swarm_chat() {
        let (tx, mut rx) = tokio::sync::broadcast::channel(1);
        let swarm_state = GlobalChatState {
            chat_queue: Arc::new(Mutex::new(VecDeque::new())),
            chat_min_index: Arc::new(Mutex::new(0)),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        };
        let mut bot_states = vec![];
        let bot0 = ClientChatState {
            swarm_state: swarm_state.clone(),
            chat_index: Arc::new(Mutex::new(0)),
            tx: tx.clone(),
        };
        let bot1 = ClientChatState {
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
        let swarm_state = GlobalChatState {
            chat_queue: Arc::new(Mutex::new(VecDeque::new())),
            chat_min_index: Arc::new(Mutex::new(0)),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        };
        let mut bot_states = vec![];
        let bot0 = ClientChatState {
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
        let bot1 = ClientChatState {
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
