//! Random utility functions that are useful for bots.

use std::f64::consts::PI;

use azalea_core::position::{BlockPos, Vec3};
use azalea_entity::LookDirection;
use azalea_world::ChunkStorage;

/// Return the look direction that would make a client at `current` be
/// looking at `target`.
pub fn direction_looking_at(current: &Vec3, target: &Vec3) -> LookDirection {
    // borrowed from mineflayer's Bot.lookAt because i didn't want to do math
    let delta = target - current;
    let y_rot = (PI - f64::atan2(-delta.x, -delta.z)) * (180.0 / PI);
    let ground_distance = f64::sqrt(delta.x * delta.x + delta.z * delta.z);
    let x_rot = f64::atan2(delta.y, ground_distance) * -(180.0 / PI);

    // clamp
    let y_rot = y_rot.rem_euclid(360.0);
    let x_rot = x_rot.clamp(-90.0, 90.0) % 360.0;

    LookDirection {
        x_rot: x_rot as f32,
        y_rot: y_rot as f32,
    }
}

/// Return the block that we'd be looking at if we were at a given position and
/// looking at a given block.
///
/// This is useful for telling if we'd be able to reach a block from a certain
/// position, like for the pathfinder's [`ReachBlockPosGoal`].
///
/// Also see [`get_hit_result_while_looking_at_with_eye_position`].
///
/// [`ReachBlockPosGoal`]: crate::pathfinder::goals::ReachBlockPosGoal
pub fn get_hit_result_while_looking_at(
    chunk_storage: &ChunkStorage,
    player_position: BlockPos,
    look_target: BlockPos,
) -> BlockPos {
    let eye_position = Vec3 {
        x: player_position.x as f64 + 0.5,
        y: player_position.y as f64 + 1.53,
        z: player_position.z as f64 + 0.5,
    };
    get_hit_result_while_looking_at_with_eye_position(chunk_storage, eye_position, look_target)
}

/// Return the block that we'd be looking at if our eyes are at a given position
/// and looking at a given block.
///
/// This is called by [`get_hit_result_while_looking_at`].
pub fn get_hit_result_while_looking_at_with_eye_position(
    chunk_storage: &azalea_world::ChunkStorage,
    eye_position: Vec3,
    look_target: BlockPos,
) -> BlockPos {
    let look_direction = direction_looking_at(&eye_position, &look_target.center());
    let block_hit_result =
        azalea_client::interact::pick(&look_direction, &eye_position, chunk_storage, 4.5);
    block_hit_result.block_pos
}

/// Get a vec of block positions that we can reach from this position.
pub fn get_reachable_blocks_around_player(
    player_position: BlockPos,
    chunk_storage: &ChunkStorage,
) -> Vec<BlockPos> {
    // check a 12x12x12 area around the player
    let mut blocks = Vec::new();

    for x in -6..=6 {
        // y is 1 up to somewhat offset for the eye height
        for y in -5..=7 {
            for z in -6..=6 {
                let block_pos = player_position + BlockPos::new(x, y, z);
                let block_state = chunk_storage
                    .get_block_state(&block_pos)
                    .unwrap_or_default();

                if block_state.is_air() {
                    // fast path, skip if it's air
                    continue;
                }

                let hit_result =
                    get_hit_result_while_looking_at(chunk_storage, player_position, block_pos);
                if hit_result == block_pos {
                    blocks.push(block_pos);
                }
            }
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_looking_at() {
        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(direction.y_rot, 0.0);
        assert_eq!(direction.x_rot, 0.0);

        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(direction.y_rot, 270.0);
        assert_eq!(direction.x_rot, 0.0);

        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(direction.y_rot, 180.0);
        assert_eq!(direction.x_rot, 0.0);

        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(direction.y_rot, 90.0);
        assert_eq!(direction.x_rot, 0.0);

        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(direction.y_rot, 0.0);
        assert_eq!(direction.x_rot, -90.0);

        let direction = direction_looking_at(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(direction.y_rot, 0.0);
        assert_eq!(direction.x_rot, 90.0);
    }
}
