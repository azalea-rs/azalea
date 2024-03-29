pub mod clientbound_cookie_request_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_finish_configuration_packet;
pub mod clientbound_keep_alive_packet;
pub mod clientbound_ping_packet;
pub mod clientbound_registry_data_packet;
pub mod clientbound_reset_chat_packet;
pub mod clientbound_resource_pack_pop_packet;
pub mod clientbound_resource_pack_push_packet;
pub mod clientbound_select_known_packs_packet;
pub mod clientbound_store_cookie_packet;
pub mod clientbound_transfer_packet;
pub mod clientbound_update_enabled_features_packet;
pub mod clientbound_update_tags_packet;
pub mod serverbound_client_information_packet;
pub mod serverbound_cookie_response_packet;
pub mod serverbound_custom_payload_packet;
pub mod serverbound_finish_configuration_packet;
pub mod serverbound_keep_alive_packet;
pub mod serverbound_pong_packet;
pub mod serverbound_resource_pack_packet;
pub mod serverbound_select_known_packs_packet;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    ConfigurationPacket,
    Serverbound => {
        0x00: serverbound_client_information_packet::ServerboundClientInformationPacket,
        0x01: serverbound_cookie_response_packet::ServerboundCookieResponsePacket,
        0x02: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        0x03: serverbound_finish_configuration_packet::ServerboundFinishConfigurationPacket,
        0x04: serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        0x05: serverbound_pong_packet::ServerboundPongPacket,
        0x06: serverbound_resource_pack_packet::ServerboundResourcePackPacket,
        0x07: serverbound_select_known_packs_packet::ServerboundSelectKnownPacksPacket,
    },
    Clientbound => {
        0x00: clientbound_cookie_request_packet::ClientboundCookieRequestPacket,
        0x01: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x02: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x03: clientbound_finish_configuration_packet::ClientboundFinishConfigurationPacket,
        0x04: clientbound_keep_alive_packet::ClientboundKeepAlivePacket,
        0x05: clientbound_ping_packet::ClientboundPingPacket,
        0x06: clientbound_reset_chat_packet::ClientboundResetChatPacket,
        0x07: clientbound_registry_data_packet::ClientboundRegistryDataPacket,
        0x08: clientbound_resource_pack_pop_packet::ClientboundResourcePackPopPacket,
        0x09: clientbound_resource_pack_push_packet::ClientboundResourcePackPushPacket,
        0x0a: clientbound_store_cookie_packet::ClientboundStoreCookiePacket,
        0x0b: clientbound_transfer_packet::ClientboundTransferPacket,
        0x0c: clientbound_update_enabled_features_packet::ClientboundUpdateEnabledFeaturesPacket,
        0x0d: clientbound_update_tags_packet::ClientboundUpdateTagsPacket,
        0x0e: clientbound_select_known_packs_packet::ClientboundSelectKnownPacksPacket,
    }
);
