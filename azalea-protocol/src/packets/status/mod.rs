// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(StatusPacket,
    Clientbound => [
        status_response, // 0x00
        pong_response, // 0x01
    ],
    Serverbound => [
        status_request, // 0x00
        ping_request, // 0x01
    ]
);
