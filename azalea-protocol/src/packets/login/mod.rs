// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(LoginPacket,
    Clientbound => [
        login_disconnect, // 0x00
        hello, // 0x01
        login_finished, // 0x02
        login_compression, // 0x03
        custom_query, // 0x04
        cookie_request, // 0x05
    ],
    Serverbound => [
        hello, // 0x00
        key, // 0x01
        custom_query_answer, // 0x02
        login_acknowledged, // 0x03
        cookie_response, // 0x04
    ]
);
