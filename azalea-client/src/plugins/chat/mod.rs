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
use uuid::Uuid;

use crate::client::Client;

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SendChatEvent>()
            .add_message::<SendChatKindEvent>()
            .add_message::<ChatReceivedEvent>()
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

impl ChatPacket {
    /// Get the message shown in chat for this packet.
    ///
    /// See [`Self::split_sender_and_content`] for more details about how this
    /// works.
    pub fn message(&self) -> FormattedText {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message(),
            ChatPacket::Disguised(p) => p.message(),
        }
    }

    /// A convenience function to determine the username of the sender and
    /// the content of a chat message.
    ///
    /// This does not preserve formatting codes.
    ///
    /// This function uses a few checks to attempt to split the chat message,
    /// and is intended to work on most servers. It won't work for every server
    /// though, so in certain cases you may have to reimplement this yourself.
    ///
    /// If it's not a player-sent chat message or the sender couldn't be
    /// determined, the username part will be None.
    ///
    /// Also see [`Self::sender`] and [`Self::content`] if you only need one of
    /// the parts.
    pub fn split_sender_and_content(&self) -> (Option<String>, String) {
        match self {
            ChatPacket::System(p) => {
                let message = p.content.to_string();
                // overlay messages aren't in chat
                if p.overlay {
                    return (None, message);
                }

                // it's a system message, so we'll have to match the content with regex

                // username surrounded by angle brackets (vanilla-like chat), and allow username
                // prefixes like [Owner]
                if let Some(m) = regex!(r"^<(?:\[[^\]]+?\] )?(\w{1,16})> (.+)$").captures(&message)
                {
                    return (Some(m[1].to_string()), m[2].to_string());
                }
                // username surrounded by square brackets (essentials whispers, vanilla-like
                // /say), and allow username prefixes
                if let Some(m) =
                    regex!(r"^\[(?:\[[^\]]+?\] )?(\w{1,16})(?: -> me)?\] (.+)$").captures(&message)
                {
                    return (Some(m[1].to_string()), m[2].to_string());
                }
                // username without angle brackets (2b2t whispers, vanilla-like whispers)
                if let Some(m) =
                    regex!(r"^(\w{1,16}) whispers(?: to you)?: (.+)$").captures(&message)
                {
                    return (Some(m[1].to_string()), m[2].to_string());
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

    /// Get the username of the sender of the message.
    ///
    /// If it's not a player-sent chat message or the sender couldn't be
    /// determined, this will be None.
    ///
    /// See [`Self::split_sender_and_content`] for more details about how this
    /// works.
    pub fn sender(&self) -> Option<String> {
        self.split_sender_and_content().0
    }

    /// Get the UUID of the sender of the message.
    ///
    /// If it's not a player-sent chat message, this will be None (this is
    /// sometimes the case when a server uses a plugin to modify chat
    /// messages).
    pub fn sender_uuid(&self) -> Option<Uuid> {
        match self {
            ChatPacket::System(_) => None,
            ChatPacket::Player(m) => Some(m.sender),
            ChatPacket::Disguised(_) => None,
        }
    }

    /// Get the content part of the message as a string.
    ///
    /// This does not preserve formatting codes. If it's not a player-sent chat
    /// message or the sender couldn't be determined, this will contain the
    /// entire message.
    pub fn content(&self) -> String {
        self.split_sender_and_content().1
    }

    /// Create a new `ChatPacket` from a string. This is meant to be used as a
    /// convenience function for testing.
    pub fn new(message: &str) -> Self {
        ChatPacket::System(Arc::new(ClientboundSystemChat {
            content: FormattedText::from(message),
            overlay: false,
        }))
    }

    /// Whether this message is an incoming whisper message (i.e. someone else
    /// messaged the bot with /msg).
    ///
    /// This is not guaranteed to work correctly on custom servers.
    pub fn is_whisper(&self) -> bool {
        match self {
            ChatPacket::System(p) => {
                let message = p.content.to_string();
                if p.overlay {
                    return false;
                }
                if regex!("^(-> me|[a-zA-Z_0-9]{1,16} whispers: )").is_match(&message) {
                    return true;
                }

                false
            }
            _ => match self.message() {
                FormattedText::Text(_) => false,
                FormattedText::Translatable(t) => t.key == "commands.message.display.incoming",
            },
        }
    }
}

impl Client {
    /// Send a chat message to the server.
    ///
    /// This only sends the chat packet and not the command packet, which means
    /// on some servers you can use this to send chat messages that start
    /// with a `/`. The [`Client::chat`] function handles checking whether
    /// the message is a command and using the proper packet for you, so you
    /// should use that instead.
    pub fn write_chat_packet(&self, message: &str) {
        self.ecs.lock().write_message(SendChatKindEvent {
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
    pub fn write_command_packet(&self, command: &str) {
        self.ecs.lock().write_message(SendChatKindEvent {
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
    pub fn chat(&self, content: impl Into<String>) {
        self.ecs.lock().write_message(SendChatEvent {
            entity: self.entity,
            content: content.into(),
        });
    }
}

/// A client received a chat message packet.
#[derive(Message, Debug, Clone)]
pub struct ChatReceivedEvent {
    pub entity: Entity,
    pub packet: ChatPacket,
}

/// Send a chat message (or command, if it starts with a slash) to the server.
#[derive(Message)]
pub struct SendChatEvent {
    pub entity: Entity,
    pub content: String,
}

pub fn handle_send_chat_event(
    mut events: MessageReader<SendChatEvent>,
    mut send_chat_kind_events: MessageWriter<SendChatKindEvent>,
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
