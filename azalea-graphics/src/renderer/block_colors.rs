use std::sync::Arc;

use azalea::{
    blocks::BlockState,
    core::{data_registry::ResolvableDataRegistry, registry_holder::{BiomeData, }},
    registry::{Biome, Block},
    world::Instance,
};
use parking_lot::RwLock;



pub fn get_block_color_tint(
    block_state: BlockState,
    biome: Biome,
    world: &Arc<RwLock<Instance>>,
    tint_index: i32,
) -> [f32; 3] {
    get_biome_color(
        biome,
        world,
        get_color_type_for_tint(tint_index, block_state),
    )
}

#[derive(Debug, Clone, Copy)]
enum ColorType {
    Grass,
    Foliage,
    Water,
    None,
}

fn get_color_type_for_tint(tint_index: i32, block_state: BlockState) -> ColorType {
    if tint_index == -1 {
        return ColorType::None;
    }

    let block: Block = Block::from(block_state);

    match block {
        Block::GrassBlock | Block::Fern => ColorType::Grass,
        Block::OakLeaves | Block::Vine => ColorType::Foliage,
        _ => ColorType::None,
    }
}

/// Get biome color based on type
fn get_biome_color(biome: Biome, world: &Arc<RwLock<Instance>>, color_type: ColorType) -> [f32; 3] {
    let instance = world.read();
    let registries = &instance.registries;

    // Try to resolve the biome data
    if let Some(Ok((_name, biome_data))) = biome.resolve_and_deserialize::<BiomeData>(registries) {
        match color_type {
            ColorType::Grass => {
                if let Some(grass_color) = biome_data.effects.grass_color {
                    return int_color_to_rgb(grass_color);
                }
            }
            ColorType::Foliage => {
                if let Some(foliage_color) = biome_data.effects.foliage_color {
                    return int_color_to_rgb(foliage_color);
                }
            }
            ColorType::Water => {
                return int_color_to_rgb(biome_data.effects.water_color);
            }
            ColorType::None => {
                return [1.0; 3];
            }
        }
    }

    match color_type {
        ColorType::Grass => [0.4, 0.0, 0.2],
        ColorType::Foliage => [0.2, 0.6, 0.1],
        ColorType::Water => [0.2, 0.4, 0.8],
        ColorType::None => [1.0; 3],
    }
}

/// Convert an integer color (0xRRGGBB) to RGB floats [0.0-1.0]
fn int_color_to_rgb(color: u32) -> [f32; 3] {
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;
    [r, g, b]
}
