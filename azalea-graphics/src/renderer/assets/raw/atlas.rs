use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteAtlas {
    pub sources: Vec<SpriteSource>,
}

impl SpriteAtlas {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SpriteSource {
    #[serde(rename = "minecraft:directory")]
    Directory {
        source: String,
        prefix: String,
    },
    #[serde(rename = "minecraft:single")]
    Single {
        resource: String,
        #[serde(default)]
        sprite: Option<String>,
    },
    #[serde(rename = "minecraft:filter")]
    Filter {
        pattern: FilterPattern,
    },
    #[serde(rename = "minecraft:unstitch")]
    Unstitch {
        resource: String,
        divisor_x: f64,
        divisor_y: f64,
        regions: Vec<UnstitchRegion>,
    },
    #[serde(rename = "minecraft:paletted_permutations")]
    PalettedPermutations {
        textures: Vec<String>,
        #[serde(default = "default_separator")]
        separator: String,
        palette_key: String,
        permutations: HashMap<String, String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterPattern {
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnstitchRegion {
    pub sprite: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

fn default_separator() -> String {
    "_".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_directory_source() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "directory",
                    "source": "block",
                    "prefix": "block/"
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 1);
        
        if let SpriteSource::Directory { source, prefix } = &atlas.sources[0] {
            assert_eq!(source, "block");
            assert_eq!(prefix, "block/");
        } else {
            panic!("Expected Directory source");
        }
    }

    #[test]
    fn deserialize_single_source() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "single",
                    "resource": "item/apple"
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 1);
        
        if let SpriteSource::Single { resource, sprite } = &atlas.sources[0] {
            assert_eq!(resource, "item/apple");
            assert_eq!(sprite, &None);
        } else {
            panic!("Expected Single source");
        }
    }

    #[test]
    fn deserialize_filter_source() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "filter",
                    "pattern": {
                        "namespace": "minecraft",
                        "path": "block/.*"
                    }
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 1);
        
        if let SpriteSource::Filter { pattern } = &atlas.sources[0] {
            assert_eq!(pattern.namespace, Some("minecraft".to_string()));
            assert_eq!(pattern.path, Some("block/.*".to_string()));
        } else {
            panic!("Expected Filter source");
        }
    }

    #[test]
    fn deserialize_unstitch_source() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "unstitch",
                    "resource": "font/ascii",
                    "divisor_x": 16.0,
                    "divisor_y": 16.0,
                    "regions": [
                        {
                            "sprite": "font/space",
                            "x": 0.0,
                            "y": 0.0,
                            "width": 4.0,
                            "height": 8.0
                        }
                    ]
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 1);
        
        if let SpriteSource::Unstitch { resource, divisor_x, divisor_y, regions } = &atlas.sources[0] {
            assert_eq!(resource, "font/ascii");
            assert_eq!(*divisor_x, 16.0);
            assert_eq!(*divisor_y, 16.0);
            assert_eq!(regions.len(), 1);
            assert_eq!(regions[0].sprite, "font/space");
        } else {
            panic!("Expected Unstitch source");
        }
    }

    #[test]
    fn deserialize_paletted_permutations_source() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "paletted_permutations",
                    "textures": ["block/leather_boots"],
                    "palette_key": "colormap/color_palettes/leather_armor_color_key",
                    "permutations": {
                        "overlay": "colormap/color_palettes/leather_armor_color"
                    }
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 1);
        
        if let SpriteSource::PalettedPermutations { textures, separator, palette_key, permutations } = &atlas.sources[0] {
            assert_eq!(textures.len(), 1);
            assert_eq!(textures[0], "block/leather_boots");
            assert_eq!(separator, "_");
            assert_eq!(palette_key, "colormap/color_palettes/leather_armor_color_key");
            assert_eq!(permutations.len(), 1);
            assert_eq!(permutations.get("overlay"), Some(&"colormap/color_palettes/leather_armor_color".to_string()));
        } else {
            panic!("Expected PalettedPermutations source");
        }
    }

    #[test]
    fn deserialize_mixed_sources() {
        let json = r#"
        {
            "sources": [
                {
                    "type": "directory",
                    "source": "block",
                    "prefix": ""
                },
                {
                    "type": "single",
                    "resource": "item/apple",
                    "sprite": "item/custom_apple"
                },
                {
                    "type": "filter",
                    "pattern": {}
                }
            ]
        }"#;
        
        let atlas = SpriteAtlas::from_str(json).unwrap();
        assert_eq!(atlas.sources.len(), 3);
    }
}
