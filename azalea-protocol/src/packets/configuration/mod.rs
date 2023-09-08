pub mod clientbound_custom_payload_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_finish_configuration_packet;
pub mod clientbound_keep_alive_packet;
pub mod clientbound_ping_packet;
pub mod clientbound_registry_data_packet;
pub mod clientbound_resource_pack_packet;
pub mod clientbound_update_enabled_features_packet;
pub mod clientbound_update_tags_packet;
pub mod serverbound_client_information_packet;
pub mod serverbound_custom_payload_packet;
pub mod serverbound_finish_configuration_packet;
pub mod serverbound_keep_alive_packet;
pub mod serverbound_pong_packet;
pub mod serverbound_resource_pack_packet;
use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    ConfigurationPacket,
    Serverbound => {
        0x00: serverbound_client_information_packet::ServerboundClientInformationPacket,
        0x01: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        0x02: serverbound_finish_configuration_packet::ServerboundFinishConfigurationPacket,
        0x03: serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        0x04: serverbound_pong_packet::ServerboundPongPacket,
        0x05: serverbound_resource_pack_packet::ServerboundResourcePackPacket,
    },
    Clientbound => {
        0x00: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x01: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x02: clientbound_finish_configuration_packet::ClientboundFinishConfigurationPacket,
        0x03: clientbound_keep_alive_packet::ClientboundKeepAlivePacket,
        0x04: clientbound_ping_packet::ClientboundPingPacket,
        0x05: clientbound_registry_data_packet::ClientboundRegistryDataPacket,
        0x06: clientbound_resource_pack_packet::ClientboundResourcePackPacket,
        0x07: clientbound_update_enabled_features_packet::ClientboundUpdateEnabledFeaturesPacket,
        0x08: clientbound_update_tags_packet::ClientboundUpdateTagsPacket,
    }
);
