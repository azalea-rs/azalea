use azalea_block::{BlockBehavior, BlockTrait};
use azalea_inventory::{ItemStack, components::Tool};
use azalea_registry::builtin::{BlockKind, MobEffect};

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
    held_item: &ItemStack,
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
    (base_destroy_speed / destroy_time) / (divisor as f32)
}

fn has_correct_tool_for_drops(block: &dyn BlockTrait, item: &ItemStack) -> bool {
    if !block.behavior().requires_correct_tool_for_drops {
        return true;
    }
    let Some(tool) = item.get_component::<Tool>() else {
        return false;
    };
    let registry_block = block.as_registry_block();
    for rule in &tool.rules {
        if let Some(correct) = rule.correct_for_drops
            && rule.blocks.contains(registry_block)
        {
            return correct;
        }
    }

    false
}

/// Returns the destroy speed of the given block with the given tool, taking
/// enchantments and effects into account.
///
/// If the player is not holding anything, then `tool` should be
/// `ItemKind::Air`.
fn destroy_speed(
    block: BlockKind,
    tool: &ItemStack,
    _fluid_on_eyes: &FluidOnEyes,
    physics: &Physics,
    attributes: &Attributes,
    active_effects: &ActiveEffects,
) -> f32 {
    let mut speed = base_destroy_speed(block, tool);

    if speed > 1. {
        // efficiency enchantment
        speed += attributes.mining_efficiency.calculate() as f32;
    }

    if let Some(dig_speed_amplifier) = active_effects.get_dig_speed_amplifier() {
        speed *= 1. + (dig_speed_amplifier + 1) as f32 * 0.2;
    }

    if let Some(dig_slowdown) = active_effects.get_level(MobEffect::MiningFatigue) {
        let multiplier = match dig_slowdown {
            0 => 0.3,
            1 => 0.09,
            2 => 0.0027,
            _ => 8.1E-4,
        };
        speed *= multiplier;
    }

    speed *= attributes.block_break_speed.calculate() as f32;

    // TODO
    // if **fluid_on_eyes == FluidKind::Water
    //     && enchantments::get_enchant_level(registry::Enchantment::AquaAffinity,
    // player_inventory)         == 0
    // {
    //     base_destroy_speed /= 5.;
    // }

    if !physics.on_ground {
        speed /= 5.;
    }

    speed
}

fn base_destroy_speed(block: BlockKind, item: &ItemStack) -> f32 {
    let tool = item.get_component::<Tool>();
    let Some(tool) = tool else { return 1. };
    for rule in &tool.rules {
        if let Some(speed) = rule.speed
            && rule.blocks.contains(block)
        {
            return speed;
        }
    }
    tool.default_mining_speed
}
