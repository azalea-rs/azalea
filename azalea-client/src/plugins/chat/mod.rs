//! Implementations of chat-related features.

pub mod handler;

use std::sync::Arc;

use azalea_chat::FormattedText;
use azalea_protocol::packets::game::{
    c_disguised_chat::ClientboundDisguisedChat, c_player_chat::ClientboundPlayerChat,
    c_system_chat::ClientboundSystemChat,
};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use handler::{SendChatKindEvent, handle_send_chat_kind_event};
use parking_lot::RwLock;
use uuid::Uuid;

use crate::client::Client;

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendChatEvent>()
            .add_event::<SendChatKindEvent>()
            .add_event::<ChatReceivedEvent>()
            .add_systems(
                Update,
                (handle_send_chat_event, handle_send_chat_kind_event).chain(),
            );
    }
}

/// A chat packet, either a system message or a chat message.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatPacket {
    System(Arc<ClientboundSystemChat>),
    Player(Arc<ClientboundPlayerChat>),
    Disguised(Arc<ClientboundDisguisedChat>),
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($re).unwrap());
        &RE
    }};
}

/// This is the regex used to split system messages into their sender parts.
/// You can override this with your own regex if you want to
/// change how system messages are parsed.
pub static SYSTEM_REGEX: RwLock<&std::sync::LazyLock<regex::Regex>> = RwLock::new(regex!(
    "^<(?P<sender>[a-zA-Z_0-9]{1,16})> (?:-> me)?(?p<content>.+)$"
));

impl ChatPacket {
    /// Get the message shown in chat for this packet.
    pub fn message(&self) -> FormattedText {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message(),
            ChatPacket::Disguised(p) => p.message(),
        }
    }

    /// Determine the username of the sender and content of the message. This
    /// does not preserve formatting codes. If it's not a player-sent chat
    /// message or the sender couldn't be determined, the username part will be
    /// None.
    pub fn split_sender_and_content(&self) -> (Option<String>, String) {
        match self {
            ChatPacket::System(p) => {
                let message = p.content.to_string();

                // Overlay messages aren't in chat
                if p.overlay {
                    return (None, message);
                }

                // It's a system message, so we'll have to match the content with regex
                if let Some(m) = SYSTEM_REGEX.read().captures(&message) {
                    return (Some(m["sender"].to_string()), m["content"].to_string());
                }

                (None, message)
            }
            ChatPacket::Player(p) => (
                // If it's a player chat packet, then the sender and content
                // are already split for us.
                Some(p.chat_type.name.to_string()),
                p.body.content.clone(),
            ),
            ChatPacket::Disguised(p) => (
                // disguised chat packets are basically the same as player chat packets but without
                // the chat signing things
                Some(p.chat_type.name.to_string()),
                p.message.to_string(),
            ),
        }
    }

    /// Get the username of the sender of the message. If it's not a
    /// player-sent chat message or the sender couldn't be determined, this
    /// will be None.
    pub fn sender(&self) -> Option<String> {
        self.split_sender_and_content().0
    }

    /// Get the UUID of the sender of the message. If it's not a
    /// player-sent chat message, this will be None (this is sometimes the case
    /// when a server uses a plugin to modify chat messages).
    pub fn sender_uuid(&self) -> Option<Uuid> {
        match self {
            ChatPacket::System(_) => None,
            ChatPacket::Player(m) => Some(m.sender),
            ChatPacket::Disguised(_) => None,
        }
    }

    /// Get the content part of the message as a string. This does not preserve
    /// formatting codes. If it's not a player-sent chat message or the sender
    /// couldn't be determined, this will contain the entire message.
    pub fn content(&self) -> String {
        self.split_sender_and_content().1
    }

    /// Create a new Chat from a string. This is meant to be used as a
    /// convenience function for testing.
    pub fn new(message: &str) -> Self {
        ChatPacket::System(Arc::new(ClientboundSystemChat {
            content: FormattedText::from(message),
            overlay: false,
        }))
    }

    /// Whether this message is an incoming whisper message (i.e. someone else
    /// dm'd the bot with /msg). It works by checking the translation key, so it
    /// won't work on servers that use their own whisper system.
    pub fn is_whisper(&self) -> bool {
        match self.message() {
            FormattedText::Text(_) => false,
            FormattedText::Translatable(t) => t.key == "commands.message.display.incoming",
        }
    }
}

impl Client {
    /// Send a chat message to the server. This only sends the chat packet and
    /// not the command packet, which means on some servers you can use this to
    /// send chat messages that start with a `/`. The [`Client::chat`] function
    /// handles checking whether the message is a command and using the
    /// proper packet for you, so you should use that instead.
    pub fn send_chat_packet(&self, message: &str) {
        self.ecs.lock().send_event(SendChatKindEvent {
            entity: self.entity,
            content: message.to_string(),
            kind: ChatKind::Message,
        });
    }

    /// Send a command packet to the server. The `command` argument should not
    /// include the slash at the front.
    ///
    /// You can also just use [`Client::chat`] and start your message with a `/`
    /// to send a command.
    pub fn send_command_packet(&self, command: &str) {
        self.ecs.lock().send_event(SendChatKindEvent {
            entity: self.entity,
            content: command.to_string(),
            kind: ChatKind::Command,
        });
    }

    /// Send a message in chat.
    ///
    /// ```rust,no_run
    /// # use azalea_client::Client;
    /// # async fn example(bot: Client) -> anyhow::Result<()> {
    /// bot.chat("Hello, world!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn chat(&self, content: &str) {
        self.ecs.lock().send_event(SendChatEvent {
            entity: self.entity,
            content: content.to_string(),
        });
    }
}

/// A client received a chat message packet.
#[derive(Event, Debug, Clone)]
pub struct ChatReceivedEvent {
    pub entity: Entity,
    pub packet: ChatPacket,
}

/// Send a chat message (or command, if it starts with a slash) to the server.
#[derive(Event)]
pub struct SendChatEvent {
    pub entity: Entity,
    pub content: String,
}

pub fn handle_send_chat_event(
    mut events: EventReader<SendChatEvent>,
    mut send_chat_kind_events: EventWriter<SendChatKindEvent>,
) {
    for event in events.read() {
        if event.content.starts_with('/') {
            send_chat_kind_events.write(SendChatKindEvent {
                entity: event.entity,
                content: event.content[1..].to_string(),
                kind: ChatKind::Command,
            });
        } else {
            send_chat_kind_events.write(SendChatKindEvent {
                entity: event.entity,
                content: event.content.clone(),
                kind: ChatKind::Message,
            });
        }
    }
}

/// A kind of chat packet, either a chat message or a command.
pub enum ChatKind {
    Message,
    Command,
}

// TODO
// MessageSigner, ChatMessageContent, LastSeenMessages
// fn sign_message() -> MessageSignature {
//     MessageSignature::default()
// }
