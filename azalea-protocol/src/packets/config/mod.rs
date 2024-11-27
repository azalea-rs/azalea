use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    ConfigPacket,
    Serverbound => [
        s_client_information::ServerboundClientInformation,
        s_cookie_response::ServerboundCookieResponse,
        s_custom_payload::ServerboundCustomPayload,
        s_finish_configuration::ServerboundFinishConfiguration,
        s_keep_alive::ServerboundKeepAlive,
        s_pong::ServerboundPong,
        s_resource_pack::ServerboundResourcePack,
        s_select_known_packs::ServerboundSelectKnownPacks,
    ],
    Clientbound => [
        c_cookie_request::ClientboundCookieRequest,
        c_custom_payload::ClientboundCustomPayload,
        c_disconnect::ClientboundDisconnect,
        c_finish_configuration::ClientboundFinishConfiguration,
        c_keep_alive::ClientboundKeepAlive,
        c_ping::ClientboundPing,
        c_reset_chat::ClientboundResetChat,
        c_registry_data::ClientboundRegistryData,
        c_resource_pack_pop::ClientboundResourcePackPop,
        c_resource_pack_push::ClientboundResourcePackPush,
        c_store_cookie::ClientboundStoreCookie,
        c_transfer::ClientboundTransfer,
        c_update_enabled_features::ClientboundUpdateEnabledFeatures,
        c_update_tags::ClientboundUpdateTags,
        c_select_known_packs::ClientboundSelectKnownPacks,
    ]
);
