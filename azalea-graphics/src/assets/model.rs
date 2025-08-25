use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct BlockModel {
    pub ambientocclusion: Option<bool>,
    pub parent: Option<String>,
    #[serde(default)]
    pub textures: HashMap<String, String>,
    pub elements: Option<Vec<Cube>>,
    pub display: Option<HashMap<String, Display>>,
}

fn r#true() -> bool {
    true
}

impl BlockModel {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Display {
    rotation: Option<glam::Vec3>,
    translation: Option<glam::Vec3>,
    scale: Option<glam::Vec3>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Cube {
    pub from: glam::Vec3,
    pub to: glam::Vec3,
    pub rotation: Option<Rotation>,
    pub faces: Faces,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Faces {
    pub down: Option<Face>,
    pub up: Option<Face>,
    pub north: Option<Face>,
    pub south: Option<Face>,
    pub west: Option<Face>,
    pub east: Option<Face>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Face {
    pub uv: Option<[f32; 4]>,
    pub texture: String,
    pub cullface: Option<String>,
    #[serde(default)]
    pub rotation: i32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Rotation {
    pub origin: glam::Vec3,
    pub axis: Axis,
    pub angle: f32,
    #[serde(default)]
    pub rescale: bool,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum Axis {
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "z")]
    Z,
}

#[cfg(test)]
mod tests {
    use super::BlockModel;

    #[test]
    fn test_deserialze() {
        dbg!(
            serde_json::from_str::<BlockModel>(
                r##"
{
   "ambientocclusion": false,
   "textures": {
       "particle": "#torch"
   },
   "elements": [
       {   "from": [ 7, 0, 7 ],
           "to": [ 9, 10, 9 ],
           "shade": false,
           "faces": {
               "down": { "uv": [ 7, 13, 9, 15 ], "texture": "#torch" },
               "up":   { "uv": [ 7,  6, 9,  8 ], "texture": "#torch" }
           }
       },
       {   "from": [ 7, 0, 0 ],
           "to": [ 9, 16, 16 ],
           "shade": false,
           "faces": {
               "west": { "uv": [ 0, 0, 16, 16 ], "texture": "#torch" },
               "east": { "uv": [ 0, 0, 16, 16 ], "texture": "#torch" }
           }
       },
       {   "from": [ 0, 0, 7 ],
           "to": [ 16, 16, 9 ],
           "shade": false,
           "faces": {
               "north": { "uv": [ 0, 0, 16, 16 ], "texture": "#torch" },
               "south": { "uv": [ 0, 0, 16, 16 ], "texture": "#torch" }
           }
       }
   ]
}
        "##,
            )
            .unwrap()
        );

        dbg!(
            serde_json::from_str::<BlockModel>(
                r##"
{
    "gui_light": "side",
    "display": {
        "gui": {
            "rotation": [ 30, 225, 0 ],
            "translation": [ 0, 0, 0],
            "scale":[ 0.625, 0.625, 0.625 ]
        },
        "ground": {
            "rotation": [ 0, 0, 0 ],
            "translation": [ 0, 3, 0],
            "scale":[ 0.25, 0.25, 0.25 ]
        },
        "fixed": {
            "rotation": [ 0, 0, 0 ],
            "translation": [ 0, 0, 0],
            "scale":[ 0.5, 0.5, 0.5 ]
        },
        "thirdperson_righthand": {
            "rotation": [ 75, 45, 0 ],
            "translation": [ 0, 2.5, 0],
            "scale": [ 0.375, 0.375, 0.375 ]
        },
        "firstperson_righthand": {
            "rotation": [ 0, 45, 0 ],
            "translation": [ 0, 0, 0 ],
            "scale": [ 0.40, 0.40, 0.40 ]
        },
        "firstperson_lefthand": {
            "rotation": [ 0, 225, 0 ],
            "translation": [ 0, 0, 0 ],
            "scale": [ 0.40, 0.40, 0.40 ]
        }
    }
}
        "##,
            )
            .unwrap()
        );
    }
}
