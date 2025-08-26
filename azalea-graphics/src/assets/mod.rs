pub(crate) mod processed;
mod raw;
use std::{collections::HashMap, fs, io::BufReader, ops::Deref, path::PathBuf, time::Instant};

use azalea::blocks::{BlockState, BlockTrait};
use log::*;
use processed::texture::Texture;
use raw::{
    block_state::{BlockRenderState, Variant},
    model::BlockModel as RawBlockModel,
};

use crate::{
    assets::processed::{VariantDesc, model::BlockModel as ResolvedBlockModel},
    vulkan::context::VkContext,
};

pub struct MeshAssets {
    block_models: HashMap<String, ResolvedBlockModel>,
    blockstate_to_models: Vec<Vec<VariantDesc>>,
    texture_to_id: HashMap<String, usize>,
}

impl MeshAssets {
    pub fn get_variant_descs(&self, state: BlockState) -> &[VariantDesc] {
        let id = state.id();
        return &self.blockstate_to_models[id as usize];
    }

    pub fn get_block_model(&self, path: &str) -> Option<&ResolvedBlockModel> {
        let key = path.strip_prefix("minecraft:").unwrap_or(path);
        self.block_models.get(key)
    }

    pub fn get_texture_id(&self, name: &str) -> Option<usize> {
        let key = name.strip_prefix("minecraft:").unwrap_or(name);
        self.texture_to_id.get(key).copied()
    }
}

pub fn load_assets(ctx: &VkContext, path: impl Into<PathBuf>) -> (MeshAssets, Vec<Texture>) {
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
    log::info!(
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
    log::info!(
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
    log::info!(
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
                    let variant: &Variant = variants
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
    log::info!("Mapped blockstates to models in {:?}", start.elapsed());

    let start = Instant::now();
    let texture_path = path.join("textures");
    info!("loading textures from {}", texture_path.display());

    let mut texture_to_id = HashMap::new();
    let mut textures = Vec::new();

    for entry in walkdir::WalkDir::new(&texture_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }

        let name = path
            .strip_prefix(&texture_path)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .to_owned();

        let tex = Texture::new(ctx, BufReader::new(fs::File::open(path).unwrap())).unwrap();
        texture_to_id.insert(name, textures.len());
        textures.push(tex);
    }
    log::info!(
        "Loaded {} textures in {:?}",
        textures.len(),
        start.elapsed()
    );

    log::info!("Total asset load time: {:?}", start_total.elapsed());

    (
        MeshAssets {
            block_models,
            blockstate_to_models,
            texture_to_id,
        },
        textures,
    )
}
