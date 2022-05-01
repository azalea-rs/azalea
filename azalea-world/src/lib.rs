#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Chunk {
    sections: Vec<Section>,
}

pub struct Section {
    states: PalettedContainer,
    biomes: PalettedContainer,
}

pub struct PalettedContainer {
    bits_per_entry: u8,
    palette: Palette,
    /// Compacted list of indices pointing to entry IDs in the Palette.
    data: Vec<i64>,
}

pub enum Palette {
    /// ID of the corresponding entry in its global palette
    SingleValue(u32),
    LinearPalette(Vec<u32>),
    HashmapPalette(Vec<u32>),
    GlobalPalette,
}

impl Palette {
    fn choose_palette_for_states(bits_per_entry: u8) -> &'static Palette {
        match bits_per_entry {
            0 => &Palette::SingleValue,
            1..=4 => &Palette::LinearPalette,
            5..=8 => &Palette::HashmapPalette,
            _ => &Palette::GlobalPalette,
        }
    }
}
