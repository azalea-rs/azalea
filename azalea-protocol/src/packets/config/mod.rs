// NOTE: This file is generated automatically by codegen/packet.py.
// Don't edit it directly!

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(ConfigPacket,
    Clientbound => [
        cookie_request, // 0x00
        custom_payload, // 0x01
        disconnect, // 0x02
        finish_configuration, // 0x03
        keep_alive, // 0x04
        ping, // 0x05
        reset_chat, // 0x06
        registry_data, // 0x07
        resource_pack_pop, // 0x08
        resource_pack_push, // 0x09
        store_cookie, // 0x0A
        transfer, // 0x0B
        update_enabled_features, // 0x0C
        update_tags, // 0x0D
        select_known_packs, // 0x0E
        custom_report_details, // 0x0F
        server_links, // 0x10
    ],
    Serverbound => [
        client_information, // 0x00
        cookie_response, // 0x01
        custom_payload, // 0x02
        finish_configuration, // 0x03
        keep_alive, // 0x04
        pong, // 0x05
        resource_pack, // 0x06
        select_known_packs, // 0x07
    ]
);
