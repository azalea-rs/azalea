// NOTE: This file is @generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(LoginPacket,
    Clientbound => [
        login_disconnect,
        hello,
        login_finished,
        login_compression,
        custom_query,
        cookie_request,
    ],
    Serverbound => [
        hello,
        key,
        custom_query_answer,
        login_acknowledged,
        cookie_response,
    ]
);
