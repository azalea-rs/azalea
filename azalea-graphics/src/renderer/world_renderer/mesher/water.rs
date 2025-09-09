use azalea::{
    blocks::{BlockState, properties::WaterLevel},
    registry::Block,
};
use glam::{IVec3, Vec3};

use crate::renderer::{
    assets::processed::atlas::PlacedSprite,
    world_renderer::{
        BlockVertex,
        mesher::{
            MeshBuilder,
            helpers::quad_uvs,
        },
    },
};

pub fn mesh_water(block: BlockState, local: IVec3, builder: &mut MeshBuilder) {
    let tint = builder
        .block_colors
        .get_color(block, builder.section, local, 0, builder.assets);

    let center_height = fluid_height(local, block, builder);

    let h_north = fluid_height(local + IVec3::new(0, 0, -1), block, builder);
    let h_south = fluid_height(local + IVec3::new(0, 0, 1), block, builder);
    let h_east = fluid_height(local + IVec3::new(1, 0, 0), block, builder);
    let h_west = fluid_height(local + IVec3::new(-1, 0, 0), block, builder);

    let h_ne = average_heights(center_height, h_north, h_east);
    let h_nw = average_heights(center_height, h_north, h_west);
    let h_se = average_heights(center_height, h_south, h_east);
    let h_sw = average_heights(center_height, h_south, h_west);

    let still = builder.assets.get_sprite_rect("block/water_still").unwrap();
    let flow = builder.assets.get_sprite_rect("block/water_flow").unwrap();

    let above = builder.section.blocks[local.x as usize][local.y as usize + 1][local.z as usize];
    if Block::from(above) != Block::Water {
        mesh_water_top(local, h_ne, h_nw, h_sw, h_se, still, flow, tint, builder);
    }

    let below = builder.section.blocks[local.x as usize][local.y as usize - 1][local.z as usize];
    if Block::from(below) != Block::Water {
        mesh_water_bottom(local, still, tint, builder);
    }

    mesh_water_sides(local, center_height, block, flow, tint, builder);
}

fn fluid_height(local: IVec3, _block: BlockState, builder: &MeshBuilder) -> f32 {
    let state = builder.section.blocks[local.x as usize][local.y as usize][local.z as usize];
    if Block::from(state) == Block::Water {
        let level = state.property::<WaterLevel>().unwrap() as u32;
        if level == 0 {
            1.0
        } else {
            (8 - level) as f32 / 8.0
        }
    } else {
        1.0
    }
}

fn average_heights(center: f32, a: f32, b: f32) -> f32 {
    let mut sum = 0.0;
    let mut weight = 0.0;
    for &h in &[center, a, b] {
        if h >= 0.8 {
            sum += h * 10.0;
            weight += 10.0;
        } else if h >= 0.0 {
            sum += h;
            weight += 1.0;
        }
    }
    if weight > 0.0 { sum / weight } else { 0.0 }
}

fn mesh_water_top(
    local: IVec3,
    h_ne: f32,
    h_nw: f32,
    h_sw: f32,
    h_se: f32,
    still: &PlacedSprite,
    _flow: &PlacedSprite,
    tint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let base = Vec3::new(
        (local.x - 1) as f32 + builder.section.spos.x as f32 * 16.0,
        (local.y - 1) as f32 + builder.section.spos.y as f32 * 16.0,
        (local.z - 1) as f32 + builder.section.spos.z as f32 * 16.0,
    );

    let positions = [
        base + Vec3::new(0.0, h_sw, 0.0),
        base + Vec3::new(0.0, h_nw, 1.0),
        base + Vec3::new(1.0, h_ne, 1.0),
        base + Vec3::new(1.0, h_se, 0.0),
    ];

    let uvs = quad_uvs(
        still,
        builder.assets.block_atlas.width,
        builder.assets.block_atlas.height,
    );

    let quad: [BlockVertex; 4] = std::array::from_fn(|i| BlockVertex {
        position: positions[i].into(),
        ao: 3.0,
        uv: uvs[i],
        tint,
    });

    builder.push_water_quad(quad);
}

fn mesh_water_bottom(
    local: IVec3,
    still: &PlacedSprite,
    tint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let base = Vec3::new(
        (local.x - 1) as f32 + builder.section.spos.x as f32 * 16.0,
        (local.y - 1) as f32 + builder.section.spos.y as f32 * 16.0,
        (local.z - 1) as f32 + builder.section.spos.z as f32 * 16.0,
    );

    let positions = [
        base + Vec3::new(0.0, 0.0, 0.0),
        base + Vec3::new(1.0, 0.0, 0.0),
        base + Vec3::new(1.0, 0.0, 1.0),
        base + Vec3::new(0.0, 0.0, 1.0),
    ];

    let uvs = quad_uvs(
        still,
        builder.assets.block_atlas.width,
        builder.assets.block_atlas.height,
    );

    let quad: [BlockVertex; 4] = std::array::from_fn(|i| BlockVertex {
        position: positions[i].into(),
        ao: 3.0,
        uv: uvs[i],
        tint,
    });

    builder.push_water_quad(quad);
}

fn mesh_water_sides(
    local: IVec3,
    height: f32,
    _block: BlockState,
    sprite: &PlacedSprite,
    tint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let base = Vec3::new(
        (local.x - 1) as f32 + builder.section.spos.x as f32 * 16.0,
        (local.y - 1) as f32 + builder.section.spos.y as f32 * 16.0,
        (local.z - 1) as f32 + builder.section.spos.z as f32 * 16.0,
    );

    let dirs = [
        // North face (z = 0, looking towards negative z)
        (
            IVec3::new(0, 0, -1),
            [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)],
        ),
        // South face (z = 1, looking towards positive z)  
        (
            IVec3::new(0, 0, 1),
            [Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0)],
        ),
        // West face (x = 0, looking towards negative x)
        (
            IVec3::new(-1, 0, 0),
            [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)],
        ),
        // East face (x = 1, looking towards positive x)
        (
            IVec3::new(1, 0, 0),
            [Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0)],
        ),
    ];

    let uvs = quad_uvs(
        sprite,
        builder.assets.block_atlas.width,
        builder.assets.block_atlas.height,
    );

    for (offset, [low_a, low_b]) in dirs {
        let neighbor = local + offset;
        let state =
            builder.section.blocks[neighbor.x as usize][neighbor.y as usize][neighbor.z as usize];
        if Block::from(state) != Block::Water {
            let positions = [
                base + low_a,
                base + low_a + Vec3::new(0.0, height, 0.0),
                base + low_b + Vec3::new(0.0, height, 0.0),
                base + low_b,
            ];

            let quad: [BlockVertex; 4] = std::array::from_fn(|i| BlockVertex {
                position: positions[i].into(),
                ao: 3.0,
                uv: uvs[i],
                tint,
            });

            builder.push_water_quad(quad);
        }
    }
}
