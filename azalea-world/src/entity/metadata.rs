// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

#![allow(clippy::clone_on_copy, clippy::derivable_impls)]
use super::{EntityDataValue, Pose, Rotations, VillagerData};
use azalea_block::BlockState;
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, Particle, Slot};
use enum_as_inner::EnumAsInner;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Allay {
    pub abstract_creature: AbstractCreature,
    pub dancing: bool,
    pub can_duplicate: bool,
}

impl Allay {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let dancing = metadata.pop_front()?.into_boolean().ok()?;
        let can_duplicate = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_creature,
            dancing,
            can_duplicate,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.dancing.clone()));
        metadata.push(EntityDataValue::Boolean(self.can_duplicate.clone()));
        metadata
    }
}

impl Default for Allay {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            dancing: false,
            can_duplicate: true,
        }
    }
}

impl Allay {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.dancing = value.into_boolean().ok()?,
            17 => self.can_duplicate = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Allay {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Allay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct AreaEffectCloud {
    pub abstract_entity: AbstractEntity,
    pub radius: f32,
    pub color: i32,
    pub waiting: bool,
    pub particle: Particle,
}

impl AreaEffectCloud {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let radius = metadata.pop_front()?.into_float().ok()?;
        let color = metadata.pop_front()?.into_int().ok()?;
        let waiting = metadata.pop_front()?.into_boolean().ok()?;
        let particle = metadata.pop_front()?.into_particle().ok()?;
        Some(Self {
            abstract_entity,
            radius,
            color,
            waiting,
            particle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::Float(self.radius.clone()));
        metadata.push(EntityDataValue::Int(self.color.clone()));
        metadata.push(EntityDataValue::Boolean(self.waiting.clone()));
        metadata.push(EntityDataValue::Particle(self.particle.clone()));
        metadata
    }
}

impl Default for AreaEffectCloud {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            radius: 0.5,
            color: 0,
            waiting: false,
            particle: Default::default(),
        }
    }
}

impl AreaEffectCloud {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.radius = value.into_float().ok()?,
            9 => self.color = value.into_int().ok()?,
            10 => self.waiting = value.into_boolean().ok()?,
            11 => self.particle = value.into_particle().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for AreaEffectCloud {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for AreaEffectCloud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
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
        let abstract_living = AbstractLiving::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let small = bitfield & 0x1 != 0;
        let show_arms = bitfield & 0x4 != 0;
        let no_base_plate = bitfield & 0x8 != 0;
        let marker = bitfield & 0x10 != 0;
        let head_pose = metadata.pop_front()?.into_rotations().ok()?;
        let body_pose = metadata.pop_front()?.into_rotations().ok()?;
        let left_arm_pose = metadata.pop_front()?.into_rotations().ok()?;
        let right_arm_pose = metadata.pop_front()?.into_rotations().ok()?;
        let left_leg_pose = metadata.pop_front()?.into_rotations().ok()?;
        let right_leg_pose = metadata.pop_front()?.into_rotations().ok()?;
        Some(Self {
            abstract_living,
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
        metadata.extend(self.abstract_living.write());
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

impl Default for ArmorStand {
    fn default() -> Self {
        Self {
            abstract_living: Default::default(),
            small: false,
            show_arms: false,
            no_base_plate: false,
            marker: false,
            head_pose: Default::default(),
            body_pose: Default::default(),
            left_arm_pose: Default::default(),
            right_arm_pose: Default::default(),
            left_leg_pose: Default::default(),
            right_leg_pose: Default::default(),
        }
    }
}

impl ArmorStand {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=14 => self.abstract_living.set_index(index, value)?,
            15 => {
                let bitfield = value.into_byte().ok()?;
                self.small = bitfield & 0x1 != 0;
                self.show_arms = bitfield & 0x4 != 0;
                self.no_base_plate = bitfield & 0x8 != 0;
                self.marker = bitfield & 0x10 != 0;
            }
            16 => self.head_pose = value.into_rotations().ok()?,
            17 => self.body_pose = value.into_rotations().ok()?,
            18 => self.left_arm_pose = value.into_rotations().ok()?,
            19 => self.right_arm_pose = value.into_rotations().ok()?,
            20 => self.left_leg_pose = value.into_rotations().ok()?,
            21 => self.right_leg_pose = value.into_rotations().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for ArmorStand {
    type Target = AbstractLiving;
    fn deref(&self) -> &Self::Target {
        &self.abstract_living
    }
}
impl DerefMut for ArmorStand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_living
    }
}

#[derive(Debug, Clone)]
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
        let abstract_entity = AbstractEntity::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let crit_arrow = bitfield & 0x1 != 0;
        let shot_from_crossbow = bitfield & 0x4 != 0;
        let no_physics = bitfield & 0x2 != 0;
        let pierce_level = metadata.pop_front()?.into_byte().ok()?;
        let effect_color = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_entity,
            crit_arrow,
            shot_from_crossbow,
            no_physics,
            pierce_level,
            effect_color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
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

impl Default for Arrow {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            crit_arrow: false,
            shot_from_crossbow: false,
            no_physics: false,
            pierce_level: 0,
            effect_color: -1,
        }
    }
}

impl Arrow {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => {
                let bitfield = value.into_byte().ok()?;
                self.crit_arrow = bitfield & 0x1 != 0;
                self.shot_from_crossbow = bitfield & 0x4 != 0;
                self.no_physics = bitfield & 0x2 != 0;
            }
            9 => self.pierce_level = value.into_byte().ok()?,
            10 => self.effect_color = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Arrow {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Arrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Axolotl {
    pub abstract_animal: AbstractAnimal,
    pub variant: i32,
    pub playing_dead: bool,
    pub from_bucket: bool,
}

impl Axolotl {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let variant = metadata.pop_front()?.into_int().ok()?;
        let playing_dead = metadata.pop_front()?.into_boolean().ok()?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            variant,
            playing_dead,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Int(self.variant.clone()));
        metadata.push(EntityDataValue::Boolean(self.playing_dead.clone()));
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}

impl Default for Axolotl {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            variant: 0,
            playing_dead: false,
            from_bucket: false,
        }
    }
}

impl Axolotl {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.variant = value.into_int().ok()?,
            18 => self.playing_dead = value.into_boolean().ok()?,
            19 => self.from_bucket = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Axolotl {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Axolotl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Bat {
    pub abstract_insentient: AbstractInsentient,
    pub resting: bool,
}

impl Bat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let resting = bitfield & 0x1 != 0;
        Some(Self {
            abstract_insentient,
            resting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        let mut bitfield = 0u8;
        if self.resting {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for Bat {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
            resting: false,
        }
    }
}

impl Bat {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_insentient.set_index(index, value)?,
            16 => {
                let bitfield = value.into_byte().ok()?;
                self.resting = bitfield & 0x1 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for Bat {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for Bat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
pub struct Bee {
    pub abstract_animal: AbstractAnimal,
    pub has_nectar: bool,
    pub has_stung: bool,
    pub rolling: bool,
    pub remaining_anger_time: i32,
}

impl Bee {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let has_nectar = bitfield & 0x8 != 0;
        let has_stung = bitfield & 0x4 != 0;
        let rolling = bitfield & 0x2 != 0;
        let remaining_anger_time = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_animal,
            has_nectar,
            has_stung,
            rolling,
            remaining_anger_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
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

impl Default for Bee {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            has_nectar: false,
            has_stung: false,
            rolling: false,
            remaining_anger_time: 0,
        }
    }
}

impl Bee {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.has_nectar = bitfield & 0x8 != 0;
                self.has_stung = bitfield & 0x4 != 0;
                self.rolling = bitfield & 0x2 != 0;
            }
            18 => self.remaining_anger_time = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Bee {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Bee {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Blaze {
    pub abstract_monster: AbstractMonster,
    pub charged: bool,
}

impl Blaze {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let charged = bitfield & 0x1 != 0;
        Some(Self {
            abstract_monster,
            charged,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        let mut bitfield = 0u8;
        if self.charged {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for Blaze {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            charged: false,
        }
    }
}

impl Blaze {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => {
                let bitfield = value.into_byte().ok()?;
                self.charged = bitfield & 0x1 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for Blaze {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Blaze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
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
        let abstract_entity = AbstractEntity::read(metadata)?;
        let hurt = metadata.pop_front()?.into_int().ok()?;
        let hurtdir = metadata.pop_front()?.into_int().ok()?;
        let damage = metadata.pop_front()?.into_float().ok()?;
        let kind = metadata.pop_front()?.into_int().ok()?;
        let paddle_left = metadata.pop_front()?.into_boolean().ok()?;
        let paddle_right = metadata.pop_front()?.into_boolean().ok()?;
        let bubble_time = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_entity,
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
        metadata.extend(self.abstract_entity.write());
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

impl Default for Boat {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            hurt: 0,
            hurtdir: 1,
            damage: 0.0,
            kind: Default::default(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
        }
    }
}

impl Boat {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.hurt = value.into_int().ok()?,
            9 => self.hurtdir = value.into_int().ok()?,
            10 => self.damage = value.into_float().ok()?,
            11 => self.kind = value.into_int().ok()?,
            12 => self.paddle_left = value.into_boolean().ok()?,
            13 => self.paddle_right = value.into_boolean().ok()?,
            14 => self.bubble_time = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Boat {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Boat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Cat {
    pub abstract_tameable: AbstractTameable,
    pub variant: azalea_registry::CatVariant,
    pub is_lying: bool,
    pub relax_state_one: bool,
    pub collar_color: i32,
}

impl Cat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_tameable = AbstractTameable::read(metadata)?;
        let variant = metadata.pop_front()?.into_cat_variant().ok()?;
        let is_lying = metadata.pop_front()?.into_boolean().ok()?;
        let relax_state_one = metadata.pop_front()?.into_boolean().ok()?;
        let collar_color = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_tameable,
            variant,
            is_lying,
            relax_state_one,
            collar_color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_tameable.write());
        metadata.push(EntityDataValue::CatVariant(self.variant.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_lying.clone()));
        metadata.push(EntityDataValue::Boolean(self.relax_state_one.clone()));
        metadata.push(EntityDataValue::Int(self.collar_color.clone()));
        metadata
    }
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            abstract_tameable: Default::default(),
            variant: azalea_registry::CatVariant::Tabby,
            is_lying: false,
            relax_state_one: false,
            collar_color: Default::default(),
        }
    }
}

impl Cat {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=18 => self.abstract_tameable.set_index(index, value)?,
            19 => self.variant = value.into_cat_variant().ok()?,
            20 => self.is_lying = value.into_boolean().ok()?,
            21 => self.relax_state_one = value.into_boolean().ok()?,
            22 => self.collar_color = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Cat {
    type Target = AbstractTameable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_tameable
    }
}
impl DerefMut for Cat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_tameable
    }
}

#[derive(Debug, Clone)]
pub struct CaveSpider {
    pub spider: Spider,
}

impl CaveSpider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let spider = Spider::read(metadata)?;
        Some(Self { spider })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.spider.write());
        metadata
    }
}

impl Default for CaveSpider {
    fn default() -> Self {
        Self {
            spider: Default::default(),
        }
    }
}

impl CaveSpider {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.spider.set_index(index, value)
    }
}
impl Deref for CaveSpider {
    type Target = Spider;
    fn deref(&self) -> &Self::Target {
        &self.spider
    }
}
impl DerefMut for CaveSpider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spider
    }
}

#[derive(Debug, Clone)]
pub struct ChestBoat {
    pub boat: Boat,
}

impl ChestBoat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let boat = Boat::read(metadata)?;
        Some(Self { boat })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.boat.write());
        metadata
    }
}

impl Default for ChestBoat {
    fn default() -> Self {
        Self {
            boat: Default::default(),
        }
    }
}

impl ChestBoat {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.boat.set_index(index, value)
    }
}
impl Deref for ChestBoat {
    type Target = Boat;
    fn deref(&self) -> &Self::Target {
        &self.boat
    }
}
impl DerefMut for ChestBoat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boat
    }
}

#[derive(Debug, Clone)]
pub struct ChestMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl ChestMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        Some(Self { abstract_minecart })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata
    }
}

impl Default for ChestMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
        }
    }
}

impl ChestMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_minecart.set_index(index, value)
    }
}
impl Deref for ChestMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for ChestMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct Chicken {
    pub abstract_animal: AbstractAnimal,
}

impl Chicken {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        Some(Self { abstract_animal })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata
    }
}

impl Default for Chicken {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
        }
    }
}

impl Chicken {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_animal.set_index(index, value)
    }
}
impl Deref for Chicken {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Chicken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Cod {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Cod {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_creature,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}

impl Default for Cod {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            from_bucket: false,
        }
    }
}

impl Cod {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.from_bucket = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Cod {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Cod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct CommandBlockMinecart {
    pub abstract_minecart: AbstractMinecart,
    pub command_name: String,
    pub last_output: Component,
}

impl CommandBlockMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        let command_name = metadata.pop_front()?.into_string().ok()?;
        let last_output = metadata.pop_front()?.into_component().ok()?;
        Some(Self {
            abstract_minecart,
            command_name,
            last_output,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata.push(EntityDataValue::String(self.command_name.clone()));
        metadata.push(EntityDataValue::Component(self.last_output.clone()));
        metadata
    }
}

impl Default for CommandBlockMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
            command_name: "".to_string(),
            last_output: Default::default(),
        }
    }
}

impl CommandBlockMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=13 => self.abstract_minecart.set_index(index, value)?,
            14 => self.command_name = value.into_string().ok()?,
            15 => self.last_output = value.into_component().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for CommandBlockMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for CommandBlockMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct Cow {
    pub abstract_animal: AbstractAnimal,
}

impl Cow {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        Some(Self { abstract_animal })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata
    }
}

impl Default for Cow {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
        }
    }
}

impl Cow {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_animal.set_index(index, value)
    }
}
impl Deref for Cow {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Cow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Creeper {
    pub abstract_monster: AbstractMonster,
    pub swell_dir: i32,
    pub is_powered: bool,
    pub is_ignited: bool,
}

impl Creeper {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let swell_dir = metadata.pop_front()?.into_int().ok()?;
        let is_powered = metadata.pop_front()?.into_boolean().ok()?;
        let is_ignited = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            swell_dir,
            is_powered,
            is_ignited,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Int(self.swell_dir.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_powered.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_ignited.clone()));
        metadata
    }
}

impl Default for Creeper {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            swell_dir: -1,
            is_powered: false,
            is_ignited: false,
        }
    }
}

impl Creeper {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.swell_dir = value.into_int().ok()?,
            17 => self.is_powered = value.into_boolean().ok()?,
            18 => self.is_ignited = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Creeper {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Creeper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Dolphin {
    pub abstract_creature: AbstractCreature,
    pub treasure_pos: BlockPos,
    pub got_fish: bool,
    pub moistness_level: i32,
}

impl Dolphin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let treasure_pos = metadata.pop_front()?.into_block_pos().ok()?;
        let got_fish = metadata.pop_front()?.into_boolean().ok()?;
        let moistness_level = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_creature,
            treasure_pos,
            got_fish,
            moistness_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::BlockPos(self.treasure_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.got_fish.clone()));
        metadata.push(EntityDataValue::Int(self.moistness_level.clone()));
        metadata
    }
}

impl Default for Dolphin {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            treasure_pos: BlockPos::new(0, 0, 0),
            got_fish: false,
            moistness_level: 2400,
        }
    }
}

impl Dolphin {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.treasure_pos = value.into_block_pos().ok()?,
            17 => self.got_fish = value.into_boolean().ok()?,
            18 => self.moistness_level = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Dolphin {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Dolphin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        let chest = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Donkey {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
            chest: false,
        }
    }
}

impl Donkey {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            19 => self.chest = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Donkey {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Donkey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct DragonFireball {
    pub abstract_entity: AbstractEntity,
}

impl DragonFireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for DragonFireball {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl DragonFireball {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for DragonFireball {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for DragonFireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Drowned {
    pub zombie: Zombie,
}

impl Drowned {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let zombie = Zombie::read(metadata)?;
        Some(Self { zombie })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.zombie.write());
        metadata
    }
}

impl Default for Drowned {
    fn default() -> Self {
        Self {
            zombie: Default::default(),
        }
    }
}

impl Drowned {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.zombie.set_index(index, value)
    }
}
impl Deref for Drowned {
    type Target = Zombie;
    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for Drowned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}

#[derive(Debug, Clone)]
pub struct Egg {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Egg {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for Egg {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl Egg {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Egg {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Egg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct ElderGuardian {
    pub guardian: Guardian,
}

impl ElderGuardian {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let guardian = Guardian::read(metadata)?;
        Some(Self { guardian })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.guardian.write());
        metadata
    }
}

impl Default for ElderGuardian {
    fn default() -> Self {
        Self {
            guardian: Default::default(),
        }
    }
}

impl ElderGuardian {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.guardian.set_index(index, value)
    }
}
impl Deref for ElderGuardian {
    type Target = Guardian;
    fn deref(&self) -> &Self::Target {
        &self.guardian
    }
}
impl DerefMut for ElderGuardian {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guardian
    }
}

#[derive(Debug, Clone)]
pub struct EndCrystal {
    pub abstract_entity: AbstractEntity,
    pub beam_target: Option<BlockPos>,
    pub show_bottom: bool,
}

impl EndCrystal {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let beam_target = metadata.pop_front()?.into_optional_block_pos().ok()?;
        let show_bottom = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
            beam_target,
            show_bottom,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::OptionalBlockPos(self.beam_target.clone()));
        metadata.push(EntityDataValue::Boolean(self.show_bottom.clone()));
        metadata
    }
}

impl Default for EndCrystal {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            beam_target: None,
            show_bottom: true,
        }
    }
}

impl EndCrystal {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.beam_target = value.into_optional_block_pos().ok()?,
            9 => self.show_bottom = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for EndCrystal {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for EndCrystal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct EnderDragon {
    pub abstract_insentient: AbstractInsentient,
    pub phase: i32,
}

impl EnderDragon {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        let phase = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_insentient,
            phase,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        metadata.push(EntityDataValue::Int(self.phase.clone()));
        metadata
    }
}

impl Default for EnderDragon {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
            phase: Default::default(),
        }
    }
}

impl EnderDragon {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_insentient.set_index(index, value)?,
            16 => self.phase = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for EnderDragon {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for EnderDragon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
pub struct EnderPearl {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl EnderPearl {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for EnderPearl {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl EnderPearl {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for EnderPearl {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for EnderPearl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Enderman {
    pub abstract_monster: AbstractMonster,
    pub carry_state: Option<BlockState>,
    pub creepy: bool,
    pub stared_at: bool,
}

impl Enderman {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let carry_state = metadata.pop_front()?.into_optional_block_state().ok()?;
        let creepy = metadata.pop_front()?.into_boolean().ok()?;
        let stared_at = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            carry_state,
            creepy,
            stared_at,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::OptionalBlockState(
            self.carry_state.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.creepy.clone()));
        metadata.push(EntityDataValue::Boolean(self.stared_at.clone()));
        metadata
    }
}

impl Default for Enderman {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            carry_state: None,
            creepy: false,
            stared_at: false,
        }
    }
}

impl Enderman {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.carry_state = value.into_optional_block_state().ok()?,
            17 => self.creepy = value.into_boolean().ok()?,
            18 => self.stared_at = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Enderman {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Enderman {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Endermite {
    pub abstract_monster: AbstractMonster,
}

impl Endermite {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        Some(Self { abstract_monster })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata
    }
}

impl Default for Endermite {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
        }
    }
}

impl Endermite {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_monster.set_index(index, value)
    }
}
impl Deref for Endermite {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Endermite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Evoker {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub spell_casting: u8,
}

impl Evoker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        let spell_casting = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
            spell_casting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Byte(self.spell_casting.clone()));
        metadata
    }
}

impl Default for Evoker {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
            spell_casting: 0,
        }
    }
}

impl Evoker {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            17 => self.spell_casting = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Evoker {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Evoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct EvokerFangs {
    pub abstract_entity: AbstractEntity,
}

impl EvokerFangs {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for EvokerFangs {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl EvokerFangs {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for EvokerFangs {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for EvokerFangs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct ExperienceBottle {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl ExperienceBottle {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for ExperienceBottle {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl ExperienceBottle {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for ExperienceBottle {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for ExperienceBottle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct ExperienceOrb {
    pub abstract_entity: AbstractEntity,
}

impl ExperienceOrb {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for ExperienceOrb {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl ExperienceOrb {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for ExperienceOrb {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for ExperienceOrb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct EyeOfEnder {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl EyeOfEnder {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for EyeOfEnder {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl EyeOfEnder {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for EyeOfEnder {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for EyeOfEnder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct FallingBlock {
    pub abstract_entity: AbstractEntity,
    pub start_pos: BlockPos,
}

impl FallingBlock {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let start_pos = metadata.pop_front()?.into_block_pos().ok()?;
        Some(Self {
            abstract_entity,
            start_pos,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::BlockPos(self.start_pos.clone()));
        metadata
    }
}

impl Default for FallingBlock {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            start_pos: BlockPos::new(0, 0, 0),
        }
    }
}

impl FallingBlock {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.start_pos = value.into_block_pos().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for FallingBlock {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for FallingBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Fireball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Fireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl Fireball {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Fireball {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Fireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct FireworkRocket {
    pub abstract_entity: AbstractEntity,
    pub fireworks_item: Slot,
    pub attached_to_target: Option<u32>,
    pub shot_at_angle: bool,
}

impl FireworkRocket {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let fireworks_item = metadata.pop_front()?.into_item_stack().ok()?;
        let attached_to_target = metadata.pop_front()?.into_optional_unsigned_int().ok()?;
        let shot_at_angle = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
            fireworks_item,
            attached_to_target,
            shot_at_angle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.fireworks_item.clone()));
        metadata.push(EntityDataValue::OptionalUnsignedInt(
            self.attached_to_target.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.shot_at_angle.clone()));
        metadata
    }
}

impl Default for FireworkRocket {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            fireworks_item: Slot::Empty,
            attached_to_target: None,
            shot_at_angle: false,
        }
    }
}

impl FireworkRocket {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.fireworks_item = value.into_item_stack().ok()?,
            9 => self.attached_to_target = value.into_optional_unsigned_int().ok()?,
            10 => self.shot_at_angle = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for FireworkRocket {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for FireworkRocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct FishingBobber {
    pub abstract_entity: AbstractEntity,
    pub hooked_entity: i32,
    pub biting: bool,
}

impl FishingBobber {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let hooked_entity = metadata.pop_front()?.into_int().ok()?;
        let biting = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
            hooked_entity,
            biting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::Int(self.hooked_entity.clone()));
        metadata.push(EntityDataValue::Boolean(self.biting.clone()));
        metadata
    }
}

impl Default for FishingBobber {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            hooked_entity: 0,
            biting: false,
        }
    }
}

impl FishingBobber {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.hooked_entity = value.into_int().ok()?,
            9 => self.biting = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for FishingBobber {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for FishingBobber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let kind = metadata.pop_front()?.into_int().ok()?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let sitting = bitfield & 0x1 != 0;
        let faceplanted = bitfield & 0x40 != 0;
        let sleeping = bitfield & 0x20 != 0;
        let pouncing = bitfield & 0x10 != 0;
        let crouching = bitfield & 0x4 != 0;
        let interested = bitfield & 0x8 != 0;
        let trusted_id_0 = metadata.pop_front()?.into_optional_uuid().ok()?;
        let trusted_id_1 = metadata.pop_front()?.into_optional_uuid().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Fox {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            kind: 0,
            sitting: false,
            faceplanted: false,
            sleeping: false,
            pouncing: false,
            crouching: false,
            interested: false,
            trusted_id_0: None,
            trusted_id_1: None,
        }
    }
}

impl Fox {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.kind = value.into_int().ok()?,
            18 => {
                let bitfield = value.into_byte().ok()?;
                self.sitting = bitfield & 0x1 != 0;
                self.faceplanted = bitfield & 0x40 != 0;
                self.sleeping = bitfield & 0x20 != 0;
                self.pouncing = bitfield & 0x10 != 0;
                self.crouching = bitfield & 0x4 != 0;
                self.interested = bitfield & 0x8 != 0;
            }
            19 => self.trusted_id_0 = value.into_optional_uuid().ok()?,
            20 => self.trusted_id_1 = value.into_optional_uuid().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Fox {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Fox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Frog {
    pub abstract_animal: AbstractAnimal,
    pub variant: azalea_registry::FrogVariant,
    pub tongue_target: Option<u32>,
}

impl Frog {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let variant = metadata.pop_front()?.into_frog_variant().ok()?;
        let tongue_target = metadata.pop_front()?.into_optional_unsigned_int().ok()?;
        Some(Self {
            abstract_animal,
            variant,
            tongue_target,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::FrogVariant(self.variant.clone()));
        metadata.push(EntityDataValue::OptionalUnsignedInt(
            self.tongue_target.clone(),
        ));
        metadata
    }
}

impl Default for Frog {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            variant: azalea_registry::FrogVariant::Temperate,
            tongue_target: None,
        }
    }
}

impl Frog {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.variant = value.into_frog_variant().ok()?,
            18 => self.tongue_target = value.into_optional_unsigned_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Frog {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Frog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct FurnaceMinecart {
    pub abstract_minecart: AbstractMinecart,
    pub fuel: bool,
}

impl FurnaceMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        let fuel = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_minecart,
            fuel,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata.push(EntityDataValue::Boolean(self.fuel.clone()));
        metadata
    }
}

impl Default for FurnaceMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
            fuel: false,
        }
    }
}

impl FurnaceMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=13 => self.abstract_minecart.set_index(index, value)?,
            14 => self.fuel = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for FurnaceMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for FurnaceMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct Ghast {
    pub abstract_insentient: AbstractInsentient,
    pub is_charging: bool,
}

impl Ghast {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        let is_charging = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_insentient,
            is_charging,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        metadata.push(EntityDataValue::Boolean(self.is_charging.clone()));
        metadata
    }
}

impl Default for Ghast {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
            is_charging: false,
        }
    }
}

impl Ghast {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_insentient.set_index(index, value)?,
            16 => self.is_charging = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Ghast {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for Ghast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
pub struct Giant {
    pub abstract_monster: AbstractMonster,
}

impl Giant {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        Some(Self { abstract_monster })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata
    }
}

impl Default for Giant {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
        }
    }
}

impl Giant {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_monster.set_index(index, value)
    }
}
impl Deref for Giant {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Giant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct GlowItemFrame {
    pub item_frame: ItemFrame,
}

impl GlowItemFrame {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let item_frame = ItemFrame::read(metadata)?;
        Some(Self { item_frame })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.item_frame.write());
        metadata
    }
}

impl Default for GlowItemFrame {
    fn default() -> Self {
        Self {
            item_frame: Default::default(),
        }
    }
}

impl GlowItemFrame {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.item_frame.set_index(index, value)
    }
}
impl Deref for GlowItemFrame {
    type Target = ItemFrame;
    fn deref(&self) -> &Self::Target {
        &self.item_frame
    }
}
impl DerefMut for GlowItemFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item_frame
    }
}

#[derive(Debug, Clone)]
pub struct GlowSquid {
    pub squid: Squid,
    pub dark_ticks_remaining: i32,
}

impl GlowSquid {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let squid = Squid::read(metadata)?;
        let dark_ticks_remaining = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            squid,
            dark_ticks_remaining,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.squid.write());
        metadata.push(EntityDataValue::Int(self.dark_ticks_remaining.clone()));
        metadata
    }
}

impl Default for GlowSquid {
    fn default() -> Self {
        Self {
            squid: Default::default(),
            dark_ticks_remaining: 0,
        }
    }
}

impl GlowSquid {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.squid.set_index(index, value)?,
            16 => self.dark_ticks_remaining = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for GlowSquid {
    type Target = Squid;
    fn deref(&self) -> &Self::Target {
        &self.squid
    }
}
impl DerefMut for GlowSquid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.squid
    }
}

#[derive(Debug, Clone)]
pub struct Goat {
    pub abstract_animal: AbstractAnimal,
    pub is_screaming_goat: bool,
    pub has_left_horn: bool,
    pub has_right_horn: bool,
}

impl Goat {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let is_screaming_goat = metadata.pop_front()?.into_boolean().ok()?;
        let has_left_horn = metadata.pop_front()?.into_boolean().ok()?;
        let has_right_horn = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            is_screaming_goat,
            has_left_horn,
            has_right_horn,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Boolean(self.is_screaming_goat.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_left_horn.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_right_horn.clone()));
        metadata
    }
}

impl Default for Goat {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            is_screaming_goat: false,
            has_left_horn: true,
            has_right_horn: true,
        }
    }
}

impl Goat {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.is_screaming_goat = value.into_boolean().ok()?,
            18 => self.has_left_horn = value.into_boolean().ok()?,
            19 => self.has_right_horn = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Goat {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Goat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Guardian {
    pub abstract_monster: AbstractMonster,
    pub moving: bool,
    pub attack_target: i32,
}

impl Guardian {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let moving = metadata.pop_front()?.into_boolean().ok()?;
        let attack_target = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_monster,
            moving,
            attack_target,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.moving.clone()));
        metadata.push(EntityDataValue::Int(self.attack_target.clone()));
        metadata
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            moving: false,
            attack_target: 0,
        }
    }
}

impl Guardian {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.moving = value.into_boolean().ok()?,
            17 => self.attack_target = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Guardian {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Guardian {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Hoglin {
    pub abstract_animal: AbstractAnimal,
    pub immune_to_zombification: bool,
}

impl Hoglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let immune_to_zombification = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            immune_to_zombification,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata
    }
}

impl Default for Hoglin {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            immune_to_zombification: false,
        }
    }
}

impl Hoglin {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.immune_to_zombification = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Hoglin {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Hoglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct HopperMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl HopperMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        Some(Self { abstract_minecart })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata
    }
}

impl Default for HopperMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
        }
    }
}

impl HopperMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_minecart.set_index(index, value)
    }
}
impl Deref for HopperMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for HopperMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        let type_variant = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Horse {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
            type_variant: 0,
        }
    }
}

impl Horse {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            19 => self.type_variant = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Horse {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Horse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Husk {
    pub zombie: Zombie,
}

impl Husk {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let zombie = Zombie::read(metadata)?;
        Some(Self { zombie })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.zombie.write());
        metadata
    }
}

impl Default for Husk {
    fn default() -> Self {
        Self {
            zombie: Default::default(),
        }
    }
}

impl Husk {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.zombie.set_index(index, value)
    }
}
impl Deref for Husk {
    type Target = Zombie;
    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for Husk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}

#[derive(Debug, Clone)]
pub struct Illusioner {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub spell_casting: u8,
}

impl Illusioner {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        let spell_casting = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
            spell_casting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Byte(self.spell_casting.clone()));
        metadata
    }
}

impl Default for Illusioner {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
            spell_casting: 0,
        }
    }
}

impl Illusioner {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            17 => self.spell_casting = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Illusioner {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Illusioner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct IronGolem {
    pub abstract_creature: AbstractCreature,
    pub player_created: bool,
}

impl IronGolem {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let player_created = bitfield & 0x1 != 0;
        Some(Self {
            abstract_creature,
            player_created,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        let mut bitfield = 0u8;
        if self.player_created {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for IronGolem {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            player_created: false,
        }
    }
}

impl IronGolem {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => {
                let bitfield = value.into_byte().ok()?;
                self.player_created = bitfield & 0x1 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for IronGolem {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for IronGolem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub abstract_entity: AbstractEntity,
    pub item: Slot,
}

impl Item {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item.clone()));
        metadata
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item: Slot::Empty,
        }
    }
}

impl Item {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Item {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct ItemFrame {
    pub abstract_entity: AbstractEntity,
    pub item: Slot,
    pub rotation: i32,
}

impl ItemFrame {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item = metadata.pop_front()?.into_item_stack().ok()?;
        let rotation = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_entity,
            item,
            rotation,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item.clone()));
        metadata.push(EntityDataValue::Int(self.rotation.clone()));
        metadata
    }
}

impl Default for ItemFrame {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item: Slot::Empty,
            rotation: 0,
        }
    }
}

impl ItemFrame {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item = value.into_item_stack().ok()?,
            9 => self.rotation = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for ItemFrame {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for ItemFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct LeashKnot {
    pub abstract_entity: AbstractEntity,
}

impl LeashKnot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for LeashKnot {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl LeashKnot {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for LeashKnot {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for LeashKnot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct LightningBolt {
    pub abstract_entity: AbstractEntity,
}

impl LightningBolt {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for LightningBolt {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl LightningBolt {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for LightningBolt {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for LightningBolt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        let chest = metadata.pop_front()?.into_boolean().ok()?;
        let strength = metadata.pop_front()?.into_int().ok()?;
        let swag = metadata.pop_front()?.into_int().ok()?;
        let variant = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Llama {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
            chest: false,
            strength: 0,
            swag: -1,
            variant: 0,
        }
    }
}

impl Llama {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            19 => self.chest = value.into_boolean().ok()?,
            20 => self.strength = value.into_int().ok()?,
            21 => self.swag = value.into_int().ok()?,
            22 => self.variant = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Llama {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Llama {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct LlamaSpit {
    pub abstract_entity: AbstractEntity,
}

impl LlamaSpit {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for LlamaSpit {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl LlamaSpit {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for LlamaSpit {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for LlamaSpit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct MagmaCube {
    pub slime: Slime,
}

impl MagmaCube {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let slime = Slime::read(metadata)?;
        Some(Self { slime })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.slime.write());
        metadata
    }
}

impl Default for MagmaCube {
    fn default() -> Self {
        Self {
            slime: Default::default(),
        }
    }
}

impl MagmaCube {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.slime.set_index(index, value)
    }
}
impl Deref for MagmaCube {
    type Target = Slime;
    fn deref(&self) -> &Self::Target {
        &self.slime
    }
}
impl DerefMut for MagmaCube {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slime
    }
}

#[derive(Debug, Clone)]
pub struct Marker {
    pub abstract_entity: AbstractEntity,
}

impl Marker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl Marker {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for Marker {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Marker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Minecart {
    pub abstract_minecart: AbstractMinecart,
}

impl Minecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        Some(Self { abstract_minecart })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata
    }
}

impl Default for Minecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
        }
    }
}

impl Minecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_minecart.set_index(index, value)
    }
}
impl Deref for Minecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for Minecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct Mooshroom {
    pub cow: Cow,
    pub kind: String,
}

impl Mooshroom {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let cow = Cow::read(metadata)?;
        let kind = metadata.pop_front()?.into_string().ok()?;
        Some(Self { cow, kind })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.cow.write());
        metadata.push(EntityDataValue::String(self.kind.clone()));
        metadata
    }
}

impl Default for Mooshroom {
    fn default() -> Self {
        Self {
            cow: Default::default(),
            kind: Default::default(),
        }
    }
}

impl Mooshroom {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.cow.set_index(index, value)?,
            17 => self.kind = value.into_string().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Mooshroom {
    type Target = Cow;
    fn deref(&self) -> &Self::Target {
        &self.cow
    }
}
impl DerefMut for Mooshroom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cow
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        let chest = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Mule {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
            chest: false,
        }
    }
}

impl Mule {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            19 => self.chest = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Mule {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Mule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Ocelot {
    pub abstract_animal: AbstractAnimal,
    pub trusting: bool,
}

impl Ocelot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let trusting = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            trusting,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Boolean(self.trusting.clone()));
        metadata
    }
}

impl Default for Ocelot {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            trusting: false,
        }
    }
}

impl Ocelot {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.trusting = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Ocelot {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Ocelot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Painting {
    pub abstract_entity: AbstractEntity,
    pub painting_variant: azalea_registry::PaintingVariant,
}

impl Painting {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let painting_variant = metadata.pop_front()?.into_painting_variant().ok()?;
        Some(Self {
            abstract_entity,
            painting_variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::PaintingVariant(
            self.painting_variant.clone(),
        ));
        metadata
    }
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            painting_variant: azalea_registry::PaintingVariant::Kebab,
        }
    }
}

impl Painting {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.painting_variant = value.into_painting_variant().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Painting {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Painting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let unhappy_counter = metadata.pop_front()?.into_int().ok()?;
        let sneeze_counter = metadata.pop_front()?.into_int().ok()?;
        let eat_counter = metadata.pop_front()?.into_int().ok()?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let sneezing = bitfield & 0x2 != 0;
        let sitting = bitfield & 0x8 != 0;
        let on_back = bitfield & 0x10 != 0;
        let rolling = bitfield & 0x4 != 0;
        let hidden_gene = metadata.pop_front()?.into_byte().ok()?;
        let flags = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for Panda {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            unhappy_counter: 0,
            sneeze_counter: 0,
            eat_counter: 0,
            sneezing: false,
            sitting: false,
            on_back: false,
            rolling: false,
            hidden_gene: 0,
            flags: 0,
        }
    }
}

impl Panda {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.unhappy_counter = value.into_int().ok()?,
            18 => self.sneeze_counter = value.into_int().ok()?,
            19 => self.eat_counter = value.into_int().ok()?,
            20 => {
                let bitfield = value.into_byte().ok()?;
                self.sneezing = bitfield & 0x2 != 0;
                self.sitting = bitfield & 0x8 != 0;
                self.on_back = bitfield & 0x10 != 0;
                self.rolling = bitfield & 0x4 != 0;
            }
            21 => self.hidden_gene = value.into_byte().ok()?,
            22 => self.flags = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Panda {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Panda {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Parrot {
    pub abstract_tameable: AbstractTameable,
    pub variant: i32,
}

impl Parrot {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_tameable = AbstractTameable::read(metadata)?;
        let variant = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_tameable,
            variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_tameable.write());
        metadata.push(EntityDataValue::Int(self.variant.clone()));
        metadata
    }
}

impl Default for Parrot {
    fn default() -> Self {
        Self {
            abstract_tameable: Default::default(),
            variant: 0,
        }
    }
}

impl Parrot {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=18 => self.abstract_tameable.set_index(index, value)?,
            19 => self.variant = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Parrot {
    type Target = AbstractTameable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_tameable
    }
}
impl DerefMut for Parrot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_tameable
    }
}

#[derive(Debug, Clone)]
pub struct Phantom {
    pub abstract_insentient: AbstractInsentient,
    pub size: i32,
}

impl Phantom {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        let size = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_insentient,
            size,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        metadata.push(EntityDataValue::Int(self.size.clone()));
        metadata
    }
}

impl Default for Phantom {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
            size: 0,
        }
    }
}

impl Phantom {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_insentient.set_index(index, value)?,
            16 => self.size = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Phantom {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for Phantom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
pub struct Pig {
    pub abstract_animal: AbstractAnimal,
    pub saddle: bool,
    pub boost_time: i32,
}

impl Pig {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let saddle = metadata.pop_front()?.into_boolean().ok()?;
        let boost_time = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_animal,
            saddle,
            boost_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Boolean(self.saddle.clone()));
        metadata.push(EntityDataValue::Int(self.boost_time.clone()));
        metadata
    }
}

impl Default for Pig {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            saddle: false,
            boost_time: 0,
        }
    }
}

impl Pig {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.saddle = value.into_boolean().ok()?,
            18 => self.boost_time = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Pig {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Pig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Piglin {
    pub abstract_monster: AbstractMonster,
    pub immune_to_zombification: bool,
    pub baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

impl Piglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let immune_to_zombification = metadata.pop_front()?.into_boolean().ok()?;
        let baby = metadata.pop_front()?.into_boolean().ok()?;
        let is_charging_crossbow = metadata.pop_front()?.into_boolean().ok()?;
        let is_dancing = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            immune_to_zombification,
            baby,
            is_charging_crossbow,
            is_dancing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_charging_crossbow.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_dancing.clone()));
        metadata
    }
}

impl Default for Piglin {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            immune_to_zombification: false,
            baby: false,
            is_charging_crossbow: false,
            is_dancing: false,
        }
    }
}

impl Piglin {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.immune_to_zombification = value.into_boolean().ok()?,
            17 => self.baby = value.into_boolean().ok()?,
            18 => self.is_charging_crossbow = value.into_boolean().ok()?,
            19 => self.is_dancing = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Piglin {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Piglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct PiglinBrute {
    pub abstract_monster: AbstractMonster,
    pub immune_to_zombification: bool,
}

impl PiglinBrute {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let immune_to_zombification = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            immune_to_zombification,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(
            self.immune_to_zombification.clone(),
        ));
        metadata
    }
}

impl Default for PiglinBrute {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            immune_to_zombification: false,
        }
    }
}

impl PiglinBrute {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.immune_to_zombification = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for PiglinBrute {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for PiglinBrute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Pillager {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub is_charging_crossbow: bool,
}

impl Pillager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        let is_charging_crossbow = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
            is_charging_crossbow,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Boolean(self.is_charging_crossbow.clone()));
        metadata
    }
}

impl Default for Pillager {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
            is_charging_crossbow: false,
        }
    }
}

impl Pillager {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            17 => self.is_charging_crossbow = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Pillager {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Pillager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
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
        let abstract_living = AbstractLiving::read(metadata)?;
        let player_absorption = metadata.pop_front()?.into_float().ok()?;
        let score = metadata.pop_front()?.into_int().ok()?;
        let player_mode_customisation = metadata.pop_front()?.into_byte().ok()?;
        let player_main_hand = metadata.pop_front()?.into_byte().ok()?;
        let shoulder_left = metadata.pop_front()?.into_compound_tag().ok()?;
        let shoulder_right = metadata.pop_front()?.into_compound_tag().ok()?;
        Some(Self {
            abstract_living,
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
        metadata.extend(self.abstract_living.write());
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

impl Default for Player {
    fn default() -> Self {
        Self {
            abstract_living: Default::default(),
            player_absorption: 0.0,
            score: 0,
            player_mode_customisation: 0,
            player_main_hand: 1,
            shoulder_left: azalea_nbt::Tag::Compound(Default::default()),
            shoulder_right: azalea_nbt::Tag::Compound(Default::default()),
        }
    }
}

impl Player {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=14 => self.abstract_living.set_index(index, value)?,
            15 => self.player_absorption = value.into_float().ok()?,
            16 => self.score = value.into_int().ok()?,
            17 => self.player_mode_customisation = value.into_byte().ok()?,
            18 => self.player_main_hand = value.into_byte().ok()?,
            19 => self.shoulder_left = value.into_compound_tag().ok()?,
            20 => self.shoulder_right = value.into_compound_tag().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Player {
    type Target = AbstractLiving;
    fn deref(&self) -> &Self::Target {
        &self.abstract_living
    }
}
impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_living
    }
}

#[derive(Debug, Clone)]
pub struct PolarBear {
    pub abstract_animal: AbstractAnimal,
    pub standing: bool,
}

impl PolarBear {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let standing = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            standing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Boolean(self.standing.clone()));
        metadata
    }
}

impl Default for PolarBear {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            standing: false,
        }
    }
}

impl PolarBear {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.standing = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for PolarBear {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for PolarBear {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Potion {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Potion {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for Potion {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl Potion {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Potion {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Potion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Pufferfish {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
    pub puff_state: i32,
}

impl Pufferfish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        let puff_state = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_creature,
            from_bucket,
            puff_state,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata.push(EntityDataValue::Int(self.puff_state.clone()));
        metadata
    }
}

impl Default for Pufferfish {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            from_bucket: false,
            puff_state: 0,
        }
    }
}

impl Pufferfish {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.from_bucket = value.into_boolean().ok()?,
            17 => self.puff_state = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Pufferfish {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Pufferfish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Rabbit {
    pub abstract_animal: AbstractAnimal,
    pub kind: i32,
}

impl Rabbit {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let kind = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_animal,
            kind,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Int(self.kind.clone()));
        metadata
    }
}

impl Default for Rabbit {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            kind: 0,
        }
    }
}

impl Rabbit {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.kind = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Rabbit {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Rabbit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Ravager {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
}

impl Ravager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata
    }
}

impl Default for Ravager {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
        }
    }
}

impl Ravager {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Ravager {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Ravager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Salmon {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Salmon {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_creature,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}

impl Default for Salmon {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            from_bucket: false,
        }
    }
}

impl Salmon {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.from_bucket = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Salmon {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Salmon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Sheep {
    pub abstract_animal: AbstractAnimal,
    pub sheared: bool,
}

impl Sheep {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let sheared = bitfield & 0x10 != 0;
        Some(Self {
            abstract_animal,
            sheared,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        let mut bitfield = 0u8;
        if self.sheared {
            bitfield &= 0x10;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for Sheep {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            sheared: false,
        }
    }
}

impl Sheep {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.sheared = bitfield & 0x10 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for Sheep {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Sheep {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Shulker {
    pub abstract_creature: AbstractCreature,
    pub attach_face: Direction,
    pub peek: u8,
    pub color: u8,
}

impl Shulker {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let attach_face = metadata.pop_front()?.into_direction().ok()?;
        let peek = metadata.pop_front()?.into_byte().ok()?;
        let color = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_creature,
            attach_face,
            peek,
            color,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Direction(self.attach_face.clone()));
        metadata.push(EntityDataValue::Byte(self.peek.clone()));
        metadata.push(EntityDataValue::Byte(self.color.clone()));
        metadata
    }
}

impl Default for Shulker {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            attach_face: Default::default(),
            peek: 0,
            color: 16,
        }
    }
}

impl Shulker {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.attach_face = value.into_direction().ok()?,
            17 => self.peek = value.into_byte().ok()?,
            18 => self.color = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Shulker {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Shulker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct ShulkerBullet {
    pub abstract_entity: AbstractEntity,
}

impl ShulkerBullet {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        Some(Self { abstract_entity })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata
    }
}

impl Default for ShulkerBullet {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
        }
    }
}

impl ShulkerBullet {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_entity.set_index(index, value)
    }
}
impl Deref for ShulkerBullet {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for ShulkerBullet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Silverfish {
    pub abstract_monster: AbstractMonster,
}

impl Silverfish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        Some(Self { abstract_monster })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata
    }
}

impl Default for Silverfish {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
        }
    }
}

impl Silverfish {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_monster.set_index(index, value)
    }
}
impl Deref for Silverfish {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Silverfish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub abstract_monster: AbstractMonster,
    pub stray_conversion: bool,
}

impl Skeleton {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let stray_conversion = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            stray_conversion,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.stray_conversion.clone()));
        metadata
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            stray_conversion: false,
        }
    }
}

impl Skeleton {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.stray_conversion = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Skeleton {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Skeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for SkeletonHorse {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
        }
    }
}

impl SkeletonHorse {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for SkeletonHorse {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for SkeletonHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Slime {
    pub abstract_insentient: AbstractInsentient,
    pub size: i32,
}

impl Slime {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        let size = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_insentient,
            size,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        metadata.push(EntityDataValue::Int(self.size.clone()));
        metadata
    }
}

impl Default for Slime {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
            size: 1,
        }
    }
}

impl Slime {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_insentient.set_index(index, value)?,
            16 => self.size = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Slime {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for Slime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
pub struct SmallFireball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl SmallFireball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for SmallFireball {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl SmallFireball {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for SmallFireball {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for SmallFireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct SnowGolem {
    pub abstract_creature: AbstractCreature,
    pub has_pumpkin: bool,
}

impl SnowGolem {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let has_pumpkin = bitfield & 0x10 != 0;
        Some(Self {
            abstract_creature,
            has_pumpkin,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        let mut bitfield = 0u8;
        if self.has_pumpkin {
            bitfield &= 0x10;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for SnowGolem {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            has_pumpkin: true,
        }
    }
}

impl SnowGolem {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => {
                let bitfield = value.into_byte().ok()?;
                self.has_pumpkin = bitfield & 0x10 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for SnowGolem {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for SnowGolem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Snowball {
    pub abstract_entity: AbstractEntity,
    pub item_stack: Slot,
}

impl Snowball {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let item_stack = metadata.pop_front()?.into_item_stack().ok()?;
        Some(Self {
            abstract_entity,
            item_stack,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::ItemStack(self.item_stack.clone()));
        metadata
    }
}

impl Default for Snowball {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            item_stack: Slot::Empty,
        }
    }
}

impl Snowball {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.item_stack = value.into_item_stack().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Snowball {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Snowball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct SpawnerMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl SpawnerMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        Some(Self { abstract_minecart })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata
    }
}

impl Default for SpawnerMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
        }
    }
}

impl SpawnerMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_minecart.set_index(index, value)
    }
}
impl Deref for SpawnerMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for SpawnerMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct SpectralArrow {
    pub abstract_entity: AbstractEntity,
    pub crit_arrow: bool,
    pub shot_from_crossbow: bool,
    pub no_physics: bool,
    pub pierce_level: u8,
}

impl SpectralArrow {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let crit_arrow = bitfield & 0x1 != 0;
        let shot_from_crossbow = bitfield & 0x4 != 0;
        let no_physics = bitfield & 0x2 != 0;
        let pierce_level = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_entity,
            crit_arrow,
            shot_from_crossbow,
            no_physics,
            pierce_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
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

impl Default for SpectralArrow {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            crit_arrow: false,
            shot_from_crossbow: false,
            no_physics: false,
            pierce_level: 0,
        }
    }
}

impl SpectralArrow {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => {
                let bitfield = value.into_byte().ok()?;
                self.crit_arrow = bitfield & 0x1 != 0;
                self.shot_from_crossbow = bitfield & 0x4 != 0;
                self.no_physics = bitfield & 0x2 != 0;
            }
            9 => self.pierce_level = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for SpectralArrow {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for SpectralArrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Spider {
    pub abstract_monster: AbstractMonster,
    pub climbing: bool,
}

impl Spider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let climbing = bitfield & 0x1 != 0;
        Some(Self {
            abstract_monster,
            climbing,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        let mut bitfield = 0u8;
        if self.climbing {
            bitfield &= 0x1;
        }
        metadata.push(EntityDataValue::Byte(bitfield));
        metadata
    }
}

impl Default for Spider {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            climbing: false,
        }
    }
}

impl Spider {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => {
                let bitfield = value.into_byte().ok()?;
                self.climbing = bitfield & 0x1 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for Spider {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Spider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Squid {
    pub abstract_creature: AbstractCreature,
}

impl Squid {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        Some(Self { abstract_creature })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata
    }
}

impl Default for Squid {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
        }
    }
}

impl Squid {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_creature.set_index(index, value)
    }
}
impl Deref for Squid {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Squid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Stray {
    pub abstract_monster: AbstractMonster,
}

impl Stray {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        Some(Self { abstract_monster })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata
    }
}

impl Default for Stray {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
        }
    }
}

impl Stray {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_monster.set_index(index, value)
    }
}
impl Deref for Stray {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Stray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Strider {
    pub abstract_animal: AbstractAnimal,
    pub boost_time: i32,
    pub suffocating: bool,
    pub saddle: bool,
}

impl Strider {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let boost_time = metadata.pop_front()?.into_int().ok()?;
        let suffocating = metadata.pop_front()?.into_boolean().ok()?;
        let saddle = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
            boost_time,
            suffocating,
            saddle,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::Int(self.boost_time.clone()));
        metadata.push(EntityDataValue::Boolean(self.suffocating.clone()));
        metadata.push(EntityDataValue::Boolean(self.saddle.clone()));
        metadata
    }
}

impl Default for Strider {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            boost_time: 0,
            suffocating: false,
            saddle: false,
        }
    }
}

impl Strider {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.boost_time = value.into_int().ok()?,
            18 => self.suffocating = value.into_boolean().ok()?,
            19 => self.saddle = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Strider {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Strider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Tadpole {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
}

impl Tadpole {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_creature,
            from_bucket,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata
    }
}

impl Default for Tadpole {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            from_bucket: false,
        }
    }
}

impl Tadpole {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.from_bucket = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Tadpole {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for Tadpole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct Tnt {
    pub abstract_entity: AbstractEntity,
    pub fuse: i32,
}

impl Tnt {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let fuse = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_entity,
            fuse,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::Int(self.fuse.clone()));
        metadata
    }
}

impl Default for Tnt {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            fuse: 80,
        }
    }
}

impl Tnt {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.fuse = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Tnt {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Tnt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct TntMinecart {
    pub abstract_minecart: AbstractMinecart,
}

impl TntMinecart {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_minecart = AbstractMinecart::read(metadata)?;
        Some(Self { abstract_minecart })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_minecart.write());
        metadata
    }
}

impl Default for TntMinecart {
    fn default() -> Self {
        Self {
            abstract_minecart: Default::default(),
        }
    }
}

impl TntMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_minecart.set_index(index, value)
    }
}
impl Deref for TntMinecart {
    type Target = AbstractMinecart;
    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for TntMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}

#[derive(Debug, Clone)]
pub struct TraderLlama {
    pub llama: Llama,
}

impl TraderLlama {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let llama = Llama::read(metadata)?;
        Some(Self { llama })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.llama.write());
        metadata
    }
}

impl Default for TraderLlama {
    fn default() -> Self {
        Self {
            llama: Default::default(),
        }
    }
}

impl TraderLlama {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.llama.set_index(index, value)
    }
}
impl Deref for TraderLlama {
    type Target = Llama;
    fn deref(&self) -> &Self::Target {
        &self.llama
    }
}
impl DerefMut for TraderLlama {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.llama
    }
}

#[derive(Debug, Clone)]
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
        let abstract_entity = AbstractEntity::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let crit_arrow = bitfield & 0x1 != 0;
        let shot_from_crossbow = bitfield & 0x4 != 0;
        let no_physics = bitfield & 0x2 != 0;
        let pierce_level = metadata.pop_front()?.into_byte().ok()?;
        let loyalty = metadata.pop_front()?.into_byte().ok()?;
        let foil = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
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
        metadata.extend(self.abstract_entity.write());
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

impl Default for Trident {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            crit_arrow: false,
            shot_from_crossbow: false,
            no_physics: false,
            pierce_level: 0,
            loyalty: 0,
            foil: false,
        }
    }
}

impl Trident {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => {
                let bitfield = value.into_byte().ok()?;
                self.crit_arrow = bitfield & 0x1 != 0;
                self.shot_from_crossbow = bitfield & 0x4 != 0;
                self.no_physics = bitfield & 0x2 != 0;
            }
            9 => self.pierce_level = value.into_byte().ok()?,
            10 => self.loyalty = value.into_byte().ok()?,
            11 => self.foil = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Trident {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for Trident {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct TropicalFish {
    pub abstract_creature: AbstractCreature,
    pub from_bucket: bool,
    pub type_variant: i32,
}

impl TropicalFish {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let from_bucket = metadata.pop_front()?.into_boolean().ok()?;
        let type_variant = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_creature,
            from_bucket,
            type_variant,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.from_bucket.clone()));
        metadata.push(EntityDataValue::Int(self.type_variant.clone()));
        metadata
    }
}

impl Default for TropicalFish {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            from_bucket: false,
            type_variant: 0,
        }
    }
}

impl TropicalFish {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.from_bucket = value.into_boolean().ok()?,
            17 => self.type_variant = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for TropicalFish {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for TropicalFish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let home_pos = metadata.pop_front()?.into_block_pos().ok()?;
        let has_egg = metadata.pop_front()?.into_boolean().ok()?;
        let laying_egg = metadata.pop_front()?.into_boolean().ok()?;
        let travel_pos = metadata.pop_front()?.into_block_pos().ok()?;
        let going_home = metadata.pop_front()?.into_boolean().ok()?;
        let travelling = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
        metadata.push(EntityDataValue::BlockPos(self.home_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.has_egg.clone()));
        metadata.push(EntityDataValue::Boolean(self.laying_egg.clone()));
        metadata.push(EntityDataValue::BlockPos(self.travel_pos.clone()));
        metadata.push(EntityDataValue::Boolean(self.going_home.clone()));
        metadata.push(EntityDataValue::Boolean(self.travelling.clone()));
        metadata
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            home_pos: BlockPos::new(0, 0, 0),
            has_egg: false,
            laying_egg: false,
            travel_pos: BlockPos::new(0, 0, 0),
            going_home: false,
            travelling: false,
        }
    }
}

impl Turtle {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => self.home_pos = value.into_block_pos().ok()?,
            18 => self.has_egg = value.into_boolean().ok()?,
            19 => self.laying_egg = value.into_boolean().ok()?,
            20 => self.travel_pos = value.into_block_pos().ok()?,
            21 => self.going_home = value.into_boolean().ok()?,
            22 => self.travelling = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Turtle {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for Turtle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct Vex {
    pub abstract_monster: AbstractMonster,
    pub flags: u8,
}

impl Vex {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let flags = metadata.pop_front()?.into_byte().ok()?;
        Some(Self {
            abstract_monster,
            flags,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Byte(self.flags.clone()));
        metadata
    }
}

impl Default for Vex {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            flags: 0,
        }
    }
}

impl Vex {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.flags = value.into_byte().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Vex {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Vex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Villager {
    pub abstract_ageable: AbstractAgeable,
    pub unhappy_counter: i32,
    pub villager_data: VillagerData,
}

impl Villager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_ageable = AbstractAgeable::read(metadata)?;
        let unhappy_counter = metadata.pop_front()?.into_int().ok()?;
        let villager_data = metadata.pop_front()?.into_villager_data().ok()?;
        Some(Self {
            abstract_ageable,
            unhappy_counter,
            villager_data,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_ageable.write());
        metadata.push(EntityDataValue::Int(self.unhappy_counter.clone()));
        metadata.push(EntityDataValue::VillagerData(self.villager_data.clone()));
        metadata
    }
}

impl Default for Villager {
    fn default() -> Self {
        Self {
            abstract_ageable: Default::default(),
            unhappy_counter: 0,
            villager_data: Default::default(),
        }
    }
}

impl Villager {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_ageable.set_index(index, value)?,
            17 => self.unhappy_counter = value.into_int().ok()?,
            18 => self.villager_data = value.into_villager_data().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Villager {
    type Target = AbstractAgeable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_ageable
    }
}
impl DerefMut for Villager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_ageable
    }
}

#[derive(Debug, Clone)]
pub struct Vindicator {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
}

impl Vindicator {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata
    }
}

impl Default for Vindicator {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
        }
    }
}

impl Vindicator {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Vindicator {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Vindicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct WanderingTrader {
    pub abstract_ageable: AbstractAgeable,
    pub unhappy_counter: i32,
}

impl WanderingTrader {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_ageable = AbstractAgeable::read(metadata)?;
        let unhappy_counter = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_ageable,
            unhappy_counter,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_ageable.write());
        metadata.push(EntityDataValue::Int(self.unhappy_counter.clone()));
        metadata
    }
}

impl Default for WanderingTrader {
    fn default() -> Self {
        Self {
            abstract_ageable: Default::default(),
            unhappy_counter: 0,
        }
    }
}

impl WanderingTrader {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_ageable.set_index(index, value)?,
            17 => self.unhappy_counter = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for WanderingTrader {
    type Target = AbstractAgeable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_ageable
    }
}
impl DerefMut for WanderingTrader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_ageable
    }
}

#[derive(Debug, Clone)]
pub struct Warden {
    pub abstract_monster: AbstractMonster,
    pub client_anger_level: i32,
}

impl Warden {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let client_anger_level = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_monster,
            client_anger_level,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Int(self.client_anger_level.clone()));
        metadata
    }
}

impl Default for Warden {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            client_anger_level: 0,
        }
    }
}

impl Warden {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.client_anger_level = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Warden {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Warden {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Witch {
    pub abstract_monster: AbstractMonster,
    pub is_celebrating: bool,
    pub using_item: bool,
}

impl Witch {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let is_celebrating = metadata.pop_front()?.into_boolean().ok()?;
        let using_item = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            is_celebrating,
            using_item,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.is_celebrating.clone()));
        metadata.push(EntityDataValue::Boolean(self.using_item.clone()));
        metadata
    }
}

impl Default for Witch {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            is_celebrating: false,
            using_item: false,
        }
    }
}

impl Witch {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.is_celebrating = value.into_boolean().ok()?,
            17 => self.using_item = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Witch {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Witch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Wither {
    pub abstract_monster: AbstractMonster,
    pub target_a: i32,
    pub target_b: i32,
    pub target_c: i32,
    pub inv: i32,
}

impl Wither {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let target_a = metadata.pop_front()?.into_int().ok()?;
        let target_b = metadata.pop_front()?.into_int().ok()?;
        let target_c = metadata.pop_front()?.into_int().ok()?;
        let inv = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_monster,
            target_a,
            target_b,
            target_c,
            inv,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Int(self.target_a.clone()));
        metadata.push(EntityDataValue::Int(self.target_b.clone()));
        metadata.push(EntityDataValue::Int(self.target_c.clone()));
        metadata.push(EntityDataValue::Int(self.inv.clone()));
        metadata
    }
}

impl Default for Wither {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            target_a: 0,
            target_b: 0,
            target_c: 0,
            inv: 0,
        }
    }
}

impl Wither {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.target_a = value.into_int().ok()?,
            17 => self.target_b = value.into_int().ok()?,
            18 => self.target_c = value.into_int().ok()?,
            19 => self.inv = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Wither {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Wither {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct WitherSkeleton {
    pub abstract_monster: AbstractMonster,
}

impl WitherSkeleton {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        Some(Self { abstract_monster })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata
    }
}

impl Default for WitherSkeleton {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
        }
    }
}

impl WitherSkeleton {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_monster.set_index(index, value)
    }
}
impl Deref for WitherSkeleton {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for WitherSkeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct WitherSkull {
    pub abstract_entity: AbstractEntity,
    pub dangerous: bool,
}

impl WitherSkull {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_entity = AbstractEntity::read(metadata)?;
        let dangerous = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
            dangerous,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::Boolean(self.dangerous.clone()));
        metadata
    }
}

impl Default for WitherSkull {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            dangerous: false,
        }
    }
}

impl WitherSkull {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.dangerous = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for WitherSkull {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for WitherSkull {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct Wolf {
    pub abstract_tameable: AbstractTameable,
    pub interested: bool,
    pub collar_color: i32,
    pub remaining_anger_time: i32,
}

impl Wolf {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_tameable = AbstractTameable::read(metadata)?;
        let interested = metadata.pop_front()?.into_boolean().ok()?;
        let collar_color = metadata.pop_front()?.into_int().ok()?;
        let remaining_anger_time = metadata.pop_front()?.into_int().ok()?;
        Some(Self {
            abstract_tameable,
            interested,
            collar_color,
            remaining_anger_time,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_tameable.write());
        metadata.push(EntityDataValue::Boolean(self.interested.clone()));
        metadata.push(EntityDataValue::Int(self.collar_color.clone()));
        metadata.push(EntityDataValue::Int(self.remaining_anger_time.clone()));
        metadata
    }
}

impl Default for Wolf {
    fn default() -> Self {
        Self {
            abstract_tameable: Default::default(),
            interested: false,
            collar_color: Default::default(),
            remaining_anger_time: 0,
        }
    }
}

impl Wolf {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=18 => self.abstract_tameable.set_index(index, value)?,
            19 => self.interested = value.into_boolean().ok()?,
            20 => self.collar_color = value.into_int().ok()?,
            21 => self.remaining_anger_time = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Wolf {
    type Target = AbstractTameable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_tameable
    }
}
impl DerefMut for Wolf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_tameable
    }
}

#[derive(Debug, Clone)]
pub struct Zoglin {
    pub abstract_monster: AbstractMonster,
    pub baby: bool,
}

impl Zoglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let baby = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            baby,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata
    }
}

impl Default for Zoglin {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            baby: false,
        }
    }
}

impl Zoglin {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.baby = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Zoglin {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Zoglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
pub struct Zombie {
    pub abstract_monster: AbstractMonster,
    pub baby: bool,
    pub special_type: i32,
    pub drowned_conversion: bool,
}

impl Zombie {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_monster = AbstractMonster::read(metadata)?;
        let baby = metadata.pop_front()?.into_boolean().ok()?;
        let special_type = metadata.pop_front()?.into_int().ok()?;
        let drowned_conversion = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_monster,
            baby,
            special_type,
            drowned_conversion,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_monster.write());
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata.push(EntityDataValue::Int(self.special_type.clone()));
        metadata.push(EntityDataValue::Boolean(self.drowned_conversion.clone()));
        metadata
    }
}

impl Default for Zombie {
    fn default() -> Self {
        Self {
            abstract_monster: Default::default(),
            baby: false,
            special_type: 0,
            drowned_conversion: false,
        }
    }
}

impl Zombie {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_monster.set_index(index, value)?,
            16 => self.baby = value.into_boolean().ok()?,
            17 => self.special_type = value.into_int().ok()?,
            18 => self.drowned_conversion = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for Zombie {
    type Target = AbstractMonster;
    fn deref(&self) -> &Self::Target {
        &self.abstract_monster
    }
}
impl DerefMut for Zombie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_monster
    }
}

#[derive(Debug, Clone)]
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
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tamed = bitfield & 0x2 != 0;
        let eating = bitfield & 0x10 != 0;
        let standing = bitfield & 0x20 != 0;
        let bred = bitfield & 0x8 != 0;
        let saddled = bitfield & 0x4 != 0;
        let owner_uuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        Some(Self {
            abstract_animal,
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
        metadata.extend(self.abstract_animal.write());
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

impl Default for ZombieHorse {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tamed: false,
            eating: false,
            standing: false,
            bred: false,
            saddled: false,
            owner_uuid: None,
        }
    }
}

impl ZombieHorse {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tamed = bitfield & 0x2 != 0;
                self.eating = bitfield & 0x10 != 0;
                self.standing = bitfield & 0x20 != 0;
                self.bred = bitfield & 0x8 != 0;
                self.saddled = bitfield & 0x4 != 0;
            }
            18 => self.owner_uuid = value.into_optional_uuid().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for ZombieHorse {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for ZombieHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone)]
pub struct ZombieVillager {
    pub zombie: Zombie,
    pub converting: bool,
    pub villager_data: VillagerData,
}

impl ZombieVillager {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let zombie = Zombie::read(metadata)?;
        let converting = metadata.pop_front()?.into_boolean().ok()?;
        let villager_data = metadata.pop_front()?.into_villager_data().ok()?;
        Some(Self {
            zombie,
            converting,
            villager_data,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.zombie.write());
        metadata.push(EntityDataValue::Boolean(self.converting.clone()));
        metadata.push(EntityDataValue::VillagerData(self.villager_data.clone()));
        metadata
    }
}

impl Default for ZombieVillager {
    fn default() -> Self {
        Self {
            zombie: Default::default(),
            converting: false,
            villager_data: Default::default(),
        }
    }
}

impl ZombieVillager {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=18 => self.zombie.set_index(index, value)?,
            19 => self.converting = value.into_boolean().ok()?,
            20 => self.villager_data = value.into_villager_data().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for ZombieVillager {
    type Target = Zombie;
    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for ZombieVillager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}

#[derive(Debug, Clone)]
pub struct ZombifiedPiglin {
    pub zombie: Zombie,
}

impl ZombifiedPiglin {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let zombie = Zombie::read(metadata)?;
        Some(Self { zombie })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.zombie.write());
        metadata
    }
}

impl Default for ZombifiedPiglin {
    fn default() -> Self {
        Self {
            zombie: Default::default(),
        }
    }
}

impl ZombifiedPiglin {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.zombie.set_index(index, value)
    }
}
impl Deref for ZombifiedPiglin {
    type Target = Zombie;
    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for ZombifiedPiglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}

#[derive(Debug, Clone)]
pub struct AbstractAgeable {
    pub abstract_creature: AbstractCreature,
    pub baby: bool,
}

impl AbstractAgeable {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        let baby = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_creature,
            baby,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata.push(EntityDataValue::Boolean(self.baby.clone()));
        metadata
    }
}

impl Default for AbstractAgeable {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
            baby: false,
        }
    }
}

impl AbstractAgeable {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=15 => self.abstract_creature.set_index(index, value)?,
            16 => self.baby = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for AbstractAgeable {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for AbstractAgeable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct AbstractAnimal {
    pub abstract_ageable: AbstractAgeable,
}

impl AbstractAnimal {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_ageable = AbstractAgeable::read(metadata)?;
        Some(Self { abstract_ageable })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_ageable.write());
        metadata
    }
}

impl Default for AbstractAnimal {
    fn default() -> Self {
        Self {
            abstract_ageable: Default::default(),
        }
    }
}

impl AbstractAnimal {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_ageable.set_index(index, value)
    }
}
impl Deref for AbstractAnimal {
    type Target = AbstractAgeable;
    fn deref(&self) -> &Self::Target {
        &self.abstract_ageable
    }
}
impl DerefMut for AbstractAnimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_ageable
    }
}

#[derive(Debug, Clone)]
pub struct AbstractCreature {
    pub abstract_insentient: AbstractInsentient,
}

impl AbstractCreature {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_insentient = AbstractInsentient::read(metadata)?;
        Some(Self {
            abstract_insentient,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_insentient.write());
        metadata
    }
}

impl Default for AbstractCreature {
    fn default() -> Self {
        Self {
            abstract_insentient: Default::default(),
        }
    }
}

impl AbstractCreature {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_insentient.set_index(index, value)
    }
}
impl Deref for AbstractCreature {
    type Target = AbstractInsentient;
    fn deref(&self) -> &Self::Target {
        &self.abstract_insentient
    }
}
impl DerefMut for AbstractCreature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_insentient
    }
}

#[derive(Debug, Clone)]
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
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let on_fire = bitfield & 0x1 != 0;
        let shift_key_down = bitfield & 0x2 != 0;
        let sprinting = bitfield & 0x8 != 0;
        let swimming = bitfield & 0x10 != 0;
        let currently_glowing = bitfield & 0x40 != 0;
        let invisible = bitfield & 0x20 != 0;
        let fall_flying = bitfield & 0x80 != 0;
        let air_supply = metadata.pop_front()?.into_int().ok()?;
        let custom_name = metadata.pop_front()?.into_optional_component().ok()?;
        let custom_name_visible = metadata.pop_front()?.into_boolean().ok()?;
        let silent = metadata.pop_front()?.into_boolean().ok()?;
        let no_gravity = metadata.pop_front()?.into_boolean().ok()?;
        let pose = metadata.pop_front()?.into_pose().ok()?;
        let ticks_frozen = metadata.pop_front()?.into_int().ok()?;
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

impl Default for AbstractEntity {
    fn default() -> Self {
        Self {
            on_fire: false,
            shift_key_down: false,
            sprinting: false,
            swimming: false,
            currently_glowing: false,
            invisible: false,
            fall_flying: false,
            air_supply: Default::default(),
            custom_name: None,
            custom_name_visible: false,
            silent: false,
            no_gravity: false,
            pose: Default::default(),
            ticks_frozen: 0,
        }
    }
}

impl AbstractEntity {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0 => {
                let bitfield = value.into_byte().ok()?;
                self.on_fire = bitfield & 0x1 != 0;
                self.shift_key_down = bitfield & 0x2 != 0;
                self.sprinting = bitfield & 0x8 != 0;
                self.swimming = bitfield & 0x10 != 0;
                self.currently_glowing = bitfield & 0x40 != 0;
                self.invisible = bitfield & 0x20 != 0;
                self.fall_flying = bitfield & 0x80 != 0;
            }
            1 => self.air_supply = value.into_int().ok()?,
            2 => self.custom_name = value.into_optional_component().ok()?,
            3 => self.custom_name_visible = value.into_boolean().ok()?,
            4 => self.silent = value.into_boolean().ok()?,
            5 => self.no_gravity = value.into_boolean().ok()?,
            6 => self.pose = value.into_pose().ok()?,
            7 => self.ticks_frozen = value.into_int().ok()?,
            _ => {}
        }
        Some(())
    }
}
#[derive(Debug, Clone)]
pub struct AbstractInsentient {
    pub abstract_living: AbstractLiving,
    pub no_ai: bool,
    pub left_handed: bool,
    pub aggressive: bool,
}

impl AbstractInsentient {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_living = AbstractLiving::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let no_ai = bitfield & 0x1 != 0;
        let left_handed = bitfield & 0x2 != 0;
        let aggressive = bitfield & 0x4 != 0;
        Some(Self {
            abstract_living,
            no_ai,
            left_handed,
            aggressive,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_living.write());
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

impl Default for AbstractInsentient {
    fn default() -> Self {
        Self {
            abstract_living: Default::default(),
            no_ai: false,
            left_handed: false,
            aggressive: false,
        }
    }
}

impl AbstractInsentient {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=14 => self.abstract_living.set_index(index, value)?,
            15 => {
                let bitfield = value.into_byte().ok()?;
                self.no_ai = bitfield & 0x1 != 0;
                self.left_handed = bitfield & 0x2 != 0;
                self.aggressive = bitfield & 0x4 != 0;
            }
            _ => {}
        }
        Some(())
    }
}
impl Deref for AbstractInsentient {
    type Target = AbstractLiving;
    fn deref(&self) -> &Self::Target {
        &self.abstract_living
    }
}
impl DerefMut for AbstractInsentient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_living
    }
}

#[derive(Debug, Clone)]
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
        let abstract_entity = AbstractEntity::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let auto_spin_attack = bitfield & 0x4 != 0;
        let using_item = bitfield & 0x1 != 0;
        let health = metadata.pop_front()?.into_float().ok()?;
        let effect_color = metadata.pop_front()?.into_int().ok()?;
        let effect_ambience = metadata.pop_front()?.into_boolean().ok()?;
        let arrow_count = metadata.pop_front()?.into_int().ok()?;
        let stinger_count = metadata.pop_front()?.into_int().ok()?;
        let sleeping_pos = metadata.pop_front()?.into_optional_block_pos().ok()?;
        Some(Self {
            abstract_entity,
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
        metadata.extend(self.abstract_entity.write());
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

impl Default for AbstractLiving {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            auto_spin_attack: false,
            using_item: false,
            health: 1.0,
            effect_color: 0,
            effect_ambience: false,
            arrow_count: 0,
            stinger_count: 0,
            sleeping_pos: None,
        }
    }
}

impl AbstractLiving {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => {
                let bitfield = value.into_byte().ok()?;
                self.auto_spin_attack = bitfield & 0x4 != 0;
                self.using_item = bitfield & 0x1 != 0;
            }
            9 => self.health = value.into_float().ok()?,
            10 => self.effect_color = value.into_int().ok()?,
            11 => self.effect_ambience = value.into_boolean().ok()?,
            12 => self.arrow_count = value.into_int().ok()?,
            13 => self.stinger_count = value.into_int().ok()?,
            14 => self.sleeping_pos = value.into_optional_block_pos().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for AbstractLiving {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for AbstractLiving {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
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
        let abstract_entity = AbstractEntity::read(metadata)?;
        let hurt = metadata.pop_front()?.into_int().ok()?;
        let hurtdir = metadata.pop_front()?.into_int().ok()?;
        let damage = metadata.pop_front()?.into_float().ok()?;
        let display_block = metadata.pop_front()?.into_int().ok()?;
        let display_offset = metadata.pop_front()?.into_int().ok()?;
        let custom_display = metadata.pop_front()?.into_boolean().ok()?;
        Some(Self {
            abstract_entity,
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
        metadata.extend(self.abstract_entity.write());
        metadata.push(EntityDataValue::Int(self.hurt.clone()));
        metadata.push(EntityDataValue::Int(self.hurtdir.clone()));
        metadata.push(EntityDataValue::Float(self.damage.clone()));
        metadata.push(EntityDataValue::Int(self.display_block.clone()));
        metadata.push(EntityDataValue::Int(self.display_offset.clone()));
        metadata.push(EntityDataValue::Boolean(self.custom_display.clone()));
        metadata
    }
}

impl Default for AbstractMinecart {
    fn default() -> Self {
        Self {
            abstract_entity: Default::default(),
            hurt: 0,
            hurtdir: 1,
            damage: 0.0,
            display_block: Default::default(),
            display_offset: 6,
            custom_display: false,
        }
    }
}

impl AbstractMinecart {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=7 => self.abstract_entity.set_index(index, value)?,
            8 => self.hurt = value.into_int().ok()?,
            9 => self.hurtdir = value.into_int().ok()?,
            10 => self.damage = value.into_float().ok()?,
            11 => self.display_block = value.into_int().ok()?,
            12 => self.display_offset = value.into_int().ok()?,
            13 => self.custom_display = value.into_boolean().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for AbstractMinecart {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        &self.abstract_entity
    }
}
impl DerefMut for AbstractMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_entity
    }
}

#[derive(Debug, Clone)]
pub struct AbstractMonster {
    pub abstract_creature: AbstractCreature,
}

impl AbstractMonster {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_creature = AbstractCreature::read(metadata)?;
        Some(Self { abstract_creature })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_creature.write());
        metadata
    }
}

impl Default for AbstractMonster {
    fn default() -> Self {
        Self {
            abstract_creature: Default::default(),
        }
    }
}

impl AbstractMonster {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        self.abstract_creature.set_index(index, value)
    }
}
impl Deref for AbstractMonster {
    type Target = AbstractCreature;
    fn deref(&self) -> &Self::Target {
        &self.abstract_creature
    }
}
impl DerefMut for AbstractMonster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_creature
    }
}

#[derive(Debug, Clone)]
pub struct AbstractTameable {
    pub abstract_animal: AbstractAnimal,
    pub tame: bool,
    pub in_sitting_pose: bool,
    pub owneruuid: Option<Uuid>,
}

impl AbstractTameable {
    pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {
        let abstract_animal = AbstractAnimal::read(metadata)?;
        let bitfield = metadata.pop_front()?.into_byte().ok()?;
        let tame = bitfield & 0x4 != 0;
        let in_sitting_pose = bitfield & 0x1 != 0;
        let owneruuid = metadata.pop_front()?.into_optional_uuid().ok()?;
        Some(Self {
            abstract_animal,
            tame,
            in_sitting_pose,
            owneruuid,
        })
    }

    pub fn write(&self) -> Vec<EntityDataValue> {
        let mut metadata = Vec::new();
        metadata.extend(self.abstract_animal.write());
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

impl Default for AbstractTameable {
    fn default() -> Self {
        Self {
            abstract_animal: Default::default(),
            tame: false,
            in_sitting_pose: false,
            owneruuid: None,
        }
    }
}

impl AbstractTameable {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match index {
            0..=16 => self.abstract_animal.set_index(index, value)?,
            17 => {
                let bitfield = value.into_byte().ok()?;
                self.tame = bitfield & 0x4 != 0;
                self.in_sitting_pose = bitfield & 0x1 != 0;
            }
            18 => self.owneruuid = value.into_optional_uuid().ok()?,
            _ => {}
        }
        Some(())
    }
}
impl Deref for AbstractTameable {
    type Target = AbstractAnimal;
    fn deref(&self) -> &Self::Target {
        &self.abstract_animal
    }
}
impl DerefMut for AbstractTameable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_animal
    }
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum EntityMetadata {
    Allay(Allay),
    AreaEffectCloud(AreaEffectCloud),
    ArmorStand(ArmorStand),
    Arrow(Arrow),
    Axolotl(Axolotl),
    Bat(Bat),
    Bee(Bee),
    Blaze(Blaze),
    Boat(Boat),
    Cat(Cat),
    CaveSpider(CaveSpider),
    ChestBoat(ChestBoat),
    ChestMinecart(ChestMinecart),
    Chicken(Chicken),
    Cod(Cod),
    CommandBlockMinecart(CommandBlockMinecart),
    Cow(Cow),
    Creeper(Creeper),
    Dolphin(Dolphin),
    Donkey(Donkey),
    DragonFireball(DragonFireball),
    Drowned(Drowned),
    Egg(Egg),
    ElderGuardian(ElderGuardian),
    EndCrystal(EndCrystal),
    EnderDragon(EnderDragon),
    EnderPearl(EnderPearl),
    Enderman(Enderman),
    Endermite(Endermite),
    Evoker(Evoker),
    EvokerFangs(EvokerFangs),
    ExperienceBottle(ExperienceBottle),
    ExperienceOrb(ExperienceOrb),
    EyeOfEnder(EyeOfEnder),
    FallingBlock(FallingBlock),
    Fireball(Fireball),
    FireworkRocket(FireworkRocket),
    FishingBobber(FishingBobber),
    Fox(Fox),
    Frog(Frog),
    FurnaceMinecart(FurnaceMinecart),
    Ghast(Ghast),
    Giant(Giant),
    GlowItemFrame(GlowItemFrame),
    GlowSquid(GlowSquid),
    Goat(Goat),
    Guardian(Guardian),
    Hoglin(Hoglin),
    HopperMinecart(HopperMinecart),
    Horse(Horse),
    Husk(Husk),
    Illusioner(Illusioner),
    IronGolem(IronGolem),
    Item(Item),
    ItemFrame(ItemFrame),
    LeashKnot(LeashKnot),
    LightningBolt(LightningBolt),
    Llama(Llama),
    LlamaSpit(LlamaSpit),
    MagmaCube(MagmaCube),
    Marker(Marker),
    Minecart(Minecart),
    Mooshroom(Mooshroom),
    Mule(Mule),
    Ocelot(Ocelot),
    Painting(Painting),
    Panda(Panda),
    Parrot(Parrot),
    Phantom(Phantom),
    Pig(Pig),
    Piglin(Piglin),
    PiglinBrute(PiglinBrute),
    Pillager(Pillager),
    Player(Player),
    PolarBear(PolarBear),
    Potion(Potion),
    Pufferfish(Pufferfish),
    Rabbit(Rabbit),
    Ravager(Ravager),
    Salmon(Salmon),
    Sheep(Sheep),
    Shulker(Shulker),
    ShulkerBullet(ShulkerBullet),
    Silverfish(Silverfish),
    Skeleton(Skeleton),
    SkeletonHorse(SkeletonHorse),
    Slime(Slime),
    SmallFireball(SmallFireball),
    SnowGolem(SnowGolem),
    Snowball(Snowball),
    SpawnerMinecart(SpawnerMinecart),
    SpectralArrow(SpectralArrow),
    Spider(Spider),
    Squid(Squid),
    Stray(Stray),
    Strider(Strider),
    Tadpole(Tadpole),
    Tnt(Tnt),
    TntMinecart(TntMinecart),
    TraderLlama(TraderLlama),
    Trident(Trident),
    TropicalFish(TropicalFish),
    Turtle(Turtle),
    Vex(Vex),
    Villager(Villager),
    Vindicator(Vindicator),
    WanderingTrader(WanderingTrader),
    Warden(Warden),
    Witch(Witch),
    Wither(Wither),
    WitherSkeleton(WitherSkeleton),
    WitherSkull(WitherSkull),
    Wolf(Wolf),
    Zoglin(Zoglin),
    Zombie(Zombie),
    ZombieHorse(ZombieHorse),
    ZombieVillager(ZombieVillager),
    ZombifiedPiglin(ZombifiedPiglin),
}

impl From<azalea_registry::EntityType> for EntityMetadata {
    fn from(value: azalea_registry::EntityType) -> Self {
        match value {
            azalea_registry::EntityType::Allay => EntityMetadata::Allay(Allay::default()),
            azalea_registry::EntityType::AreaEffectCloud => {
                EntityMetadata::AreaEffectCloud(AreaEffectCloud::default())
            }
            azalea_registry::EntityType::ArmorStand => {
                EntityMetadata::ArmorStand(ArmorStand::default())
            }
            azalea_registry::EntityType::Arrow => EntityMetadata::Arrow(Arrow::default()),
            azalea_registry::EntityType::Axolotl => EntityMetadata::Axolotl(Axolotl::default()),
            azalea_registry::EntityType::Bat => EntityMetadata::Bat(Bat::default()),
            azalea_registry::EntityType::Bee => EntityMetadata::Bee(Bee::default()),
            azalea_registry::EntityType::Blaze => EntityMetadata::Blaze(Blaze::default()),
            azalea_registry::EntityType::Boat => EntityMetadata::Boat(Boat::default()),
            azalea_registry::EntityType::Cat => EntityMetadata::Cat(Cat::default()),
            azalea_registry::EntityType::CaveSpider => {
                EntityMetadata::CaveSpider(CaveSpider::default())
            }
            azalea_registry::EntityType::ChestBoat => {
                EntityMetadata::ChestBoat(ChestBoat::default())
            }
            azalea_registry::EntityType::ChestMinecart => {
                EntityMetadata::ChestMinecart(ChestMinecart::default())
            }
            azalea_registry::EntityType::Chicken => EntityMetadata::Chicken(Chicken::default()),
            azalea_registry::EntityType::Cod => EntityMetadata::Cod(Cod::default()),
            azalea_registry::EntityType::CommandBlockMinecart => {
                EntityMetadata::CommandBlockMinecart(CommandBlockMinecart::default())
            }
            azalea_registry::EntityType::Cow => EntityMetadata::Cow(Cow::default()),
            azalea_registry::EntityType::Creeper => EntityMetadata::Creeper(Creeper::default()),
            azalea_registry::EntityType::Dolphin => EntityMetadata::Dolphin(Dolphin::default()),
            azalea_registry::EntityType::Donkey => EntityMetadata::Donkey(Donkey::default()),
            azalea_registry::EntityType::DragonFireball => {
                EntityMetadata::DragonFireball(DragonFireball::default())
            }
            azalea_registry::EntityType::Drowned => EntityMetadata::Drowned(Drowned::default()),
            azalea_registry::EntityType::Egg => EntityMetadata::Egg(Egg::default()),
            azalea_registry::EntityType::ElderGuardian => {
                EntityMetadata::ElderGuardian(ElderGuardian::default())
            }
            azalea_registry::EntityType::EndCrystal => {
                EntityMetadata::EndCrystal(EndCrystal::default())
            }
            azalea_registry::EntityType::EnderDragon => {
                EntityMetadata::EnderDragon(EnderDragon::default())
            }
            azalea_registry::EntityType::EnderPearl => {
                EntityMetadata::EnderPearl(EnderPearl::default())
            }
            azalea_registry::EntityType::Enderman => EntityMetadata::Enderman(Enderman::default()),
            azalea_registry::EntityType::Endermite => {
                EntityMetadata::Endermite(Endermite::default())
            }
            azalea_registry::EntityType::Evoker => EntityMetadata::Evoker(Evoker::default()),
            azalea_registry::EntityType::EvokerFangs => {
                EntityMetadata::EvokerFangs(EvokerFangs::default())
            }
            azalea_registry::EntityType::ExperienceBottle => {
                EntityMetadata::ExperienceBottle(ExperienceBottle::default())
            }
            azalea_registry::EntityType::ExperienceOrb => {
                EntityMetadata::ExperienceOrb(ExperienceOrb::default())
            }
            azalea_registry::EntityType::EyeOfEnder => {
                EntityMetadata::EyeOfEnder(EyeOfEnder::default())
            }
            azalea_registry::EntityType::FallingBlock => {
                EntityMetadata::FallingBlock(FallingBlock::default())
            }
            azalea_registry::EntityType::Fireball => EntityMetadata::Fireball(Fireball::default()),
            azalea_registry::EntityType::FireworkRocket => {
                EntityMetadata::FireworkRocket(FireworkRocket::default())
            }
            azalea_registry::EntityType::FishingBobber => {
                EntityMetadata::FishingBobber(FishingBobber::default())
            }
            azalea_registry::EntityType::Fox => EntityMetadata::Fox(Fox::default()),
            azalea_registry::EntityType::Frog => EntityMetadata::Frog(Frog::default()),
            azalea_registry::EntityType::FurnaceMinecart => {
                EntityMetadata::FurnaceMinecart(FurnaceMinecart::default())
            }
            azalea_registry::EntityType::Ghast => EntityMetadata::Ghast(Ghast::default()),
            azalea_registry::EntityType::Giant => EntityMetadata::Giant(Giant::default()),
            azalea_registry::EntityType::GlowItemFrame => {
                EntityMetadata::GlowItemFrame(GlowItemFrame::default())
            }
            azalea_registry::EntityType::GlowSquid => {
                EntityMetadata::GlowSquid(GlowSquid::default())
            }
            azalea_registry::EntityType::Goat => EntityMetadata::Goat(Goat::default()),
            azalea_registry::EntityType::Guardian => EntityMetadata::Guardian(Guardian::default()),
            azalea_registry::EntityType::Hoglin => EntityMetadata::Hoglin(Hoglin::default()),
            azalea_registry::EntityType::HopperMinecart => {
                EntityMetadata::HopperMinecart(HopperMinecart::default())
            }
            azalea_registry::EntityType::Horse => EntityMetadata::Horse(Horse::default()),
            azalea_registry::EntityType::Husk => EntityMetadata::Husk(Husk::default()),
            azalea_registry::EntityType::Illusioner => {
                EntityMetadata::Illusioner(Illusioner::default())
            }
            azalea_registry::EntityType::IronGolem => {
                EntityMetadata::IronGolem(IronGolem::default())
            }
            azalea_registry::EntityType::Item => EntityMetadata::Item(Item::default()),
            azalea_registry::EntityType::ItemFrame => {
                EntityMetadata::ItemFrame(ItemFrame::default())
            }
            azalea_registry::EntityType::LeashKnot => {
                EntityMetadata::LeashKnot(LeashKnot::default())
            }
            azalea_registry::EntityType::LightningBolt => {
                EntityMetadata::LightningBolt(LightningBolt::default())
            }
            azalea_registry::EntityType::Llama => EntityMetadata::Llama(Llama::default()),
            azalea_registry::EntityType::LlamaSpit => {
                EntityMetadata::LlamaSpit(LlamaSpit::default())
            }
            azalea_registry::EntityType::MagmaCube => {
                EntityMetadata::MagmaCube(MagmaCube::default())
            }
            azalea_registry::EntityType::Marker => EntityMetadata::Marker(Marker::default()),
            azalea_registry::EntityType::Minecart => EntityMetadata::Minecart(Minecart::default()),
            azalea_registry::EntityType::Mooshroom => {
                EntityMetadata::Mooshroom(Mooshroom::default())
            }
            azalea_registry::EntityType::Mule => EntityMetadata::Mule(Mule::default()),
            azalea_registry::EntityType::Ocelot => EntityMetadata::Ocelot(Ocelot::default()),
            azalea_registry::EntityType::Painting => EntityMetadata::Painting(Painting::default()),
            azalea_registry::EntityType::Panda => EntityMetadata::Panda(Panda::default()),
            azalea_registry::EntityType::Parrot => EntityMetadata::Parrot(Parrot::default()),
            azalea_registry::EntityType::Phantom => EntityMetadata::Phantom(Phantom::default()),
            azalea_registry::EntityType::Pig => EntityMetadata::Pig(Pig::default()),
            azalea_registry::EntityType::Piglin => EntityMetadata::Piglin(Piglin::default()),
            azalea_registry::EntityType::PiglinBrute => {
                EntityMetadata::PiglinBrute(PiglinBrute::default())
            }
            azalea_registry::EntityType::Pillager => EntityMetadata::Pillager(Pillager::default()),
            azalea_registry::EntityType::Player => EntityMetadata::Player(Player::default()),
            azalea_registry::EntityType::PolarBear => {
                EntityMetadata::PolarBear(PolarBear::default())
            }
            azalea_registry::EntityType::Potion => EntityMetadata::Potion(Potion::default()),
            azalea_registry::EntityType::Pufferfish => {
                EntityMetadata::Pufferfish(Pufferfish::default())
            }
            azalea_registry::EntityType::Rabbit => EntityMetadata::Rabbit(Rabbit::default()),
            azalea_registry::EntityType::Ravager => EntityMetadata::Ravager(Ravager::default()),
            azalea_registry::EntityType::Salmon => EntityMetadata::Salmon(Salmon::default()),
            azalea_registry::EntityType::Sheep => EntityMetadata::Sheep(Sheep::default()),
            azalea_registry::EntityType::Shulker => EntityMetadata::Shulker(Shulker::default()),
            azalea_registry::EntityType::ShulkerBullet => {
                EntityMetadata::ShulkerBullet(ShulkerBullet::default())
            }
            azalea_registry::EntityType::Silverfish => {
                EntityMetadata::Silverfish(Silverfish::default())
            }
            azalea_registry::EntityType::Skeleton => EntityMetadata::Skeleton(Skeleton::default()),
            azalea_registry::EntityType::SkeletonHorse => {
                EntityMetadata::SkeletonHorse(SkeletonHorse::default())
            }
            azalea_registry::EntityType::Slime => EntityMetadata::Slime(Slime::default()),
            azalea_registry::EntityType::SmallFireball => {
                EntityMetadata::SmallFireball(SmallFireball::default())
            }
            azalea_registry::EntityType::SnowGolem => {
                EntityMetadata::SnowGolem(SnowGolem::default())
            }
            azalea_registry::EntityType::Snowball => EntityMetadata::Snowball(Snowball::default()),
            azalea_registry::EntityType::SpawnerMinecart => {
                EntityMetadata::SpawnerMinecart(SpawnerMinecart::default())
            }
            azalea_registry::EntityType::SpectralArrow => {
                EntityMetadata::SpectralArrow(SpectralArrow::default())
            }
            azalea_registry::EntityType::Spider => EntityMetadata::Spider(Spider::default()),
            azalea_registry::EntityType::Squid => EntityMetadata::Squid(Squid::default()),
            azalea_registry::EntityType::Stray => EntityMetadata::Stray(Stray::default()),
            azalea_registry::EntityType::Strider => EntityMetadata::Strider(Strider::default()),
            azalea_registry::EntityType::Tadpole => EntityMetadata::Tadpole(Tadpole::default()),
            azalea_registry::EntityType::Tnt => EntityMetadata::Tnt(Tnt::default()),
            azalea_registry::EntityType::TntMinecart => {
                EntityMetadata::TntMinecart(TntMinecart::default())
            }
            azalea_registry::EntityType::TraderLlama => {
                EntityMetadata::TraderLlama(TraderLlama::default())
            }
            azalea_registry::EntityType::Trident => EntityMetadata::Trident(Trident::default()),
            azalea_registry::EntityType::TropicalFish => {
                EntityMetadata::TropicalFish(TropicalFish::default())
            }
            azalea_registry::EntityType::Turtle => EntityMetadata::Turtle(Turtle::default()),
            azalea_registry::EntityType::Vex => EntityMetadata::Vex(Vex::default()),
            azalea_registry::EntityType::Villager => EntityMetadata::Villager(Villager::default()),
            azalea_registry::EntityType::Vindicator => {
                EntityMetadata::Vindicator(Vindicator::default())
            }
            azalea_registry::EntityType::WanderingTrader => {
                EntityMetadata::WanderingTrader(WanderingTrader::default())
            }
            azalea_registry::EntityType::Warden => EntityMetadata::Warden(Warden::default()),
            azalea_registry::EntityType::Witch => EntityMetadata::Witch(Witch::default()),
            azalea_registry::EntityType::Wither => EntityMetadata::Wither(Wither::default()),
            azalea_registry::EntityType::WitherSkeleton => {
                EntityMetadata::WitherSkeleton(WitherSkeleton::default())
            }
            azalea_registry::EntityType::WitherSkull => {
                EntityMetadata::WitherSkull(WitherSkull::default())
            }
            azalea_registry::EntityType::Wolf => EntityMetadata::Wolf(Wolf::default()),
            azalea_registry::EntityType::Zoglin => EntityMetadata::Zoglin(Zoglin::default()),
            azalea_registry::EntityType::Zombie => EntityMetadata::Zombie(Zombie::default()),
            azalea_registry::EntityType::ZombieHorse => {
                EntityMetadata::ZombieHorse(ZombieHorse::default())
            }
            azalea_registry::EntityType::ZombieVillager => {
                EntityMetadata::ZombieVillager(ZombieVillager::default())
            }
            azalea_registry::EntityType::ZombifiedPiglin => {
                EntityMetadata::ZombifiedPiglin(ZombifiedPiglin::default())
            }
        }
    }
}

impl EntityMetadata {
    pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
        match self {
            EntityMetadata::Allay(entity) => entity.set_index(index, value),
            EntityMetadata::AreaEffectCloud(entity) => entity.set_index(index, value),
            EntityMetadata::ArmorStand(entity) => entity.set_index(index, value),
            EntityMetadata::Arrow(entity) => entity.set_index(index, value),
            EntityMetadata::Axolotl(entity) => entity.set_index(index, value),
            EntityMetadata::Bat(entity) => entity.set_index(index, value),
            EntityMetadata::Bee(entity) => entity.set_index(index, value),
            EntityMetadata::Blaze(entity) => entity.set_index(index, value),
            EntityMetadata::Boat(entity) => entity.set_index(index, value),
            EntityMetadata::Cat(entity) => entity.set_index(index, value),
            EntityMetadata::CaveSpider(entity) => entity.set_index(index, value),
            EntityMetadata::ChestBoat(entity) => entity.set_index(index, value),
            EntityMetadata::ChestMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::Chicken(entity) => entity.set_index(index, value),
            EntityMetadata::Cod(entity) => entity.set_index(index, value),
            EntityMetadata::CommandBlockMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::Cow(entity) => entity.set_index(index, value),
            EntityMetadata::Creeper(entity) => entity.set_index(index, value),
            EntityMetadata::Dolphin(entity) => entity.set_index(index, value),
            EntityMetadata::Donkey(entity) => entity.set_index(index, value),
            EntityMetadata::DragonFireball(entity) => entity.set_index(index, value),
            EntityMetadata::Drowned(entity) => entity.set_index(index, value),
            EntityMetadata::Egg(entity) => entity.set_index(index, value),
            EntityMetadata::ElderGuardian(entity) => entity.set_index(index, value),
            EntityMetadata::EndCrystal(entity) => entity.set_index(index, value),
            EntityMetadata::EnderDragon(entity) => entity.set_index(index, value),
            EntityMetadata::EnderPearl(entity) => entity.set_index(index, value),
            EntityMetadata::Enderman(entity) => entity.set_index(index, value),
            EntityMetadata::Endermite(entity) => entity.set_index(index, value),
            EntityMetadata::Evoker(entity) => entity.set_index(index, value),
            EntityMetadata::EvokerFangs(entity) => entity.set_index(index, value),
            EntityMetadata::ExperienceBottle(entity) => entity.set_index(index, value),
            EntityMetadata::ExperienceOrb(entity) => entity.set_index(index, value),
            EntityMetadata::EyeOfEnder(entity) => entity.set_index(index, value),
            EntityMetadata::FallingBlock(entity) => entity.set_index(index, value),
            EntityMetadata::Fireball(entity) => entity.set_index(index, value),
            EntityMetadata::FireworkRocket(entity) => entity.set_index(index, value),
            EntityMetadata::FishingBobber(entity) => entity.set_index(index, value),
            EntityMetadata::Fox(entity) => entity.set_index(index, value),
            EntityMetadata::Frog(entity) => entity.set_index(index, value),
            EntityMetadata::FurnaceMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::Ghast(entity) => entity.set_index(index, value),
            EntityMetadata::Giant(entity) => entity.set_index(index, value),
            EntityMetadata::GlowItemFrame(entity) => entity.set_index(index, value),
            EntityMetadata::GlowSquid(entity) => entity.set_index(index, value),
            EntityMetadata::Goat(entity) => entity.set_index(index, value),
            EntityMetadata::Guardian(entity) => entity.set_index(index, value),
            EntityMetadata::Hoglin(entity) => entity.set_index(index, value),
            EntityMetadata::HopperMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::Horse(entity) => entity.set_index(index, value),
            EntityMetadata::Husk(entity) => entity.set_index(index, value),
            EntityMetadata::Illusioner(entity) => entity.set_index(index, value),
            EntityMetadata::IronGolem(entity) => entity.set_index(index, value),
            EntityMetadata::Item(entity) => entity.set_index(index, value),
            EntityMetadata::ItemFrame(entity) => entity.set_index(index, value),
            EntityMetadata::LeashKnot(entity) => entity.set_index(index, value),
            EntityMetadata::LightningBolt(entity) => entity.set_index(index, value),
            EntityMetadata::Llama(entity) => entity.set_index(index, value),
            EntityMetadata::LlamaSpit(entity) => entity.set_index(index, value),
            EntityMetadata::MagmaCube(entity) => entity.set_index(index, value),
            EntityMetadata::Marker(entity) => entity.set_index(index, value),
            EntityMetadata::Minecart(entity) => entity.set_index(index, value),
            EntityMetadata::Mooshroom(entity) => entity.set_index(index, value),
            EntityMetadata::Mule(entity) => entity.set_index(index, value),
            EntityMetadata::Ocelot(entity) => entity.set_index(index, value),
            EntityMetadata::Painting(entity) => entity.set_index(index, value),
            EntityMetadata::Panda(entity) => entity.set_index(index, value),
            EntityMetadata::Parrot(entity) => entity.set_index(index, value),
            EntityMetadata::Phantom(entity) => entity.set_index(index, value),
            EntityMetadata::Pig(entity) => entity.set_index(index, value),
            EntityMetadata::Piglin(entity) => entity.set_index(index, value),
            EntityMetadata::PiglinBrute(entity) => entity.set_index(index, value),
            EntityMetadata::Pillager(entity) => entity.set_index(index, value),
            EntityMetadata::Player(entity) => entity.set_index(index, value),
            EntityMetadata::PolarBear(entity) => entity.set_index(index, value),
            EntityMetadata::Potion(entity) => entity.set_index(index, value),
            EntityMetadata::Pufferfish(entity) => entity.set_index(index, value),
            EntityMetadata::Rabbit(entity) => entity.set_index(index, value),
            EntityMetadata::Ravager(entity) => entity.set_index(index, value),
            EntityMetadata::Salmon(entity) => entity.set_index(index, value),
            EntityMetadata::Sheep(entity) => entity.set_index(index, value),
            EntityMetadata::Shulker(entity) => entity.set_index(index, value),
            EntityMetadata::ShulkerBullet(entity) => entity.set_index(index, value),
            EntityMetadata::Silverfish(entity) => entity.set_index(index, value),
            EntityMetadata::Skeleton(entity) => entity.set_index(index, value),
            EntityMetadata::SkeletonHorse(entity) => entity.set_index(index, value),
            EntityMetadata::Slime(entity) => entity.set_index(index, value),
            EntityMetadata::SmallFireball(entity) => entity.set_index(index, value),
            EntityMetadata::SnowGolem(entity) => entity.set_index(index, value),
            EntityMetadata::Snowball(entity) => entity.set_index(index, value),
            EntityMetadata::SpawnerMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::SpectralArrow(entity) => entity.set_index(index, value),
            EntityMetadata::Spider(entity) => entity.set_index(index, value),
            EntityMetadata::Squid(entity) => entity.set_index(index, value),
            EntityMetadata::Stray(entity) => entity.set_index(index, value),
            EntityMetadata::Strider(entity) => entity.set_index(index, value),
            EntityMetadata::Tadpole(entity) => entity.set_index(index, value),
            EntityMetadata::Tnt(entity) => entity.set_index(index, value),
            EntityMetadata::TntMinecart(entity) => entity.set_index(index, value),
            EntityMetadata::TraderLlama(entity) => entity.set_index(index, value),
            EntityMetadata::Trident(entity) => entity.set_index(index, value),
            EntityMetadata::TropicalFish(entity) => entity.set_index(index, value),
            EntityMetadata::Turtle(entity) => entity.set_index(index, value),
            EntityMetadata::Vex(entity) => entity.set_index(index, value),
            EntityMetadata::Villager(entity) => entity.set_index(index, value),
            EntityMetadata::Vindicator(entity) => entity.set_index(index, value),
            EntityMetadata::WanderingTrader(entity) => entity.set_index(index, value),
            EntityMetadata::Warden(entity) => entity.set_index(index, value),
            EntityMetadata::Witch(entity) => entity.set_index(index, value),
            EntityMetadata::Wither(entity) => entity.set_index(index, value),
            EntityMetadata::WitherSkeleton(entity) => entity.set_index(index, value),
            EntityMetadata::WitherSkull(entity) => entity.set_index(index, value),
            EntityMetadata::Wolf(entity) => entity.set_index(index, value),
            EntityMetadata::Zoglin(entity) => entity.set_index(index, value),
            EntityMetadata::Zombie(entity) => entity.set_index(index, value),
            EntityMetadata::ZombieHorse(entity) => entity.set_index(index, value),
            EntityMetadata::ZombieVillager(entity) => entity.set_index(index, value),
            EntityMetadata::ZombifiedPiglin(entity) => entity.set_index(index, value),
        }
    }
}

impl Deref for EntityMetadata {
    type Target = AbstractEntity;
    fn deref(&self) -> &Self::Target {
        match self {
            EntityMetadata::Allay(entity) => entity,
            EntityMetadata::AreaEffectCloud(entity) => entity,
            EntityMetadata::ArmorStand(entity) => entity,
            EntityMetadata::Arrow(entity) => entity,
            EntityMetadata::Axolotl(entity) => entity,
            EntityMetadata::Bat(entity) => entity,
            EntityMetadata::Bee(entity) => entity,
            EntityMetadata::Blaze(entity) => entity,
            EntityMetadata::Boat(entity) => entity,
            EntityMetadata::Cat(entity) => entity,
            EntityMetadata::CaveSpider(entity) => entity,
            EntityMetadata::ChestBoat(entity) => entity,
            EntityMetadata::ChestMinecart(entity) => entity,
            EntityMetadata::Chicken(entity) => entity,
            EntityMetadata::Cod(entity) => entity,
            EntityMetadata::CommandBlockMinecart(entity) => entity,
            EntityMetadata::Cow(entity) => entity,
            EntityMetadata::Creeper(entity) => entity,
            EntityMetadata::Dolphin(entity) => entity,
            EntityMetadata::Donkey(entity) => entity,
            EntityMetadata::DragonFireball(entity) => entity,
            EntityMetadata::Drowned(entity) => entity,
            EntityMetadata::Egg(entity) => entity,
            EntityMetadata::ElderGuardian(entity) => entity,
            EntityMetadata::EndCrystal(entity) => entity,
            EntityMetadata::EnderDragon(entity) => entity,
            EntityMetadata::EnderPearl(entity) => entity,
            EntityMetadata::Enderman(entity) => entity,
            EntityMetadata::Endermite(entity) => entity,
            EntityMetadata::Evoker(entity) => entity,
            EntityMetadata::EvokerFangs(entity) => entity,
            EntityMetadata::ExperienceBottle(entity) => entity,
            EntityMetadata::ExperienceOrb(entity) => entity,
            EntityMetadata::EyeOfEnder(entity) => entity,
            EntityMetadata::FallingBlock(entity) => entity,
            EntityMetadata::Fireball(entity) => entity,
            EntityMetadata::FireworkRocket(entity) => entity,
            EntityMetadata::FishingBobber(entity) => entity,
            EntityMetadata::Fox(entity) => entity,
            EntityMetadata::Frog(entity) => entity,
            EntityMetadata::FurnaceMinecart(entity) => entity,
            EntityMetadata::Ghast(entity) => entity,
            EntityMetadata::Giant(entity) => entity,
            EntityMetadata::GlowItemFrame(entity) => entity,
            EntityMetadata::GlowSquid(entity) => entity,
            EntityMetadata::Goat(entity) => entity,
            EntityMetadata::Guardian(entity) => entity,
            EntityMetadata::Hoglin(entity) => entity,
            EntityMetadata::HopperMinecart(entity) => entity,
            EntityMetadata::Horse(entity) => entity,
            EntityMetadata::Husk(entity) => entity,
            EntityMetadata::Illusioner(entity) => entity,
            EntityMetadata::IronGolem(entity) => entity,
            EntityMetadata::Item(entity) => entity,
            EntityMetadata::ItemFrame(entity) => entity,
            EntityMetadata::LeashKnot(entity) => entity,
            EntityMetadata::LightningBolt(entity) => entity,
            EntityMetadata::Llama(entity) => entity,
            EntityMetadata::LlamaSpit(entity) => entity,
            EntityMetadata::MagmaCube(entity) => entity,
            EntityMetadata::Marker(entity) => entity,
            EntityMetadata::Minecart(entity) => entity,
            EntityMetadata::Mooshroom(entity) => entity,
            EntityMetadata::Mule(entity) => entity,
            EntityMetadata::Ocelot(entity) => entity,
            EntityMetadata::Painting(entity) => entity,
            EntityMetadata::Panda(entity) => entity,
            EntityMetadata::Parrot(entity) => entity,
            EntityMetadata::Phantom(entity) => entity,
            EntityMetadata::Pig(entity) => entity,
            EntityMetadata::Piglin(entity) => entity,
            EntityMetadata::PiglinBrute(entity) => entity,
            EntityMetadata::Pillager(entity) => entity,
            EntityMetadata::Player(entity) => entity,
            EntityMetadata::PolarBear(entity) => entity,
            EntityMetadata::Potion(entity) => entity,
            EntityMetadata::Pufferfish(entity) => entity,
            EntityMetadata::Rabbit(entity) => entity,
            EntityMetadata::Ravager(entity) => entity,
            EntityMetadata::Salmon(entity) => entity,
            EntityMetadata::Sheep(entity) => entity,
            EntityMetadata::Shulker(entity) => entity,
            EntityMetadata::ShulkerBullet(entity) => entity,
            EntityMetadata::Silverfish(entity) => entity,
            EntityMetadata::Skeleton(entity) => entity,
            EntityMetadata::SkeletonHorse(entity) => entity,
            EntityMetadata::Slime(entity) => entity,
            EntityMetadata::SmallFireball(entity) => entity,
            EntityMetadata::SnowGolem(entity) => entity,
            EntityMetadata::Snowball(entity) => entity,
            EntityMetadata::SpawnerMinecart(entity) => entity,
            EntityMetadata::SpectralArrow(entity) => entity,
            EntityMetadata::Spider(entity) => entity,
            EntityMetadata::Squid(entity) => entity,
            EntityMetadata::Stray(entity) => entity,
            EntityMetadata::Strider(entity) => entity,
            EntityMetadata::Tadpole(entity) => entity,
            EntityMetadata::Tnt(entity) => entity,
            EntityMetadata::TntMinecart(entity) => entity,
            EntityMetadata::TraderLlama(entity) => entity,
            EntityMetadata::Trident(entity) => entity,
            EntityMetadata::TropicalFish(entity) => entity,
            EntityMetadata::Turtle(entity) => entity,
            EntityMetadata::Vex(entity) => entity,
            EntityMetadata::Villager(entity) => entity,
            EntityMetadata::Vindicator(entity) => entity,
            EntityMetadata::WanderingTrader(entity) => entity,
            EntityMetadata::Warden(entity) => entity,
            EntityMetadata::Witch(entity) => entity,
            EntityMetadata::Wither(entity) => entity,
            EntityMetadata::WitherSkeleton(entity) => entity,
            EntityMetadata::WitherSkull(entity) => entity,
            EntityMetadata::Wolf(entity) => entity,
            EntityMetadata::Zoglin(entity) => entity,
            EntityMetadata::Zombie(entity) => entity,
            EntityMetadata::ZombieHorse(entity) => entity,
            EntityMetadata::ZombieVillager(entity) => entity,
            EntityMetadata::ZombifiedPiglin(entity) => entity,
        }
    }
}
impl DerefMut for EntityMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            EntityMetadata::Allay(entity) => entity,
            EntityMetadata::AreaEffectCloud(entity) => entity,
            EntityMetadata::ArmorStand(entity) => entity,
            EntityMetadata::Arrow(entity) => entity,
            EntityMetadata::Axolotl(entity) => entity,
            EntityMetadata::Bat(entity) => entity,
            EntityMetadata::Bee(entity) => entity,
            EntityMetadata::Blaze(entity) => entity,
            EntityMetadata::Boat(entity) => entity,
            EntityMetadata::Cat(entity) => entity,
            EntityMetadata::CaveSpider(entity) => entity,
            EntityMetadata::ChestBoat(entity) => entity,
            EntityMetadata::ChestMinecart(entity) => entity,
            EntityMetadata::Chicken(entity) => entity,
            EntityMetadata::Cod(entity) => entity,
            EntityMetadata::CommandBlockMinecart(entity) => entity,
            EntityMetadata::Cow(entity) => entity,
            EntityMetadata::Creeper(entity) => entity,
            EntityMetadata::Dolphin(entity) => entity,
            EntityMetadata::Donkey(entity) => entity,
            EntityMetadata::DragonFireball(entity) => entity,
            EntityMetadata::Drowned(entity) => entity,
            EntityMetadata::Egg(entity) => entity,
            EntityMetadata::ElderGuardian(entity) => entity,
            EntityMetadata::EndCrystal(entity) => entity,
            EntityMetadata::EnderDragon(entity) => entity,
            EntityMetadata::EnderPearl(entity) => entity,
            EntityMetadata::Enderman(entity) => entity,
            EntityMetadata::Endermite(entity) => entity,
            EntityMetadata::Evoker(entity) => entity,
            EntityMetadata::EvokerFangs(entity) => entity,
            EntityMetadata::ExperienceBottle(entity) => entity,
            EntityMetadata::ExperienceOrb(entity) => entity,
            EntityMetadata::EyeOfEnder(entity) => entity,
            EntityMetadata::FallingBlock(entity) => entity,
            EntityMetadata::Fireball(entity) => entity,
            EntityMetadata::FireworkRocket(entity) => entity,
            EntityMetadata::FishingBobber(entity) => entity,
            EntityMetadata::Fox(entity) => entity,
            EntityMetadata::Frog(entity) => entity,
            EntityMetadata::FurnaceMinecart(entity) => entity,
            EntityMetadata::Ghast(entity) => entity,
            EntityMetadata::Giant(entity) => entity,
            EntityMetadata::GlowItemFrame(entity) => entity,
            EntityMetadata::GlowSquid(entity) => entity,
            EntityMetadata::Goat(entity) => entity,
            EntityMetadata::Guardian(entity) => entity,
            EntityMetadata::Hoglin(entity) => entity,
            EntityMetadata::HopperMinecart(entity) => entity,
            EntityMetadata::Horse(entity) => entity,
            EntityMetadata::Husk(entity) => entity,
            EntityMetadata::Illusioner(entity) => entity,
            EntityMetadata::IronGolem(entity) => entity,
            EntityMetadata::Item(entity) => entity,
            EntityMetadata::ItemFrame(entity) => entity,
            EntityMetadata::LeashKnot(entity) => entity,
            EntityMetadata::LightningBolt(entity) => entity,
            EntityMetadata::Llama(entity) => entity,
            EntityMetadata::LlamaSpit(entity) => entity,
            EntityMetadata::MagmaCube(entity) => entity,
            EntityMetadata::Marker(entity) => entity,
            EntityMetadata::Minecart(entity) => entity,
            EntityMetadata::Mooshroom(entity) => entity,
            EntityMetadata::Mule(entity) => entity,
            EntityMetadata::Ocelot(entity) => entity,
            EntityMetadata::Painting(entity) => entity,
            EntityMetadata::Panda(entity) => entity,
            EntityMetadata::Parrot(entity) => entity,
            EntityMetadata::Phantom(entity) => entity,
            EntityMetadata::Pig(entity) => entity,
            EntityMetadata::Piglin(entity) => entity,
            EntityMetadata::PiglinBrute(entity) => entity,
            EntityMetadata::Pillager(entity) => entity,
            EntityMetadata::Player(entity) => entity,
            EntityMetadata::PolarBear(entity) => entity,
            EntityMetadata::Potion(entity) => entity,
            EntityMetadata::Pufferfish(entity) => entity,
            EntityMetadata::Rabbit(entity) => entity,
            EntityMetadata::Ravager(entity) => entity,
            EntityMetadata::Salmon(entity) => entity,
            EntityMetadata::Sheep(entity) => entity,
            EntityMetadata::Shulker(entity) => entity,
            EntityMetadata::ShulkerBullet(entity) => entity,
            EntityMetadata::Silverfish(entity) => entity,
            EntityMetadata::Skeleton(entity) => entity,
            EntityMetadata::SkeletonHorse(entity) => entity,
            EntityMetadata::Slime(entity) => entity,
            EntityMetadata::SmallFireball(entity) => entity,
            EntityMetadata::SnowGolem(entity) => entity,
            EntityMetadata::Snowball(entity) => entity,
            EntityMetadata::SpawnerMinecart(entity) => entity,
            EntityMetadata::SpectralArrow(entity) => entity,
            EntityMetadata::Spider(entity) => entity,
            EntityMetadata::Squid(entity) => entity,
            EntityMetadata::Stray(entity) => entity,
            EntityMetadata::Strider(entity) => entity,
            EntityMetadata::Tadpole(entity) => entity,
            EntityMetadata::Tnt(entity) => entity,
            EntityMetadata::TntMinecart(entity) => entity,
            EntityMetadata::TraderLlama(entity) => entity,
            EntityMetadata::Trident(entity) => entity,
            EntityMetadata::TropicalFish(entity) => entity,
            EntityMetadata::Turtle(entity) => entity,
            EntityMetadata::Vex(entity) => entity,
            EntityMetadata::Villager(entity) => entity,
            EntityMetadata::Vindicator(entity) => entity,
            EntityMetadata::WanderingTrader(entity) => entity,
            EntityMetadata::Warden(entity) => entity,
            EntityMetadata::Witch(entity) => entity,
            EntityMetadata::Wither(entity) => entity,
            EntityMetadata::WitherSkeleton(entity) => entity,
            EntityMetadata::WitherSkull(entity) => entity,
            EntityMetadata::Wolf(entity) => entity,
            EntityMetadata::Zoglin(entity) => entity,
            EntityMetadata::Zombie(entity) => entity,
            EntityMetadata::ZombieHorse(entity) => entity,
            EntityMetadata::ZombieVillager(entity) => entity,
            EntityMetadata::ZombifiedPiglin(entity) => entity,
        }
    }
}
