pub mod clientbound_add_entity_packet;
pub mod clientbound_add_player_packet;
pub mod clientbound_animate_packet;
pub mod clientbound_block_changed_ack_packet;
pub mod clientbound_block_update_packet;
pub mod clientbound_change_difficulty_packet;
pub mod clientbound_chat_preview_packet;
pub mod clientbound_container_set_content_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_declare_commands_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_entity_event_packet;
pub mod clientbound_entity_velocity_packet;
pub mod clientbound_game_event_packet;
pub mod clientbound_initialize_border_packet;
pub mod clientbound_keep_alive_packet;
pub mod clientbound_level_chunk_with_light_packet;
pub mod clientbound_level_event_packet;
pub mod clientbound_level_particles_packet;
pub mod clientbound_light_update_packet;
pub mod clientbound_login_packet;
pub mod clientbound_move_entity_pos_packet;
pub mod clientbound_move_entity_posrot_packet;
pub mod clientbound_move_entity_rot_packet;
pub mod clientbound_player_abilities_packet;
pub mod clientbound_player_chat_packet;
pub mod clientbound_player_info_packet;
pub mod clientbound_player_position_packet;
pub mod clientbound_recipe_packet;
pub mod clientbound_remove_entities_packet;
pub mod clientbound_rotate_head_packet;
pub mod clientbound_section_blocks_update_packet;
pub mod clientbound_server_data_packet;
pub mod clientbound_set_carried_item_packet;
pub mod clientbound_set_chunk_cache_center_packet;
pub mod clientbound_set_default_spawn_position_packet;
pub mod clientbound_set_display_chat_preview_packet;
pub mod clientbound_set_entity_data_packet;
pub mod clientbound_set_entity_link_packet;
pub mod clientbound_set_equipment_packet;
pub mod clientbound_set_experience_packet;
pub mod clientbound_set_health_packet;
pub mod clientbound_set_time_packet;
pub mod clientbound_sound_packet;
pub mod clientbound_system_chat_packet;
pub mod clientbound_teleport_entity_packet;
pub mod clientbound_update_advancements_packet;
pub mod clientbound_update_attributes_packet;
pub mod clientbound_update_recipes_packet;
pub mod clientbound_update_tags_packet;
pub mod clientbound_update_view_distance_packet;
pub mod serverbound_chat_command_packet;
pub mod serverbound_chat_preview_packet;
pub mod serverbound_custom_payload_packet;
pub mod serverbound_keep_alive_packet;
pub mod serverbound_move_player_packet_pos;
pub mod serverbound_move_player_packet_pos_rot;
pub mod serverbound_move_player_packet_rot;
pub mod serverbound_move_player_packet_status_only;

use packet_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {
        0x03: serverbound_chat_command_packet::ServerboundChatCommandPacket,
        0x05: serverbound_chat_preview_packet::ServerboundChatPreviewPacket,
        0x0c: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        0x11: serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        0x13: serverbound_move_player_packet_pos::ServerboundMovePlayerPacketPos,
        0x14: serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot,
        0x15: serverbound_move_player_packet_rot::ServerboundMovePlayerPacketRot,
        0x16: serverbound_move_player_packet_status_only::ServerboundMovePlayerPacketStatusOnly,
    },
    Clientbound => {
        0x00: clientbound_add_entity_packet::ClientboundAddEntityPacket,
        0x02: clientbound_add_player_packet::ClientboundAddPlayerPacket,
        0x03: clientbound_animate_packet::ClientboundAnimatePacket,
        0x05: clientbound_block_changed_ack_packet::ClientboundBlockChangedAckPacket,
        0x09: clientbound_block_update_packet::ClientboundBlockUpdatePacket,
        0x0b: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x0c: clientbound_chat_preview_packet::ClientboundChatPreviewPacket,
        0x0f: clientbound_declare_commands_packet::ClientboundDeclareCommandsPacket,
        0x11: clientbound_container_set_content_packet::ClientboundContainerSetContentPacket,
        0x15: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x17: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x18: clientbound_entity_event_packet::ClientboundEntityEventPacket,
        0x1b: clientbound_game_event_packet::ClientboundGameEventPacket,
        0x1d: clientbound_initialize_border_packet::ClientboundInitializeBorderPacket,
        0x1e: clientbound_keep_alive_packet::ClientboundKeepAlivePacket,
        0x1f: clientbound_level_chunk_with_light_packet::ClientboundLevelChunkWithLightPacket,
        0x20: clientbound_level_event_packet::ClientboundLevelEventPacket,
        0x21: clientbound_level_particles_packet::ClientboundLevelParticlesPacket,
        0x22: clientbound_light_update_packet::ClientboundLightUpdatePacket,
        0x23: clientbound_login_packet::ClientboundLoginPacket,
        0x26: clientbound_move_entity_pos_packet::ClientboundMoveEntityPosPacket,
        0x27: clientbound_move_entity_posrot_packet::ClientboundMoveEntityPosRotPacket,
        0x28: clientbound_move_entity_rot_packet::ClientboundMoveEntityRotPacket,
        0x2f: clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
        0x30: clientbound_player_chat_packet::ClientboundPlayerChatPacket,
        0x34: clientbound_player_info_packet::ClientboundPlayerInfoPacket,
        0x36: clientbound_player_position_packet::ClientboundPlayerPositionPacket,
        0x37: clientbound_recipe_packet::ClientboundRecipePacket,
        0x38: clientbound_remove_entities_packet::ClientboundRemoveEntitiesPacket,
        0x3c: clientbound_rotate_head_packet::ClientboundRotateHeadPacket,
        0x3d: clientbound_section_blocks_update_packet::ClientboundSectionBlocksUpdatePacket,
        0x3f: clientbound_server_data_packet::ClientboundServerDataPacket,
        0x44: clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket,
        0x47: clientbound_set_carried_item_packet::ClientboundSetCarriedItemPacket,
        0x48: clientbound_set_chunk_cache_center_packet::ClientboundSetChunkCacheCenterPacket,
        0x49: clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
        0x4a: clientbound_set_default_spawn_position_packet::ClientboundSetDefaultSpawnPositionPacket,
        0x4b: clientbound_set_display_chat_preview_packet::ClientboundSetDisplayChatPreviewPacket,
        0x4d: clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket,
        0x4f: clientbound_entity_velocity_packet::ClientboundEntityVelocityPacket,
        0x50: clientbound_set_equipment_packet::ClientboundSetEquipmentPacket,
        0x51: clientbound_set_experience_packet::ClientboundSetExperiencePacket,
        0x52: clientbound_set_health_packet::ClientboundSetHealthPacket,
        0x59: clientbound_set_time_packet::ClientboundSetTimePacket,
        0x5d: clientbound_sound_packet::ClientboundSoundPacket,
        0x5f: clientbound_system_chat_packet::ClientboundSystemChatPacket,
        0x63: clientbound_teleport_entity_packet::ClientboundTeleportEntityPacket,
        0x64: clientbound_update_advancements_packet::ClientboundUpdateAdvancementsPacket,
        0x65: clientbound_update_attributes_packet::ClientboundUpdateAttributesPacket,
        0x67: clientbound_update_recipes_packet::ClientboundUpdateRecipesPacket,
        0x68: clientbound_update_tags_packet::ClientboundUpdateTagsPacket,
    }
);
