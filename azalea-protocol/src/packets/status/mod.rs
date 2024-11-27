use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    StatusPacket,
    Serverbound => [
        s_status_request::ServerboundStatusRequest,
        s_ping_request::ServerboundPingRequest,
    ],
    Clientbound => [
        c_status_response::ClientboundStatusResponse,
        c_pong_response::ClientboundPongResponse,
    ]
);
