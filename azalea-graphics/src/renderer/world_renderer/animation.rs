use std::collections::HashMap;

use image::{GenericImageView, RgbaImage};

use crate::renderer::assets::processed::{
    animation::Animation,
    atlas::{Atlas, TextureEntry},
};

pub struct AnimationManager {
    block_states: HashMap<String, BlockAnimationState>,
}

impl AnimationManager {
    pub fn from_textures(textures: &HashMap<String, TextureEntry>) -> Self {
        let mut block_states = HashMap::new();

        for (name, tex) in textures {
            if let Some(animation) = &tex.animation {
                let frame_time = animation.frame_time(0);
                block_states.insert(
                    name.clone(),
                    BlockAnimationState {
                        current_frame: 0,
                        next_frame: if animation.frames_len(tex.size()) > 1 {
                            1
                        } else {
                            0
                        },
                        ticks_elapsed: 0,
                        frame_time,
                        dirty: true,
                    },
                );
            }
        }

        Self { block_states }
    }

    pub fn dirty_textures<'a>(
        &'a mut self,
        textures: &'a HashMap<String, TextureEntry>,
    ) -> Vec<(&'a str, &'a TextureEntry, (u32, u32), usize)> {
        let mut result = Vec::new();

        for (name, state) in self.block_states.iter_mut() {
            if state.dirty {
                if let Some(texture) = textures.get(name) {
                    let size = texture.size();
                    result.push((name.as_str(), texture, size, state.current_frame));
                }
            }
        }

        result
    }

    pub fn tick(&mut self, textures: &HashMap<String, TextureEntry>) {
        for (name, state) in self.block_states.iter_mut() {
            if let Some(texture) = textures.get(name) {
                if let Some(animation) = &texture.animation {
                    state.update(animation, texture.data.dimensions());
                }
            }
        }
    }
}

pub struct BlockAnimationState {
    pub current_frame: usize,
    pub next_frame: usize,
    pub ticks_elapsed: u32,
    pub frame_time: u32,
    pub dirty: bool,
}

impl BlockAnimationState {
    pub fn update(&mut self, animation: &Animation, image_size: (u32, u32)) {
        self.ticks_elapsed += 1;
        if self.ticks_elapsed >= self.frame_time {
            let frames_len = animation.frames_len(image_size);

            self.current_frame = (self.current_frame + 1) % frames_len;
            self.next_frame = (self.current_frame + 1) % frames_len;
            self.frame_time = animation.frame_time(self.current_frame);
            self.ticks_elapsed = 0;
            self.dirty = true;
        }
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

pub fn create_initial_atlas(atlas: &Atlas, textures: &HashMap<String, TextureEntry>) -> RgbaImage {
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
