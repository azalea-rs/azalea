use std::collections::HashMap;

use azalea::blocks::BlockTrait;

#[derive(serde::Deserialize, Debug)]
pub enum BlockRenderState {
    #[serde(rename = "variants", with = "tuple_vec_map")]
    Variants(Vec<(String, Variant)>),

    #[serde(rename = "multipart")]
    MultiPart(Vec<MultipartCase>),
}

impl BlockRenderState {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MultipartCase {
    #[serde(default)]
    pub when: Option<CaseCondition>,
    pub apply: Variant,
}



pub type DirectCase = HashMap<String, String>;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum CaseCondition {
    Direct(DirectCase),

    Or {
        #[serde(rename = "OR")]
        or: Vec<DirectCase>,
    },

    And {
        #[serde(rename = "AND")]
        and: Vec<DirectCase>,
    },
}

impl CaseCondition {
    pub fn matches(&self, block: &dyn BlockTrait) -> bool {
        match self {
            CaseCondition::Direct(map) => matches_direct(map, block),
            CaseCondition::Or { or } => or.iter().any(|m| matches_direct(m, block)),
            CaseCondition::And { and } => and.iter().all(|m| matches_direct(m, block)),
        }
    }
}

fn matches_direct(map: &DirectCase, block: &dyn BlockTrait) -> bool {
    for (prop, expected) in map {
        let actual = match block.get_property(prop.as_str()) {
            Some(v) => v,
            None => return false,
        };

        if !expected.split('|').any(|cand| cand == actual) {
            return false;
        }
    }
    true
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Variant {
    Single(VariantDesc),
    Multiple(Vec<VariantDesc>),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct VariantDesc {
    pub model: String,

    #[serde(default)]
    #[serde(rename = "x")]
    pub x_rotation: i32,

    #[serde(default)]
    #[serde(rename = "y")]
    pub y_rotation: i32,

    #[serde(default)]
    pub uvlock: bool,
}

#[cfg(test)]
mod tests {
    use super::BlockRenderState;

    #[test]
    fn deserialize_test() {
        {
            _ = BlockRenderState::from_str(
                r#"{
    "variants": {
        "snowy=false": [
            { "model": "block/grass_block" },
            { "model": "block/grass_block", "y": 90 },
            { "model": "block/grass_block", "y": 180 },
            { "model": "block/grass_block", "y": 270 }
        ],
        "snowy=true":  { "model": "block/grass_block_snow" }
    }
}"#,
            )
            .unwrap();
        }
    }
}
