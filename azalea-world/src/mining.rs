use azalea_block::BlockBehavior;

pub fn get_destroy_progress(block_behavior: &BlockBehavior) -> f32 {
    //    public float getDestroyProgress(BlockState blockState, Player player,
    // BlockGetter world, BlockPos blockPos) {       float destroySpeed =
    // blockState.getDestroySpeed(world, blockPos);       if (destroySpeed
    // == -1.0F) {          return 0.0F;
    //       } else {
    //          int divider = player.hasCorrectToolForDrops(blockState) ? 30 : 100;
    //          return player.getDestroySpeed(blockState) / destroySpeed /
    // (float)divider;       }
    //    }

    let destroy_speed = block_behavior.destroy_time;
    if destroy_speed == -1. {
        return 0.;
    }
    let divider = 30; // 100 if wrong tool
}

fn get_destroy_speed_for_item(item_kind: azalea_registry::Item) {
    //
}
