pub mod clientbound_add_entity_packet;
pub mod clientbound_add_mob_packet;
pub mod clientbound_change_difficulty_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_declare_commands_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_entity_event_packet;
pub mod clientbound_level_chunk_with_light_packet;
pub mod clientbound_light_update_packet;
pub mod clientbound_login_packet;
pub mod clientbound_player_abilities_packet;
pub mod clientbound_player_info_packet;
pub mod clientbound_player_position_packet;
pub mod clientbound_recipe_packet;
pub mod clientbound_set_carried_item_packet;
pub mod clientbound_set_chunk_cache_center;
pub mod clientbound_set_entity_data_packet;
pub mod clientbound_update_recipes_packet;
pub mod clientbound_update_tags_packet;
pub mod clientbound_update_view_distance_packet;
pub mod serverbound_custom_payload_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {
        0x0a: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
    },
    Clientbound => {
        0x00: clientbound_add_entity_packet::ClientboundAddEntityPacket,
        0x02: clientbound_add_mob_packet::ClientboundAddMobPacket,
        0x0e: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x12: clientbound_declare_commands_packet::ClientboundDeclareCommandsPacket,
        0x1a: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x1b: clientbound_entity_event_packet::ClientboundEntityEventPacket,
        0x18: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x22: clientbound_level_chunk_with_light_packet::ClientboundLevelChunkWithLightPacket,
        0x25: clientbound_light_update_packet::ClientboundLightUpdatePacket,
        0x26: clientbound_login_packet::ClientboundLoginPacket,
        0x32: clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
        0x36: clientbound_player_info_packet::ClientboundPlayerInfoPacket,
        0x38: clientbound_player_position_packet::ClientboundPlayerPositionPacket,
        0x39: clientbound_recipe_packet::ClientboundRecipePacket,
        0x48: clientbound_set_carried_item_packet::ClientboundSetCarriedItemPacket,
        0x49: clientbound_set_chunk_cache_center::ClientboundSetChunkCacheCenterPacket,
        0x4a: clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
        0x4d: clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket,
        0x66: clientbound_update_recipes_packet::ClientboundUpdateRecipesPacket,
        0x67: clientbound_update_tags_packet::ClientboundUpdateTagsPacket
    }
);
