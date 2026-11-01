use azalea_chat::FormattedText;
use azalea_inventory::{ItemStack, components::Lore};
use azalea_registry::builtin::ItemKind;

#[global_allocator]
static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

// This integration binary has a single test so other tests cannot affect its
// process-wide allocator counters.
#[test]
fn replacing_and_removing_components_releases_previous_values() {
    let lore = || Lore {
        lines: vec![FormattedText::from("x".repeat(1024))],
    };
    let region = stats_alloc::Region::new(GLOBAL);
    let mut item = ItemStack::from(ItemKind::Stone);
    for _ in 0..1024 {
        item = item.with_component(lore()).with_component(lore());
        assert_eq!(item.get_component::<Lore>().unwrap().lines.len(), 1);
        item = item.with_component::<Lore>(None);
        assert!(item.component_patch().get::<Lore>().is_none());
    }
    drop(item);
    let stats = region.change();
    assert!(
        stats.bytes_allocated > 2 * 1024 * 1024,
        "allocator must observe the component payloads: {stats:?}"
    );
    assert!(
        stats
            .bytes_allocated
            .saturating_sub(stats.bytes_deallocated)
            < 32768,
        "replaced components retained: {stats:?}"
    );
}
