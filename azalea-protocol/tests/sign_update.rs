use azalea_core::position::BlockPos;
use azalea_protocol::{
    common::sign::SignTextSlot,
    packets::{Packet, game::s_sign_update::ServerboundSignUpdate},
    write::serialize_packet,
};

#[test]
fn mc26_3_sign_update_places_slot_after_lines() {
    let packet = ServerboundSignUpdate {
        pos: BlockPos { x: 1, y: 2, z: 3 },
        lines: [
            "17799000".to_string(),
            String::new(),
            String::new(),
            String::new(),
        ],
        slot: SignTextSlot::Front,
    };

    // Packet 62, packed BlockPos(1, 2, 3), four length-prefixed strings, then
    // SignTextSlot::Front (1). This matches Mojang's 26.3 STREAM_CODEC. The
    // old boolean-before-lines layout shifted a price into the wrong fields
    // and disconnected Hypixel while it parsed the auction sign.
    let expected = [
        0x3e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x30, 0x02, 0x08, b'1', b'7', b'7', b'9', b'9',
        b'0', b'0', b'0', 0x00, 0x00, 0x00, 0x01,
    ];
    assert_eq!(
        serialize_packet(&packet.into_variant()).unwrap().as_ref(),
        expected
    );
}
