use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    LoginPacket,
    Serverbound => [
        s_hello::ServerboundHello,
        s_key::ServerboundKey,
        s_custom_query_answer::ServerboundCustomQueryAnswer,
        s_login_acknowledged::ServerboundLoginAcknowledged,
        s_cookie_response::ServerboundCookieResponse,
    ],
    Clientbound => [
        c_login_disconnect::ClientboundLoginDisconnect,
        c_hello::ClientboundHello,
        c_login_finished::ClientboundLoginFinished,
        c_login_compression::ClientboundLoginCompression,
        c_custom_query::ClientboundCustomQuery,
        c_cookie_request::ClientboundCookieRequest,
    ]
);
