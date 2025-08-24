use serde::{de::IgnoredAny, Deserializer};

#[derive(serde::Deserialize, Debug)]
pub enum BlockRenderState {
    #[serde(rename = "variants")]
    #[serde(with = "tuple_vec_map")]
    Variants(Vec<(String, Variant)>),

    #[serde(rename = "multipart", deserialize_with = "deserialize_multipart")]
    MultiPart,
}

fn deserialize_multipart<'de, D>(data: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    data.deserialize_ignored_any(IgnoredAny)?;
    Ok(())
}

impl BlockRenderState {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Variants {
    Map(Vec<(String, Variant)>),
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Variant {
    Single(VariantDesc),
    Array(Vec<VariantDesc>),
}

#[derive(serde::Deserialize, Debug)]
pub struct VariantDesc {
    pub model: String,

    #[serde(default)]
    #[serde(rename = "x")]
    pub x_rotation: i32,

    #[serde(default)]
    #[serde(rename = "y")]
    pub y_rotation: i32,

    #[serde(default)]
    pub ublock: bool,
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
