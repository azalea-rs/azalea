use std::collections::HashMap;

use crate::renderer::assets::raw;
pub(crate) use crate::renderer::assets::raw::model::Cube;
pub(crate) use crate::renderer::assets::raw::model::Face;

#[derive(Debug)]
pub struct BlockModel {
    pub ambient_occlusion: bool,
    pub textures: HashMap<String, String>,
    pub elements: Vec<Cube>,
}

impl BlockModel {
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

    pub fn resolve(
        raw: &raw::model::BlockModel,
        all: &HashMap<String, raw::model::BlockModel>,
    ) -> Self {
        let mut textures = HashMap::new();
        let mut elements = raw.elements.clone().unwrap_or_default();

        let mut ambient = raw.ambientocclusion.unwrap_or(true);

        if let Some(parent_name) = &raw.parent {
            let name = parent_name
                .strip_prefix("minecraft:")
                .unwrap_or(&parent_name);
            if let Some(parent_raw) = all.get(name) {
                let parent = Self::resolve(parent_raw, all);

                for (k, v) in parent.textures {
                    textures.entry(k).or_insert(v);
                }

                if elements.is_empty() {
                    elements = parent.elements;
                }

                if raw.ambientocclusion.is_none() {
                    ambient = parent.ambient_occlusion;
                }
            } else {
                log::warn!("parent {} is empty", name);
            }
        }

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
