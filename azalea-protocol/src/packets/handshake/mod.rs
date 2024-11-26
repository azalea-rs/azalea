pub mod s_client_intention;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    HandshakePacket,
    Serverbound => {
        0x00: s_client_intention::ServerboundClientIntention,
    },
    Clientbound => {}
);
