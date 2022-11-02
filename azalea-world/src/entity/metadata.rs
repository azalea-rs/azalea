// This file is generated from codegen/lib/code/entity.py

use super::{EntityDataValue, Pose, Rotations, VillagerData};
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, Particle, Slot};
use std::collections::VecDeque;
use uuid::Uuid;

pub struct Allay {
    pub abstract_creature: AbstractCreature,
    pub dancing: bool,
    pub can_duplicate: bool,
}

impl Allay {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let dancing = metadata.pop_front()?.as_boolean()?.clone();
        let can_duplicate = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            dancing,
            can_duplicate,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.dancing.clone()));
        metadata.push(EntityDataValue::Boolean(self.can_duplicate.clone()));
        metadata
    }
}
pub struct AreaEffectCloud {
    pub abstract_entity: AbstractEntity,
    pub radius: f32,
    pub color: i32,
    pub waiting: bool,
    pub particle: Particle,
}

impl AreaEffectCloud {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let radius = metadata.pop_front()?.as_float()?.clone();
        let color = metadata.pop_front()?.as_int()?.clone();
        let waiting = metadata.pop_front()?.as_boolean()?.clone();
        let particle = metadata.pop_front()?.as_particle()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            radius,
            color,
            waiting,
            particle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Float(self.radius.clone()));
        metadata.push(EntityDataValue::Int(self.color.clone()));
        metadata.push(EntityDataValue::Boolean(self.waiting.clone()));
        metadata.push(EntityDataValue::Particle(self.particle.clone()));
        metadata
    }
}
pub struct ArmorStand {
    pub abstract_living: AbstractLiving,
    pub small: bool,
    pub show_arms: bool,
    pub no_base_plate: bool,
    pub marker: bool,
    pub head_pose: Rotations,
    pub body_pose: Rotations,
    pub left_arm_pose: Rotations,
    pub right_arm_pose: Rotations,
    pub left_leg_pose: Rotations,
    pub right_leg_pose: Rotations,
}

impl ArmorStand {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let small = bitfield & 0x1 != 1;
        let show_arms = bitfield & 0x4 != 1;
        let no_base_plate = bitfield & 0x8 != 1;
        let marker = bitfield & 0x10 != 1;
        let head_pose = metadata.pop_front()?.as_rotations()?.clone();
        let body_pose = metadata.pop_front()?.as_rotations()?.clone();
        let left_arm_pose = metadata.pop_front()?.as_rotations()?.clone();
        let right_arm_pose = metadata.pop_front()?.as_rotations()?.clone();
        let left_leg_pose = metadata.pop_front()?.as_rotations()?.clone();
        let right_leg_pose = metadata.pop_front()?.as_rotations()?.clone();
        Some(Self {
            abstract_living: AbstractLiving::read(metadata)?,
            small,
            show_arms,
            no_base_plate,
            marker,
            head_pose,
            body_pose,
            left_arm_pose,
            right_arm_pose,
            left_leg_pose,
            right_leg_pose,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.small {
            bitfield &= 0x1;
        }
        if self.show_arms {
            bitfield &= 0x4;
        }
        if self.no_base_plate {
            bitfield &= 0x8;
        }
        if self.marker {
            bitfield &= 0x10;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Rotations(self.head_pose.clone()));
        metadata.push(EntityDataValue::Rotations(self.body_pose.clone()));
        metadata.push(EntityDataValue::Rotations(self.left_arm_pose.clone()));
        metadata.push(EntityDataValue::Rotations(self.right_arm_pose.clone()));
        metadata.push(EntityDataValue::Rotations(self.left_leg_pose.clone()));
        metadata.push(EntityDataValue::Rotations(self.right_leg_pose.clone()));
        metadata
    }
}
pub struct Arrow {
    pub abstract_entity: AbstractEntity,
    pub crit_arrow: bool,
    pub shot_from_crossbow: bool,
    pub no_physics: bool,
    pub pierce_level: u8,
    pub effect_color: i32,
}

impl Arrow {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let crit_arrow = bitfield & 0x1 != 1;
        let shot_from_crossbow = bitfield & 0x4 != 1;
        let no_physics = bitfield & 0x2 != 1;
        let pierce_level = metadata.pop_front()?.as_byte()?.clone();
        let effect_color = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            crit_arrow,
            shot_from_crossbow,
            no_physics,
            pierce_level,
            effect_color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.crit_arrow {
            bitfield &= 0x1;
        }
        if self.shot_from_crossbow {
            bitfield &= 0x4;
        }
        if self.no_physics {
            bitfield &= 0x2;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Byte(self.pierce_level.clone()));
        metadata.push(EntityDataValue::Int(self.effect_color.clone()));
        metadata
    }
}
pub struct Axolotl {
    pub abstract_animal: AbstractAnimal,
    pub variant: i32,
    pub playing_dead: bool,
    pub from_bucket: bool,
}

impl Axolotl {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let variant = metadata.pop_front()?.as_int()?.clone();
        let playing_dead = metadata.pop_front()?.as_boolean()?.clone();
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            variant,
            playing_dead,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.variant.clone()));
        metadata.push(EntityDataValue::Boolean(self.playing_dead.clone()));
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}
pub struct Bat {
    pub abstract_insentient: AbstractInsentient,
    pub resting: bool,
}

impl Bat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let resting = bitfield & 0x1 != 1;
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
            resting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.resting {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Bee {
    pub abstract_animal: AbstractAnimal,
    pub has_nectar: bool,
    pub has_stung: bool,
    pub rolling: bool,
    pub remaining_anger_time: i32,
}

impl Bee {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let has_nectar = bitfield & 0x8 != 1;
        let has_stung = bitfield & 0x4 != 1;
        let rolling = bitfield & 0x2 != 1;
        let remaining_anger_time = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            has_nectar,
            has_stung,
            rolling,
            remaining_anger_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.has_nectar {
            bitfield &= 0x8;
        }
        if self.has_stung {
            bitfield &= 0x4;
        }
        if self.rolling {
            bitfield &= 0x2;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Int(self.remaining_anger_time.clone()));
        metadata
    }
}
pub struct Blaze {
    pub abstract_monster: AbstractMonster,
    pub charged: bool,
}

impl Blaze {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let charged = bitfield & 0x1 != 1;
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            charged,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.charged {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Boat {
    pub abstract_entity: AbstractEntity,
    pub hurt: i32,
    pub hurtdir: i32,
    pub damage: f32,
    pub kind: i32,
    pub paddle_left: bool,
    pub paddle_right: bool,
    pub bubble_time: i32,
}

impl Boat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let hurt = metadata.pop_front()?.as_int()?.clone();
        let hurtdir = metadata.pop_front()?.as_int()?.clone();
        let damage = metadata.pop_front()?.as_float()?.clone();
        let kind = metadata.pop_front()?.as_int()?.clone();
        let paddle_left = metadata.pop_front()?.as_boolean()?.clone();
        let paddle_right = metadata.pop_front()?.as_boolean()?.clone();
        let bubble_time = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            hurt,
            hurtdir,
            damage,
            kind,
            paddle_left,
            paddle_right,
            bubble_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.hurt.clone()));
        metadata.push(EntityDataValue::Int(self.hurtdir.clone()));
        metadata.push(EntityDataValue::Float(self.damage.clone()));
        metadata.push(EntityDataValue::Int(self.kind.clone()));
        metadata.push(EntityDataValue::Boolean(self.paddle_left.clone()));
        metadata.push(EntityDataValue::Boolean(self.paddle_right.clone()));
        metadata.push(EntityDataValue::Int(self.bubble_time.clone()));
        metadata
    }
}
pub struct Cat {
    pub abstract_tameable: AbstractTameable,
    pub variant: azalea_registry::CatVariant,
    pub is_lying: bool,
    pub relax_state_one: bool,
    pub collar_color: i32,
}

impl Cat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let variant = metadata.pop_front()?.as_cat_variant()?.clone();
        let is_lying = metadata.pop_front()?.as_boolean()?.clone();
        let relax_state_one = metadata.pop_front()?.as_boolean()?.clone();
        let collar_color = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_tameable: AbstractTameable::read(metadata)?,
            variant,
            is_lying,
            relax_state_one,
            collar_color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::CatVariant(self.variant.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_lying.clone()));
        metadata.push(EntityDataValue::Boolean(self.relax_state_one.clone()));
        metadata.push(EntityDataValue::Int(self.collar_color.clone()));
        metadata
    }
}
pub struct CaveSpider {
    pub spider: Spider,
}

impl CaveSpider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            spider: Spider::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct ChestBoat {
    pub boat: Boat,
}

impl ChestBoat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            boat: Boat::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct ChestMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl ChestMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Chicken {
    pub abstract_animal: AbstractAnimal,
}

impl Chicken {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Cod {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Cod {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}
pub struct CommandBlockMinecart {
    pub abstract_minecart: AbstractMinecart,
    pub command_name: String,
    pub last_output: Component,
}

impl CommandBlockMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let command_name = metadata.pop_front()?.as_string()?.clone();
        let last_output = metadata.pop_front()?.as_component()?.clone();
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
            command_name,
            last_output,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::String(self.command_name.clone()));
        metadata.push(EntityDataValue::Component(self.last_output.clone()));
        metadata
    }
}
pub struct Cow {
    pub abstract_animal: AbstractAnimal,
}

impl Cow {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Creeper {
    pub abstract_monster: AbstractMonster,
    pub swell_dir: i32,
    pub is_powered: bool,
    pub is_ignited: bool,
}

impl Creeper {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let swell_dir = metadata.pop_front()?.as_int()?.clone();
        let is_powered = metadata.pop_front()?.as_boolean()?.clone();
        let is_ignited = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            swell_dir,
            is_powered,
            is_ignited,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.swell_dir.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_powered.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_ignited.clone()));
        metadata
    }
}
pub struct Dolphin {
    pub abstract_creature: AbstractCreature,
    pub treasure_pos: BlockPos,
    pub got_fish: bool,
    pub moistness_level: i32,
}

impl Dolphin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let treasure_pos = metadata.pop_front()?.as_block_pos()?.clone();
        let got_fish = metadata.pop_front()?.as_boolean()?.clone();
        let moistness_level = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            treasure_pos,
            got_fish,
            moistness_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::BlockPos(self.treasure_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.got_fish.clone()));
        metadata.push(EntityDataValue::Int(self.moistness_level.clone()));
        metadata
    }
}
pub struct Donkey {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
    pub chest: bool,
}

impl Donkey {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        let chest = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
            chest,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata.push(EntityDataValue::Boolean(self.chest.clone()));
        metadata
    }
}
pub struct DragonFireball {
    pub abstract_entity: AbstractEntity,
}

impl DragonFireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Drowned {
    pub zombie: Zombie,
}

impl Drowned {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            zombie: Zombie::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Egg {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Egg {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct ElderGuardian {
    pub guardian: Guardian,
}

impl ElderGuardian {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            guardian: Guardian::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct EndCrystal {
    pub abstract_entity: AbstractEntity,
    pub beam_target: Option<BlockPos>,
    pub show_bottom: bool,
}

impl EndCrystal {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let beam_target = metadata.pop_front()?.as_optional_block_pos()?.clone();
        let show_bottom = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            beam_target,
            show_bottom,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::OptionalBlockPos(self.beam_target.clone()));
        metadata.push(EntityDataValue::Boolean(self.show_bottom.clone()));
        metadata
    }
}
pub struct EnderDragon {
    pub abstract_insentient: AbstractInsentient,
    pub phase: i32,
}

impl EnderDragon {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let phase = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
            phase,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.phase.clone()));
        metadata
    }
}
pub struct EnderPearl {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl EnderPearl {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct Enderman {
    pub abstract_monster: AbstractMonster,
    pub carry_state: Option<i32>,
    pub creepy: bool,
    pub stared_at: bool,
}

impl Enderman {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let carry_state = metadata.pop_front()?.as_optional_block_state()?.clone();
        let creepy = metadata.pop_front()?.as_boolean()?.clone();
        let stared_at = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            carry_state,
            creepy,
            stared_at,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::OptionalBlockState(
            self.carry_state.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.creepy.clone()));
        metadata.push(EntityDataValue::Boolean(self.stared_at.clone()));
        metadata
    }
}
pub struct Endermite {
    pub abstract_monster: AbstractMonster,
}

impl Endermite {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Evoker {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub spell_casting: u8,
}

impl Evoker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        let spell_casting = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
            spell_casting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Byte(self.spell_casting.clone()));
        metadata
    }
}
pub struct EvokerFangs {
    pub abstract_entity: AbstractEntity,
}

impl EvokerFangs {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct ExperienceBottle {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl ExperienceBottle {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct ExperienceOrb {
    pub abstract_entity: AbstractEntity,
}

impl ExperienceOrb {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct EyeOfEnder {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl EyeOfEnder {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct FallingBlock {
    pub abstract_entity: AbstractEntity,
    pub start_pos: BlockPos,
}

impl FallingBlock {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let start_pos = metadata.pop_front()?.as_block_pos()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            start_pos,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::BlockPos(self.start_pos.clone()));
        metadata
    }
}
pub struct Fireball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Fireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct FireworkRocket {
    pub abstract_entity: AbstractEntity,
    pub fireworks_item: Slot,
    pub attached_to_target: Option<u32>,
    pub shot_at_angle: bool,
}

impl FireworkRocket {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let fireworks_item = metadata.pop_front()?.as_item_stack()?.clone();
        let attached_to_target = metadata.pop_front()?.as_optional_unsigned_int()?.clone();
        let shot_at_angle = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            fireworks_item,
            attached_to_target,
            shot_at_angle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.fireworks_item.clone()));
        metadata.push(EntityDataValue::OptionalUnsignedInt(
            self.attached_to_target.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.shot_at_angle.clone()));
        metadata
    }
}
pub struct FishingBobber {
    pub abstract_entity: AbstractEntity,
    pub hooked_entity: i32,
    pub biting: bool,
}

impl FishingBobber {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let hooked_entity = metadata.pop_front()?.as_int()?.clone();
        let biting = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            hooked_entity,
            biting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.hooked_entity.clone()));
        metadata.push(EntityDataValue::Boolean(self.biting.clone()));
        metadata
    }
}
pub struct Fox {
    pub abstract_animal: AbstractAnimal,
    pub kind: i32,
    pub sitting: bool,
    pub faceplanted: bool,
    pub sleeping: bool,
    pub pouncing: bool,
    pub crouching: bool,
    pub interested: bool,
    pub trusted_id_0: Option<Uuid>,
    pub trusted_id_1: Option<Uuid>,
}

impl Fox {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let kind = metadata.pop_front()?.as_int()?.clone();
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let sitting = bitfield & 0x1 != 1;
        let faceplanted = bitfield & 0x40 != 1;
        let sleeping = bitfield & 0x20 != 1;
        let pouncing = bitfield & 0x10 != 1;
        let crouching = bitfield & 0x4 != 1;
        let interested = bitfield & 0x8 != 1;
        let trusted_id_0 = metadata.pop_front()?.as_optional_uuid()?.clone();
        let trusted_id_1 = metadata.pop_front()?.as_optional_uuid()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            kind,
            sitting,
            faceplanted,
            sleeping,
            pouncing,
            crouching,
            interested,
            trusted_id_0,
            trusted_id_1,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.kind.clone()));
        let mut bitfield = 0u8;
        if self.sitting {
            bitfield &= 0x1;
        }
        if self.faceplanted {
            bitfield &= 0x40;
        }
        if self.sleeping {
            bitfield &= 0x20;
        }
        if self.pouncing {
            bitfield &= 0x10;
        }
        if self.crouching {
            bitfield &= 0x4;
        }
        if self.interested {
            bitfield &= 0x8;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.trusted_id_0.clone()));
        metadata.push(EntityDataValue::OptionalUuid(self.trusted_id_1.clone()));
        metadata
    }
}
pub struct Frog {
    pub abstract_animal: AbstractAnimal,
    pub variant: azalea_registry::FrogVariant,
    pub tongue_target: Option<u32>,
}

impl Frog {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let variant = metadata.pop_front()?.as_frog_variant()?.clone();
        let tongue_target = metadata.pop_front()?.as_optional_unsigned_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            variant,
            tongue_target,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::FrogVariant(self.variant.clone()));
        metadata.push(EntityDataValue::OptionalUnsignedInt(
            self.tongue_target.clone(),
        ));
        metadata
    }
}
pub struct FurnaceMinecart {
    pub abstract_minecart: AbstractMinecart,
    pub fuel: bool,
}

impl FurnaceMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let fuel = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
            fuel,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.fuel.clone()));
        metadata
    }
}
pub struct Ghast {
    pub abstract_insentient: AbstractInsentient,
    pub is_charging: bool,
}

impl Ghast {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_charging = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
            is_charging,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_charging.clone()));
        metadata
    }
}
pub struct Giant {
    pub abstract_monster: AbstractMonster,
}

impl Giant {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct GlowItemFrame {
    pub item_frame: ItemFrame,
}

impl GlowItemFrame {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            item_frame: ItemFrame::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct GlowSquid {
    pub squid: Squid,
    pub dark_ticks_remaining: i32,
}

impl GlowSquid {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let dark_ticks_remaining = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            squid: Squid::read(metadata)?,
            dark_ticks_remaining,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.dark_ticks_remaining.clone()));
        metadata
    }
}
pub struct Goat {
    pub abstract_animal: AbstractAnimal,
    pub is_screaming_goat: bool,
    pub has_left_horn: bool,
    pub has_right_horn: bool,
}

impl Goat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_screaming_goat = metadata.pop_front()?.as_boolean()?.clone();
        let has_left_horn = metadata.pop_front()?.as_boolean()?.clone();
        let has_right_horn = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            is_screaming_goat,
            has_left_horn,
            has_right_horn,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_screaming_goat.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_left_horn.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_right_horn.clone()));
        metadata
    }
}
pub struct Guardian {
    pub abstract_monster: AbstractMonster,
    pub moving: bool,
    pub attack_target: i32,
}

impl Guardian {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let moving = metadata.pop_front()?.as_boolean()?.clone();
        let attack_target = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            moving,
            attack_target,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.moving.clone()));
        metadata.push(EntityDataValue::Int(self.attack_target.clone()));
        metadata
    }
}
pub struct Hoglin {
    pub abstract_animal: AbstractAnimal,
    pub immune_to_zombification: bool,
}

impl Hoglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let immune_to_zombification = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            immune_to_zombification,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata
    }
}
pub struct HopperMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl HopperMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Horse {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
    pub type_variant: i32,
}

impl Horse {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        let type_variant = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
            type_variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata.push(EntityDataValue::Int(self.type_variant.clone()));
        metadata
    }
}
pub struct Husk {
    pub zombie: Zombie,
}

impl Husk {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            zombie: Zombie::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Illusioner {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub spell_casting: u8,
}

impl Illusioner {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        let spell_casting = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
            spell_casting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Byte(self.spell_casting.clone()));
        metadata
    }
}
pub struct IronGolem {
    pub abstract_creature: AbstractCreature,
    pub player_created: bool,
}

impl IronGolem {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let player_created = bitfield & 0x1 != 1;
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            player_created,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.player_created {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Item {
    pub abstract_entity: AbstractEntity,
    pub item: Slot,
}

impl Item {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item.clone()));
        metadata
    }
}
pub struct ItemFrame {
    pub abstract_entity: AbstractEntity,
    pub item: Slot,
    pub rotation: i32,
}

impl ItemFrame {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item = metadata.pop_front()?.as_item_stack()?.clone();
        let rotation = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item,
            rotation,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item.clone()));
        metadata.push(EntityDataValue::Int(self.rotation.clone()));
        metadata
    }
}
pub struct LeashKnot {
    pub abstract_entity: AbstractEntity,
}

impl LeashKnot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct LightningBolt {
    pub abstract_entity: AbstractEntity,
}

impl LightningBolt {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Llama {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
    pub chest: bool,
    pub strength: i32,
    pub swag: i32,
    pub variant: i32,
}

impl Llama {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        let chest = metadata.pop_front()?.as_boolean()?.clone();
        let strength = metadata.pop_front()?.as_int()?.clone();
        let swag = metadata.pop_front()?.as_int()?.clone();
        let variant = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
            chest,
            strength,
            swag,
            variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata.push(EntityDataValue::Boolean(self.chest.clone()));
        metadata.push(EntityDataValue::Int(self.strength.clone()));
        metadata.push(EntityDataValue::Int(self.swag.clone()));
        metadata.push(EntityDataValue::Int(self.variant.clone()));
        metadata
    }
}
pub struct LlamaSpit {
    pub abstract_entity: AbstractEntity,
}

impl LlamaSpit {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct MagmaCube {
    pub slime: Slime,
}

impl MagmaCube {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            slime: Slime::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Marker {
    pub abstract_entity: AbstractEntity,
}

impl Marker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Minecart {
    pub abstract_minecart: AbstractMinecart,
}

impl Minecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Mooshroom {
    pub cow: Cow,
    pub kind: String,
}

impl Mooshroom {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let kind = metadata.pop_front()?.as_string()?.clone();
        Some(Self {
            cow: Cow::read(metadata)?,
            kind,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::String(self.kind.clone()));
        metadata
    }
}
pub struct Mule {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
    pub chest: bool,
}

impl Mule {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        let chest = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
            chest,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata.push(EntityDataValue::Boolean(self.chest.clone()));
        metadata
    }
}
pub struct Ocelot {
    pub abstract_animal: AbstractAnimal,
    pub trusting: bool,
}

impl Ocelot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let trusting = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            trusting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.trusting.clone()));
        metadata
    }
}
pub struct Painting {
    pub abstract_entity: AbstractEntity,
    pub painting_variant: azalea_registry::PaintingVariant,
}

impl Painting {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let painting_variant = metadata.pop_front()?.as_painting_variant()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            painting_variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::PaintingVariant(
            self.painting_variant.clone(),
        ));
        metadata
    }
}
pub struct Panda {
    pub abstract_animal: AbstractAnimal,
    pub unhappy_counter: i32,
    pub sneeze_counter: i32,
    pub eat_counter: i32,
    pub sneezing: bool,
    pub sitting: bool,
    pub on_back: bool,
    pub rolling: bool,
    pub hidden_gene: u8,
    pub flags: u8,
}

impl Panda {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let unhappy_counter = metadata.pop_front()?.as_int()?.clone();
        let sneeze_counter = metadata.pop_front()?.as_int()?.clone();
        let eat_counter = metadata.pop_front()?.as_int()?.clone();
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let sneezing = bitfield & 0x2 != 1;
        let sitting = bitfield & 0x8 != 1;
        let on_back = bitfield & 0x10 != 1;
        let rolling = bitfield & 0x4 != 1;
        let hidden_gene = metadata.pop_front()?.as_byte()?.clone();
        let flags = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            unhappy_counter,
            sneeze_counter,
            eat_counter,
            sneezing,
            sitting,
            on_back,
            rolling,
            hidden_gene,
            flags,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.unhappy_counter.clone()));
        metadata.push(EntityDataValue::Int(self.sneeze_counter.clone()));
        metadata.push(EntityDataValue::Int(self.eat_counter.clone()));
        let mut bitfield = 0u8;
        if self.sneezing {
            bitfield &= 0x2;
        }
        if self.sitting {
            bitfield &= 0x8;
        }
        if self.on_back {
            bitfield &= 0x10;
        }
        if self.rolling {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Byte(self.hidden_gene.clone()));
        metadata.push(EntityDataValue::Byte(self.flags.clone()));
        metadata
    }
}
pub struct Parrot {
    pub abstract_tameable: AbstractTameable,
    pub variant: i32,
}

impl Parrot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let variant = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_tameable: AbstractTameable::read(metadata)?,
            variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.variant.clone()));
        metadata
    }
}
pub struct Phantom {
    pub abstract_insentient: AbstractInsentient,
    pub size: i32,
}

impl Phantom {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let size = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
            size,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.size.clone()));
        metadata
    }
}
pub struct Pig {
    pub abstract_animal: AbstractAnimal,
    pub saddle: bool,
    pub boost_time: i32,
}

impl Pig {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let saddle = metadata.pop_front()?.as_boolean()?.clone();
        let boost_time = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            saddle,
            boost_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.saddle.clone()));
        metadata.push(EntityDataValue::Int(self.boost_time.clone()));
        metadata
    }
}
pub struct Piglin {
    pub abstract_monster: AbstractMonster,
    pub immune_to_zombification: bool,
    pub baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

impl Piglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let immune_to_zombification = metadata.pop_front()?.as_boolean()?.clone();
        let baby = metadata.pop_front()?.as_boolean()?.clone();
        let is_charging_crossbow = metadata.pop_front()?.as_boolean()?.clone();
        let is_dancing = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            immune_to_zombification,
            baby,
            is_charging_crossbow,
            is_dancing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_charging_crossbow.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_dancing.clone()));
        metadata
    }
}
pub struct PiglinBrute {
    pub abstract_monster: AbstractMonster,
    pub immune_to_zombification: bool,
}

impl PiglinBrute {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let immune_to_zombification = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            immune_to_zombification,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata
    }
}
pub struct Pillager {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub is_charging_crossbow: bool,
}

impl Pillager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        let is_charging_crossbow = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
            is_charging_crossbow,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_charging_crossbow.clone()));
        metadata
    }
}
pub struct Player {
    pub abstract_living: AbstractLiving,
    pub player_absorption: f32,
    pub score: i32,
    pub player_mode_customisation: u8,
    pub player_main_hand: u8,
    pub shoulder_left: azalea_nbt::Tag,
    pub shoulder_right: azalea_nbt::Tag,
}

impl Player {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let player_absorption = metadata.pop_front()?.as_float()?.clone();
        let score = metadata.pop_front()?.as_int()?.clone();
        let player_mode_customisation = metadata.pop_front()?.as_byte()?.clone();
        let player_main_hand = metadata.pop_front()?.as_byte()?.clone();
        let shoulder_left = metadata.pop_front()?.as_compound_tag()?.clone();
        let shoulder_right = metadata.pop_front()?.as_compound_tag()?.clone();
        Some(Self {
            abstract_living: AbstractLiving::read(metadata)?,
            player_absorption,
            score,
            player_mode_customisation,
            player_main_hand,
            shoulder_left,
            shoulder_right,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Float(self.player_absorption.clone()));
        metadata.push(EntityDataValue::Int(self.score.clone()));
        metadata.push(EntityDataValue::Byte(
            self.player_mode_customisation.clone(),
        ));
        metadata.push(EntityDataValue::Byte(self.player_main_hand.clone()));
        metadata.push(EntityDataValue::CompoundTag(self.shoulder_left.clone()));
        metadata.push(EntityDataValue::CompoundTag(self.shoulder_right.clone()));
        metadata
    }
}
pub struct PolarBear {
    pub abstract_animal: AbstractAnimal,
    pub standing: bool,
}

impl PolarBear {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let standing = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            standing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.standing.clone()));
        metadata
    }
}
pub struct Potion {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Potion {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct Pufferfish {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
    pub puff_state: i32,
}

impl Pufferfish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        let puff_state = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            from_bucket,
            puff_state,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata.push(EntityDataValue::Int(self.puff_state.clone()));
        metadata
    }
}
pub struct Rabbit {
    pub abstract_animal: AbstractAnimal,
    pub kind: i32,
}

impl Rabbit {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let kind = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            kind,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.kind.clone()));
        metadata
    }
}
pub struct Ravager {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
}

impl Ravager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata
    }
}
pub struct Salmon {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Salmon {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}
pub struct Sheep {
    pub abstract_animal: AbstractAnimal,
    pub sheared: bool,
}

impl Sheep {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let sheared = bitfield & 0x10 != 1;
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            sheared,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.sheared {
            bitfield &= 0x10;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Shulker {
    pub abstract_creature: AbstractCreature,
    pub attach_face: Direction,
    pub peek: u8,
    pub color: u8,
}

impl Shulker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let attach_face = metadata.pop_front()?.as_direction()?.clone();
        let peek = metadata.pop_front()?.as_byte()?.clone();
        let color = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            attach_face,
            peek,
            color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Direction(self.attach_face.clone()));
        metadata.push(EntityDataValue::Byte(self.peek.clone()));
        metadata.push(EntityDataValue::Byte(self.color.clone()));
        metadata
    }
}
pub struct ShulkerBullet {
    pub abstract_entity: AbstractEntity,
}

impl ShulkerBullet {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Silverfish {
    pub abstract_monster: AbstractMonster,
}

impl Silverfish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Skeleton {
    pub abstract_monster: AbstractMonster,
    pub stray_conversion: bool,
}

impl Skeleton {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let stray_conversion = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            stray_conversion,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.stray_conversion.clone()));
        metadata
    }
}
pub struct SkeletonHorse {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
}

impl SkeletonHorse {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata
    }
}
pub struct Slime {
    pub abstract_insentient: AbstractInsentient,
    pub size: i32,
}

impl Slime {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let size = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
            size,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.size.clone()));
        metadata
    }
}
pub struct SmallFireball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl SmallFireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct SnowGolem {
    pub abstract_creature: AbstractCreature,
    pub has_pumpkin: bool,
}

impl SnowGolem {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let has_pumpkin = bitfield & 0x10 != 1;
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            has_pumpkin,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.has_pumpkin {
            bitfield &= 0x10;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Snowball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Snowball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_stack = metadata.pop_front()?.as_item_stack()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}
pub struct SpawnerMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl SpawnerMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct SpectralArrow {
    pub abstract_entity: AbstractEntity,
    pub crit_arrow: bool,
    pub shot_from_crossbow: bool,
    pub no_physics: bool,
    pub pierce_level: u8,
}

impl SpectralArrow {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let crit_arrow = bitfield & 0x1 != 1;
        let shot_from_crossbow = bitfield & 0x4 != 1;
        let no_physics = bitfield & 0x2 != 1;
        let pierce_level = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            crit_arrow,
            shot_from_crossbow,
            no_physics,
            pierce_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.crit_arrow {
            bitfield &= 0x1;
        }
        if self.shot_from_crossbow {
            bitfield &= 0x4;
        }
        if self.no_physics {
            bitfield &= 0x2;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Byte(self.pierce_level.clone()));
        metadata
    }
}
pub struct Spider {
    pub abstract_monster: AbstractMonster,
    pub climbing: bool,
}

impl Spider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let climbing = bitfield & 0x1 != 1;
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            climbing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.climbing {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct Squid {
    pub abstract_creature: AbstractCreature,
}

impl Squid {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Stray {
    pub abstract_monster: AbstractMonster,
}

impl Stray {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Strider {
    pub abstract_animal: AbstractAnimal,
    pub boost_time: i32,
    pub suffocating: bool,
    pub saddle: bool,
}

impl Strider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let boost_time = metadata.pop_front()?.as_int()?.clone();
        let suffocating = metadata.pop_front()?.as_boolean()?.clone();
        let saddle = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            boost_time,
            suffocating,
            saddle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.boost_time.clone()));
        metadata.push(EntityDataValue::Boolean(self.suffocating.clone()));
        metadata.push(EntityDataValue::Boolean(self.saddle.clone()));
        metadata
    }
}
pub struct Tadpole {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Tadpole {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}
pub struct Tnt {
    pub abstract_entity: AbstractEntity,
    pub fuse: i32,
}

impl Tnt {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let fuse = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            fuse,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.fuse.clone()));
        metadata
    }
}
pub struct TntMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl TntMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_minecart: AbstractMinecart::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct TraderLlama {
    pub llama: Llama,
}

impl TraderLlama {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            llama: Llama::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct Trident {
    pub abstract_entity: AbstractEntity,
    pub crit_arrow: bool,
    pub shot_from_crossbow: bool,
    pub no_physics: bool,
    pub pierce_level: u8,
    pub loyalty: u8,
    pub foil: bool,
}

impl Trident {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let crit_arrow = bitfield & 0x1 != 1;
        let shot_from_crossbow = bitfield & 0x4 != 1;
        let no_physics = bitfield & 0x2 != 1;
        let pierce_level = metadata.pop_front()?.as_byte()?.clone();
        let loyalty = metadata.pop_front()?.as_byte()?.clone();
        let foil = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            crit_arrow,
            shot_from_crossbow,
            no_physics,
            pierce_level,
            loyalty,
            foil,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.crit_arrow {
            bitfield &= 0x1;
        }
        if self.shot_from_crossbow {
            bitfield &= 0x4;
        }
        if self.no_physics {
            bitfield &= 0x2;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Byte(self.pierce_level.clone()));
        metadata.push(EntityDataValue::Byte(self.loyalty.clone()));
        metadata.push(EntityDataValue::Boolean(self.foil.clone()));
        metadata
    }
}
pub struct TropicalFish {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
    pub type_variant: i32,
}

impl TropicalFish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let from_bucket = metadata.pop_front()?.as_boolean()?.clone();
        let type_variant = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            from_bucket,
            type_variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata.push(EntityDataValue::Int(self.type_variant.clone()));
        metadata
    }
}
pub struct Turtle {
    pub abstract_animal: AbstractAnimal,
    pub home_pos: BlockPos,
    pub has_egg: bool,
    pub laying_egg: bool,
    pub travel_pos: BlockPos,
    pub going_home: bool,
    pub travelling: bool,
}

impl Turtle {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let home_pos = metadata.pop_front()?.as_block_pos()?.clone();
        let has_egg = metadata.pop_front()?.as_boolean()?.clone();
        let laying_egg = metadata.pop_front()?.as_boolean()?.clone();
        let travel_pos = metadata.pop_front()?.as_block_pos()?.clone();
        let going_home = metadata.pop_front()?.as_boolean()?.clone();
        let travelling = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            home_pos,
            has_egg,
            laying_egg,
            travel_pos,
            going_home,
            travelling,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::BlockPos(self.home_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_egg.clone()));
        metadata.push(EntityDataValue::Boolean(self.laying_egg.clone()));
        metadata.push(EntityDataValue::BlockPos(self.travel_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.going_home.clone()));
        metadata.push(EntityDataValue::Boolean(self.travelling.clone()));
        metadata
    }
}
pub struct Vex {
    pub abstract_monster: AbstractMonster,
    pub flags: u8,
}

impl Vex {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let flags = metadata.pop_front()?.as_byte()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            flags,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Byte(self.flags.clone()));
        metadata
    }
}
pub struct Villager {
    pub abstract_ageable: AbstractAgeable,
    pub unhappy_counter: i32,
    pub villager_data: VillagerData,
}

impl Villager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let unhappy_counter = metadata.pop_front()?.as_int()?.clone();
        let villager_data = metadata.pop_front()?.as_villager_data()?.clone();
        Some(Self {
            abstract_ageable: AbstractAgeable::read(metadata)?,
            unhappy_counter,
            villager_data,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.unhappy_counter.clone()));
        metadata.push(EntityDataValue::VillagerData(self.villager_data.clone()));
        metadata
    }
}
pub struct Vindicator {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
}

impl Vindicator {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata
    }
}
pub struct WanderingTrader {
    pub abstract_ageable: AbstractAgeable,
    pub unhappy_counter: i32,
}

impl WanderingTrader {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let unhappy_counter = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_ageable: AbstractAgeable::read(metadata)?,
            unhappy_counter,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.unhappy_counter.clone()));
        metadata
    }
}
pub struct Warden {
    pub abstract_monster: AbstractMonster,
    pub client_anger_level: i32,
}

impl Warden {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let client_anger_level = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            client_anger_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.client_anger_level.clone()));
        metadata
    }
}
pub struct Witch {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub using_item: bool,
}

impl Witch {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let is_celebrating = metadata.pop_front()?.as_boolean()?.clone();
        let using_item = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            is_celebrating,
            using_item,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Boolean(self.using_item.clone()));
        metadata
    }
}
pub struct Wither {
    pub abstract_monster: AbstractMonster,
    pub target_a: i32,
    pub target_b: i32,
    pub target_c: i32,
    pub inv: i32,
}

impl Wither {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let target_a = metadata.pop_front()?.as_int()?.clone();
        let target_b = metadata.pop_front()?.as_int()?.clone();
        let target_c = metadata.pop_front()?.as_int()?.clone();
        let inv = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            target_a,
            target_b,
            target_c,
            inv,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.target_a.clone()));
        metadata.push(EntityDataValue::Int(self.target_b.clone()));
        metadata.push(EntityDataValue::Int(self.target_c.clone()));
        metadata.push(EntityDataValue::Int(self.inv.clone()));
        metadata
    }
}
pub struct WitherSkeleton {
    pub abstract_monster: AbstractMonster,
}

impl WitherSkeleton {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct WitherSkull {
    pub abstract_entity: AbstractEntity,
    pub dangerous: bool,
}

impl WitherSkull {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let dangerous = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            dangerous,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.dangerous.clone()));
        metadata
    }
}
pub struct Wolf {
    pub abstract_tameable: AbstractTameable,
    pub interested: bool,
    pub collar_color: i32,
    pub remaining_anger_time: i32,
}

impl Wolf {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let interested = metadata.pop_front()?.as_boolean()?.clone();
        let collar_color = metadata.pop_front()?.as_int()?.clone();
        let remaining_anger_time = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            abstract_tameable: AbstractTameable::read(metadata)?,
            interested,
            collar_color,
            remaining_anger_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.interested.clone()));
        metadata.push(EntityDataValue::Int(self.collar_color.clone()));
        metadata.push(EntityDataValue::Int(self.remaining_anger_time.clone()));
        metadata
    }
}
pub struct Zoglin {
    pub abstract_monster: AbstractMonster,
    pub baby: bool,
}

impl Zoglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let baby = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            baby,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata
    }
}
pub struct Zombie {
    pub abstract_monster: AbstractMonster,
    pub baby: bool,
    pub special_type: i32,
    pub drowned_conversion: bool,
}

impl Zombie {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let baby = metadata.pop_front()?.as_boolean()?.clone();
        let special_type = metadata.pop_front()?.as_int()?.clone();
        let drowned_conversion = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_monster: AbstractMonster::read(metadata)?,
            baby,
            special_type,
            drowned_conversion,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata.push(EntityDataValue::Int(self.special_type.clone()));
        metadata.push(EntityDataValue::Boolean(self.drowned_conversion.clone()));
        metadata
    }
}
pub struct ZombieHorse {
    pub abstract_animal: AbstractAnimal,
    pub tamed: bool,
    pub eating: bool,
    pub standing: bool,
    pub bred: bool,
    pub saddled: bool,
    pub owner_uuid: Option<Uuid>,
}

impl ZombieHorse {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tamed = bitfield & 0x2 != 1;
        let eating = bitfield & 0x10 != 1;
        let standing = bitfield & 0x20 != 1;
        let bred = bitfield & 0x8 != 1;
        let saddled = bitfield & 0x4 != 1;
        let owner_uuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tamed,
            eating,
            standing,
            bred,
            saddled,
            owner_uuid,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tamed {
            bitfield &= 0x2;
        }
        if self.eating {
            bitfield &= 0x10;
        }
        if self.standing {
            bitfield &= 0x20;
        }
        if self.bred {
            bitfield &= 0x8;
        }
        if self.saddled {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owner_uuid.clone()));
        metadata
    }
}
pub struct ZombieVillager {
    pub zombie: Zombie,
    pub converting: bool,
    pub villager_data: VillagerData,
}

impl ZombieVillager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let converting = metadata.pop_front()?.as_boolean()?.clone();
        let villager_data = metadata.pop_front()?.as_villager_data()?.clone();
        Some(Self {
            zombie: Zombie::read(metadata)?,
            converting,
            villager_data,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.converting.clone()));
        metadata.push(EntityDataValue::VillagerData(self.villager_data.clone()));
        metadata
    }
}
pub struct ZombifiedPiglin {
    pub zombie: Zombie,
}

impl ZombifiedPiglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            zombie: Zombie::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct AbstractAgeable {
    pub abstract_creature: AbstractCreature,
    pub baby: bool,
}

impl AbstractAgeable {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let baby = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
            baby,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata
    }
}
pub struct AbstractAnimal {
    pub abstract_ageable: AbstractAgeable,
}

impl AbstractAnimal {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_ageable: AbstractAgeable::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct AbstractCreature {
    pub abstract_insentient: AbstractInsentient,
}

impl AbstractCreature {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_insentient: AbstractInsentient::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct AbstractEntity {
    pub on_fire: bool,
    pub shift_key_down: bool,
    pub sprinting: bool,
    pub swimming: bool,
    pub currently_glowing: bool,
    pub invisible: bool,
    pub fall_flying: bool,
    pub air_supply: i32,
    pub custom_name: Option<Component>,
    pub custom_name_visible: bool,
    pub silent: bool,
    pub no_gravity: bool,
    pub pose: Pose,
    pub ticks_frozen: i32,
}

impl AbstractEntity {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let on_fire = bitfield & 0x1 != 1;
        let shift_key_down = bitfield & 0x2 != 1;
        let sprinting = bitfield & 0x8 != 1;
        let swimming = bitfield & 0x10 != 1;
        let currently_glowing = bitfield & 0x40 != 1;
        let invisible = bitfield & 0x20 != 1;
        let fall_flying = bitfield & 0x80 != 1;
        let air_supply = metadata.pop_front()?.as_int()?.clone();
        let custom_name = metadata.pop_front()?.as_optional_component()?.clone();
        let custom_name_visible = metadata.pop_front()?.as_boolean()?.clone();
        let silent = metadata.pop_front()?.as_boolean()?.clone();
        let no_gravity = metadata.pop_front()?.as_boolean()?.clone();
        let pose = metadata.pop_front()?.as_pose()?.clone();
        let ticks_frozen = metadata.pop_front()?.as_int()?.clone();
        Some(Self {
            on_fire,
            shift_key_down,
            sprinting,
            swimming,
            currently_glowing,
            invisible,
            fall_flying,
            air_supply,
            custom_name,
            custom_name_visible,
            silent,
            no_gravity,
            pose,
            ticks_frozen,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.on_fire {
            bitfield &= 0x1;
        }
        if self.shift_key_down {
            bitfield &= 0x2;
        }
        if self.sprinting {
            bitfield &= 0x8;
        }
        if self.swimming {
            bitfield &= 0x10;
        }
        if self.currently_glowing {
            bitfield &= 0x40;
        }
        if self.invisible {
            bitfield &= 0x20;
        }
        if self.fall_flying {
            bitfield &= 0x80;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Int(self.air_supply.clone()));
        metadata.push(EntityDataValue::OptionalComponent(self.custom_name.clone()));
        metadata.push(EntityDataValue::Boolean(self.custom_name_visible.clone()));
        metadata.push(EntityDataValue::Boolean(self.silent.clone()));
        metadata.push(EntityDataValue::Boolean(self.no_gravity.clone()));
        metadata.push(EntityDataValue::Pose(self.pose.clone()));
        metadata.push(EntityDataValue::Int(self.ticks_frozen.clone()));
        metadata
    }
}
pub struct AbstractInsentient {
    pub abstract_living: AbstractLiving,
    pub no_ai: bool,
    pub left_handed: bool,
    pub aggressive: bool,
}

impl AbstractInsentient {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let no_ai = bitfield & 0x1 != 1;
        let left_handed = bitfield & 0x2 != 1;
        let aggressive = bitfield & 0x4 != 1;
        Some(Self {
            abstract_living: AbstractLiving::read(metadata)?,
            no_ai,
            left_handed,
            aggressive,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.no_ai {
            bitfield &= 0x1;
        }
        if self.left_handed {
            bitfield &= 0x2;
        }
        if self.aggressive {
            bitfield &= 0x4;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}
pub struct AbstractLiving {
    pub abstract_entity: AbstractEntity,
    pub auto_spin_attack: bool,
    pub using_item: bool,
    pub health: f32,
    pub effect_color: i32,
    pub effect_ambience: bool,
    pub arrow_count: i32,
    pub stinger_count: i32,
    pub sleeping_pos: Option<BlockPos>,
}

impl AbstractLiving {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let auto_spin_attack = bitfield & 0x4 != 1;
        let using_item = bitfield & 0x1 != 1;
        let health = metadata.pop_front()?.as_float()?.clone();
        let effect_color = metadata.pop_front()?.as_int()?.clone();
        let effect_ambience = metadata.pop_front()?.as_boolean()?.clone();
        let arrow_count = metadata.pop_front()?.as_int()?.clone();
        let stinger_count = metadata.pop_front()?.as_int()?.clone();
        let sleeping_pos = metadata.pop_front()?.as_optional_block_pos()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            auto_spin_attack,
            using_item,
            health,
            effect_color,
            effect_ambience,
            arrow_count,
            stinger_count,
            sleeping_pos,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.auto_spin_attack {
            bitfield &= 0x4;
        }
        if self.using_item {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::Float(self.health.clone()));
        metadata.push(EntityDataValue::Int(self.effect_color.clone()));
        metadata.push(EntityDataValue::Boolean(self.effect_ambience.clone()));
        metadata.push(EntityDataValue::Int(self.arrow_count.clone()));
        metadata.push(EntityDataValue::Int(self.stinger_count.clone()));
        metadata.push(EntityDataValue::OptionalBlockPos(self.sleeping_pos.clone()));
        metadata
    }
}
pub struct AbstractMinecart {
    pub abstract_entity: AbstractEntity,
    pub hurt: i32,
    pub hurtdir: i32,
    pub damage: f32,
    pub display_block: i32,
    pub display_offset: i32,
    pub custom_display: bool,
}

impl AbstractMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let hurt = metadata.pop_front()?.as_int()?.clone();
        let hurtdir = metadata.pop_front()?.as_int()?.clone();
        let damage = metadata.pop_front()?.as_float()?.clone();
        let display_block = metadata.pop_front()?.as_int()?.clone();
        let display_offset = metadata.pop_front()?.as_int()?.clone();
        let custom_display = metadata.pop_front()?.as_boolean()?.clone();
        Some(Self {
            abstract_entity: AbstractEntity::read(metadata)?,
            hurt,
            hurtdir,
            damage,
            display_block,
            display_offset,
            custom_display,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.push(EntityDataValue::Int(self.hurt.clone()));
        metadata.push(EntityDataValue::Int(self.hurtdir.clone()));
        metadata.push(EntityDataValue::Float(self.damage.clone()));
        metadata.push(EntityDataValue::Int(self.display_block.clone()));
        metadata.push(EntityDataValue::Int(self.display_offset.clone()));
        metadata.push(EntityDataValue::Boolean(self.custom_display.clone()));
        metadata
    }
}
pub struct AbstractMonster {
    pub abstract_creature: AbstractCreature,
}

impl AbstractMonster {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        Some(Self {
            abstract_creature: AbstractCreature::read(metadata)?,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata
    }
}
pub struct AbstractTameable {
    pub abstract_animal: AbstractAnimal,
    pub tame: bool,
    pub in_sitting_pose: bool,
    pub owneruuid: Option<Uuid>,
}

impl AbstractTameable {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let bitfield = *metadata.pop_front()?.as_byte()?;
        let tame = bitfield & 0x4 != 1;
        let in_sitting_pose = bitfield & 0x1 != 1;
        let owneruuid = metadata.pop_front()?.as_optional_uuid()?.clone();
        Some(Self {
            abstract_animal: AbstractAnimal::read(metadata)?,
            tame,
            in_sitting_pose,
            owneruuid,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        let mut bitfield = 0u8;
        if self.tame {
            bitfield &= 0x4;
        }
        if self.in_sitting_pose {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata.push(EntityDataValue::OptionalUuid(self.owneruuid.clone()));
        metadata
    }
}
