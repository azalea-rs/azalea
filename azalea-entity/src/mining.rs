use azalea_block::{Block, BlockBehavior};
use azalea_core::tier::get_item_tier;
use azalea_registry as registry;

use crate::{effects, enchantments, FluidOnEyes, Physics};

pub fn get_mine_progress(
    block: &Box<dyn Block>,
    held_item: registry::Item,
    player_inventory: &azalea_inventory::Menu,
    fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
) -> f32 {
    // public float getDestroyProgress(BlockState blockState, Player player,
    // BlockGetter world, BlockPos blockPos) {     float destroySpeed =
    // blockState.getDestroySpeed(world, blockPos);       if (destroySpeed ==
    // -1.0F) {          return 0.0F;
    //       } else {
    //          int divider = player.hasCorrectToolForDrops(blockState) ? 30 : 100;
    //          return player.getDestroySpeed(blockState) / destroySpeed /
    // (float)divider;       }
    //    }

    let block_behavior: BlockBehavior = block.behavior();

    let destroy_time = block_behavior.destroy_time;
    if destroy_time == -1. {
        return 0.;
    }
    let divider = if has_correct_tool_for_drops(block, held_item) {
        30
    } else {
        100
    };

    (destroy_speed(
        block.as_registry_block(),
        held_item,
        player_inventory,
        fluid_on_eyes,
        physics,
    ) / destroy_time)
        / divider as f32
}

fn has_correct_tool_for_drops(block: &Box<dyn Block>, tool: registry::Item) -> bool {
    if !block.behavior().requires_correct_tool_for_drops {
        return true;
    }
    let registry_block = block.as_registry_block();
    if tool == registry::Item::Shears {
        matches!(
            registry_block,
            registry::Block::Cobweb | registry::Block::RedstoneWire | registry::Block::Tripwire
        )
    } else if registry::tags::items::SWORDS.contains(&tool) {
        registry_block == registry::Block::Cobweb
    } else if registry::tags::items::PICKAXES.contains(&tool)
        || registry::tags::items::SHOVELS.contains(&tool)
        || registry::tags::items::HOES.contains(&tool)
        || registry::tags::items::AXES.contains(&tool)
    {
        let tier = get_item_tier(tool).expect("all pickaxes and shovels should be matched");
        let tier_level = tier.level();
        if tier_level < 3 && registry::tags::blocks::NEEDS_DIAMOND_TOOL.contains(&registry_block) {
            false
        } else if tier_level < 2
            && registry::tags::blocks::NEEDS_IRON_TOOL.contains(&registry_block)
        {
            false
        } else if tier_level < 1
            && registry::tags::blocks::NEEDS_STONE_TOOL.contains(&registry_block)
        {
            false
        } else {
            true
        }
    } else {
        false
    }
}

/// Returns the destroy speed of the given block with the given tool, taking
/// into account enchantments and effects. If the player is not holding anything
/// then `tool` should be `Item::Air`.
fn destroy_speed(
    block: registry::Block,
    tool: registry::Item,
    player_inventory: &azalea_inventory::Menu,
    fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
) -> f32 {
    let mut base_destroy_speed = base_destroy_speed(block, tool);

    // add efficiency enchantment
    if base_destroy_speed > 1. {
        let efficiency_level =
            enchantments::get_enchant_level(registry::Enchantment::Efficiency, player_inventory);
        if efficiency_level > 0 && tool != registry::Item::Air {
            base_destroy_speed += (efficiency_level * efficiency_level + 1) as f32;
        }
    }

    if let Some(dig_speed_amplifier) = effects::get_dig_speed_amplifier() {
        base_destroy_speed *= 1. + (dig_speed_amplifier + 1) as f32 * 0.2;
    }

    if let Some(dig_slowdown) = effects::get_effect(registry::MobEffect::MiningFatigue) {
        let multiplier = match dig_slowdown {
            0 => 0.3,
            1 => 0.09,
            2 => 0.0027,
            _ => 8.1E-4,
        };
        base_destroy_speed *= multiplier;
    }

    if registry::tags::fluids::WATER.contains(&fluid_on_eyes)
        && enchantments::get_enchant_level(registry::Enchantment::AquaAffinity, player_inventory)
            == 0
    {
        base_destroy_speed /= 5.;
    }

    if !physics.on_ground {
        base_destroy_speed /= 5.;
    }

    println!("base destroy speed: {base_destroy_speed}");
    base_destroy_speed
}

fn base_destroy_speed(block: registry::Block, tool: registry::Item) -> f32 {
    if tool == registry::Item::Shears {
        if block == registry::Block::Cobweb || registry::tags::blocks::LEAVES.contains(&block) {
            return 15.;
        } else if registry::tags::blocks::WOOL.contains(&block) {
            return 5.;
        } else if matches!(block, registry::Block::Vine | registry::Block::GlowLichen) {
            return 2.;
        } else {
            1.
        }
    } else if registry::tags::items::SWORDS.contains(&tool) {
        if block == registry::Block::Cobweb {
            return 15.;
        } else if registry::tags::blocks::SWORD_EFFICIENT.contains(&block) {
            return 1.5;
        } else {
            1.
        }
    } else if registry::tags::items::PICKAXES.contains(&tool) {
        if registry::tags::blocks::MINEABLE_PICKAXE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if registry::tags::items::SHOVELS.contains(&tool) {
        if registry::tags::blocks::MINEABLE_SHOVEL.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if registry::tags::items::HOES.contains(&tool) {
        if registry::tags::blocks::MINEABLE_HOE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if registry::tags::items::AXES.contains(&tool) {
        if registry::tags::blocks::MINEABLE_AXE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else {
        1.
    }
}
