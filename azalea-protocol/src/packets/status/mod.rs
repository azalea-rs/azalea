// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(StatusPacket,
    Clientbound => [
        pong_response,
        status_response,
    ],
    Serverbound => [
        ping_request,
        status_request,
    ]
);
