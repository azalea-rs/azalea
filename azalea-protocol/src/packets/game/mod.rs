pub mod clientbound_add_entity_packet;
pub mod clientbound_add_experience_orb_packet;
pub mod clientbound_animate_packet;
pub mod clientbound_award_stats_packet;
pub mod clientbound_block_changed_ack_packet;
pub mod clientbound_block_destruction_packet;
pub mod clientbound_block_entity_data_packet;
pub mod clientbound_block_event_packet;
pub mod clientbound_block_update_packet;
pub mod clientbound_boss_event_packet;
pub mod clientbound_bundle_packet;
pub mod clientbound_change_difficulty_packet;
pub mod clientbound_chunk_batch_finished_packet;
pub mod clientbound_chunk_batch_start_packet;
pub mod clientbound_chunks_biomes_packet;
pub mod clientbound_clear_titles_packet;
pub mod clientbound_command_suggestions_packet;
pub mod clientbound_commands_packet;
pub mod clientbound_container_close_packet;
pub mod clientbound_container_set_content_packet;
pub mod clientbound_container_set_data_packet;
pub mod clientbound_container_set_slot_packet;
pub mod clientbound_cookie_request_packet;
pub mod clientbound_cooldown_packet;
pub mod clientbound_custom_chat_completions_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_damage_event_packet;
pub mod clientbound_debug_sample_packet;
pub mod clientbound_delete_chat_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_disguised_chat_packet;
pub mod clientbound_entity_event_packet;
pub mod clientbound_explode_packet;
pub mod clientbound_forget_level_chunk_packet;
pub mod clientbound_game_event_packet;
pub mod clientbound_horse_screen_open_packet;
pub mod clientbound_hurt_animation_packet;
pub mod clientbound_initialize_border_packet;
pub mod clientbound_keep_alive_packet;
pub mod clientbound_level_chunk_with_light_packet;
pub mod clientbound_level_event_packet;
pub mod clientbound_level_particles_packet;
pub mod clientbound_light_update_packet;
pub mod clientbound_login_packet;
pub mod clientbound_map_item_data_packet;
pub mod clientbound_merchant_offers_packet;
pub mod clientbound_move_entity_pos_packet;
pub mod clientbound_move_entity_pos_rot_packet;
pub mod clientbound_move_entity_rot_packet;
pub mod clientbound_move_vehicle_packet;
pub mod clientbound_open_book_packet;
pub mod clientbound_open_screen_packet;
pub mod clientbound_open_sign_editor_packet;
pub mod clientbound_ping_packet;
pub mod clientbound_place_ghost_recipe_packet;
pub mod clientbound_player_abilities_packet;
pub mod clientbound_player_chat_packet;
pub mod clientbound_player_combat_end_packet;
pub mod clientbound_player_combat_enter_packet;
pub mod clientbound_player_combat_kill_packet;
pub mod clientbound_player_info_remove_packet;
pub mod clientbound_player_info_update_packet;
pub mod clientbound_player_look_at_packet;
pub mod clientbound_player_position_packet;
pub mod clientbound_pong_response_packet;
pub mod clientbound_recipe_packet;
pub mod clientbound_remove_entities_packet;
pub mod clientbound_remove_mob_effect_packet;
pub mod clientbound_reset_score_packet;
pub mod clientbound_resource_pack_pop_packet;
pub mod clientbound_resource_pack_push_packet;
pub mod clientbound_respawn_packet;
pub mod clientbound_rotate_head_packet;
pub mod clientbound_section_blocks_update_packet;
pub mod clientbound_select_advancements_tab_packet;
pub mod clientbound_server_data_packet;
pub mod clientbound_set_action_bar_text_packet;
pub mod clientbound_set_border_center_packet;
pub mod clientbound_set_border_lerp_size_packet;
pub mod clientbound_set_border_size_packet;
pub mod clientbound_set_border_warning_delay_packet;
pub mod clientbound_set_border_warning_distance_packet;
pub mod clientbound_set_camera_packet;
pub mod clientbound_set_carried_item_packet;
pub mod clientbound_set_chunk_cache_center_packet;
pub mod clientbound_set_chunk_cache_radius_packet;
pub mod clientbound_set_default_spawn_position_packet;
pub mod clientbound_set_display_objective_packet;
pub mod clientbound_set_entity_data_packet;
pub mod clientbound_set_entity_link_packet;
pub mod clientbound_set_entity_motion_packet;
pub mod clientbound_set_equipment_packet;
pub mod clientbound_set_experience_packet;
pub mod clientbound_set_health_packet;
pub mod clientbound_set_objective_packet;
pub mod clientbound_set_passengers_packet;
pub mod clientbound_set_player_team_packet;
pub mod clientbound_set_score_packet;
pub mod clientbound_set_simulation_distance_packet;
pub mod clientbound_set_subtitle_text_packet;
pub mod clientbound_set_time_packet;
pub mod clientbound_set_title_text_packet;
pub mod clientbound_set_titles_animation_packet;
pub mod clientbound_sound_entity_packet;
pub mod clientbound_sound_packet;
pub mod clientbound_start_configuration_packet;
pub mod clientbound_stop_sound_packet;
pub mod clientbound_store_cookie_packet;
pub mod clientbound_system_chat_packet;
pub mod clientbound_tab_list_packet;
pub mod clientbound_tag_query_packet;
pub mod clientbound_take_item_entity_packet;
pub mod clientbound_teleport_entity_packet;
pub mod clientbound_ticking_state_packet;
pub mod clientbound_ticking_step_packet;
pub mod clientbound_transfer_packet;
pub mod clientbound_update_advancements_packet;
pub mod clientbound_update_attributes_packet;
pub mod clientbound_update_mob_effect_packet;
pub mod clientbound_update_recipes_packet;
pub mod clientbound_update_tags_packet;
pub mod serverbound_accept_teleportation_packet;
pub mod serverbound_block_entity_tag_query_packet;
pub mod serverbound_change_difficulty_packet;
pub mod serverbound_chat_ack_packet;
pub mod serverbound_chat_command_packet;
pub mod serverbound_chat_command_signed_packet;
pub mod serverbound_chat_packet;
pub mod serverbound_chat_session_update_packet;
pub mod serverbound_chunk_batch_received_packet;
pub mod serverbound_client_command_packet;
pub mod serverbound_client_information_packet;
pub mod serverbound_command_suggestion_packet;
pub mod serverbound_configuration_acknowledged_packet;
pub mod serverbound_container_button_click_packet;
pub mod serverbound_container_click_packet;
pub mod serverbound_container_close_packet;
pub mod serverbound_container_slot_state_changed_packet;
pub mod serverbound_cookie_response_packet;
pub mod serverbound_custom_payload_packet;
pub mod serverbound_debug_sample_subscription;
pub mod serverbound_edit_book_packet;
pub mod serverbound_entity_tag_query_packet;
pub mod serverbound_interact_packet;
pub mod serverbound_jigsaw_generate_packet;
pub mod serverbound_keep_alive_packet;
pub mod serverbound_lock_difficulty_packet;
pub mod serverbound_move_player_pos_packet;
pub mod serverbound_move_player_pos_rot_packet;
pub mod serverbound_move_player_rot_packet;
pub mod serverbound_move_player_status_only_packet;
pub mod serverbound_move_vehicle_packet;
pub mod serverbound_paddle_boat_packet;
pub mod serverbound_pick_item_packet;
pub mod serverbound_ping_request_packet;
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

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {
        0x00: serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
        0x01: serverbound_block_entity_tag_query_packet::ServerboundBlockEntityTagQueryPacket,
        0x02: serverbound_change_difficulty_packet::ServerboundChangeDifficultyPacket,
        0x03: serverbound_chat_ack_packet::ServerboundChatAckPacket,
        0x04: serverbound_chat_command_packet::ServerboundChatCommandPacket,
        0x05: serverbound_chat_command_signed_packet::ServerboundChatCommandSignedPacket,
        0x06: serverbound_chat_packet::ServerboundChatPacket,
        0x07: serverbound_chat_session_update_packet::ServerboundChatSessionUpdatePacket,
        0x08: serverbound_chunk_batch_received_packet::ServerboundChunkBatchReceivedPacket,
        0x09: serverbound_client_command_packet::ServerboundClientCommandPacket,
        0x0a: serverbound_client_information_packet::ServerboundClientInformationPacket,
        0x0b: serverbound_command_suggestion_packet::ServerboundCommandSuggestionPacket,
        0x0c: serverbound_configuration_acknowledged_packet::ServerboundConfigurationAcknowledgedPacket,
        0x0d: serverbound_container_button_click_packet::ServerboundContainerButtonClickPacket,
        0x0e: serverbound_container_click_packet::ServerboundContainerClickPacket,
        0x0f: serverbound_container_close_packet::ServerboundContainerClosePacket,
        0x10: serverbound_container_slot_state_changed_packet::ServerboundContainerSlotStateChangedPacket,
        0x11: serverbound_cookie_response_packet::ServerboundCookieResponsePacket,
        0x12: serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        0x13: serverbound_debug_sample_subscription::ServerboundDebugSampleSubscription,
        0x14: serverbound_edit_book_packet::ServerboundEditBookPacket,
        0x15: serverbound_entity_tag_query_packet::ServerboundEntityTagQueryPacket,
        0x16: serverbound_interact_packet::ServerboundInteractPacket,
        0x17: serverbound_jigsaw_generate_packet::ServerboundJigsawGeneratePacket,
        0x18: serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        0x19: serverbound_lock_difficulty_packet::ServerboundLockDifficultyPacket,
        0x1a: serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
        0x1b: serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
        0x1c: serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
        0x1d: serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
        0x1e: serverbound_move_vehicle_packet::ServerboundMoveVehiclePacket,
        0x1f: serverbound_paddle_boat_packet::ServerboundPaddleBoatPacket,
        0x20: serverbound_pick_item_packet::ServerboundPickItemPacket,
        0x21: serverbound_ping_request_packet::ServerboundPingRequestPacket,
        0x22: serverbound_place_recipe_packet::ServerboundPlaceRecipePacket,
        0x23: serverbound_player_abilities_packet::ServerboundPlayerAbilitiesPacket,
        0x24: serverbound_player_action_packet::ServerboundPlayerActionPacket,
        0x25: serverbound_player_command_packet::ServerboundPlayerCommandPacket,
        0x26: serverbound_player_input_packet::ServerboundPlayerInputPacket,
        0x27: serverbound_pong_packet::ServerboundPongPacket,
        0x28: serverbound_recipe_book_change_settings_packet::ServerboundRecipeBookChangeSettingsPacket,
        0x29: serverbound_recipe_book_seen_recipe_packet::ServerboundRecipeBookSeenRecipePacket,
        0x2a: serverbound_rename_item_packet::ServerboundRenameItemPacket,
        0x2b: serverbound_resource_pack_packet::ServerboundResourcePackPacket,
        0x2c: serverbound_seen_advancements_packet::ServerboundSeenAdvancementsPacket,
        0x2d: serverbound_select_trade_packet::ServerboundSelectTradePacket,
        0x2e: serverbound_set_beacon_packet::ServerboundSetBeaconPacket,
        0x2f: serverbound_set_carried_item_packet::ServerboundSetCarriedItemPacket,
        0x30: serverbound_set_command_block_packet::ServerboundSetCommandBlockPacket,
        0x31: serverbound_set_command_minecart_packet::ServerboundSetCommandMinecartPacket,
        0x32: serverbound_set_creative_mode_slot_packet::ServerboundSetCreativeModeSlotPacket,
        0x33: serverbound_set_jigsaw_block_packet::ServerboundSetJigsawBlockPacket,
        0x34: serverbound_set_structure_block_packet::ServerboundSetStructureBlockPacket,
        0x35: serverbound_sign_update_packet::ServerboundSignUpdatePacket,
        0x36: serverbound_swing_packet::ServerboundSwingPacket,
        0x37: serverbound_teleport_to_entity_packet::ServerboundTeleportToEntityPacket,
        0x38: serverbound_use_item_on_packet::ServerboundUseItemOnPacket,
        0x39: serverbound_use_item_packet::ServerboundUseItemPacket,
    },
    Clientbound => {
        0x00: clientbound_bundle_packet::ClientboundBundlePacket,
        0x01: clientbound_add_entity_packet::ClientboundAddEntityPacket,
        0x02: clientbound_add_experience_orb_packet::ClientboundAddExperienceOrbPacket,
        0x03: clientbound_animate_packet::ClientboundAnimatePacket,
        0x04: clientbound_award_stats_packet::ClientboundAwardStatsPacket,
        0x05: clientbound_block_changed_ack_packet::ClientboundBlockChangedAckPacket,
        0x06: clientbound_block_destruction_packet::ClientboundBlockDestructionPacket,
        0x07: clientbound_block_entity_data_packet::ClientboundBlockEntityDataPacket,
        0x08: clientbound_block_event_packet::ClientboundBlockEventPacket,
        0x09: clientbound_block_update_packet::ClientboundBlockUpdatePacket,
        0x0a: clientbound_boss_event_packet::ClientboundBossEventPacket,
        0x0b: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x0c: clientbound_chunk_batch_finished_packet::ClientboundChunkBatchFinishedPacket,
        0x0d: clientbound_chunk_batch_start_packet::ClientboundChunkBatchStartPacket,
        0x0e: clientbound_chunks_biomes_packet::ClientboundChunksBiomesPacket,
        0x0f: clientbound_clear_titles_packet::ClientboundClearTitlesPacket,
        0x10: clientbound_command_suggestions_packet::ClientboundCommandSuggestionsPacket,
        0x11: clientbound_commands_packet::ClientboundCommandsPacket,
        0x12: clientbound_container_close_packet::ClientboundContainerClosePacket,
        0x13: clientbound_container_set_content_packet::ClientboundContainerSetContentPacket,
        0x14: clientbound_container_set_data_packet::ClientboundContainerSetDataPacket,
        0x15: clientbound_container_set_slot_packet::ClientboundContainerSetSlotPacket,
        0x16: clientbound_cookie_request_packet::ClientboundCookieRequestPacket,
        0x17: clientbound_cooldown_packet::ClientboundCooldownPacket,
        0x18: clientbound_custom_chat_completions_packet::ClientboundCustomChatCompletionsPacket,
        0x19: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x1a: clientbound_damage_event_packet::ClientboundDamageEventPacket,
        0x1b: clientbound_debug_sample_packet::ClientboundDebugSamplePacket,
        0x1c: clientbound_delete_chat_packet::ClientboundDeleteChatPacket,
        0x1d: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x1e: clientbound_disguised_chat_packet::ClientboundDisguisedChatPacket,
        0x1f: clientbound_entity_event_packet::ClientboundEntityEventPacket,
        0x20: clientbound_explode_packet::ClientboundExplodePacket,
        0x21: clientbound_forget_level_chunk_packet::ClientboundForgetLevelChunkPacket,
        0x22: clientbound_game_event_packet::ClientboundGameEventPacket,
        0x23: clientbound_horse_screen_open_packet::ClientboundHorseScreenOpenPacket,
        0x24: clientbound_hurt_animation_packet::ClientboundHurtAnimationPacket,
        0x25: clientbound_initialize_border_packet::ClientboundInitializeBorderPacket,
        0x26: clientbound_keep_alive_packet::ClientboundKeepAlivePacket,
        0x27: clientbound_level_chunk_with_light_packet::ClientboundLevelChunkWithLightPacket,
        0x28: clientbound_level_event_packet::ClientboundLevelEventPacket,
        0x29: clientbound_level_particles_packet::ClientboundLevelParticlesPacket,
        0x2a: clientbound_light_update_packet::ClientboundLightUpdatePacket,
        0x2b: clientbound_login_packet::ClientboundLoginPacket,
        0x2c: clientbound_map_item_data_packet::ClientboundMapItemDataPacket,
        0x2d: clientbound_merchant_offers_packet::ClientboundMerchantOffersPacket,
        0x2e: clientbound_move_entity_pos_packet::ClientboundMoveEntityPosPacket,
        0x2f: clientbound_move_entity_pos_rot_packet::ClientboundMoveEntityPosRotPacket,
        0x30: clientbound_move_entity_rot_packet::ClientboundMoveEntityRotPacket,
        0x31: clientbound_move_vehicle_packet::ClientboundMoveVehiclePacket,
        0x32: clientbound_open_book_packet::ClientboundOpenBookPacket,
        0x33: clientbound_open_screen_packet::ClientboundOpenScreenPacket,
        0x34: clientbound_open_sign_editor_packet::ClientboundOpenSignEditorPacket,
        0x35: clientbound_ping_packet::ClientboundPingPacket,
        0x36: clientbound_pong_response_packet::ClientboundPongResponsePacket,
        0x37: clientbound_place_ghost_recipe_packet::ClientboundPlaceGhostRecipePacket,
        0x38: clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
        0x39: clientbound_player_chat_packet::ClientboundPlayerChatPacket,
        0x3a: clientbound_player_combat_end_packet::ClientboundPlayerCombatEndPacket,
        0x3b: clientbound_player_combat_enter_packet::ClientboundPlayerCombatEnterPacket,
        0x3c: clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket,
        0x3d: clientbound_player_info_remove_packet::ClientboundPlayerInfoRemovePacket,
        0x3e: clientbound_player_info_update_packet::ClientboundPlayerInfoUpdatePacket,
        0x3f: clientbound_player_look_at_packet::ClientboundPlayerLookAtPacket,
        0x40: clientbound_player_position_packet::ClientboundPlayerPositionPacket,
        0x41: clientbound_recipe_packet::ClientboundRecipePacket,
        0x42: clientbound_remove_entities_packet::ClientboundRemoveEntitiesPacket,
        0x43: clientbound_remove_mob_effect_packet::ClientboundRemoveMobEffectPacket,
        0x44: clientbound_reset_score_packet::ClientboundResetScorePacket,
        0x45: clientbound_resource_pack_pop_packet::ClientboundResourcePackPopPacket,
        0x46: clientbound_resource_pack_push_packet::ClientboundResourcePackPushPacket,
        0x47: clientbound_respawn_packet::ClientboundRespawnPacket,
        0x48: clientbound_rotate_head_packet::ClientboundRotateHeadPacket,
        0x49: clientbound_section_blocks_update_packet::ClientboundSectionBlocksUpdatePacket,
        0x4a: clientbound_select_advancements_tab_packet::ClientboundSelectAdvancementsTabPacket,
        0x4b: clientbound_server_data_packet::ClientboundServerDataPacket,
        0x4c: clientbound_set_action_bar_text_packet::ClientboundSetActionBarTextPacket,
        0x4d: clientbound_set_border_center_packet::ClientboundSetBorderCenterPacket,
        0x4e: clientbound_set_border_lerp_size_packet::ClientboundSetBorderLerpSizePacket,
        0x4f: clientbound_set_border_size_packet::ClientboundSetBorderSizePacket,
        0x50: clientbound_set_border_warning_delay_packet::ClientboundSetBorderWarningDelayPacket,
        0x51: clientbound_set_border_warning_distance_packet::ClientboundSetBorderWarningDistancePacket,
        0x52: clientbound_set_camera_packet::ClientboundSetCameraPacket,
        0x53: clientbound_set_carried_item_packet::ClientboundSetCarriedItemPacket,
        0x54: clientbound_set_chunk_cache_center_packet::ClientboundSetChunkCacheCenterPacket,
        0x55: clientbound_set_chunk_cache_radius_packet::ClientboundSetChunkCacheRadiusPacket,
        0x56: clientbound_set_default_spawn_position_packet::ClientboundSetDefaultSpawnPositionPacket,
        0x57: clientbound_set_display_objective_packet::ClientboundSetDisplayObjectivePacket,
        0x58: clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket,
        0x59: clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket,
        0x5a: clientbound_set_entity_motion_packet::ClientboundSetEntityMotionPacket,
        0x5b: clientbound_set_equipment_packet::ClientboundSetEquipmentPacket,
        0x5c: clientbound_set_experience_packet::ClientboundSetExperiencePacket,
        0x5d: clientbound_set_health_packet::ClientboundSetHealthPacket,
        0x5e: clientbound_set_objective_packet::ClientboundSetObjectivePacket,
        0x5f: clientbound_set_passengers_packet::ClientboundSetPassengersPacket,
        0x60: clientbound_set_player_team_packet::ClientboundSetPlayerTeamPacket,
        0x61: clientbound_set_score_packet::ClientboundSetScorePacket,
        0x62: clientbound_set_simulation_distance_packet::ClientboundSetSimulationDistancePacket,
        0x63: clientbound_set_subtitle_text_packet::ClientboundSetSubtitleTextPacket,
        0x64: clientbound_set_time_packet::ClientboundSetTimePacket,
        0x65: clientbound_set_title_text_packet::ClientboundSetTitleTextPacket,
        0x66: clientbound_set_titles_animation_packet::ClientboundSetTitlesAnimationPacket,
        0x67: clientbound_sound_entity_packet::ClientboundSoundEntityPacket,
        0x68: clientbound_sound_packet::ClientboundSoundPacket,
        0x69: clientbound_start_configuration_packet::ClientboundStartConfigurationPacket,
        0x6a: clientbound_stop_sound_packet::ClientboundStopSoundPacket,
        0x6b: clientbound_store_cookie_packet::ClientboundStoreCookiePacket,
        0x6c: clientbound_system_chat_packet::ClientboundSystemChatPacket,
        0x6d: clientbound_tab_list_packet::ClientboundTabListPacket,
        0x6e: clientbound_tag_query_packet::ClientboundTagQueryPacket,
        0x6f: clientbound_take_item_entity_packet::ClientboundTakeItemEntityPacket,
        0x70: clientbound_teleport_entity_packet::ClientboundTeleportEntityPacket,
        0x71: clientbound_ticking_state_packet::ClientboundTickingStatePacket,
        0x72: clientbound_ticking_step_packet::ClientboundTickingStepPacket,
        0x73: clientbound_transfer_packet::ClientboundTransferPacket,
        0x74: clientbound_update_advancements_packet::ClientboundUpdateAdvancementsPacket,
        0x75: clientbound_update_attributes_packet::ClientboundUpdateAttributesPacket,
        0x76: clientbound_update_mob_effect_packet::ClientboundUpdateMobEffectPacket,
        0x77: clientbound_update_recipes_packet::ClientboundUpdateRecipesPacket,
        0x78: clientbound_update_tags_packet::ClientboundUpdateTagsPacket,
    }
);
