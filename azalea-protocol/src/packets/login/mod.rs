// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(LoginPacket,
    Clientbound => [
        hello,
        cookie_request,
        login_compression,
        login_disconnect,
        login_finished,
        custom_query,
    ],
    Serverbound => [
        hello,
        key,
        cookie_response,
        login_acknowledged,
        custom_query_answer,
    ]
);
