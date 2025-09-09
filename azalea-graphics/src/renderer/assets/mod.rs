pub(crate) mod processed;
mod raw;
use std::{collections::HashMap, fs, ops::Deref, path::PathBuf, time::Instant};

use azalea::blocks::{BlockState, BlockTrait};
use log::*;
use raw::{
    block_state::{BlockRenderState, Variant},
    model::BlockModel as RawBlockModel,
};

use self::{
    processed::{
        VariantDesc,
        atlas::{Atlas, PlacedSprite, build_atlas, render_atlas_image, stitch_sprites},
        model::BlockModel as ResolvedBlockModel,
    },
    raw::atlas::SpriteAtlas,
};
use crate::renderer::vulkan::context::VkContext;

pub struct MeshAssets {
    block_models: HashMap<String, ResolvedBlockModel>,
    blockstate_to_models: Vec<Vec<VariantDesc>>,
    pub block_atlas: Atlas,
    pub grass_colormap: Option<image::RgbaImage>,
    pub foliage_colormap: Option<image::RgbaImage>,
}

impl MeshAssets {
    pub fn get_variant_descs(&self, state: BlockState) -> &[VariantDesc] {
        let id = state.id();
        &self.blockstate_to_models[id as usize]
    }

    pub fn get_block_model(&self, path: &str) -> Option<&ResolvedBlockModel> {
        let key = path.strip_prefix("minecraft:").unwrap_or(path);
        self.block_models.get(key)
    }

    pub fn get_sprite_rect(&self, name: &str) -> Option<&PlacedSprite> {
        self.block_atlas.sprites.get(name)
    }

    /// Sample grass colormap at the given temperature and downfall
    /// Returns RGB values as [f32; 3] in range [0.0, 1.0]
    pub fn sample_grass_colormap(&self, temperature: f64, downfall: f64) -> Option<[f32; 3]> {
        self.grass_colormap
            .as_ref()
            .map(|colormap| sample_colormap_at_climate(colormap, temperature, downfall))
    }

    /// Sample foliage colormap at the given temperature and downfall  
    /// Returns RGB values as [f32; 3] in range [0.0, 1.0]
    pub fn sample_foliage_colormap(&self, temperature: f64, downfall: f64) -> Option<[f32; 3]> {
        self.foliage_colormap
            .as_ref()
            .map(|colormap| sample_colormap_at_climate(colormap, temperature, downfall))
    }
}

/// Sample a colormap texture at the given temperature and downfall coordinates
/// This follows Java Minecraft's exact colormap sampling logic
fn sample_colormap_at_climate(
    colormap: &image::RgbaImage,
    temperature: f64,
    downfall: f64,
) -> [f32; 3] {
    let width = colormap.width() as f64;
    let height = colormap.height() as f64;

    // Clamp temperature and downfall to [0.0, 1.0] range
    let temp = temperature.clamp(0.0, 1.0);
    let rain = downfall.clamp(0.0, 1.0);

    // Java Minecraft's exact coordinate calculation:
    // 1. Adjust downfall by multiplying with temperature (creates triangular mask)
    // 2. X = (1.0 - temperature) * (width-1) -> cold=right, hot=left
    // 3. Y = (1.0 - adjusted_downfall) * (height-1) -> dry=bottom, wet=top
    let adjusted_downfall = rain * temp;
    let x = ((1.0 - temp) * (width - 1.0)) as u32;
    let y = ((1.0 - adjusted_downfall) * (height - 1.0)) as u32;

    // Clamp to valid texture coordinates
    let x = x.min(colormap.width() - 1);
    let y = y.min(colormap.height() - 1);

    // Sample pixel and convert to RGB floats
    let pixel = colormap.get_pixel(x, y);
    [
        pixel[0] as f32 / 255.0,
        pixel[1] as f32 / 255.0,
        pixel[2] as f32 / 255.0,
    ]
}

pub fn load_assets(ctx: &VkContext, path: impl Into<PathBuf>) -> (MeshAssets, image::RgbaImage) {
    let path = path.into();

    let start_total = Instant::now();

    let start = Instant::now();
    let block_model_path = path.join("models/block");
    let mut raw_models = HashMap::new();

    for entry in fs::read_dir(&block_model_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let name = format!(
            "block/{}",
            path.strip_prefix(&block_model_path)
                .unwrap()
                .with_extension("")
                .to_str()
                .unwrap()
        );

        let s = fs::read_to_string(path).unwrap();
        raw_models.insert(name, RawBlockModel::from_str(&s).unwrap());
    }
    info!(
        "Loaded {} raw block models in {:?}",
        raw_models.len(),
        start.elapsed()
    );

    let start = Instant::now();
    let mut block_models = HashMap::new();
    for (name, raw) in &raw_models {
        let resolved = ResolvedBlockModel::resolve(raw, &raw_models);
        block_models.insert(name.clone(), resolved);
    }
    info!(
        "Resolved {} block models in {:?}",
        block_models.len(),
        start.elapsed()
    );

    let start = Instant::now();
    let block_state_path = path.join("blockstates");
    let mut blockstate_defs = HashMap::new();

    for entry in fs::read_dir(&block_state_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let s = fs::read_to_string(path).unwrap();
        let state = BlockRenderState::from_str(&s).unwrap();
        blockstate_defs.insert(name, state);
    }
    info!(
        "Loaded {} blockstate definitions in {:?}",
        blockstate_defs.len(),
        start.elapsed()
    );

    let start = Instant::now();
    let blockstate_to_models: Vec<Vec<VariantDesc>> = (0..=BlockState::MAX_STATE)
        .map(|raw: u16| {
            let bs = BlockState::try_from(raw).unwrap();
            let dyn_block = Box::<dyn BlockTrait>::from(bs);

            let Some(render_state) = blockstate_defs.get(dyn_block.id()) else {
                return vec![];
            };

            match render_state {
                BlockRenderState::Variants(variants) => {
                    let variant = variants
                        .iter()
                        .find(|(states, _)| {
                            states.is_empty()
                                || states.split(',').all(|state| {
                                    state.split_once('=').map_or(false, |(prop_name, value)| {
                                        dyn_block.get_property(prop_name) == Some(value.to_string())
                                    })
                                })
                        })
                        .map(|(_, v)| v)
                        .unwrap_or(&variants[0].1);

                    match variant {
                        Variant::Single(desc) => vec![desc.clone()],
                        Variant::Multiple(arr) => arr.first().cloned().into_iter().collect(),
                    }
                }
                BlockRenderState::MultiPart(multi_part) => multi_part
                    .iter()
                    .filter(|case| {
                        case.when
                            .as_ref()
                            .map_or(true, |cond| cond.matches(dyn_block.deref()))
                    })
                    .filter_map(|case| match &case.apply {
                        Variant::Single(desc) => Some(desc.clone()),
                        Variant::Multiple(arr) => arr.first().cloned(),
                    })
                    .collect(),
            }
        })
        .collect();
    info!("Mapped blockstates to models in {:?}", start.elapsed());

    let start = Instant::now();

    let blocks_atlas_path = path.join("atlases/blocks.json");
    let blocks_atlas_json = fs::read_to_string(&blocks_atlas_path)
        .unwrap_or_else(|_| panic!("missing {}", blocks_atlas_path.display()));
    let blocks_atlas =
        SpriteAtlas::from_str(&blocks_atlas_json).expect("invalid atlases/blocks.json");

    let textures_root = path.join("textures");
    let (entries, name_to_path) =
        build_atlas(&textures_root, &blocks_atlas).expect("build entries");

    let max_tex = vk_max_texture_2d(ctx);
    let (max_w, max_h) = (max_tex, max_tex);
    let packed_atlas = stitch_sprites(entries, max_w, max_h).expect("stitch sprites");

    let atlas_image = render_atlas_image(&packed_atlas, &name_to_path).expect("render atlas");
    let debug_path = path.join("debug_blocks_atlas.png");
    if let Err(e) = atlas_image.save(&debug_path) {
        warn!("Failed to save debug atlas {}: {e}", debug_path.display());
    } else {
        info!("Saved debug atlas to {}", debug_path.display());
    }

    info!(
        "Built blocks atlas {}x{} in {:?}",
        packed_atlas.width,
        packed_atlas.height,
        start.elapsed()
    );

    // Load colormaps
    let start = Instant::now();
    let grass_colormap = load_colormap(&textures_root, "colormap/grass.png");
    let foliage_colormap = load_colormap(&textures_root, "colormap/foliage.png");

    if grass_colormap.is_some() {
        info!("Loaded grass colormap");
    } else {
        warn!("Failed to load grass colormap");
    }

    if foliage_colormap.is_some() {
        info!("Loaded foliage colormap");
    } else {
        warn!("Failed to load foliage colormap");
    }

    info!("Loaded colormaps in {:?}", start.elapsed());

    info!("Total asset load time: {:?}", start_total.elapsed());

    (
        MeshAssets {
            block_models,
            blockstate_to_models,
            block_atlas: packed_atlas,
            grass_colormap,
            foliage_colormap,
        },
        atlas_image,
    )
}

/// Load a colormap texture from the given path
fn load_colormap(textures_root: &PathBuf, relative_path: &str) -> Option<image::RgbaImage> {
    let colormap_path = textures_root.join(relative_path);

    match image::open(&colormap_path) {
        Ok(img) => {
            let rgba_img = img.to_rgba8();
            info!(
                "Loaded colormap: {} ({}x{})",
                relative_path,
                rgba_img.width(),
                rgba_img.height()
            );
            Some(rgba_img)
        }
        Err(e) => {
            warn!("Failed to load colormap {}: {}", colormap_path.display(), e);
            None
        }
    }
}

fn vk_max_texture_2d(ctx: &VkContext) -> u32 {
    unsafe {
        let props = ctx
            .instance()
            .get_physical_device_properties(ctx.physical_device());
        props.limits.max_image_dimension2_d
    }
}
