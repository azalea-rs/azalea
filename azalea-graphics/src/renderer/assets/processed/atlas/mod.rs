use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use image::RgbaImage;
use thiserror::Error;

use crate::renderer::assets::{processed::animation::Animation, raw::atlas::{SpriteAtlas, SpriteSource}};

mod sticher;

pub use sticher::{Atlas, PlacedSprite, StitchError, stitch_sprites};

#[derive(Error, Debug)]
pub enum AtlasError {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("image: {0}")]
    Image(#[from] image::ImageError),
    #[error("regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("stitch: {0}")]
    Stitch(#[from] StitchError),
    #[error("missing texture file: {0}")]
    MissingTexture(PathBuf),
    #[error("Serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub struct TextureEntry {
    pub data: RgbaImage,

    pub animation: Option<Animation>,
}

impl TextureEntry {
    pub fn size(&self) -> (u32, u32) {
        let image_size = self.data.dimensions();

        self.animation
            .as_ref()
            .map(|animation| animation.size(image_size))
            .unwrap_or(image_size)
    }
}

pub fn build_atlas(
    textures_root: impl AsRef<Path>,
    def: &SpriteAtlas,
) -> Result<HashMap<String, TextureEntry>, AtlasError> {
    let textures_root = textures_root.as_ref();
    let mut textures: HashMap<String, TextureEntry> = HashMap::new();

    for src in &def.sources {
        match src {
            SpriteSource::Directory { source, prefix } => {
                let source = strip_namespace(&source);
                let dir = textures_root.join(source);
                if dir.is_dir() {
                    visit_pngs(&dir, |path, rel| {
                        let name = format!("{}{}", prefix, rel.replace('\\', "/"));
                        let entry = load_texture_entry(&path)?;
                        textures.insert(name, entry);
                        Ok(())
                    })?;
                }
            }

            SpriteSource::Single { resource, sprite } => {
                let resource = strip_namespace(&resource);
                let path = textures_root.join(format!("{resource}.png"));
                if !path.exists() {
                    return Err(AtlasError::MissingTexture(path));
                }

                let name = sprite.clone().unwrap_or_else(|| resource.to_string());
                let entry = load_texture_entry(&path)?;
                textures.insert(name, entry);
            }

            SpriteSource::Filter { pattern } => {
                if let Some(pat) = &pattern.path {
                    let re = regex::Regex::new(pat)?;
                    textures.retain(|name, _| re.is_match(name));
                }
            }

            SpriteSource::Unstitch { .. } => {
                todo!("unstitch is not used in minecraft, we dont need it for now")
            }

            SpriteSource::PalettedPermutations {
                textures: bases,
                separator,
                palette_key: _,
                permutations,
            } => {
                for base in bases {
                    let base = strip_namespace(base);
                    let base_path = textures_root.join(format!("{base}.png"));
                    if !base_path.exists() {
                        return Err(AtlasError::MissingTexture(base_path));
                    }

                    let entry = load_texture_entry(&base_path)?;
                    textures.insert(base.to_string(), entry);

                    for key in permutations.keys() {
                        let name = format!("{base}{separator}{key}");
                        let entry = load_texture_entry(&base_path)?;
                        textures.insert(name, entry);
                    }
                }
            }
        }
    }

    Ok(textures)
}

fn load_texture_entry(path: &Path) -> Result<TextureEntry, AtlasError> {
    let data = image::open(path)?.into_rgba8();

    let mcmeta_path = {
        let mut p = PathBuf::from(path);
        p.set_extension("png.mcmeta");
        p
    };

    let animation = if mcmeta_path.exists() {
        let json = fs::read_to_string(&mcmeta_path)?;
        Some(serde_json::from_str::<Animation>(&json)?)
    } else {
        None
    };

    Ok(TextureEntry { data, animation })
}

fn visit_pngs<F>(dir: &Path, mut f: F) -> Result<(), AtlasError>
where
    F: FnMut(PathBuf, String) -> Result<(), AtlasError>,
{
    fn walk<F>(base: &Path, cur: &Path, f: &mut F) -> Result<(), AtlasError>
    where
        F: FnMut(PathBuf, String) -> Result<(), AtlasError>,
    {
        for entry in fs::read_dir(cur)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, f)?;
            } else if path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("png"))
            {
                let rel = path.strip_prefix(base).unwrap().to_owned();
                let mut rel = rel.to_string_lossy().to_string();
                if rel.ends_with(".png") {
                    rel.truncate(rel.len() - 4);
                }
                rel = rel.replace('\\', "/");
                f(path.clone(), rel)?;
            }
        }
        Ok(())
    }
    walk(dir, dir, &mut f)
}

#[inline]
fn strip_namespace(s: &str) -> &str {
    if let Some((_, rest)) = s.split_once(':') {
        rest
    } else {
        s
    }
}
