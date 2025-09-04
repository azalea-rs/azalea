use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::renderer::assets::raw::atlas::{SpriteAtlas, SpriteSource};

mod sticher;

pub use sticher::{Atlas, PlacedSprite, SpriteEntry, StitchError, stitch_sprites};

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
}

pub fn build_atlas(
    textures_root: impl AsRef<Path>,
    def: &SpriteAtlas,
) -> Result<(Vec<SpriteEntry>, HashMap<String, PathBuf>), AtlasError> {
    let textures_root = textures_root.as_ref();

    let mut sizes: HashMap<String, (u32, u32)> = HashMap::new();
    let mut sources: HashMap<String, PathBuf> = HashMap::new();

    for src in &def.sources {
        match src {
            SpriteSource::Directory { source, prefix } => {
                let source = strip_namespace(&source);
                let dir = textures_root.join(source);
                if dir.is_dir() {
                    visit_pngs(&dir, |path, rel| {
                        let name = format!("{}{}", prefix, rel.replace('\\', "/"));
                        let (w, h) = image::image_dimensions(&path)?;
                        sizes.insert(name.clone(), (w, h));
                        sources.insert(name, path.clone());
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
                let (w, h) = image::image_dimensions(&path)?;
                let name = sprite.clone().unwrap_or_else(|| resource.to_string());
                sizes.insert(name.clone(), (w, h));
                sources.insert(name, path.clone());
            }

            SpriteSource::Filter { pattern } => {
                if let Some(pat) = &pattern.path {
                    let re = regex::Regex::new(pat)?;
                    sizes.retain(|name, _| re.is_match(name));
                    sources.retain(|name, _| re.is_match(name));
                }
            }

            SpriteSource::Unstitch { .. } => {
                todo!("unstitch is not used in minecraft, we dont need it for now")
            }

            SpriteSource::PalettedPermutations {
                textures,
                separator,
                palette_key: _,
                permutations,
            } => {
                for base in textures {
                    let base = strip_namespace(&base);
                    let base_path = textures_root.join(format!("{base}.png"));
                    if !base_path.exists() {
                        return Err(AtlasError::MissingTexture(base_path));
                    }
                    let (w, h) = image::image_dimensions(&base_path)?;
                    sizes.insert(base.to_string(), (w, h));
                    sources.insert(base.to_string(), base_path.clone());

                    for key in permutations.keys() {
                        // We dont color the permutations yet, but atleast they sort of work
                        let name = format!("{base}{separator}{key}");
                        sizes.insert(name.clone(), (w, h));
                        sources.insert(name, base_path.clone());
                    }
                }
            }
        }
    }

    let entries = sizes
        .into_iter()
        .map(|(name, (w, h))| SpriteEntry {
            name,
            width: w,
            height: h,
        })
        .collect();

    Ok((entries, sources))
}

pub fn render_atlas_image(
    stitched: &Atlas,
    sources: &HashMap<String, PathBuf>,
) -> Result<image::RgbaImage, AtlasError> {
    use image::{GenericImage, RgbaImage, imageops};

    let mut atlas =
        RgbaImage::from_pixel(stitched.width, stitched.height, image::Rgba([0, 0, 0, 0]));

    for (name, rect) in &stitched.sprites {
        let path = sources.get(name).ok_or_else(|| {
            AtlasError::MissingTexture(PathBuf::from(format!("<unknown for {name}>")))
        })?;

        let mut img = image::open(path)?.into_rgba8();

        if img.width() != rect.width || img.height() != rect.height {
            img = imageops::resize(&img, rect.width, rect.height, imageops::FilterType::Nearest);
        }

        atlas
            .copy_from(&img, rect.x, rect.y)
            .expect("copy_into atlas bounds");
    }

    Ok(atlas)
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
