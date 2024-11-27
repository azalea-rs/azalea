use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    HandshakePacket,
    Serverbound => [
        s_client_intention::ServerboundClientIntention,
    ],
    Clientbound => []
);
