use azalea_block::{BlockBehavior, BlockTrait};
use azalea_core::tier::get_item_tier;
use azalea_registry::{self as registry, MobEffect};

use crate::{ActiveEffects, FluidOnEyes, Physics};

/// How much progress is made towards mining the block per tick, as a
/// percentage.
///
/// If this is 1, then the block gets broken instantly.
///
/// You can divide 1 by this and then round up to get the number of ticks it
/// takes to mine the block.
///
/// The player inventory is needed to check your armor and offhand for modifiers
/// to your mining speed.
pub fn get_mine_progress(
    block: &dyn BlockTrait,
    held_item: registry::Item,
    player_inventory: &azalea_inventory::Menu,
    fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
    active_effects: &ActiveEffects,
) -> f32 {
    let block_behavior: BlockBehavior = block.behavior();

    let destroy_time = block_behavior.destroy_time;
    if destroy_time == -1. {
        return 0.;
    }
    let divisor = if has_correct_tool_for_drops(block, held_item) {
        30
    } else {
        100
    };

    let base_destroy_speed = destroy_speed(
        block.as_registry_block(),
        held_item,
        player_inventory,
        fluid_on_eyes,
        physics,
        active_effects,
    );
    (base_destroy_speed / destroy_time) / divisor as f32
}

fn has_correct_tool_for_drops(block: &dyn BlockTrait, tool: registry::Item) -> bool {
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
        !((tier_level < 3 && registry::tags::blocks::NEEDS_DIAMOND_TOOL.contains(&registry_block))
            || (tier_level < 2
                && registry::tags::blocks::NEEDS_IRON_TOOL.contains(&registry_block))
            || (tier_level < 1
                && registry::tags::blocks::NEEDS_STONE_TOOL.contains(&registry_block)))
    } else {
        false
    }
}

/// Returns the destroy speed of the given block with the given tool, taking
/// enchantments and effects into account.
///
/// If the player is not holding anything, then `tool` should be `Item::Air`.
fn destroy_speed(
    block: registry::Block,
    tool: registry::Item,
    _player_inventory: &azalea_inventory::Menu,
    _fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
    active_effects: &ActiveEffects,
) -> f32 {
    let mut base_destroy_speed = base_destroy_speed(block, tool);

    // add efficiency enchantment
    // TODO
    // if base_destroy_speed > 1. {
    //     let efficiency_level =
    //         enchantments::get_enchant_level(registry::Enchantment::Efficiency,
    // player_inventory);     if efficiency_level > 0 && tool !=
    // registry::Item::Air {         base_destroy_speed += (efficiency_level *
    // efficiency_level + 1) as f32;     }
    // }

    if let Some(dig_speed_amplifier) = active_effects.get_dig_speed_amplifier() {
        base_destroy_speed *= 1. + (dig_speed_amplifier + 1) as f32 * 0.2;
    }

    if let Some(dig_slowdown) = active_effects.get_level(MobEffect::MiningFatigue) {
        let multiplier = match dig_slowdown {
            0 => 0.3,
            1 => 0.09,
            2 => 0.0027,
            _ => 8.1E-4,
        };
        base_destroy_speed *= multiplier;
    }

    // TODO
    // if **fluid_on_eyes == FluidKind::Water
    //     && enchantments::get_enchant_level(registry::Enchantment::AquaAffinity,
    // player_inventory)         == 0
    // {
    //     base_destroy_speed /= 5.;
    // }

    if !physics.on_ground {
        base_destroy_speed /= 5.;
    }

    base_destroy_speed
}

fn base_destroy_speed(block: registry::Block, tool: registry::Item) -> f32 {
    if tool == registry::Item::Shears {
        if block == registry::Block::Cobweb || registry::tags::blocks::LEAVES.contains(&block) {
            15.
        } else if registry::tags::blocks::WOOL.contains(&block) {
            5.
        } else if matches!(block, registry::Block::Vine | registry::Block::GlowLichen) {
            2.
        } else {
            1.
        }
    } else if registry::tags::items::SWORDS.contains(&tool) {
        if block == registry::Block::Cobweb {
            15.
        } else if registry::tags::blocks::SWORD_EFFICIENT.contains(&block) {
            1.5
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
