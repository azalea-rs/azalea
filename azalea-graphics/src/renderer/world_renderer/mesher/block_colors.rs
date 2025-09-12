use std::collections::HashMap;

use azalea::{
    blocks::BlockState,
    registry::{Biome, Block, DataRegistry},
};
use glam::IVec3;

use crate::{
    plugin::BiomeCache,
    renderer::{assets::Assets, chunk::LocalSection},
};

/// Function signature for block color providers
/// Takes block_state, section (with biome_cache), local_pos, tint_index, and
/// mesh_assets
pub type BlockColorFn = fn(BlockState, &LocalSection, IVec3, i32, &Assets) -> [f32; 3];

/// Block color registry similar to Minecraft's BlockColors
pub struct BlockColors {
    color_providers: HashMap<Block, BlockColorFn>,
}

impl BlockColors {
    /// Create default block color mappings similar to Minecraft
    pub fn create_default() -> Self {
        let mut block_colors = BlockColors {
            color_providers: HashMap::new(),
        };

        // Grass-colored blocks (normal)
        block_colors.register(
            grass_color_provider,
            &[
                Block::GrassBlock,
                Block::Fern,
                Block::ShortGrass,
                Block::SugarCane,
            ],
        );

        // Double plants (special biome sampling for upper half)
        block_colors.register(
            double_plant_grass_color_provider,
            &[Block::TallGrass, Block::LargeFern],
        );

        // Foliage-colored blocks
        block_colors.register(
            foliage_color_provider,
            &[
                Block::OakLeaves,
                Block::JungleLeaves,
                Block::AcaciaLeaves,
                Block::DarkOakLeaves,
                Block::Vine,
                Block::MangroveLeaves,
            ],
        );

        // Special foliage colors
        block_colors.register(birch_foliage_color_provider, &[Block::BirchLeaves]);
        block_colors.register(spruce_foliage_color_provider, &[Block::SpruceLeaves]);

        // Water-colored blocks
        block_colors.register(water_color_provider, &[Block::Water, Block::BubbleColumn]);

        // Redstone wire (power-based color)
        block_colors.register(redstone_wire_color_provider, &[Block::RedstoneWire]);

        // Crop stems (age-based color)
        block_colors.register(pumpkin_stem_color_provider, &[Block::PumpkinStem]);
        block_colors.register(melon_stem_color_provider, &[Block::MelonStem]);

        // Attached stems (fixed color)
        block_colors.register(
            attached_stem_color_provider,
            &[Block::AttachedPumpkinStem, Block::AttachedMelonStem],
        );

        // Special cases
        block_colors.register(lily_pad_color_provider, &[Block::LilyPad]);

        block_colors
    }

    /// Register a color provider for multiple blocks
    pub fn register(&mut self, color_fn: BlockColorFn, blocks: &[Block]) {
        for &block in blocks {
            self.color_providers.insert(block, color_fn);
        }
    }

    /// Get color for a block at specific tint index
    pub fn get_color(
        &self,
        block_state: BlockState,
        section: &LocalSection,
        local_pos: IVec3,
        tint_index: i32,
        assets: &Assets,
    ) -> [f32; 3] {
        let block = Block::from(block_state);

        if let Some(&color_fn) = self.color_providers.get(&block) {
            color_fn(block_state, section, local_pos, tint_index, assets)
        } else {
            // Default white color for blocks without special coloring
            [1.0; 3]
        }
    }
}

/// Grass color provider
fn grass_color_provider(
    _block_state: BlockState,
    section: &LocalSection,
    local_pos: IVec3,
    tint_index: i32,
    assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    let biome = get_biome_at_local_pos(section, local_pos);
    BiomeColors::get_grass_color_with_modifier(&section.biome_cache, biome, local_pos, assets)
}

/// Double plant grass color provider (handles upper/lower half sampling)
fn double_plant_grass_color_provider(
    block_state: BlockState,
    section: &LocalSection,
    local_pos: IVec3,
    tint_index: i32,
    assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    use azalea::blocks::properties::Half;

    let mut sample_pos = local_pos;

    // If this is upper half of double plant, sample biome from below
    if let Some(half) = block_state.property::<Half>() {
        if half == Half::Upper && local_pos.y > 0 {
            sample_pos.y -= 1;
        }
    }

    let biome = get_biome_at_local_pos(section, sample_pos);
    BiomeColors::get_grass_color_with_modifier(&section.biome_cache, biome, sample_pos, assets)
}

/// Foliage color provider
fn foliage_color_provider(
    _block_state: BlockState,
    section: &LocalSection,
    local_pos: IVec3,
    tint_index: i32,
    assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    let biome = get_biome_at_local_pos(section, local_pos);
    BiomeColors::get_average_foliage_color(&section.biome_cache, biome, assets)
}

/// Birch foliage color provider (fixed color)
fn birch_foliage_color_provider(
    _block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }
    // Birch has a fixed foliage color
    int_color_to_rgb(-8345771) // Birch foliage color
}

/// Spruce foliage color provider (fixed color)
fn spruce_foliage_color_provider(
    _block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }
    // Spruce has a fixed foliage color
    int_color_to_rgb(-10380959) // Spruce foliage color
}

/// Water color provider
fn water_color_provider(
    _block_state: BlockState,
    section: &LocalSection,
    local_pos: IVec3,
    tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    let biome = get_biome_at_local_pos(section, local_pos);
    BiomeColors::get_average_water_color(&section.biome_cache, biome)
}

/// Redstone wire color provider (power-based)
fn redstone_wire_color_provider(
    block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    _tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    use azalea::blocks::properties::RedstoneWirePower;

    let power = block_state
        .property::<RedstoneWirePower>()
        .unwrap_or(RedstoneWirePower::_0);
    let power_level = power as i32;

    RedstoneWire::get_color_for_power(power_level)
}

/// Pumpkin stem color provider (age-based)
fn pumpkin_stem_color_provider(
    block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    _tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    use azalea::blocks::properties::PumpkinStemAge;

    let age = block_state
        .property::<PumpkinStemAge>()
        .unwrap_or(PumpkinStemAge::_0);
    let age_level = age as i32;
    ARGB::color(age_level * 32, 255 - age_level * 8, age_level * 4)
}

/// Melon stem color provider (age-based)
fn melon_stem_color_provider(
    block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    _tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    use azalea::blocks::properties::MelonStemAge;

    let age = block_state
        .property::<MelonStemAge>()
        .unwrap_or(MelonStemAge::_0);
    let age_level = age as i32; // _0 = 0, _1 = 1, ..., _7 = 7

    ARGB::color(age_level * 32, 255 - age_level * 8, age_level * 4)
}

/// Attached stem color provider (fixed color)
fn attached_stem_color_provider(
    _block_state: BlockState,
    _section: &LocalSection,
    _local_pos: IVec3,
    tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    // Java: -2046180 = 0xFFE0C860
    int_color_to_rgb(-2046180) // Attached stem color
}

/// Lily pad color provider  
fn lily_pad_color_provider(
    _block_state: BlockState,
    section: &LocalSection,
    local_pos: IVec3,
    tint_index: i32,
    _assets: &Assets,
) -> [f32; 3] {
    if tint_index == -1 {
        return [1.0; 3];
    }

    // Check if we have biome context, if so use in-world color, otherwise default
    let _biome = get_biome_at_local_pos(section, local_pos);
    int_color_to_rgb(-14647248) // LILY_PAD_IN_WORLD (always in-world in our case)
}

/// Get grass color from biome data following Java logic
fn get_biome_grass_color(biome: Biome, biome_cache: &BiomeCache, assets: &Assets) -> [f32; 3] {
    let biome_index = biome.protocol_id() as usize;

    let biome_data = &biome_cache.biomes[biome_index];
    // First check for grass color override (BiomeSpecialEffects.grassColorOverride)
    if let Some(grass_color) = biome_data.effects.grass_color {
        return int_color_to_rgb(grass_color);
    }

    // If no override, use temperature/downfall to calculate from texture
    // Java: Biome.getGrassColorFromTexture() -> GrassColor.get(temperature,
    // downfall)
    let temperature = biome_data.temperature.clamp(0.0, 1.0) as f64;
    let downfall = biome_data.downfall.clamp(0.0, 1.0) as f64;

    return get_grass_color_from_texture(temperature, downfall, assets);
}

/// Get foliage color from biome data following Java logic
fn get_biome_foliage_color(
    biome: Biome,
    biome_cache: &BiomeCache,
    assets: &Assets,
) -> [f32; 3] {
    let biome_index = biome.protocol_id() as usize;

    if let Some(biome_data) = biome_cache.biomes.get(biome_index) {
        // First check for foliage color override
        // (BiomeSpecialEffects.foliageColorOverride)
        if let Some(foliage_color) = biome_data.effects.foliage_color {
            return int_color_to_rgb(foliage_color);
        }

        // If no override, use temperature/downfall to calculate from texture
        // Java: Biome.getFoliageColorFromTexture() -> FoliageColor.get(temperature,
        // downfall)
        let temperature = biome_data.temperature.clamp(0.0, 1.0) as f64;
        let downfall = biome_data.downfall.clamp(0.0, 1.0) as f64;

        return get_foliage_color_from_texture(temperature, downfall, assets);
    }

    // Fallback
    [0.2, 0.6, 0.2]
}

/// Get water color from biome data (always specified)
fn get_biome_water_color(biome: Biome, biome_cache: &BiomeCache) -> [f32; 3] {
    let biome_index = biome.protocol_id() as usize;

    if let Some(biome_data) = biome_cache.biomes.get(biome_index) {
        return int_color_to_rgb(biome_data.effects.water_color);
    }

    // Default water color
    [0.2, 0.4, 0.8]
}

/// Sample grass color from texture (Java: GrassColor.get(temperature,
/// downfall))
fn get_grass_color_from_texture(temperature: f64, downfall: f64, assets: &Assets) -> [f32; 3] {
    // Try to sample from the actual grass colormap texture
    if let Some(color) = assets.sample_grass_colormap(temperature, downfall) {
        return color;
    }

    unreachable!()
}

/// Sample foliage color from texture (Java: FoliageColor.get(temperature,
/// downfall))
fn get_foliage_color_from_texture(
    temperature: f64,
    downfall: f64,
    assets: &Assets,
) -> [f32; 3] {
    // Try to sample from the actual foliage colormap texture
    if let Some(color) = assets.sample_foliage_colormap(temperature, downfall) {
        return color;
    }

    // Fallback: interpolate between typical foliage colors based on climate
    let cold_factor = (1.0 - temperature) as f32;
    let dry_factor = (1.0 - downfall) as f32;

    let base_r = 0.1 + dry_factor * 0.5; // More red/brown when dry
    let base_g = 0.5 + downfall as f32 * 0.4; // More green when wet  
    let base_b = 0.1 + cold_factor * 0.2; // Slightly more blue when cold

    [base_r, base_g, base_b]
}

/// Get biome at local position within the section
fn get_biome_at_local_pos(section: &LocalSection, local_pos: IVec3) -> Biome {
    // Convert local block position (1-16) to biome position (0-3, biomes are 4x4x4)
    let biome_x = ((local_pos.x - 1) / 4).max(0).min(3) as usize;
    let biome_y = ((local_pos.y - 1) / 4).max(0).min(3) as usize;
    let biome_z = ((local_pos.z - 1) / 4).max(0).min(3) as usize;

    section.biomes[biome_x][biome_y][biome_z]
}

/// BiomeColors utility struct (like Java's BiomeColors)
pub struct BiomeColors;

impl BiomeColors {
    /// Get grass color with grass color modifier applied (Java:
    /// Biome.getGrassColor)
    pub fn get_grass_color_with_modifier(
        biome_cache: &BiomeCache,
        biome: Biome,
        local_pos: IVec3,
        assets: &Assets,
    ) -> [f32; 3] {
        let base_color = get_biome_grass_color(biome, biome_cache, assets);

        base_color
    }

    /// Get average grass color from biome
    pub fn get_average_grass_color(
        biome_cache: &BiomeCache,
        biome: Biome,
        assets: &Assets,
    ) -> [f32; 3] {
        get_biome_grass_color(biome, biome_cache, assets)
    }

    /// Get average foliage color from biome
    pub fn get_average_foliage_color(
        biome_cache: &BiomeCache,
        biome: Biome,
        assets: &Assets,
    ) -> [f32; 3] {
        get_biome_foliage_color(biome, biome_cache, assets)
    }

    /// Get average water color from biome
    pub fn get_average_water_color(biome_cache: &BiomeCache, biome: Biome) -> [f32; 3] {
        get_biome_water_color(biome, biome_cache)
    }
}

/// Apply dark forest grass color modifier (Java:
/// GrassColorModifier.DARK_FOREST)
fn apply_dark_forest_grass_modifier(base_color: [f32; 3]) -> [f32; 3] {
    // Java: (color & 16711422) + 2634762 >> 1
    // This darkens the grass color
    let [r, g, b] = base_color;
    [(r * 0.8).min(1.0), (g * 0.8).min(1.0), (b * 0.8).min(1.0)]
}

/// Apply swamp grass color modifier (Java: GrassColorModifier.SWAMP)
fn apply_swamp_grass_modifier(base_color: [f32; 3], local_pos: IVec3) -> [f32; 3] {
    // Java: uses BIOME_INFO_NOISE.getValue(x * 0.0225, z * 0.0225, false)
    // For now, use simple position-based variation
    let noise_x = local_pos.x as f64 * 0.0225;
    let noise_z = local_pos.z as f64 * 0.0225;

    // Simple noise approximation
    let noise = (noise_x.sin() * noise_z.cos()) * 0.5 + 0.5;

    if noise < 0.4 {
        // Java: 5011004 = 0x4C7653 (brownish green)
        int_color_to_rgb(5011004)
    } else {
        // Java: 6975545 = 0x6A7039 (yellowish green)
        int_color_to_rgb(6975545)
    }
}

/// RedstoneWire utility struct (like Java's RedStoneWireBlock)
pub struct RedstoneWire;

impl RedstoneWire {
    /// Get color for redstone power level (Java:
    /// RedStoneWireBlock.getColorForPower)
    pub fn get_color_for_power(power: i32) -> [f32; 3] {
        // Java implementation: more sophisticated than simple intensity
        let red_component = if power == 0 {
            0.3125
        } else {
            (power as f32 / 15.0) * 0.6875 + 0.3125
        };
        [red_component, 0.0, 0.0]
    }
}

/// ARGB utility struct (like Java's ARGB)
pub struct ARGB;

impl ARGB {
    /// Create color from RGB components (Java: ARGB.color)
    pub fn color(r: i32, g: i32, b: i32) -> [f32; 3] {
        [
            (r.max(0).min(255) as f32) / 255.0,
            (g.max(0).min(255) as f32) / 255.0,
            (b.max(0).min(255) as f32) / 255.0,
        ]
    }
}

/// Convert an integer color (0xRRGGBB or 0xAARRGGBB) to RGB floats [0.0-1.0]
fn int_color_to_rgb(color: i32) -> [f32; 3] {
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;
    [r, g, b]
}
