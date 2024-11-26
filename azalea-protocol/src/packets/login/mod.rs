pub mod c_cookie_request;
pub mod c_custom_query;
pub mod c_hello;
pub mod c_login_compression;
pub mod c_login_disconnect;
pub mod c_login_finished;
pub mod s_cookie_response;
pub mod s_custom_query_answer;
pub mod s_hello;
pub mod s_key;
pub mod s_login_acknowledged;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    LoginPacket,
    Serverbound => {
        0x00: s_hello::ServerboundHello,
        0x01: s_key::ServerboundKey,
        0x02: s_custom_query_answer::ServerboundCustomQueryAnswer,
        0x03: s_login_acknowledged::ServerboundLoginAcknowledged,
        0x04: s_cookie_response::ServerboundCookieResponse,
    },
    Clientbound => {
        0x00: c_login_disconnect::ClientboundLoginDisconnect,
        0x01: c_hello::ClientboundHello,
        0x02: c_login_finished::ClientboundLoginFinished,
        0x03: c_login_compression::ClientboundLoginCompression,
        0x04: c_custom_query::ClientboundCustomQuery,
        0x05: c_cookie_request::ClientboundCookieRequest,
    }
);
