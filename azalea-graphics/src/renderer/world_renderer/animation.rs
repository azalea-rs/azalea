use std::collections::HashMap;

use image::{GenericImageView, RgbaImage};

use crate::renderer::assets::processed::atlas::{Atlas, TextureEntry};

struct AnimationManager {
    block_states: HashMap<String, BlockAnimationState>,
}

impl AnimationManager {
    pub fn tick(&mut self, animations: &HashMap<String, (Vec<u32>, u32)>) {
        for (name, state) in self.block_states.iter_mut() {
            if let Some((frames, default_time)) = animations.get(name) {
                state.update(frames, *default_time);
            }
        }
    }
}

struct BlockAnimationState {
    current_frame: usize,
    next_frame: usize,
    ticks_elapsed: u32,
    frame_time: u32,
}

impl BlockAnimationState {
    fn update(&mut self, frames: &[u32], default_time: u32) {
        self.ticks_elapsed += 1;
        if self.ticks_elapsed >= self.frame_time {
            self.current_frame = (self.current_frame + 1) % frames.len();
            self.next_frame = (self.current_frame + 1) % frames.len();
            self.frame_time = default_time;
            self.ticks_elapsed = 0;
        }
    }

    fn alpha(&self) -> f32 {
        self.ticks_elapsed as f32 / self.frame_time as f32
    }
}

pub fn create_initial_atlas(
    atlas: &Atlas,
    textures: &HashMap<String, TextureEntry>,
) -> RgbaImage {
    let mut atlas_img = RgbaImage::new(atlas.width, atlas.height);

    for (name, placed) in &atlas.sprites {
        if let Some(tex) = textures.get(name) {
            let (fw, fh) = tex.size();
            let frame_img = &tex.data;

            let first_frame = frame_img.view(0, 0, fw, fh).to_image();

            for y in 0..placed.height {
                for x in 0..placed.width {
                    let px = first_frame.get_pixel(x, y);
                    atlas_img.put_pixel(placed.x + x, placed.y + y, *px);
                }
            }
        }
    }

    atlas_img
}
