use std::time::{SystemTime, UNIX_EPOCH};

use azalea_protocol::packets::{
    Packet,
    game::{ServerboundChat, ServerboundChatCommand, s_chat::LastSeenMessagesUpdate},
};
use bevy_ecs::prelude::*;

use super::ChatKind;
use crate::packet::game::SendPacketEvent;

/// Send a chat packet to the server of a specific kind (chat message or
/// command). Usually you just want [`SendChatEvent`] instead.
///
/// Usually setting the kind to `Message` will make it send a chat message even
/// if it starts with a slash, but some server implementations will always do a
/// command if it starts with a slash.
///
/// If you're wondering why this isn't two separate events, it's so ordering is
/// preserved if multiple chat messages and commands are sent at the same time.
///
/// [`SendChatEvent`]: super::SendChatEvent
#[derive(Event)]
pub struct SendChatKindEvent {
    pub entity: Entity,
    pub content: String,
    pub kind: ChatKind,
}

pub fn handle_send_chat_kind_event(
    mut events: EventReader<SendChatKindEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        let content = event
            .content
            .chars()
            .filter(|c| !matches!(c, '\x00'..='\x1F' | '\x7F' | '§'))
            .take(256)
            .collect::<String>();
        let packet = match event.kind {
            ChatKind::Message => ServerboundChat {
                message: content,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time shouldn't be before epoch")
                    .as_millis()
                    .try_into()
                    .expect("Instant should fit into a u64"),
                salt: azalea_crypto::make_salt(),
                signature: None,
                last_seen_messages: LastSeenMessagesUpdate::default(),
            }
            .into_variant(),
            ChatKind::Command => {
                // TODO: chat signing
                ServerboundChatCommand { command: content }.into_variant()
            }
        };

        send_packet_events.send(SendPacketEvent::new(event.entity, packet));
    }
}
