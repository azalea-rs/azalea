use std::collections::HashMap;

use thiserror::Error;

use crate::renderer::assets::processed::atlas::TextureEntry;

#[derive(Debug, Clone)]
pub struct PlacedSprite {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}


#[derive(Debug)]
pub struct Atlas {
    pub width: u32,
    pub height: u32,
    pub sprites: HashMap<String, PlacedSprite>,
}

#[derive(Debug, Error)]
pub enum StitchError {
    #[error("Cannot fit sprites into atlas of size {max_width}x{max_height}")]
    CannotFit { max_width: u32, max_height: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Rect {
    #[inline]
    fn right(&self) -> u32 {
        self.x + self.w
    }
    #[inline]
    fn bottom(&self) -> u32 {
        self.y + self.h
    }
    #[inline]
    fn contains(&self, other: &Rect) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.right() >= other.right()
            && self.bottom() >= other.bottom()
    }
    #[inline]
    fn intersects(&self, other: &Rect) -> bool {
        !(other.x >= self.right()
            || other.right() <= self.x
            || other.y >= self.bottom()
            || other.bottom() <= self.y)
    }
}

fn split_and_prune_free_list(free: &mut Vec<Rect>, used: Rect) {
    let mut new_free = Vec::with_capacity(free.len() + 4);
    for fr in free.drain(..) {
        if !fr.intersects(&used) {
            new_free.push(fr);
            continue;
        }
        if used.x > fr.x {
            new_free.push(Rect {
                x: fr.x,
                y: fr.y,
                w: used.x - fr.x,
                h: fr.h,
            });
        }
        if used.right() < fr.right() {
            new_free.push(Rect {
                x: used.right(),
                y: fr.y,
                w: fr.right() - used.right(),
                h: fr.h,
            });
        }
        if used.y > fr.y {
            new_free.push(Rect {
                x: fr.x,
                y: fr.y,
                w: fr.w,
                h: used.y - fr.y,
            });
        }
        if used.bottom() < fr.bottom() {
            new_free.push(Rect {
                x: fr.x,
                y: used.bottom(),
                w: fr.w,
                h: fr.bottom() - used.bottom(),
            });
        }
    }
    new_free.retain(|r| r.w > 0 && r.h > 0);
    new_free.sort_by_key(|r| (r.x, r.y, r.w, r.h));
    new_free.dedup();

    let mut pruned = Vec::with_capacity(new_free.len());
    'outer: for i in 0..new_free.len() {
        for j in 0..new_free.len() {
            if i != j && new_free[j].contains(&new_free[i]) {
                continue 'outer; // drop i
            }
        }
        pruned.push(new_free[i]);
    }
    *free = pruned;
}

fn choose_position(free: &[Rect], w: u32, h: u32) -> Option<(usize, Rect, i32, i32)> {
    let mut best: Option<(usize, Rect, i32, i32)> = None;
    for (idx, fr) in free.iter().enumerate() {
        if w <= fr.w && h <= fr.h {
            let short = (fr.w as i32 - w as i32).min(fr.h as i32 - h as i32);
            let long = (fr.w as i32 - w as i32).max(fr.h as i32 - h as i32);
            let cand = (
                idx,
                Rect {
                    x: fr.x,
                    y: fr.y,
                    w,
                    h,
                },
                short,
                long,
            );
            best = Some(match best {
                None => cand,
                Some(cur) => {
                    if cand.2 < cur.2
                        || (cand.2 == cur.2
                            && (cand.3 < cur.3
                                || (cand.3 == cur.3
                                    && (cand.1.y < cur.1.y
                                        || (cand.1.y == cur.1.y && cand.1.x < cur.1.x)))))
                    {
                        cand
                    } else {
                        cur
                    }
                }
            });
        }
    }
    best
}

pub fn stitch_sprites(
    textures: &HashMap<String, TextureEntry>,
    max_width: u32,
    max_height: u32,
) -> Result<Atlas, StitchError> {
    if textures.is_empty() {
        return Ok(Atlas {
            width: 0,
            height: 0,
            sprites: HashMap::new(),
        });
    }

    for (name, entry) in textures {
        let (w, h) = entry.size();
        if w == 0 || h == 0 || w > max_width || h > max_height {
            return Err(StitchError::CannotFit { max_width, max_height });
        }
    }

    let mut tex_list: Vec<(&String, &TextureEntry)> = textures.iter().collect();
    tex_list.sort_by_key(|(_, entry)| {
        let (w, h) = entry.size();
        std::cmp::Reverse((w as u64 * h as u64, h, w))
    });

    let mut free: Vec<Rect> = vec![Rect {
        x: 0,
        y: 0,
        w: max_width,
        h: max_height,
    }];

    let mut placed = HashMap::with_capacity(tex_list.len());
    let mut used_right = 0u32;
    let mut used_bottom = 0u32;

    for (name, entry) in tex_list {
        let (w, h) = entry.size();

        if let Some((_idx, pos_rect, _short, _long)) = choose_position(&free, w, h) {
            placed.insert(
                name.clone(),
                PlacedSprite {
                    x: pos_rect.x,
                    y: pos_rect.y,
                    width: pos_rect.w,
                    height: pos_rect.h,
                },
            );
            used_right = used_right.max(pos_rect.right());
            used_bottom = used_bottom.max(pos_rect.bottom());

            split_and_prune_free_list(&mut free, pos_rect);
        } else {
            return Err(StitchError::CannotFit { max_width, max_height });
        }
    }

    Ok(Atlas {
        width: used_right,
        height: used_bottom,
        sprites: placed,
    })
}
