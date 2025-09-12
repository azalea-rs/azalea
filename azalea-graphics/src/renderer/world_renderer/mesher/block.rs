use azalea::{
    blocks::{BlockState, BlockTrait},
    core::direction::Direction,
    physics::collision::BlockWithShape,
};
use glam::{IVec3, Vec3};

use crate::renderer::{
    assets::processed::{
        VariantDesc,
        model::{self, Cube},
    },
    chunk::LocalSection,
    world_renderer::{
        BlockVertex,
        mesher::{
            MeshBuilder,
            helpers::{
                 FACES, compute_ao, generate_uv, offset_to_coord,
                remap_uv_to_atlas
            },
        },
    },
};

pub fn mesh_block(block: BlockState, local: IVec3, builder: &mut MeshBuilder) {
    for desc in builder.assets.get_variant_descs(block) {
        let model = builder
            .assets
            .get_block_model(&desc.model)
            .expect("all block models must be loaded");

        for element in &model.elements {
            for face in FACES {
                if let Some(model_face) = face_for_direction(&element, face.dir) {
                    if let Some(cull_dir) = resolve_cullface(desc, model_face) {
                        if face_is_occluded(local, cull_dir, builder.section) {
                            continue;
                        }
                    }

                    // uv mapping
                    let uvs = generate_uv(face.dir, model_face.uv);


                    let tint = builder.block_colors.get_color(
                        block,
                        builder.section,
                        local,
                        model_face.tintindex,
                        builder.assets,
                    );

                    let sprite_name = builder
                        .assets
                        .get_block_model(&desc.model)
                        .unwrap()
                        .resolve_texture(&model_face.texture)
                        .unwrap_or("empty");

                    if let Some(spr) = builder.assets.get_sprite_rect(sprite_name) {
                        let mut quad = [BlockVertex {
                            position: [0.0; 3],
                            ao: 3.0,
                            uv: [0.0; 2],
                            tint,
                        }; 4];

                        for (i, &offset) in face.offsets.iter().enumerate() {
                            let local_pos = offset_to_coord(offset, element) / 16.0;

                            let world_pos = Vec3::new(
                                (local.x - 1) as f32 + builder.section.spos.x as f32 * 16.0,
                                (local.y - 1) as f32 + builder.section.spos.y as f32 * 16.0,
                                (local.z - 1) as f32 + builder.section.spos.z as f32 * 16.0,
                            );

                            let uv = remap_uv_to_atlas(
                                uvs[i],
                                spr,
                                builder.assets.block_atlas.width,
                                builder.assets.block_atlas.height,
                            );

                            quad[i] = BlockVertex {
                                position: (local_pos + world_pos).into(),
                                ao: if model.ambient_occlusion {
                                    compute_ao(local, offset, face.dir, builder.section)
                                        as f32
                                } else {
                                    3.0
                                },
                                uv,
                                tint,
                            };
                        }

                        builder.push_block_quad(quad);
                    }
                }
            }
        }
    }
}

/// Get the model face for a given direction (without applying rotations)
fn face_for_direction<'a>(element: &'a Cube, dir: Direction) -> Option<&'a model::Face> {
    match dir {
        Direction::Up => element.faces.up.as_ref(),
        Direction::Down => element.faces.down.as_ref(),
        Direction::North => element.faces.north.as_ref(),
        Direction::South => element.faces.south.as_ref(),
        Direction::East => element.faces.east.as_ref(),
        Direction::West => element.faces.west.as_ref(),
    }
}

/// Resolve the cullface direction considering rotations
fn resolve_cullface(desc: &VariantDesc, model_face: &model::Face) -> Option<Direction> {
    model_face.cullface.as_ref().and_then(|dir_str| {
        let dir = match dir_str.as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "north" => Direction::North,
            "south" => Direction::South,
            "east" => Direction::East,
            "west" => Direction::West,
            _ => return None,
        };
        Some(dir)
    })
}

fn face_is_occluded(local: IVec3, cull_dir: Direction, section: &LocalSection) -> bool {
    let offset = match cull_dir {
        Direction::Up => IVec3::new(0, 1, 0),
        Direction::Down => IVec3::new(0, -1, 0),
        Direction::North => IVec3::new(0, 0, -1),
        Direction::South => IVec3::new(0, 0, 1),
        Direction::East => IVec3::new(1, 0, 0),
        Direction::West => IVec3::new(-1, 0, 0),
    };

    let neighbor_pos = local + offset;

    if neighbor_pos.x < 0
        || neighbor_pos.y < 0
        || neighbor_pos.z < 0
        || neighbor_pos.x >= 18
        || neighbor_pos.y >= 18
        || neighbor_pos.z >= 18
    {
        return false;
    }

    let neighbor_state =
        section.blocks[neighbor_pos.x as usize][neighbor_pos.y as usize][neighbor_pos.z as usize].unwrap_or(BlockState::AIR);

    if neighbor_state.is_air() {
        return false;
    }

    let dyn_state: Box<dyn BlockTrait> = Box::from(neighbor_state);
    dyn_state.behavior().can_occlude && neighbor_state.is_collision_shape_full()
}
