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

use azalea_client::{packet_handling::ChatReceivedEvent, ChatPacket};
use azalea_ecs::{
    app::{App, Plugin},
    component::Component,
    event::{EventReader, EventWriter},
    schedule::IntoSystemDescriptor,
    system::{Commands, Query, Res, ResMut, Resource},
};
use std::collections::VecDeque;

use super::{Swarm, SwarmEvent};

#[derive(Clone)]
pub struct SwarmChatPlugin;
impl Plugin for SwarmChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewChatMessageEvent>()
            .add_system(chat_listener.label("chat_listener"))
            .add_system(update_min_index_and_shrink_queue.after("chat_listener"))
            .insert_resource(GlobalChatState {
                chat_queue: VecDeque::new(),
                chat_min_index: 0,
            });
    }
}

#[derive(Component, Debug)]
pub struct ClientChatState {
    pub chat_index: usize,
}

/// A chat message that no other bots have seen yet was received by a bot.
#[derive(Debug)]
pub struct NewChatMessageEvent(ChatPacket);

#[derive(Resource)]
pub struct GlobalChatState {
    pub chat_queue: VecDeque<ChatPacket>,
    pub chat_min_index: usize,
}

fn chat_listener(
    mut commands: Commands,
    mut query: Query<&mut ClientChatState>,
    mut events: EventReader<ChatReceivedEvent>,
    mut global_chat_state: ResMut<GlobalChatState>,
    mut new_chat_messages_events: EventWriter<NewChatMessageEvent>,
) {
    for event in events.iter() {
        let mut client_chat_state = query.get_mut(event.entity);
        let mut client_chat_index = if let Ok(ref client_chat_state) = client_chat_state {
            client_chat_state.chat_index
        } else {
            // if the client has no chat state, we default to this and insert it at the end
            global_chat_state.chat_min_index
        };

        // When a bot receives a chat messages, it looks into the queue to find the
        // earliest instance of the message content that's after the bot's chat index.
        // If it finds it, then its personal index is simply updated. Otherwise, fire
        // the event and add to the queue.

        let actual_vec_index = client_chat_index - global_chat_state.chat_min_index;

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
                client_chat_index = i + global_chat_state.chat_min_index + 1;
                found = true;
                break;
            }
        }

        if !found {
            // didn't find the message, so fire the swarm event and add to the queue
            new_chat_messages_events.send(NewChatMessageEvent(event.packet.clone()));
            global_chat_state.chat_queue.push_back(event.packet.clone());
            client_chat_index =
                global_chat_state.chat_queue.len() + global_chat_state.chat_min_index;
        }
        if let Ok(ref mut client_chat_state) = client_chat_state {
            client_chat_state.chat_index = client_chat_index;
        } else {
            commands.entity(event.entity).insert(ClientChatState {
                chat_index: client_chat_index,
            });
        }
    }
}

fn update_min_index_and_shrink_queue(
    query: Query<&ClientChatState>,
    mut global_chat_state: ResMut<GlobalChatState>,
    mut events: EventReader<NewChatMessageEvent>,
    swarm: Option<Res<Swarm>>,
) {
    for event in events.iter() {
        if let Some(swarm) = &swarm {
            // it should also work if Swarm isn't present (so the tests don't need it)
            swarm
                .swarm_tx
                .send(SwarmEvent::Chat(event.0.clone()))
                .unwrap();
        }
        // To make sure the queue doesn't grow too large, we keep a `chat_min_index`
        // in Swarm that's set to the smallest index of all the bots, and we remove all
        // messages from the queue that are before that index.

        let mut new_chat_min_index = global_chat_state.chat_min_index;
        for client_chat_state in query.iter() {
            let this_chat_index = client_chat_state.chat_index;
            if this_chat_index < new_chat_min_index {
                new_chat_min_index = this_chat_index;
            }
        }

        if global_chat_state.chat_min_index > new_chat_min_index {
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
    use azalea_ecs::{ecs::Ecs, event::Events, system::SystemState};

    use super::*;

    fn make_test_app() -> App {
        let mut app = App::new();
        // we add the events like this instead of with .add_event so we can have our own
        // event mangement in drain_events
        app.init_resource::<Events<ChatReceivedEvent>>()
            .init_resource::<Events<NewChatMessageEvent>>()
            .add_system(chat_listener.label("chat_listener"))
            .add_system(update_min_index_and_shrink_queue.after("chat_listener"))
            .insert_resource(GlobalChatState {
                chat_queue: VecDeque::new(),
                chat_min_index: 0,
            });
        app
    }

    fn drain_events(ecs: &mut Ecs) -> Vec<ChatPacket> {
        let mut system_state: SystemState<ResMut<Events<NewChatMessageEvent>>> =
            SystemState::new(ecs);
        let mut events = system_state.get_mut(ecs);

        events.drain().map(|e| e.0.clone()).collect::<Vec<_>>()
    }

    #[tokio::test]
    async fn test_swarm_chat() {
        let mut app = make_test_app();

        let bot0 = app.world.spawn_empty().id();
        let bot1 = app.world.spawn_empty().id();

        app.world.send_event(ChatReceivedEvent {
            entity: bot0,
            packet: ChatPacket::new("a"),
        });
        app.update();

        // the swarm should get the event immediately after the bot gets it
        assert_eq!(drain_events(&mut app.world), vec![ChatPacket::new("a")]);
        assert_eq!(
            app.world.get::<ClientChatState>(bot0).unwrap().chat_index,
            1
        );
        // and a second bot sending the event shouldn't do anything
        app.world.send_event(ChatReceivedEvent {
            entity: bot1,
            packet: ChatPacket::new("a"),
        });
        app.update();
        assert_eq!(drain_events(&mut app.world), vec![]);
        assert_eq!(
            app.world.get::<ClientChatState>(bot1).unwrap().chat_index,
            1
        );

        // but if the first one gets it again, it should sent it again
        app.world.send_event(ChatReceivedEvent {
            entity: bot0,
            packet: ChatPacket::new("a"),
        });
        app.update();
        assert_eq!(drain_events(&mut app.world), vec![ChatPacket::new("a")]);

        // alright and now the second bot got a different chat message and it should be
        // sent
        app.world.send_event(ChatReceivedEvent {
            entity: bot1,
            packet: ChatPacket::new("b"),
        });
        app.update();
        assert_eq!(drain_events(&mut app.world), vec![ChatPacket::new("b")]);
    }

    #[tokio::test]
    async fn test_new_bot() {
        let mut app = make_test_app();

        let bot0 = app.world.spawn_empty().id();

        // bot0 gets a chat message
        app.world.send_event(ChatReceivedEvent {
            entity: bot0,
            packet: ChatPacket::new("a"),
        });
        app.update();
        assert_eq!(drain_events(&mut app.world), vec![ChatPacket::new("a")]);
        let bot1 = app.world.spawn_empty().id();
        app.world.send_event(ChatReceivedEvent {
            entity: bot1,
            packet: ChatPacket::new("b"),
        });
        app.update();
        assert_eq!(drain_events(&mut app.world), vec![ChatPacket::new("b")]);
    }
}
