// NOTE: This file is @generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(ConfigPacket,
    Clientbound => [
        cookie_request,
        custom_payload,
        disconnect,
        finish_configuration,
        keep_alive,
        ping,
        reset_chat,
        registry_data,
        resource_pack_pop,
        resource_pack_push,
        store_cookie,
        transfer,
        update_enabled_features,
        update_tags,
        select_known_packs,
        custom_report_details,
        server_links,
        clear_dialog,
        show_dialog,
        code_of_conduct,
    ],
    Serverbound => [
        client_information,
        cookie_response,
        custom_payload,
        finish_configuration,
        keep_alive,
        pong,
        resource_pack,
        select_known_packs,
        custom_click_action,
        accept_code_of_conduct,
    ]
);
