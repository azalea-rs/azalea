pub mod clientbound_add_entity_packet;
pub mod clientbound_add_experience_orb_packet;
pub mod clientbound_add_player_packet;
pub mod clientbound_animate_packet;
pub mod clientbound_award_stats_packet;
pub mod clientbound_block_changed_ack_packet;
pub mod clientbound_block_destruction_packet;
pub mod clientbound_block_entity_data_packet;
pub mod clientbound_block_event_packet;
pub mod clientbound_block_update_packet;
pub mod clientbound_boss_event_packet;
pub mod clientbound_change_difficulty_packet;
pub mod clientbound_chat_preview_packet;
pub mod clientbound_command_suggestions_packet;
pub mod clientbound_commands_packet;
pub mod clientbound_container_set_content_packet;
pub mod clientbound_container_set_data_packet;
pub mod clientbound_container_set_slot_packet;
pub mod clientbound_cooldown_packet;
pub mod clientbound_custom_chat_completions_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_custom_sound_packet;
pub mod clientbound_delete_chat_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_entity_event_packet;
pub mod clientbound_entity_velocity_packet;
pub mod clientbound_explode_packet;
pub mod clientbound_forget_level_chunk_packet;
pub mod clientbound_game_event_packet;
pub mod clientbound_horse_screen_open_packet;
pub mod clientbound_initialize_border_packet;
pub mod clientbound_keep_alive_packet;
pub mod clientbound_level_chunk_with_light_packet;
pub mod clientbound_level_event_packet;
pub mod clientbound_level_particles_packet;
pub mod clientbound_light_update_packet;
pub mod clientbound_login_packet;
pub mod clientbound_map_item_data_packet;
pub mod clientbound_move_entity_pos_packet;
pub mod clientbound_move_entity_posrot_packet;
pub mod clientbound_move_entity_rot_packet;
pub mod clientbound_player_abilities_packet;
pub mod clientbound_player_chat_header_packet;
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
pub mod clientbound_update_mob_effect_packet;
pub mod clientbound_update_recipes_packet;
pub mod clientbound_update_tags_packet;
pub mod clientbound_update_view_distance_packet;
pub mod serverbound_accept_teleportation_packet;
pub mod serverbound_block_entity_tag_query;
pub mod serverbound_change_difficulty_packet;
pub mod serverbound_chat_ack_packet;
pub mod serverbound_chat_command_packet;
pub mod serverbound_chat_packet;
pub mod serverbound_chat_preview_packet;
pub mod serverbound_client_command_packet;
pub mod serverbound_client_information_packet;
pub mod serverbound_command_suggestion_packet;
pub mod serverbound_container_button_click_packet;
pub mod serverbound_container_click_packet;
pub mod serverbound_container_close_packet;
pub mod serverbound_custom_payload_packet;
pub mod serverbound_edit_book_packet;
pub mod serverbound_entity_tag_query;
pub mod serverbound_interact_packet;
pub mod serverbound_jigsaw_generate_packet;
pub mod serverbound_keep_alive_packet;
pub mod serverbound_lock_difficulty_packet;
pub mod serverbound_move_player_packet_pos;
pub mod serverbound_move_player_packet_pos_rot;
pub mod serverbound_move_player_packet_rot;
pub mod serverbound_move_player_packet_status_only;
pub mod serverbound_move_vehicle_packet;
pub mod serverbound_paddle_boat_packet;
pub mod serverbound_pick_item_packet;
pub mod serverbound_place_recipe_packet;
pub mod serverbound_player_abilities_packet;
pub mod serverbound_player_action_packet;
pub mod serverbound_player_command_packet;
pub mod serverbound_player_input_packet;
pub mod serverbound_pong_packet;
pub mod serverbound_recipe_book_change_settings_packet;
pub mod serverbound_recipe_book_seen_recipe_packet;
pub mod serverbound_rename_item_packet;
pub mod serverbound_resource_pack_packet;
pub mod serverbound_seen_advancements_packet;
pub mod serverbound_select_trade_packet;
pub mod serverbound_set_beacon_packet;
pub mod serverbound_set_carried_item_packet;
pub mod serverbound_set_command_block_packet;
pub mod serverbound_set_command_minecart_packet;
pub mod serverbound_set_creative_mode_slot_packet;
pub mod serverbound_set_jigsaw_block_packet;
pub mod serverbound_set_structure_block_packet;
pub mod serverbound_sign_update_packet;
pub mod serverbound_swing_packet;
pub mod serverbound_teleport_to_entity_packet;
pub mod serverbound_use_item_on_packet;
pub mod serverbound_use_item_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {
        0x00: serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
        0x01: serverbound_block_entity_tag_query::ServerboundBlockEntityTagQuery,
        0x02: serverbound_change_difficulty_packet::ServerboundChangeDifficultyPacket,
        0x03: serverbound_chat_ack_packet::ServerboundChatAckPacket,
        0x04: serverbound_chat_command_packet::ServerboundChatCommandPacket,
        0x05: serverbound_chat_packet::ServerboundChatPacket,
        0x06: serverbound_chat_preview_packet::ServerboundChatPreviewPacket,
        0x07: serverbound_client_command_packet::ServerboundClientCommandPacket,
        0x08: serverbound_client_information_packet::ServerboundClientInformationPacket,
        0x09: serverbound_command_suggestion_packet::ServerboundCommandSuggestionPacket,
        0x0a: serverbound_container_button_click_packet::ServerboundContainerButtonClickPacket,
        0x0b: serverbound_container_click_packet::ServerboundContainerClickPacket,
        0x0c: serverbound_container_close_packet::ServerboundContainerClosePacket,
        0x0d: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        0x0e: serverbound_edit_book_packet::ServerboundEditBookPacket,
        0x0f: serverbound_entity_tag_query::ServerboundEntityTagQuery,
        0x10: serverbound_interact_packet::ServerboundInteractPacket,
        0x11: serverbound_jigsaw_generate_packet::ServerboundJigsawGeneratePacket,
        0x12: serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        0x13: serverbound_lock_difficulty_packet::ServerboundLockDifficultyPacket,
        0x14: serverbound_move_player_packet_pos::ServerboundMovePlayerPacketPos,
        0x15: serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot,
        0x16: serverbound_move_player_packet_rot::ServerboundMovePlayerPacketRot,
        0x17: serverbound_move_player_packet_status_only::ServerboundMovePlayerPacketStatusOnly,
        0x18: serverbound_move_vehicle_packet::ServerboundMoveVehiclePacket,
        0x19: serverbound_paddle_boat_packet::ServerboundPaddleBoatPacket,
        0x1a: serverbound_pick_item_packet::ServerboundPickItemPacket,
        0x1b: serverbound_place_recipe_packet::ServerboundPlaceRecipePacket,
        0x1c: serverbound_player_abilities_packet::ServerboundPlayerAbilitiesPacket,
        0x1d: serverbound_player_action_packet::ServerboundPlayerActionPacket,
        0x1e: serverbound_player_command_packet::ServerboundPlayerCommandPacket,
        0x1f: serverbound_player_input_packet::ServerboundPlayerInputPacket,
        0x20: serverbound_pong_packet::ServerboundPongPacket,
        0x21: serverbound_recipe_book_change_settings_packet::ServerboundRecipeBookChangeSettingsPacket,
        0x22: serverbound_recipe_book_seen_recipe_packet::ServerboundRecipeBookSeenRecipePacket,
        0x23: serverbound_rename_item_packet::ServerboundRenameItemPacket,
        0x24: serverbound_resource_pack_packet::ServerboundResourcePackPacket,
        0x25: serverbound_seen_advancements_packet::ServerboundSeenAdvancementsPacket,
        0x26: serverbound_select_trade_packet::ServerboundSelectTradePacket,
        0x27: serverbound_set_beacon_packet::ServerboundSetBeaconPacket,
        0x28: serverbound_set_carried_item_packet::ServerboundSetCarriedItemPacket,
        0x29: serverbound_set_command_block_packet::ServerboundSetCommandBlockPacket,
        0x2a: serverbound_set_command_minecart_packet::ServerboundSetCommandMinecartPacket,
        0x2b: serverbound_set_creative_mode_slot_packet::ServerboundSetCreativeModeSlotPacket,
        0x2c: serverbound_set_jigsaw_block_packet::ServerboundSetJigsawBlockPacket,
        0x2d: serverbound_set_structure_block_packet::ServerboundSetStructureBlockPacket,
        0x2e: serverbound_sign_update_packet::ServerboundSignUpdatePacket,
        0x2f: serverbound_swing_packet::ServerboundSwingPacket,
        0x30: serverbound_teleport_to_entity_packet::ServerboundTeleportToEntityPacket,
        0x31: serverbound_use_item_on_packet::ServerboundUseItemOnPacket,
        0x32: serverbound_use_item_packet::ServerboundUseItemPacket,
    },
    Clientbound => {
        0x00: clientbound_add_entity_packet::ClientboundAddEntityPacket,
        0x01: clientbound_add_experience_orb_packet::ClientboundAddExperienceOrbPacket,
        0x02: clientbound_add_player_packet::ClientboundAddPlayerPacket,
        0x03: clientbound_animate_packet::ClientboundAnimatePacket,
        0x04: clientbound_award_stats_packet::ClientboundAwardStatsPacket,
        0x05: clientbound_block_changed_ack_packet::ClientboundBlockChangedAckPacket,
        0x06: clientbound_block_destruction_packet::ClientboundBlockDestructionPacket,
        0x07: clientbound_block_entity_data_packet::ClientboundBlockEntityDataPacket,
        0x08: clientbound_block_event_packet::ClientboundBlockEventPacket,
        0x09: clientbound_block_update_packet::ClientboundBlockUpdatePacket,
        0x0a: clientbound_boss_event_packet::ClientboundBossEventPacket,
        0x0b: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x0c: clientbound_chat_preview_packet::ClientboundChatPreviewPacket,
        0x0e: clientbound_command_suggestions_packet::ClientboundCommandSuggestionsPacket,
        0x0f: clientbound_commands_packet::ClientboundCommandsPacket,
        0x11: clientbound_container_set_content_packet::ClientboundContainerSetContentPacket,
        0x12: clientbound_container_set_data_packet::ClientboundContainerSetDataPacket,
        0x13: clientbound_container_set_slot_packet::ClientboundContainerSetSlotPacket,
        0x14: clientbound_cooldown_packet::ClientboundCooldownPacket,
        0x15: clientbound_custom_chat_completions_packet::ClientboundCustomChatCompletionsPacket,
        0x16: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x17: clientbound_custom_sound_packet::ClientboundCustomSoundPacket,
        0x18: clientbound_delete_chat_packet::ClientboundDeleteChatPacket,
        0x19: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x1a: clientbound_entity_event_packet::ClientboundEntityEventPacket,
        0x1b: clientbound_explode_packet::ClientboundExplodePacket,
        0x1c: clientbound_forget_level_chunk_packet::ClientboundForgetLevelChunkPacket,
        0x1d: clientbound_game_event_packet::ClientboundGameEventPacket,
        0x1e: clientbound_horse_screen_open_packet::ClientboundHorseScreenOpenPacket,
        0x1f: clientbound_initialize_border_packet::ClientboundInitializeBorderPacket,
        0x20: clientbound_keep_alive_packet::ClientboundKeepAlivePacket,
        0x21: clientbound_level_chunk_with_light_packet::ClientboundLevelChunkWithLightPacket,
        0x22: clientbound_level_event_packet::ClientboundLevelEventPacket,
        0x23: clientbound_level_particles_packet::ClientboundLevelParticlesPacket,
        0x24: clientbound_light_update_packet::ClientboundLightUpdatePacket,
        0x25: clientbound_login_packet::ClientboundLoginPacket,
        0x26: clientbound_map_item_data_packet::ClientboundMapItemDataPacket,
        0x28: clientbound_move_entity_pos_packet::ClientboundMoveEntityPosPacket,
        0x29: clientbound_move_entity_posrot_packet::ClientboundMoveEntityPosrotPacket,
        0x2a: clientbound_move_entity_rot_packet::ClientboundMoveEntityRotPacket,
        0x31: clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
        0x32: clientbound_player_chat_header_packet::ClientboundPlayerChatHeaderPacket,
        0x33: clientbound_player_chat_packet::ClientboundPlayerChatPacket,
        0x37: clientbound_player_info_packet::ClientboundPlayerInfoPacket,
        0x39: clientbound_player_position_packet::ClientboundPlayerPositionPacket,
        0x3a: clientbound_recipe_packet::ClientboundRecipePacket,
        0x3b: clientbound_remove_entities_packet::ClientboundRemoveEntitiesPacket,
        0x3f: clientbound_rotate_head_packet::ClientboundRotateHeadPacket,
        0x40: clientbound_section_blocks_update_packet::ClientboundSectionBlocksUpdatePacket,
        0x42: clientbound_server_data_packet::ClientboundServerDataPacket,
        0x4a: clientbound_set_carried_item_packet::ClientboundSetCarriedItemPacket,
        0x4b: clientbound_set_chunk_cache_center_packet::ClientboundSetChunkCacheCenterPacket,
        0x4c: clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
        0x4d: clientbound_set_default_spawn_position_packet::ClientboundSetDefaultSpawnPositionPacket,
        0x4e: clientbound_set_display_chat_preview_packet::ClientboundSetDisplayChatPreviewPacket,
        0x50: clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket,
        0x51: clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket,
        0x52: clientbound_entity_velocity_packet::ClientboundEntityVelocityPacket,
        0x53: clientbound_set_equipment_packet::ClientboundSetEquipmentPacket,
        0x54: clientbound_set_experience_packet::ClientboundSetExperiencePacket,
        0x55: clientbound_set_health_packet::ClientboundSetHealthPacket,
        0x5c: clientbound_set_time_packet::ClientboundSetTimePacket,
        0x60: clientbound_sound_packet::ClientboundSoundPacket,
        0x62: clientbound_system_chat_packet::ClientboundSystemChatPacket,
        0x66: clientbound_teleport_entity_packet::ClientboundTeleportEntityPacket,
        0x67: clientbound_update_advancements_packet::ClientboundUpdateAdvancementsPacket,
        0x68: clientbound_update_attributes_packet::ClientboundUpdateAttributesPacket,
        0x69: clientbound_update_mob_effect_packet::ClientboundUpdateMobEffectPacket,
        0x6a: clientbound_update_recipes_packet::ClientboundUpdateRecipesPacket,
        0x6b: clientbound_update_tags_packet::ClientboundUpdateTagsPacket,
    }
);
