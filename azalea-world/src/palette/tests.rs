use azalea_block::BlockState;
use azalea_core::position::ChunkSectionBlockPos;

use super::*;

#[test]
fn test_resize_0_bits_to_1() {
    let mut palette_container = PalettedContainer::<BlockState>::new();

    assert_eq!(palette_container.bits_per_entry, 0);
    assert_eq!(palette_container.get_at_index(0), BlockState::AIR);
    assert_eq!(
        PaletteKind::from(&palette_container.palette),
        PaletteKind::SingleValue
    );
    let block_state_1 = BlockState::try_from(1_u32).unwrap();
    palette_container.set_at_index(0, block_state_1);
    assert_eq!(palette_container.get_at_index(0), block_state_1);
    assert_eq!(
        PaletteKind::from(&palette_container.palette),
        PaletteKind::Linear
    );
}

#[test]
fn test_resize_0_bits_to_5() {
    let mut palette_container = PalettedContainer::<BlockState>::new();

    let set = |pc: &mut PalettedContainer<BlockState>, i, v: u32| {
        pc.set_at_index(i, BlockState::try_from(v).unwrap());
    };

    set(&mut palette_container, 0, 0); // 0 bits
    assert_eq!(palette_container.bits_per_entry, 0);

    set(&mut palette_container, 1, 1); // 1 bit
    assert_eq!(palette_container.bits_per_entry, 1);

    set(&mut palette_container, 2, 2); // 2 bits
    assert_eq!(palette_container.bits_per_entry, 2);
    set(&mut palette_container, 3, 3);

    set(&mut palette_container, 4, 4); // 3 bits
    assert_eq!(palette_container.bits_per_entry, 3);
    set(&mut palette_container, 5, 5);
    set(&mut palette_container, 6, 6);
    set(&mut palette_container, 7, 7);

    set(&mut palette_container, 8, 8); // 4 bits
    assert_eq!(palette_container.bits_per_entry, 4);
    set(&mut palette_container, 9, 9);
    set(&mut palette_container, 10, 10);
    set(&mut palette_container, 11, 11);
    set(&mut palette_container, 12, 12);
    set(&mut palette_container, 13, 13);
    set(&mut palette_container, 14, 14);
    set(&mut palette_container, 15, 15);
    assert_eq!(palette_container.bits_per_entry, 4);

    set(&mut palette_container, 16, 16); // 5 bits
    assert_eq!(palette_container.bits_per_entry, 5);
}

#[test]
fn test_coords_from_index() {
    let palette_container = PalettedContainer::<BlockState>::new();

    for x in 0..15 {
        for y in 0..15 {
            for z in 0..15 {
                assert_eq!(
                    palette_container.pos_from_index(
                        palette_container.index_from_pos(ChunkSectionBlockPos::new(x, y, z))
                    ),
                    ChunkSectionBlockPos::new(x, y, z)
                );
            }
        }
    }
}
