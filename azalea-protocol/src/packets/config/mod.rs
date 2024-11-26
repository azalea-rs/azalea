pub mod c_cookie_request;
pub mod c_custom_payload;
pub mod c_disconnect;
pub mod c_finish_configuration;
pub mod c_keep_alive;
pub mod c_ping;
pub mod c_registry_data;
pub mod c_reset_chat;
pub mod c_resource_pack_pop;
pub mod c_resource_pack_push;
pub mod c_select_known_packs;
pub mod c_store_cookie;
pub mod c_transfer;
pub mod c_update_enabled_features;
pub mod c_update_tags;
pub mod s_client_information;
pub mod s_cookie_response;
pub mod s_custom_payload;
pub mod s_finish_configuration;
pub mod s_keep_alive;
pub mod s_pong;
pub mod s_resource_pack;
pub mod s_select_known_packs;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    ConfigPacket,
    Serverbound => {
        0x00: s_client_information::ServerboundClientInformation,
        0x01: s_cookie_response::ServerboundCookieResponse,
        0x02: s_custom_payload::ServerboundCustomPayload,
        0x03: s_finish_configuration::ServerboundFinishConfiguration,
        0x04: s_keep_alive::ServerboundKeepAlive,
        0x05: s_pong::ServerboundPong,
        0x06: s_resource_pack::ServerboundResourcePack,
        0x07: s_select_known_packs::ServerboundSelectKnownPacks,
    },
    Clientbound => {
        0x00: c_cookie_request::ClientboundCookieRequest,
        0x01: c_custom_payload::ClientboundCustomPayload,
        0x02: c_disconnect::ClientboundDisconnect,
        0x03: c_finish_configuration::ClientboundFinishConfiguration,
        0x04: c_keep_alive::ClientboundKeepAlive,
        0x05: c_ping::ClientboundPing,
        0x06: c_reset_chat::ClientboundResetChat,
        0x07: c_registry_data::ClientboundRegistryData,
        0x08: c_resource_pack_pop::ClientboundResourcePackPop,
        0x09: c_resource_pack_push::ClientboundResourcePackPush,
        0x0a: c_store_cookie::ClientboundStoreCookie,
        0x0b: c_transfer::ClientboundTransfer,
        0x0c: c_update_enabled_features::ClientboundUpdateEnabledFeatures,
        0x0d: c_update_tags::ClientboundUpdateTags,
        0x0e: c_select_known_packs::ClientboundSelectKnownPacks,
    }
);
