pub mod block_state;
pub mod model;
pub mod texture;

use std::{collections::HashMap, fs, io::BufReader, path::PathBuf};

use azalea::blocks::{BlockState, BlockTrait};
use log::*;
use texture::Texture;

use self::{
    block_state::BlockRenderState,
    model::{BlockModel, Cube},
};
use crate::{assets::block_state::Variant, vulkan::context::VkContext};

#[derive(Debug)]
pub struct ResolvedBlockModel {
    pub ambient_occlusion: bool,
    pub textures: HashMap<String, String>,
    pub elements: Vec<Cube>,
}

impl ResolvedBlockModel {
    pub fn resolve_texture<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        let key = name.strip_prefix("minecraft:").unwrap_or(name);

        if let Some(ref_name) = key.strip_prefix('#') {
            if let Some(mapped) = self.textures.get(ref_name) {
                self.resolve_texture(mapped)
            } else {
                log::warn!(
                    "Texture reference '{}' not found in {:?}",
                    ref_name,
                    self.textures.keys()
                );
                None
            }
        } else {
            Some(key)
        }
    }

    fn resolve(raw: &BlockModel, all: &HashMap<String, BlockModel>) -> Self {
        let mut textures = HashMap::new();
        let mut elements = raw.elements.clone().unwrap_or_default();
        let mut ambient = raw.ambientocclusion;

        if let Some(parent_name) = &raw.parent {
            if let Some(parent_raw) = all.get(parent_name) {
                let parent = Self::resolve(parent_raw, all);
                // inherit textures (only if not already present in child)
                for (k, v) in parent.textures {
                    textures.entry(k).or_insert(v);
                }
                if elements.is_empty() {
                    elements = parent.elements;
                }
                if !ambient {
                    ambient = parent.ambient_occlusion;
                }
            }
        }

        // override / add child textures
        for (k, v) in &raw.textures {
            textures.insert(k.clone(), v.clone());
        }

        Self {
            ambient_occlusion: ambient,
            textures,
            elements,
        }
    }
}

pub struct MeshAssets {
    block_models: HashMap<String, ResolvedBlockModel>,
    blockstate_to_model: Vec<Option<String>>,
    texture_to_id: HashMap<String, usize>,
}

impl MeshAssets {
    pub fn get_block_model_for(&self, state: BlockState) -> Option<&ResolvedBlockModel> {
        let id = state.id();
        let model_name = self.blockstate_to_model[id as usize].as_ref()?;
        self.block_models.get(model_name)
    }

    pub fn get_texture_id(&self, name: &str) -> Option<usize> {
        let key = name.strip_prefix("minecraft:").unwrap_or(name);
        self.texture_to_id.get(key).copied()
    }
}

pub fn load_assets(ctx: &VkContext, path: impl Into<PathBuf>) -> (MeshAssets, Vec<Texture>) {
    let path = path.into();

    let block_model_path = path.join("models/block");
    let mut raw_models = HashMap::new();

    for entry in walkdir::WalkDir::new(&block_model_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let mut name = "minecraft:block/".to_string();
        name.push_str(
            path.strip_prefix(&block_model_path)
                .unwrap()
                .with_extension("")
                .to_str()
                .unwrap(),
        );

        let s = fs::read_to_string(path).unwrap();
        raw_models.insert(name, BlockModel::from_str(&s).unwrap());
    }

    let mut block_models = HashMap::new();
    for (name, raw) in &raw_models {
        let resolved = ResolvedBlockModel::resolve(raw, &raw_models);
        block_models.insert(name.clone(), resolved);
    }

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

    let mut blockstate_to_model = vec![None; BlockState::MAX_STATE as usize + 1];
    for raw in 0..=BlockState::MAX_STATE {
        let bs = BlockState::try_from(raw as u16).unwrap();
        let dyn_block = Box::<dyn BlockTrait>::from(bs);

        let Some(render_state) = blockstate_defs.get(dyn_block.id()) else {
            continue;
        };

        if let BlockRenderState::Variants(variants) = render_state {
            let variant = 'outer: {
                for (states, variant) in variants {
                    if states.is_empty() {
                        break 'outer variant;
                    }

                    let mut matched = true;
                    for state in states.split(',') {
                        let Some((prop_name, value)) = state.split_once('=') else {
                            log::error!("bad state {}, states {:?}", state, states);
                            continue;
                        };
                        if dyn_block.get_property(prop_name) != Some(value.to_string()) {
                            matched = false;
                        }
                    }
                    if matched {
                        break 'outer variant;
                    }
                }
                &variants[0].1
            };

            let desc = match variant {
                Variant::Single(desc) => desc,
                Variant::Array(arr) => &arr[0],
            };

            blockstate_to_model[raw as usize] = Some(desc.model.clone());
        }
    }

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

    (
        MeshAssets {
            block_models,
            blockstate_to_model,
            texture_to_id,
        },
        textures,
    )
}
