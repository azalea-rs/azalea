pub mod client_intention_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    HandshakePacket,
    Serverbound => {
        0x00: client_intention_packet::ClientIntentionPacket,
    },
    Clientbound => {}
);
