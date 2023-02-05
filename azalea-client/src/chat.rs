//! Implementations of chat-related features.

use azalea_chat::FormattedText;
use azalea_crypto::MessageSignature;
use azalea_protocol::packets::game::{
    clientbound_player_chat_packet::{ClientboundPlayerChatPacket, LastSeenMessagesUpdate},
    clientbound_system_chat_packet::ClientboundSystemChatPacket,
    serverbound_chat_command_packet::ServerboundChatCommandPacket,
    serverbound_chat_packet::ServerboundChatPacket,
};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use crate::client::Client;

/// A chat packet, either a system message or a chat message.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatPacket {
    System(Arc<ClientboundSystemChatPacket>),
    Player(Arc<ClientboundPlayerChatPacket>),
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

impl ChatPacket {
    /// Get the message shown in chat for this packet.
    pub fn message(&self) -> FormattedText {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message(false),
        }
    }

    /// Determine the username of the sender and content of the message. This
    /// does not preserve formatting codes. If it's not a player-sent chat
    /// message or the sender couldn't be determined, the username part will be
    /// None.
    pub fn split_sender_and_content(&self) -> (Option<String>, String) {
        match self {
            ChatPacket::Player(p) => (
                // If it's a player chat packet, then the sender and content
                // are already split for us.
                Some(p.chat_type.name.to_string()),
                p.message.content(false).to_string(),
            ),
            ChatPacket::System(p) => {
                let message = p.content.to_string();
                // Overlay messages aren't in chat
                if p.overlay {
                    return (None, message);
                }
                // It's a system message, so we'll have to match the content
                // with regex
                if let Some(m) = regex!("^<([a-zA-Z_0-9]{1,16})> (.+)$").captures(&message) {
                    return (Some(m[1].to_string()), m[2].to_string());
                }

                (None, message)
            }
        }
    }

    /// Get the username of the sender of the message. If it's not a
    /// player-sent chat message or the sender couldn't be determined, this
    /// will be None.
    pub fn username(&self) -> Option<String> {
        self.split_sender_and_content().0
    }

    /// Get the UUID of the sender of the message. If it's not a
    /// player-sent chat message, this will be None (this is sometimes the case
    /// when a server uses a plugin to modify chat messages).
    pub fn uuid(&self) -> Option<Uuid> {
        match self {
            ChatPacket::System(_) => None,
            ChatPacket::Player(m) => Some(m.message.signed_header.sender),
        }
    }

    /// Get the content part of the message as a string. This does not preserve
    /// formatting codes. If it's not a player-sent chat message or the sender
    /// couldn't be determined, this will contain the entire message.
    pub fn content(&self) -> String {
        self.split_sender_and_content().1
    }

    /// Create a new ChatPacket from a string. This is meant to be used as a
    /// convenience function for testing.
    pub fn new(message: &str) -> Self {
        ChatPacket::System(Arc::new(ClientboundSystemChatPacket {
            content: FormattedText::from(message),
            overlay: false,
        }))
    }
}

impl Client {
    /// Sends chat message to the server. This only sends the chat packet and
    /// not the command packet. The [`Client::chat`] function handles checking
    /// whether the message is a command and using the proper packet for you,
    /// so you should use that instead.
    pub fn send_chat_packet(&self, message: &str) {
        // TODO: chat signing
        let signature = sign_message();
        let packet = ServerboundChatPacket {
            message: message.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time shouldn't be before epoch")
                .as_millis()
                .try_into()
                .expect("Instant should fit into a u64"),
            salt: azalea_crypto::make_salt(),
            signature,
            signed_preview: false,
            last_seen_messages: LastSeenMessagesUpdate::default(),
        }
        .get();
        self.write_packet(packet);
    }

    /// Send a command packet to the server. The `command` argument should not
    /// include the slash at the front.
    pub fn send_command_packet(&self, command: &str) {
        // TODO: chat signing
        let packet = ServerboundChatCommandPacket {
            command: command.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time shouldn't be before epoch")
                .as_millis()
                .try_into()
                .expect("Instant should fit into a u64"),
            salt: azalea_crypto::make_salt(),
            argument_signatures: vec![],
            signed_preview: false,
            last_seen_messages: LastSeenMessagesUpdate::default(),
        }
        .get();
        self.write_packet(packet);
    }

    /// Send a message in chat.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, Event};
    /// # async fn handle(bot: Client, event: Event) -> anyhow::Result<()> {
    /// bot.chat("Hello, world!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn chat(&self, message: &str) {
        if let Some(command) = message.strip_prefix('/') {
            self.send_command_packet(command);
        } else {
            self.send_chat_packet(message);
        }
    }

    // will be used for when the server tells the client about a chat preview
    // with custom formatting
    // pub fn acknowledge_preview(&self, message: &str) {}
}

// TODO
// MessageSigner, ChatMessageContent, LastSeenMessages
fn sign_message() -> MessageSignature {
    MessageSignature::default()
}
