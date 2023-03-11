use azalea_core::{lerp, BlockHitResult, BlockPos, Direction, Vec3, EPSILON};
use azalea_world::ChunkStorage;

pub struct ClipContext {
    pub from: Vec3,
    pub to: Vec3,
    pub block: BlockClipContext,
    pub fluid: FluidClipContext,
    pub collision_context: CollisionContext,
}

pub struct BlockClipContext {
    pub shape_getter
}
pub struct FluidClipContext {}
pub struct CollisionContext {}

pub fn clip(chunk_storage: &mut ChunkStorage, context: ClipContext) -> BlockHitResult {
    traverse_blocks(
        context.from,
        context.to,
        context,
        |context, block_pos| {
            let block_state = chunk_storage.get_block_state(block_pos);
            // TODO: add fluid stuff to this (see getFluidState in vanilla source)
            let block_shape = context.block_shape(block_state, chunk_storage, block_pos);
            let block_hit_result =
                chunk_storage.clip_with_interaction_override(context.from, context.to, block_pos);
            // let block_distance =

            None
        },
        |context| {
            let vec = context.from - context.to;
            BlockHitResult::miss(
                context.to,
                Direction::nearest(vec),
                BlockPos::from(context.to),
            )
        },
    )
}

pub fn traverse_blocks<C, T>(
    from: Vec3,
    to: Vec3,
    context: C,
    get_hit_result: impl Fn(&C, &BlockPos) -> Option<T>,
    get_miss_result: impl Fn(&C) -> T,
) -> T {
    if from == to {
        return get_miss_result(&context);
    }

    let right_after_end = Vec3 {
        x: lerp(-EPSILON, to.x, from.x),
        y: lerp(-EPSILON, to.y, from.y),
        z: lerp(-EPSILON, to.z, from.z),
    };

    let right_before_start = Vec3 {
        x: lerp(-EPSILON, from.x, to.x),
        y: lerp(-EPSILON, from.y, to.y),
        z: lerp(-EPSILON, from.z, to.z),
    };

    let mut current_block = BlockPos::from(right_before_start);
    if let Some(data) = get_hit_result(&context, &current_block) {
        return data;
    }

    let vec = right_after_end - right_before_start;

    /// Returns either -1, 0, or 1, depending on whether the number is negative,
    /// zero, or positive.
    ///
    /// This function exists because f64::signum doesn't check for 0.
    fn get_number_sign(num: f64) -> f64 {
        if num == 0. {
            0.
        } else {
            num.signum()
        }
    }

    let vec_sign = Vec3 {
        x: get_number_sign(vec.x),
        y: get_number_sign(vec.y),
        z: get_number_sign(vec.z),
    };

    #[rustfmt::skip]
    let percentage_step = Vec3 {
        x: if vec_sign.x == 0. { f64::MAX } else { vec_sign.x / vec.x },
        y: if vec_sign.y == 0. { f64::MAX } else { vec_sign.y / vec.y },
        z: if vec_sign.z == 0. { f64::MAX } else { vec_sign.z / vec.z },
    };

    let mut percentage = Vec3 {
        x: percentage_step.x
            * if vec_sign.x > 0. {
                1. - right_before_start.x.fract()
            } else {
                right_before_start.x.fract()
            },
        y: percentage_step.y
            * if vec_sign.y > 0. {
                1. - right_before_start.y.fract()
            } else {
                right_before_start.y.fract()
            },
        z: percentage_step.z
            * if vec_sign.z > 0. {
                1. - right_before_start.z.fract()
            } else {
                right_before_start.z.fract()
            },
    };

    loop {
        if percentage.x > 1. && percentage.y > 1. && percentage.z > 1. {
            return get_miss_result(&context);
        }

        if percentage.x < percentage.y {
            if percentage.x < percentage.z {
                current_block.x += vec_sign.x as i32;
                percentage.x += percentage_step.x;
            } else {
                current_block.z += vec_sign.z as i32;
                percentage.z += percentage_step.z;
            }
        } else if percentage.y < percentage.z {
            current_block.y += vec_sign.y as i32;
            percentage.y += percentage_step.y;
        } else {
            current_block.z += vec_sign.z as i32;
            percentage.z += percentage_step.z;
        }

        if let Some(data) = get_hit_result(&context, &current_block) {
            return data;
        }
    }
}
