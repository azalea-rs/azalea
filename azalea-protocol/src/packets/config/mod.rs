// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(ConfigPacket,
    Clientbound => [
        keep_alive,
        registry_data,
        reset_chat,
        resource_pack_pop,
        resource_pack_push,
        select_known_packs,
        server_links,
        disconnect,
        finish_configuration,
        ping,
        cookie_request,
        update_enabled_features,
        update_tags,
        transfer,
        store_cookie,
        custom_payload,
        custom_report_details,
    ],
    Serverbound => [
        keep_alive,
        resource_pack,
        select_known_packs,
        finish_configuration,
        client_information,
        cookie_response,
        pong,
        custom_payload,
    ]
);
