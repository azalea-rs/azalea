// login packets aren't actually handled here because compression/encryption
// would make packet handling a lot messier

use std::{collections::HashSet, sync::Arc};

use azalea_protocol::packets::login::{
    serverbound_custom_query_answer_packet::ServerboundCustomQueryAnswerPacket,
    ClientboundLoginPacket, ServerboundLoginPacket,
};
use bevy_ecs::{prelude::*, system::SystemState};
use derive_more::{Deref, DerefMut};
use tokio::sync::mpsc;
use tracing::error;

// this struct is defined here anyways though so it's consistent with the other
// ones

/// An event that's sent when we receive a login packet from the server. Note
/// that if you want to handle this in a system, you must add
/// `.before(azalea::packet_handling::login::process_packet_events)` to it
/// because that system clears the events.
#[derive(Event, Debug, Clone)]
pub struct LoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundLoginPacket>,
}

/// Event for sending a login packet to the server.
#[derive(Event)]
pub struct SendLoginPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundLoginPacket,
}

#[derive(Component)]
pub struct LoginSendPacketQueue {
    pub tx: mpsc::UnboundedSender<ServerboundLoginPacket>,
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendLoginPacketEvent>,
    mut query: Query<&mut LoginSendPacketQueue>,
) {
    for event in send_packet_events.read() {
        if let Ok(queue) = query.get_mut(event.entity) {
            let _ = queue.tx.send(event.packet.clone());
        } else {
            error!("Sent SendPacketEvent for entity that doesn't have a LoginSendPacketQueue");
        }
    }
}

/// Plugins can add to this set if they want to handle a custom query packet
/// themselves. This component removed after the login state ends.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct IgnoreQueryIds(HashSet<u32>);

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<ResMut<Events<LoginPacketEvent>>> = SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for LoginPacketEvent {
        entity: player_entity,
        packet,
    } in events.drain()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((player_entity, packet));
    }
    for (player_entity, packet) in events_owned {
        #[allow(clippy::single_match)]
        match packet.as_ref() {
            ClientboundLoginPacket::CustomQuery(p) => {
                let mut system_state: SystemState<(
                    EventWriter<SendLoginPacketEvent>,
                    Query<&IgnoreQueryIds>,
                )> = SystemState::new(ecs);
                let (mut send_packet_events, query) = system_state.get_mut(ecs);

                let ignore_query_ids = query.get(player_entity).ok().map(|x| x.0.clone());
                if let Some(ignore_query_ids) = ignore_query_ids {
                    if ignore_query_ids.contains(&p.transaction_id) {
                        continue;
                    }
                }

                send_packet_events.send(SendLoginPacketEvent {
                    entity: player_entity,
                    packet: ServerboundCustomQueryAnswerPacket {
                        transaction_id: p.transaction_id,
                        data: None,
                    }
                    .get(),
                });
            }
            _ => {}
        }
    }
}
