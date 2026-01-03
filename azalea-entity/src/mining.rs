use azalea_block::{BlockBehavior, BlockTrait};
use azalea_core::tier::get_item_tier;
use azalea_registry::{
    builtin::{BlockKind, ItemKind, MobEffect},
    tags,
};

use crate::{ActiveEffects, Attributes, FluidOnEyes, Physics};

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
    held_item: ItemKind,
    fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
    attributes: &Attributes,
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
        fluid_on_eyes,
        physics,
        attributes,
        active_effects,
    );
    (base_destroy_speed / destroy_time) / divisor as f32
}

fn has_correct_tool_for_drops(block: &dyn BlockTrait, tool: ItemKind) -> bool {
    if !block.behavior().requires_correct_tool_for_drops {
        return true;
    }
    let registry_block = block.as_registry_block();
    if tool == ItemKind::Shears {
        matches!(
            registry_block,
            BlockKind::Cobweb | BlockKind::RedstoneWire | BlockKind::Tripwire
        )
    } else if tags::items::SWORDS.contains(&tool) {
        registry_block == BlockKind::Cobweb
    } else if tags::items::PICKAXES.contains(&tool)
        || tags::items::SHOVELS.contains(&tool)
        || tags::items::HOES.contains(&tool)
        || tags::items::AXES.contains(&tool)
    {
        let tier = get_item_tier(tool).expect("all pickaxes and shovels should be matched");
        let tier_level = tier.level();
        !((tier_level < 3 && tags::blocks::NEEDS_DIAMOND_TOOL.contains(&registry_block))
            || (tier_level < 2 && tags::blocks::NEEDS_IRON_TOOL.contains(&registry_block))
            || (tier_level < 1 && tags::blocks::NEEDS_STONE_TOOL.contains(&registry_block)))
    } else {
        false
    }
}

/// Returns the destroy speed of the given block with the given tool, taking
/// enchantments and effects into account.
///
/// If the player is not holding anything, then `tool` should be
/// `ItemKind::Air`.
fn destroy_speed(
    block: BlockKind,
    tool: ItemKind,
    _fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
    attributes: &Attributes,
    active_effects: &ActiveEffects,
) -> f32 {
    let mut base_destroy_speed = base_destroy_speed(block, tool);

    if base_destroy_speed > 1. {
        // efficiency enchantment
        base_destroy_speed += attributes.mining_efficiency.calculate() as f32;
    }

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

fn base_destroy_speed(block: BlockKind, tool: ItemKind) -> f32 {
    if tool == ItemKind::Shears {
        if block == BlockKind::Cobweb || tags::blocks::LEAVES.contains(&block) {
            15.
        } else if tags::blocks::WOOL.contains(&block) {
            5.
        } else if matches!(block, BlockKind::Vine | BlockKind::GlowLichen) {
            2.
        } else {
            1.
        }
    } else if tags::items::SWORDS.contains(&tool) {
        if block == BlockKind::Cobweb {
            15.
        } else if tags::blocks::SWORD_EFFICIENT.contains(&block) {
            1.5
        } else {
            1.
        }
    } else if tags::items::PICKAXES.contains(&tool) {
        if tags::blocks::MINEABLE_PICKAXE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if tags::items::SHOVELS.contains(&tool) {
        if tags::blocks::MINEABLE_SHOVEL.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if tags::items::HOES.contains(&tool) {
        if tags::blocks::MINEABLE_HOE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else if tags::items::AXES.contains(&tool) {
        if tags::blocks::MINEABLE_AXE.contains(&block) {
            get_item_tier(tool).unwrap().speed()
        } else {
            1.
        }
    } else {
        1.
    }
}
