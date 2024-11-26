pub mod c_add_entity;
pub mod c_add_experience_orb;
pub mod c_animate;
pub mod c_award_stats;
pub mod c_block_changed_ack;
pub mod c_block_destruction;
pub mod c_block_entity_data;
pub mod c_block_event;
pub mod c_block_update;
pub mod c_boss_event;
pub mod c_bundle;
pub mod c_change_difficulty;
pub mod c_chunk_batch_finished;
pub mod c_chunk_batch_start;
pub mod c_chunks_biomes;
pub mod c_clear_titles;
pub mod c_command_suggestions;
pub mod c_commands;
pub mod c_container_close;
pub mod c_container_set_content;
pub mod c_container_set_data;
pub mod c_container_set_slot;
pub mod c_cookie_request;
pub mod c_cooldown;
pub mod c_custom_chat_completions;
pub mod c_custom_payload;
pub mod c_custom_report_details;
pub mod c_damage_event;
pub mod c_debug_sample;
pub mod c_delete_chat;
pub mod c_disconnect;
pub mod c_disguised_chat;
pub mod c_entity_event;
pub mod c_entity_position_sync;
pub mod c_explode;
pub mod c_forget_level_chunk;
pub mod c_game_event;
pub mod c_horse_screen_open;
pub mod c_hurt_animation;
pub mod c_initialize_border;
pub mod c_keep_alive;
pub mod c_level_chunk_with_light;
pub mod c_level_event;
pub mod c_level_particles;
pub mod c_light_update;
pub mod c_login;
pub mod c_map_item_data;
pub mod c_merchant_offers;
pub mod c_move_entity_pos;
pub mod c_move_entity_pos_rot;
pub mod c_move_entity_rot;
pub mod c_move_minecart;
pub mod c_move_vehicle;
pub mod c_open_book;
pub mod c_open_screen;
pub mod c_open_sign_editor;
pub mod c_ping;
pub mod c_place_ghost_recipe;
pub mod c_player_abilities;
pub mod c_player_chat;
pub mod c_player_combat_end;
pub mod c_player_combat_enter;
pub mod c_player_combat_kill;
pub mod c_player_info_remove;
pub mod c_player_info_update;
pub mod c_player_look_at;
pub mod c_player_position;
pub mod c_player_rotation;
pub mod c_pong_response;
pub mod c_projectile_power;
pub mod c_recipe_book_add;
pub mod c_recipe_book_remove;
pub mod c_recipe_book_settings;
pub mod c_remove_entities;
pub mod c_remove_mob_effect;
pub mod c_reset_score;
pub mod c_resource_pack_pop;
pub mod c_resource_pack_push;
pub mod c_respawn;
pub mod c_rotate_head;
pub mod c_section_blocks_update;
pub mod c_select_advancements_tab;
pub mod c_server_data;
pub mod c_server_links;
pub mod c_set_action_bar_text;
pub mod c_set_border_center;
pub mod c_set_border_lerp_size;
pub mod c_set_border_size;
pub mod c_set_border_warning_delay;
pub mod c_set_border_warning_distance;
pub mod c_set_camera;
pub mod c_set_chunk_cache_center;
pub mod c_set_chunk_cache_radius;
pub mod c_set_cursor_item;
pub mod c_set_default_spawn_position;
pub mod c_set_display_objective;
pub mod c_set_entity_data;
pub mod c_set_entity_link;
pub mod c_set_entity_motion;
pub mod c_set_equipment;
pub mod c_set_experience;
pub mod c_set_health;
pub mod c_set_held_slot;
pub mod c_set_objective;
pub mod c_set_passengers;
pub mod c_set_player_inventory;
pub mod c_set_player_team;
pub mod c_set_score;
pub mod c_set_simulation_distance;
pub mod c_set_subtitle_text;
pub mod c_set_time;
pub mod c_set_title_text;
pub mod c_set_titles_animation;
pub mod c_sound;
pub mod c_sound_entity;
pub mod c_start_configuration;
pub mod c_stop_sound;
pub mod c_store_cookie;
pub mod c_system_chat;
pub mod c_tab_list;
pub mod c_tag_query;
pub mod c_take_item_entity;
pub mod c_teleport_entity;
pub mod c_ticking_state;
pub mod c_ticking_step;
pub mod c_transfer;
pub mod c_update_advancements;
pub mod c_update_attributes;
pub mod c_update_mob_effect;
pub mod c_update_recipes;
pub mod c_update_tags;
pub mod s_accept_teleportation;
pub mod s_block_entity_tag_query;
pub mod s_change_difficulty;
pub mod s_chat;
pub mod s_chat_ack;
pub mod s_chat_command;
pub mod s_chat_command_signed;
pub mod s_chat_session_update;
pub mod s_chunk_batch_received;
pub mod s_client_command;
pub mod s_client_information;
pub mod s_client_tick_end;
pub mod s_command_suggestion;
pub mod s_configuration_acknowledged;
pub mod s_container_button_click;
pub mod s_container_click;
pub mod s_container_close;
pub mod s_container_slot_state_changed;
pub mod s_cookie_response;
pub mod s_custom_payload;
pub mod s_debug_sample_subscription;
pub mod s_edit_book;
pub mod s_entity_tag_query;
pub mod s_interact;
pub mod s_jigsaw_generate;
pub mod s_keep_alive;
pub mod s_lock_difficulty;
pub mod s_move_player_pos;
pub mod s_move_player_pos_rot;
pub mod s_move_player_rot;
pub mod s_move_player_status_only;
pub mod s_move_vehicle;
pub mod s_paddle_boat;
pub mod s_pick_item;
pub mod s_ping_request;
pub mod s_place_recipe;
pub mod s_player_abilities;
pub mod s_player_action;
pub mod s_player_command;
pub mod s_player_input;
pub mod s_pong;
pub mod s_recipe_book_change_settings;
pub mod s_recipe_book_seen_recipe;
pub mod s_rename_item;
pub mod s_resource_pack;
pub mod s_seen_advancements;
pub mod s_select_bundle_item;
pub mod s_select_trade;
pub mod s_set_beacon;
pub mod s_set_carried_item;
pub mod s_set_command_block;
pub mod s_set_command_minecart;
pub mod s_set_creative_mode_slot;
pub mod s_set_jigsaw_block;
pub mod s_set_structure_block;
pub mod s_sign_update;
pub mod s_swing;
pub mod s_teleport_to_entity;
pub mod s_use_item;
pub mod s_use_item_on;

use azalea_protocol_macros::declare_state_packets;

// see GameProtocols.java in the decompiled vanilla source

declare_state_packets!(
    GamePacket,
    Serverbound => {
        0x00: s_accept_teleportation::ServerboundAcceptTeleportation,
        0x01: s_block_entity_tag_query::ServerboundBlockEntityTagQuery,
        0x02: s_select_bundle_item::ServerboundSelectBundleItem,
        0x03: s_change_difficulty::ServerboundChangeDifficulty,
        0x04: s_chat_ack::ServerboundChatAck,
        0x05: s_chat_command::ServerboundChatCommand,
        0x06: s_chat_command_signed::ServerboundChatCommandSigned,
        0x07: s_chat::ServerboundChat,
        0x08: s_chat_session_update::ServerboundChatSessionUpdate,
        0x09: s_chunk_batch_received::ServerboundChunkBatchReceived,
        0x0a: s_client_command::ServerboundClientCommand,
        0x0b: s_client_tick_end::ServerboundTickEnd,
        0x0c: s_client_information::ServerboundClientInformation,
        0x0d: s_command_suggestion::ServerboundCommandSuggestion,
        0x0e: s_configuration_acknowledged::ServerboundConfigurationAcknowledged,
        0x0f: s_container_button_click::ServerboundContainerButtonClick,
        0x10: s_container_click::ServerboundContainerClick,
        0x11: s_container_close::ServerboundContainerClose,
        0x12: s_container_slot_state_changed::ServerboundContainerSlotStateChanged,
        0x13: s_cookie_response::ServerboundCookieResponse,
        0x14: s_custom_payload::ServerboundCustomPayload,
        0x15: s_debug_sample_subscription::ServerboundDebugSampleSubscription,
        0x16: s_edit_book::ServerboundEditBook,
        0x17: s_entity_tag_query::ServerboundEntityTagQuery,
        0x18: s_interact::ServerboundInteract,
        0x19: s_jigsaw_generate::ServerboundJigsawGenerate,
        0x1a: s_keep_alive::ServerboundKeepAlive,
        0x1b: s_lock_difficulty::ServerboundLockDifficulty,
        0x1c: s_move_player_pos::ServerboundMovePlayerPos,
        0x1d: s_move_player_pos_rot::ServerboundMovePlayerPosRot,
        0x1e: s_move_player_rot::ServerboundMovePlayerRot,
        0x1f: s_move_player_status_only::ServerboundMovePlayerStatusOnly,
        0x20: s_move_vehicle::ServerboundMoveVehicle,
        0x21: s_paddle_boat::ServerboundPaddleBoat,
        0x22: s_pick_item::ServerboundPickItem,
        0x23: s_ping_request::ServerboundPingRequest,
        0x24: s_place_recipe::ServerboundPlaceRecipe,
        0x25: s_player_abilities::ServerboundPlayerAbilities,
        0x26: s_player_action::ServerboundPlayerAction,
        0x27: s_player_command::ServerboundPlayerCommand,
        0x28: s_player_input::ServerboundPlayerInput,
        0x29: s_pong::ServerboundPong,
        0x2a: s_recipe_book_change_settings::ServerboundRecipeBookChangeSettings,
        0x2b: s_recipe_book_seen_recipe::ServerboundRecipeBookSeenRecipe,
        0x2c: s_rename_item::ServerboundRenameItem,
        0x2d: s_resource_pack::ServerboundResourcePack,
        0x2e: s_seen_advancements::ServerboundSeenAdvancements,
        0x2f: s_select_trade::ServerboundSelectTrade,
        0x30: s_set_beacon::ServerboundSetBeacon,
        0x31: s_set_carried_item::ServerboundSetCarriedItem,
        0x32: s_set_command_block::ServerboundSetCommandBlock,
        0x33: s_set_command_minecart::ServerboundSetCommandMinecart,
        0x34: s_set_creative_mode_slot::ServerboundSetCreativeModeSlot,
        0x35: s_set_jigsaw_block::ServerboundSetJigsawBlock,
        0x36: s_set_structure_block::ServerboundSetStructureBlock,
        0x37: s_sign_update::ServerboundSignUpdate,
        0x38: s_swing::ServerboundSwing,
        0x39: s_teleport_to_entity::ServerboundTeleportToEntity,
        0x3a: s_use_item_on::ServerboundUseItemOn,
        0x3b: s_use_item::ServerboundUseItem,
    },
    Clientbound => {
        0x00: c_bundle::ClientboundBundle,
        0x01: c_add_entity::ClientboundAddEntity,
        0x02: c_add_experience_orb::ClientboundAddExperienceOrb,
        0x03: c_animate::ClientboundAnimate,
        0x04: c_award_stats::ClientboundAwardStats,
        0x05: c_block_changed_ack::ClientboundBlockChangedAck,
        0x06: c_block_destruction::ClientboundBlockDestruction,
        0x07: c_block_entity_data::ClientboundBlockEntityData,
        0x08: c_block_event::ClientboundBlockEvent,
        0x09: c_block_update::ClientboundBlockUpdate,
        0x0a: c_boss_event::ClientboundBossEvent,
        0x0b: c_change_difficulty::ClientboundChangeDifficulty,
        0x0c: c_chunk_batch_finished::ClientboundChunkBatchFinished,
        0x0d: c_chunk_batch_start::ClientboundChunkBatchStart,
        0x0e: c_chunks_biomes::ClientboundChunksBiomes,
        0x0f: c_clear_titles::ClientboundClearTitles,
        0x10: c_command_suggestions::ClientboundCommandSuggestions,
        0x11: c_commands::ClientboundCommands,
        0x12: c_container_close::ClientboundContainerClose,
        0x13: c_container_set_content::ClientboundContainerSetContent,
        0x14: c_container_set_data::ClientboundContainerSetData,
        0x15: c_container_set_slot::ClientboundContainerSetSlot,
        0x16: c_cookie_request::ClientboundCookieRequest,
        0x17: c_cooldown::ClientboundCooldown,
        0x18: c_custom_chat_completions::ClientboundCustomChatCompletions,
        0x19: c_custom_payload::ClientboundCustomPayload,
        0x1a: c_damage_event::ClientboundDamageEvent,
        0x1b: c_debug_sample::ClientboundDebugSample,
        0x1c: c_delete_chat::ClientboundDeleteChat,
        0x1d: c_disconnect::ClientboundDisconnect,
        0x1e: c_disguised_chat::ClientboundDisguisedChat,
        0x1f: c_entity_event::ClientboundEntityEvent,
        0x20: c_entity_position_sync::ClientboundEntityPositionSync,
        0x21: c_explode::ClientboundExplode,
        0x22: c_forget_level_chunk::ClientboundForgetLevelChunk,
        0x23: c_game_event::ClientboundGameEvent,
        0x24: c_horse_screen_open::ClientboundHorseScreenOpen,
        0x25: c_hurt_animation::ClientboundHurtAnimation,
        0x26: c_initialize_border::ClientboundInitializeBorder,
        0x27: c_keep_alive::ClientboundKeepAlive,
        0x28: c_level_chunk_with_light::ClientboundLevelChunkWithLight,
        0x29: c_level_event::ClientboundLevelEvent,
        0x2a: c_level_particles::ClientboundLevelParticles,
        0x2b: c_light_update::ClientboundLightUpdate,
        0x2c: c_login::ClientboundLogin,
        0x2d: c_map_item_data::ClientboundMapItemData,
        0x2e: c_merchant_offers::ClientboundMerchantOffers,
        0x2f: c_move_entity_pos::ClientboundMoveEntityPos,
        0x30: c_move_entity_pos_rot::ClientboundMoveEntityPosRot,
        0x31: c_move_minecart::ClientboundMoveMinecart,
        0x32: c_move_entity_rot::ClientboundMoveEntityRot,
        0x33: c_move_vehicle::ClientboundMoveVehicle,
        0x34: c_open_book::ClientboundOpenBook,
        0x35: c_open_screen::ClientboundOpenScreen,
        0x36: c_open_sign_editor::ClientboundOpenSignEditor,
        0x37: c_ping::ClientboundPing,
        0x38: c_pong_response::ClientboundPongResponse,
        0x39: c_place_ghost_recipe::ClientboundPlaceGhostRecipe,
        0x3a: c_player_abilities::ClientboundPlayerAbilities,
        0x3b: c_player_chat::ClientboundPlayerChat,
        0x3c: c_player_combat_end::ClientboundPlayerCombatEnd,
        0x3d: c_player_combat_enter::ClientboundPlayerCombatEnter,
        0x3e: c_player_combat_kill::ClientboundPlayerCombatKill,
        0x3f: c_player_info_remove::ClientboundPlayerInfoRemove,
        0x40: c_player_info_update::ClientboundPlayerInfoUpdate,
        0x41: c_player_look_at::ClientboundPlayerLookAt,
        0x42: c_player_position::ClientboundPlayerPosition,
        0x43: c_player_rotation::ClientboundPlayerRotation,
        0x44: c_recipe_book_add::ClientboundRecipeBookAdd,
        0x45: c_recipe_book_remove::ClientboundRecipeBookRemove,
        0x46: c_recipe_book_settings::ClientboundRecipeBookSettings,
        0x47: c_remove_entities::ClientboundRemoveEntities,
        0x48: c_remove_mob_effect::ClientboundRemoveMobEffect,
        0x49: c_reset_score::ClientboundResetScore,
        0x4a: c_resource_pack_pop::ClientboundResourcePackPop,
        0x4b: c_resource_pack_push::ClientboundResourcePackPush,
        0x4c: c_respawn::ClientboundRespawn,
        0x4d: c_rotate_head::ClientboundRotateHead,
        0x4e: c_section_blocks_update::ClientboundSectionBlocksUpdate,
        0x4f: c_select_advancements_tab::ClientboundSelectAdvancementsTab,
        0x50: c_server_data::ClientboundServerData,
        0x51: c_set_action_bar_text::ClientboundSetActionBarText,
        0x52: c_set_border_center::ClientboundSetBorderCenter,
        0x53: c_set_border_lerp_size::ClientboundSetBorderLerpSize,
        0x54: c_set_border_size::ClientboundSetBorderSize,
        0x55: c_set_border_warning_delay::ClientboundSetBorderWarningDelay,
        0x56: c_set_border_warning_distance::ClientboundSetBorderWarningDistance,
        0x57: c_set_camera::ClientboundSetCamera,
        0x58: c_set_chunk_cache_center::ClientboundSetChunkCacheCenter,
        0x59: c_set_chunk_cache_radius::ClientboundSetChunkCacheRadius,
        0x5a: c_set_cursor_item::ClientboundSetCursorItem,
        0x5b: c_set_default_spawn_position::ClientboundSetDefaultSpawnPosition,
        0x5c: c_set_display_objective::ClientboundSetDisplayObjective,
        0x5d: c_set_entity_data::ClientboundSetEntityData,
        0x5e: c_set_entity_link::ClientboundSetEntityLink,
        0x5f: c_set_entity_motion::ClientboundSetEntityMotion,
        0x60: c_set_equipment::ClientboundSetEquipment,
        0x61: c_set_experience::ClientboundSetExperience,
        0x62: c_set_health::ClientboundSetHealth,
        0x63: c_set_held_slot::ClientboundSetHeldSlot,
        0x64: c_set_objective::ClientboundSetObjective,
        0x65: c_set_passengers::ClientboundSetPassengers,
        0x66: c_set_player_inventory::ClientboundSetPlayerInventory,
        0x67: c_set_player_team::ClientboundSetPlayerTeam,
        0x68: c_set_score::ClientboundSetScore,
        0x69: c_set_simulation_distance::ClientboundSetSimulationDistance,
        0x6a: c_set_subtitle_text::ClientboundSetSubtitleText,
        0x6b: c_set_time::ClientboundSetTime,
        0x6c: c_set_title_text::ClientboundSetTitleText,
        0x6d: c_set_titles_animation::ClientboundSetTitlesAnimation,
        0x6e: c_sound_entity::ClientboundSoundEntity,
        0x6f: c_sound::ClientboundSound,
        0x70: c_start_configuration::ClientboundStartConfiguration,
        0x71: c_stop_sound::ClientboundStopSound,
        0x72: c_store_cookie::ClientboundStoreCookie,
        0x73: c_system_chat::ClientboundSystemChat,
        0x74: c_tab_list::ClientboundTabList,
        0x75: c_tag_query::ClientboundTagQuery,
        0x76: c_take_item_entity::ClientboundTakeItemEntity,
        0x77: c_teleport_entity::ClientboundTeleportEntity,
        0x78: c_ticking_state::ClientboundTickingState,
        0x79: c_ticking_step::ClientboundTickingStep,
        0x7a: c_transfer::ClientboundTransfer,
        0x7b: c_update_advancements::ClientboundUpdateAdvancements,
        0x7c: c_update_attributes::ClientboundUpdateAttributes,
        0x7d: c_update_mob_effect::ClientboundUpdateMobEffect,
        0x7e: c_update_recipes::ClientboundUpdateRecipes,
        0x7f: c_update_tags::ClientboundUpdateTags,
        0x80: c_projectile_power::ClientboundProjectilePower,
        0x81: c_custom_report_details::ClientboundCustomReportDetails,
        0x82: c_server_links::ClientboundServerLinks
    }
);
