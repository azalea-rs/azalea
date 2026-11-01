#![cfg(feature = "serde")]

use std::io::Cursor;

use azalea_core::registry_holder::{components::Tick, enchantment::EnchantmentData};
use simdnbt::Deserialize;

#[global_allocator]
static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

fn named(bytes: &mut Vec<u8>, tag: u8, name: &str, data: &[u8]) {
    bytes.push(tag);
    bytes.extend_from_slice(&(name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(data);
}

fn string(bytes: &mut Vec<u8>, name: &str, value: &str) {
    let mut data = (value.len() as u16).to_be_bytes().to_vec();
    data.extend_from_slice(value.as_bytes());
    named(bytes, 8, name, &data);
}

fn fixture(fail_inside_list: bool, fail_after_kind: bool, duplicate: bool) -> Vec<u8> {
    let mut effect = Vec::new();
    string(&mut effect, "type", "minecraft:run_function");
    string(
        &mut effect,
        "function",
        &format!("test:{}", "x".repeat(1024)),
    );
    effect.push(0);
    let mut conditional = Vec::new();
    named(&mut conditional, 10, "effect", &effect);
    conditional.push(0);
    let mut list = vec![10];
    list.extend_from_slice(&(if fail_inside_list { 2_i32 } else { 1 }).to_be_bytes());
    list.extend_from_slice(&conditional);
    if fail_inside_list {
        list.push(0);
    } // Missing required effect in second element.
    let mut effects = Vec::new();
    named(&mut effects, 9, "minecraft:tick", &list);
    if duplicate {
        named(&mut effects, 9, "minecraft:tick", &list);
    }
    if fail_after_kind {
        named(&mut effects, 10, "test:unknown_effect", &[0]);
    }
    effects.push(0);
    let mut root = vec![10];
    named(&mut root, 10, "effects", &effects);
    root.push(0);
    root
}

#[test]
fn registry_effects_release_success_clones_and_partially_decoded_lists() {
    let buffers = [
        fixture(false, false, false),
        fixture(true, false, false),
        fixture(false, true, false),
        fixture(false, false, true),
    ];
    let tags: Vec<_> = buffers
        .iter()
        .map(|bytes| {
            simdnbt::borrow::read_optional_tag(&mut Cursor::new(&bytes[..]))
                .unwrap()
                .unwrap()
        })
        .collect();
    let decode =
        |index: usize| EnchantmentData::from_compound(tags[index].as_tag().compound().unwrap());
    drop(decode(0).unwrap());
    let region = stats_alloc::Region::new(&stats_alloc::INSTRUMENTED_SYSTEM);
    for _ in 0..1024 {
        let value = decode(0).unwrap();
        assert_eq!(value.get::<Tick>().unwrap().len(), 1);
        drop(value.clone());
        drop(value);
        assert!(decode(1).is_err());
        assert!(decode(2).is_err());
        let duplicate = decode(3).unwrap();
        assert_eq!(duplicate.get::<Tick>().unwrap().len(), 1);
        drop(duplicate);
    }
    let stats = region.change();
    assert!(
        stats.bytes_allocated > 4 * 1024 * 1024,
        "allocator must be active: {stats:?}"
    );
    assert!(
        stats
            .bytes_allocated
            .saturating_sub(stats.bytes_deallocated)
            < 32768,
        "enchantment data retained: {stats:?}"
    );
}
