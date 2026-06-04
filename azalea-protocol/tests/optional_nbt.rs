use std::io::Cursor;

use azalea_buf::{AzBuf, AzBufVar};
use azalea_protocol::packets::{
    OptionalNbt, game::s_custom_click_action::ServerboundCustomClickAction,
};
use azalea_registry::identifier::Identifier;
use simdnbt::owned::{BaseNbt, Nbt, NbtCompound};

fn nbt_with_byte() -> Nbt {
    let mut compound = NbtCompound::new();
    compound.insert("a", 1_i8);
    Nbt::Some(BaseNbt::new("", compound))
}

fn dialog_probe_nbt() -> Nbt {
    let mut compound = NbtCompound::new();
    compound.insert("args", "");
    compound.insert("cmd", "azaleatest:probe");
    compound.insert("field", "x".repeat(60));
    Nbt::Some(BaseNbt::new("", compound))
}

fn parse_hex_fixture(hex: &str) -> Vec<u8> {
    hex.split_ascii_whitespace()
        .map(|byte| u8::from_str_radix(byte, 16).expect("hex fixture should be valid"))
        .collect()
}

#[test]
fn optional_nbt_roundtrip_matches_vanilla_capture() {
    let mut encoded = Vec::new();
    OptionalNbt(nbt_with_byte())
        .azalea_write(&mut encoded)
        .expect("present NBT should encode");

    assert_eq!(encoded[0], 7);
    assert_eq!(
        encoded,
        include_bytes!("data/optional_nbt_present_minimal.bin")
    );

    let mut cursor = Cursor::new(encoded.as_slice());
    let decoded =
        OptionalNbt::azalea_read(&mut cursor).expect("present NBT should decode from fixture");
    assert_eq!(decoded, OptionalNbt(nbt_with_byte()));

    let mut absent = Vec::new();
    OptionalNbt(Nbt::None)
        .azalea_write(&mut absent)
        .expect("absent NBT should encode");
    assert_eq!(absent, include_bytes!("data/optional_nbt_absent.bin"));

    let fixture = parse_hex_fixture(include_str!(
        "data/optional_nbt_vanilla_dialog_probe.hex.txt"
    ));
    let mut cursor = Cursor::new(fixture.as_slice());
    let decoded =
        OptionalNbt::azalea_read(&mut cursor).expect("dialog payload fixture should decode");
    assert!(decoded.0.contains("args"));
    assert!(decoded.0.contains("cmd"));
    assert!(decoded.0.contains("field"));
}

#[test]
fn optional_nbt_present_prefix_is_varint_length_not_bool() {
    let mut payload = Vec::new();
    dialog_probe_nbt()
        .azalea_write(&mut payload)
        .expect("bare NBT should encode");

    let id = Identifier::from("azaleatest:probe");
    let mut id_bytes = Vec::new();
    id.azalea_write(&mut id_bytes)
        .expect("packet id should encode");

    let mut encoded = Vec::new();
    ServerboundCustomClickAction {
        id,
        payload: dialog_probe_nbt().into(),
    }
    .azalea_write(&mut encoded)
    .expect("packet should encode");

    let mut expected_prefix = Vec::new();
    (payload.len() as u32)
        .azalea_write_var(&mut expected_prefix)
        .expect("payload length should encode as VarInt");

    let prefix_start = id_bytes.len();
    let prefix_end = prefix_start + expected_prefix.len();

    assert_eq!(
        &encoded[prefix_start..prefix_end],
        expected_prefix.as_slice()
    );
    assert_ne!(encoded[prefix_start], 1);
    assert_eq!(&encoded[prefix_end..], payload.as_slice());
}
