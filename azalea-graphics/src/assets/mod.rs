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

pub struct BlockModelRef<'a> {
    pub ambient_occlusion: bool,
    pub parent: Option<Box<BlockModelRef<'a>>>,
    pub textures: &'a HashMap<String, String>,
    pub elements: &'a Option<Vec<Cube>>,
}

impl<'a> BlockModelRef<'a> {
    pub fn elements(&self) -> Option<&'a Vec<Cube>> {
        if let Some(elements) = self.elements {
            Some(elements)
        } else {
            if let Some(parent) = &self.parent {
                parent.elements()
            } else {
                None
            }
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<String> {
        self.get_texture_helper(self, name)
    }

    fn get_texture_helper(&self, top: &Self, name: &str) -> Option<String> {
        let name = name.strip_prefix("minecraft:").unwrap_or(name);
        let texture = if let Some(strip_name) = name.strip_prefix('#') {
            self.textures
                .get(strip_name)
                .map(|n| top.get_texture_helper(top, n))
                .unwrap_or_else(|| {
                    self.parent
                        .as_ref()
                        .map(|parent| parent.get_texture_helper(top, name))
                        .flatten()
                })
        } else {
            Some(name.to_owned())
        };

        if texture.is_none() {
            error!(
                "could not load texture {}, from textures: {:?}",
                name, self.textures
            );
        }
        texture
    }
}

pub struct LoadedAssets {
    texture_to_id: HashMap<String, usize>,

    pub textures: Vec<Texture>,

    block_models: HashMap<String, BlockModel>,

    blockstate_to_model: Vec<Option<String>>,
}

impl LoadedAssets {
    pub fn from_path(ctx: &VkContext, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

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

        let block_model_path = path.join("models/block");
        let mut block_models = HashMap::new();

        for entry in walkdir::WalkDir::new(&block_model_path) {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let mut name = "block/".to_string();
            name.push_str(
                path.strip_prefix(&block_model_path)
                    .unwrap()
                    .with_extension("")
                    .to_str()
                    .unwrap(),
            );

            let s = fs::read_to_string(path).unwrap();
            block_models.insert(name, BlockModel::from_str(&s).unwrap());
        }

        let block_state_path = path.join("blockstates");
        let mut blockstate_defs = HashMap::new();

        for entry in walkdir::WalkDir::new(&block_state_path) {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let mut name = "block/".to_string();
            name.push_str(
                path.strip_prefix(&block_state_path)
                    .unwrap()
                    .with_extension("")
                    .to_str()
                    .unwrap(),
            );

            let s = fs::read_to_string(path).unwrap();
            let state = BlockRenderState::from_str(&s).unwrap();
            blockstate_defs.insert(name, state);
        }

        let mut blockstate_to_model = vec![None; BlockState::MAX_STATE as usize + 1];

        for raw in 0..=BlockState::MAX_STATE {
            let bs = BlockState::try_from(raw as u16).unwrap();
            let dyn_block = Box::<dyn BlockTrait>::from(bs);

            let id = format!("block/{}", dyn_block.id());
            let Some(render_state) = blockstate_defs.get(&id) else {
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

        Self {
            texture_to_id,
            textures,
            block_models,
            blockstate_to_model,
        }
    }

    pub fn get_block_model_for<'a>(&'a self, state: BlockState) -> Option<BlockModelRef<'a>> {
        let model_name = self.blockstate_to_model[state.id() as usize].as_ref()?;
        self.get_block_model(model_name)
    }

    pub fn get_texture_id(&self, name: &str) -> Option<usize> {
        self.texture_to_id.get(name).copied()
    }

    pub fn get_block_model<'a>(&'a self, name: &str) -> Option<BlockModelRef<'a>> {
        let name = name.strip_prefix("minecraft:").unwrap_or(name);
        if let Some(block_model) = self.block_models.get(name) {
            let parent = block_model
                .parent
                .clone()
                .map(|parent| self.get_block_model(&parent));
            match parent {
                Some(Some(parent)) => Some(BlockModelRef {
                    ambient_occlusion: block_model.ambientocclusion,
                    parent: Some(Box::new(parent)),
                    textures: &block_model.textures,
                    elements: &block_model.elements,
                }),

                Some(None) => None,
                _ => Some(BlockModelRef {
                    ambient_occlusion: block_model.ambientocclusion,
                    parent: None,
                    textures: &block_model.textures,
                    elements: &block_model.elements,
                }),
            }
        } else {
            None
        }
    }
}
