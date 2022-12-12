// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

#![allow(clippy::clone_on_copy, clippy::derivable_impls)]
use super::{
    EntityDataValue, EntityMetadataItems, OptionalUnsignedInt, Pose, Rotations, VillagerData,
};
use azalea_block::BlockState;
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, Particle, Slot};
use hecs::{BuiltEntity, EntityBuilder, Query};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UpdateMetadataError {
    #[error("Wrong type ({0:?})")]
    WrongType(EntityDataValue),
}
impl From<EntityDataValue> for UpdateMetadataError {
    fn from(value: EntityDataValue) -> Self {
        Self::WrongType(value)
    }
}

pub struct OnFire(pub bool);
pub struct ShiftKeyDown(pub bool);
pub struct Sprinting(pub bool);
pub struct Swimming(pub bool);
pub struct CurrentlyGlowing(pub bool);
pub struct Invisible(pub bool);
pub struct FallFlying(pub bool);
pub struct AirSupply(pub i32);
pub struct CustomName(pub Option<Component>);
pub struct CustomNameVisible(pub bool);
pub struct Silent(pub bool);
pub struct NoGravity(pub bool);
pub struct TicksFrozen(pub i32);
pub struct AutoSpinAttack(pub bool);
pub struct AbstractLivingUsingItem(pub bool);
pub struct Health(pub f32);
pub struct AbstractLivingEffectColor(pub i32);
pub struct EffectAmbience(pub bool);
pub struct ArrowCount(pub i32);
pub struct StingerCount(pub i32);
pub struct SleepingPos(pub Option<BlockPos>);
pub struct NoAi(pub bool);
pub struct LeftHanded(pub bool);
pub struct Aggressive(pub bool);
pub struct Dancing(pub bool);
pub struct CanDuplicate(pub bool);
pub struct Allay;
impl Allay {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Dancing(false))
            .add(CanDuplicate(true))
            .build()
    }
}

#[derive(Query)]
struct AllayQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    dancing: &'a mut Dancing,
    can_duplicate: &'a mut CanDuplicate,
}
impl AllayQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.dancing = Dancing(d.value.into_boolean()?),
                17 => *self.can_duplicate = CanDuplicate(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Radius(pub f32);
pub struct AreaEffectCloudColor(pub i32);
pub struct Waiting(pub bool);
pub struct AreaEffectCloud;
impl AreaEffectCloud {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(Radius(3.0))
            .add(AreaEffectCloudColor(0))
            .add(Waiting(false))
            .add(Particle::default())
            .build()
    }
}

#[derive(Query)]
struct AreaEffectCloudQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    radius: &'a mut Radius,
    area_effect_cloud_color: &'a mut AreaEffectCloudColor,
    waiting: &'a mut Waiting,
    particle: &'a mut Particle,
}
impl AreaEffectCloudQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.radius = Radius(d.value.into_float()?),
                9 => *self.area_effect_cloud_color = AreaEffectCloudColor(d.value.into_int()?),
                10 => *self.waiting = Waiting(d.value.into_boolean()?),
                11 => *self.particle = d.value.into_particle()?,
            }
        }
        Ok(())
    }
}

pub struct Small(pub bool);
pub struct ShowArms(pub bool);
pub struct NoBasePlate(pub bool);
pub struct ArmorStandMarker(pub bool);
pub struct HeadPose(pub Rotations);
pub struct BodyPose(pub Rotations);
pub struct LeftArmPose(pub Rotations);
pub struct RightArmPose(pub Rotations);
pub struct LeftLegPose(pub Rotations);
pub struct RightLegPose(pub Rotations);
pub struct ArmorStand;
impl ArmorStand {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(Small(false))
            .add(ShowArms(false))
            .add(NoBasePlate(false))
            .add(ArmorStandMarker(false))
            .add(HeadPose(Default::default()))
            .add(BodyPose(Default::default()))
            .add(LeftArmPose(Default::default()))
            .add(RightArmPose(Default::default()))
            .add(LeftLegPose(Default::default()))
            .add(RightLegPose(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct ArmorStandQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    small: &'a mut Small,
    show_arms: &'a mut ShowArms,
    no_base_plate: &'a mut NoBasePlate,
    armor_stand_marker: &'a mut ArmorStandMarker,
    head_pose: &'a mut HeadPose,
    body_pose: &'a mut BodyPose,
    left_arm_pose: &'a mut LeftArmPose,
    right_arm_pose: &'a mut RightArmPose,
    left_leg_pose: &'a mut LeftLegPose,
    right_leg_pose: &'a mut RightLegPose,
}
impl ArmorStandQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.small = Small(bitfield & 0x1 != 0);
                    *self.show_arms = ShowArms(bitfield & 0x4 != 0);
                    *self.no_base_plate = NoBasePlate(bitfield & 0x8 != 0);
                    *self.armor_stand_marker = ArmorStandMarker(bitfield & 0x10 != 0);
                }
                16 => *self.head_pose = HeadPose(d.value.into_rotations()?),
                17 => *self.body_pose = BodyPose(d.value.into_rotations()?),
                18 => *self.left_arm_pose = LeftArmPose(d.value.into_rotations()?),
                19 => *self.right_arm_pose = RightArmPose(d.value.into_rotations()?),
                20 => *self.left_leg_pose = LeftLegPose(d.value.into_rotations()?),
                21 => *self.right_leg_pose = RightLegPose(d.value.into_rotations()?),
            }
        }
        Ok(())
    }
}

pub struct ArrowCritArrow(pub bool);
pub struct ArrowShotFromCrossbow(pub bool);
pub struct ArrowNoPhysics(pub bool);
pub struct ArrowPierceLevel(pub u8);
pub struct ArrowEffectColor(pub i32);
pub struct Arrow;
impl Arrow {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(ArrowCritArrow(false))
            .add(ArrowShotFromCrossbow(false))
            .add(ArrowNoPhysics(false))
            .add(ArrowPierceLevel(0))
            .add(ArrowEffectColor(-1))
            .build()
    }
}

#[derive(Query)]
struct ArrowQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    arrow_crit_arrow: &'a mut ArrowCritArrow,
    arrow_shot_from_crossbow: &'a mut ArrowShotFromCrossbow,
    arrow_no_physics: &'a mut ArrowNoPhysics,
    arrow_pierce_level: &'a mut ArrowPierceLevel,
    arrow_effect_color: &'a mut ArrowEffectColor,
}
impl ArrowQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.arrow_crit_arrow = ArrowCritArrow(bitfield & 0x1 != 0);
                    *self.arrow_shot_from_crossbow = ArrowShotFromCrossbow(bitfield & 0x4 != 0);
                    *self.arrow_no_physics = ArrowNoPhysics(bitfield & 0x2 != 0);
                }
                9 => *self.arrow_pierce_level = ArrowPierceLevel(d.value.into_byte()?),
                10 => *self.arrow_effect_color = ArrowEffectColor(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct AbstractAgeableBaby(pub bool);
pub struct AxolotlVariant(pub i32);
pub struct PlayingDead(pub bool);
pub struct AxolotlFromBucket(pub bool);
pub struct Axolotl;
impl Axolotl {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(AxolotlVariant(0))
            .add(PlayingDead(false))
            .add(AxolotlFromBucket(false))
            .build()
    }
}

#[derive(Query)]
struct AxolotlQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    axolotl_variant: &'a mut AxolotlVariant,
    playing_dead: &'a mut PlayingDead,
    axolotl_from_bucket: &'a mut AxolotlFromBucket,
}
impl AxolotlQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.axolotl_variant = AxolotlVariant(d.value.into_int()?),
                18 => *self.playing_dead = PlayingDead(d.value.into_boolean()?),
                19 => *self.axolotl_from_bucket = AxolotlFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Resting(pub bool);
pub struct Bat;
impl Bat {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Resting(false))
            .build()
    }
}

#[derive(Query)]
struct BatQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    resting: &'a mut Resting,
}
impl BatQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.resting = Resting(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct HasNectar(pub bool);
pub struct HasStung(pub bool);
pub struct BeeRolling(pub bool);
pub struct BeeRemainingAngerTime(pub i32);
pub struct Bee;
impl Bee {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(HasNectar(false))
            .add(HasStung(false))
            .add(BeeRolling(false))
            .add(BeeRemainingAngerTime(0))
            .build()
    }
}

#[derive(Query)]
struct BeeQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    has_nectar: &'a mut HasNectar,
    has_stung: &'a mut HasStung,
    bee_rolling: &'a mut BeeRolling,
    bee_remaining_anger_time: &'a mut BeeRemainingAngerTime,
}
impl BeeQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.has_nectar = HasNectar(bitfield & 0x8 != 0);
                    *self.has_stung = HasStung(bitfield & 0x4 != 0);
                    *self.bee_rolling = BeeRolling(bitfield & 0x2 != 0);
                }
                18 => *self.bee_remaining_anger_time = BeeRemainingAngerTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Charged(pub bool);
pub struct Blaze;
impl Blaze {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Charged(false))
            .build()
    }
}

#[derive(Query)]
struct BlazeQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    charged: &'a mut Charged,
}
impl BlazeQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.charged = Charged(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct BoatHurt(pub i32);
pub struct BoatHurtdir(pub i32);
pub struct BoatDamage(pub f32);
pub struct BoatKind(pub i32);
pub struct PaddleLeft(pub bool);
pub struct PaddleRight(pub bool);
pub struct BubbleTime(pub i32);
pub struct Boat;
impl Boat {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(BoatHurt(0))
            .add(BoatHurtdir(1))
            .add(BoatDamage(0.0))
            .add(BoatKind(Default::default()))
            .add(PaddleLeft(false))
            .add(PaddleRight(false))
            .add(BubbleTime(0))
            .build()
    }
}

#[derive(Query)]
struct BoatQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    boat_hurt: &'a mut BoatHurt,
    boat_hurtdir: &'a mut BoatHurtdir,
    boat_damage: &'a mut BoatDamage,
    boat_kind: &'a mut BoatKind,
    paddle_left: &'a mut PaddleLeft,
    paddle_right: &'a mut PaddleRight,
    bubble_time: &'a mut BubbleTime,
}
impl BoatQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.boat_hurt = BoatHurt(d.value.into_int()?),
                9 => *self.boat_hurtdir = BoatHurtdir(d.value.into_int()?),
                10 => *self.boat_damage = BoatDamage(d.value.into_float()?),
                11 => *self.boat_kind = BoatKind(d.value.into_int()?),
                12 => *self.paddle_left = PaddleLeft(d.value.into_boolean()?),
                13 => *self.paddle_right = PaddleRight(d.value.into_boolean()?),
                14 => *self.bubble_time = BubbleTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct CamelTamed(pub bool);
pub struct CamelEating(pub bool);
pub struct CamelStanding(pub bool);
pub struct CamelBred(pub bool);
pub struct CamelSaddled(pub bool);
pub struct CamelOwnerUuid(pub Option<Uuid>);
pub struct Dash(pub bool);
pub struct LastPoseChangeTick(pub i64);
pub struct Camel;
impl Camel {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(CamelTamed(false))
            .add(CamelEating(false))
            .add(CamelStanding(false))
            .add(CamelBred(false))
            .add(CamelSaddled(false))
            .add(CamelOwnerUuid(None))
            .add(Dash(false))
            .add(LastPoseChangeTick(-52))
            .build()
    }
}

#[derive(Query)]
struct CamelQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    camel_tamed: &'a mut CamelTamed,
    camel_eating: &'a mut CamelEating,
    camel_standing: &'a mut CamelStanding,
    camel_bred: &'a mut CamelBred,
    camel_saddled: &'a mut CamelSaddled,
    camel_owner_uuid: &'a mut CamelOwnerUuid,
    dash: &'a mut Dash,
    last_pose_change_tick: &'a mut LastPoseChangeTick,
}
impl CamelQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.camel_tamed = CamelTamed(bitfield & 0x2 != 0);
                    *self.camel_eating = CamelEating(bitfield & 0x10 != 0);
                    *self.camel_standing = CamelStanding(bitfield & 0x20 != 0);
                    *self.camel_bred = CamelBred(bitfield & 0x8 != 0);
                    *self.camel_saddled = CamelSaddled(bitfield & 0x4 != 0);
                }
                18 => *self.camel_owner_uuid = CamelOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.dash = Dash(d.value.into_boolean()?),
                20 => *self.last_pose_change_tick = LastPoseChangeTick(d.value.into_long()?),
            }
        }
        Ok(())
    }
}

pub struct Tame(pub bool);
pub struct InSittingPose(pub bool);
pub struct Owneruuid(pub Option<Uuid>);
pub struct CatVariant(pub azalea_registry::CatVariant);
pub struct IsLying(pub bool);
pub struct RelaxStateOne(pub bool);
pub struct CatCollarColor(pub i32);
pub struct Cat;
impl Cat {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(Tame(false))
            .add(InSittingPose(false))
            .add(Owneruuid(None))
            .add(CatVariant(azalea_registry::CatVariant::Tabby))
            .add(IsLying(false))
            .add(RelaxStateOne(false))
            .add(CatCollarColor(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct CatQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    tame: &'a mut Tame,
    in_sitting_pose: &'a mut InSittingPose,
    owneruuid: &'a mut Owneruuid,
    cat_variant: &'a mut CatVariant,
    is_lying: &'a mut IsLying,
    relax_state_one: &'a mut RelaxStateOne,
    cat_collar_color: &'a mut CatCollarColor,
}
impl CatQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.tame = Tame(bitfield & 0x4 != 0);
                    *self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => *self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => *self.cat_variant = CatVariant(d.value.into_cat_variant()?),
                20 => *self.is_lying = IsLying(d.value.into_boolean()?),
                21 => *self.relax_state_one = RelaxStateOne(d.value.into_boolean()?),
                22 => *self.cat_collar_color = CatCollarColor(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Climbing(pub bool);
pub struct CaveSpider;
impl CaveSpider {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Climbing(false))
            .build()
    }
}

#[derive(Query)]
struct CaveSpiderQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    climbing: &'a mut Climbing,
}
impl CaveSpiderQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.climbing = Climbing(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct ChestBoat;
impl ChestBoat {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(BoatHurt(0))
            .add(BoatHurtdir(1))
            .add(BoatDamage(0.0))
            .add(BoatKind(Default::default()))
            .add(PaddleLeft(false))
            .add(PaddleRight(false))
            .add(BubbleTime(0))
            .build()
    }
}

#[derive(Query)]
struct ChestBoatQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    boat_hurt: &'a mut BoatHurt,
    boat_hurtdir: &'a mut BoatHurtdir,
    boat_damage: &'a mut BoatDamage,
    boat_kind: &'a mut BoatKind,
    paddle_left: &'a mut PaddleLeft,
    paddle_right: &'a mut PaddleRight,
    bubble_time: &'a mut BubbleTime,
}
impl ChestBoatQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.boat_hurt = BoatHurt(d.value.into_int()?),
                9 => *self.boat_hurtdir = BoatHurtdir(d.value.into_int()?),
                10 => *self.boat_damage = BoatDamage(d.value.into_float()?),
                11 => *self.boat_kind = BoatKind(d.value.into_int()?),
                12 => *self.paddle_left = PaddleLeft(d.value.into_boolean()?),
                13 => *self.paddle_right = PaddleRight(d.value.into_boolean()?),
                14 => *self.bubble_time = BubbleTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct AbstractMinecartHurt(pub i32);
pub struct AbstractMinecartHurtdir(pub i32);
pub struct AbstractMinecartDamage(pub f32);
pub struct DisplayBlock(pub i32);
pub struct DisplayOffset(pub i32);
pub struct CustomDisplay(pub bool);
pub struct ChestMinecart;
impl ChestMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .build()
    }
}

#[derive(Query)]
struct ChestMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
}
impl ChestMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Chicken;
impl Chicken {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .build()
    }
}

#[derive(Query)]
struct ChickenQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
}
impl ChickenQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct CodFromBucket(pub bool);
pub struct Cod;
impl Cod {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(CodFromBucket(false))
            .build()
    }
}

#[derive(Query)]
struct CodQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    cod_from_bucket: &'a mut CodFromBucket,
}
impl CodQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.cod_from_bucket = CodFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct CommandName(pub String);
pub struct LastOutput(pub Component);
pub struct CommandBlockMinecart;
impl CommandBlockMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .add(CommandName("".to_string()))
            .add(LastOutput(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct CommandBlockMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
    command_name: &'a mut CommandName,
    last_output: &'a mut LastOutput,
}
impl CommandBlockMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
                14 => *self.command_name = CommandName(d.value.into_string()?),
                15 => *self.last_output = LastOutput(d.value.into_component()?),
            }
        }
        Ok(())
    }
}

pub struct Cow;
impl Cow {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .build()
    }
}

#[derive(Query)]
struct CowQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
}
impl CowQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct SwellDir(pub i32);
pub struct IsPowered(pub bool);
pub struct IsIgnited(pub bool);
pub struct Creeper;
impl Creeper {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(SwellDir(-1))
            .add(IsPowered(false))
            .add(IsIgnited(false))
            .build()
    }
}

#[derive(Query)]
struct CreeperQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    swell_dir: &'a mut SwellDir,
    is_powered: &'a mut IsPowered,
    is_ignited: &'a mut IsIgnited,
}
impl CreeperQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.swell_dir = SwellDir(d.value.into_int()?),
                17 => *self.is_powered = IsPowered(d.value.into_boolean()?),
                18 => *self.is_ignited = IsIgnited(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct TreasurePos(pub BlockPos);
pub struct GotFish(pub bool);
pub struct MoistnessLevel(pub i32);
pub struct Dolphin;
impl Dolphin {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(TreasurePos(BlockPos::new(0, 0, 0)))
            .add(GotFish(false))
            .add(MoistnessLevel(2400))
            .build()
    }
}

#[derive(Query)]
struct DolphinQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    treasure_pos: &'a mut TreasurePos,
    got_fish: &'a mut GotFish,
    moistness_level: &'a mut MoistnessLevel,
}
impl DolphinQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.treasure_pos = TreasurePos(d.value.into_block_pos()?),
                17 => *self.got_fish = GotFish(d.value.into_boolean()?),
                18 => *self.moistness_level = MoistnessLevel(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct DonkeyTamed(pub bool);
pub struct DonkeyEating(pub bool);
pub struct DonkeyStanding(pub bool);
pub struct DonkeyBred(pub bool);
pub struct DonkeySaddled(pub bool);
pub struct DonkeyOwnerUuid(pub Option<Uuid>);
pub struct DonkeyChest(pub bool);
pub struct Donkey;
impl Donkey {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(DonkeyTamed(false))
            .add(DonkeyEating(false))
            .add(DonkeyStanding(false))
            .add(DonkeyBred(false))
            .add(DonkeySaddled(false))
            .add(DonkeyOwnerUuid(None))
            .add(DonkeyChest(false))
            .build()
    }
}

#[derive(Query)]
struct DonkeyQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    donkey_tamed: &'a mut DonkeyTamed,
    donkey_eating: &'a mut DonkeyEating,
    donkey_standing: &'a mut DonkeyStanding,
    donkey_bred: &'a mut DonkeyBred,
    donkey_saddled: &'a mut DonkeySaddled,
    donkey_owner_uuid: &'a mut DonkeyOwnerUuid,
    donkey_chest: &'a mut DonkeyChest,
}
impl DonkeyQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.donkey_tamed = DonkeyTamed(bitfield & 0x2 != 0);
                    *self.donkey_eating = DonkeyEating(bitfield & 0x10 != 0);
                    *self.donkey_standing = DonkeyStanding(bitfield & 0x20 != 0);
                    *self.donkey_bred = DonkeyBred(bitfield & 0x8 != 0);
                    *self.donkey_saddled = DonkeySaddled(bitfield & 0x4 != 0);
                }
                18 => *self.donkey_owner_uuid = DonkeyOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.donkey_chest = DonkeyChest(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct DragonFireball;
impl DragonFireball {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct DragonFireballQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl DragonFireballQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct ZombieBaby(pub bool);
pub struct SpecialType(pub i32);
pub struct DrownedConversion(pub bool);
pub struct Drowned;
impl Drowned {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZombieBaby(false))
            .add(SpecialType(0))
            .add(DrownedConversion(false))
            .build()
    }
}

#[derive(Query)]
struct DrownedQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zombie_baby: &'a mut ZombieBaby,
    special_type: &'a mut SpecialType,
    drowned_conversion: &'a mut DrownedConversion,
}
impl DrownedQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => *self.special_type = SpecialType(d.value.into_int()?),
                18 => *self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct EggItemStack(pub Slot);
pub struct Egg;
impl Egg {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(EggItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct EggQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    egg_item_stack: &'a mut EggItemStack,
}
impl EggQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.egg_item_stack = EggItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct Moving(pub bool);
pub struct AttackTarget(pub i32);
pub struct ElderGuardian;
impl ElderGuardian {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Moving(false))
            .add(AttackTarget(0))
            .build()
    }
}

#[derive(Query)]
struct ElderGuardianQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    moving: &'a mut Moving,
    attack_target: &'a mut AttackTarget,
}
impl ElderGuardianQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.moving = Moving(d.value.into_boolean()?),
                17 => *self.attack_target = AttackTarget(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct BeamTarget(pub Option<BlockPos>);
pub struct ShowBottom(pub bool);
pub struct EndCrystal;
impl EndCrystal {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(BeamTarget(None))
            .add(ShowBottom(true))
            .build()
    }
}

#[derive(Query)]
struct EndCrystalQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    beam_target: &'a mut BeamTarget,
    show_bottom: &'a mut ShowBottom,
}
impl EndCrystalQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.beam_target = BeamTarget(d.value.into_optional_block_pos()?),
                9 => *self.show_bottom = ShowBottom(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Phase(pub i32);
pub struct EnderDragon;
impl EnderDragon {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Phase(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct EnderDragonQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    phase: &'a mut Phase,
}
impl EnderDragonQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.phase = Phase(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct EnderPearlItemStack(pub Slot);
pub struct EnderPearl;
impl EnderPearl {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(EnderPearlItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct EnderPearlQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    ender_pearl_item_stack: &'a mut EnderPearlItemStack,
}
impl EnderPearlQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.ender_pearl_item_stack = EnderPearlItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct CarryState(pub BlockState);
pub struct Creepy(pub bool);
pub struct StaredAt(pub bool);
pub struct Enderman;
impl Enderman {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(CarryState(BlockState::Air))
            .add(Creepy(false))
            .add(StaredAt(false))
            .build()
    }
}

#[derive(Query)]
struct EndermanQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    carry_state: &'a mut CarryState,
    creepy: &'a mut Creepy,
    stared_at: &'a mut StaredAt,
}
impl EndermanQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.carry_state = CarryState(d.value.into_block_state()?),
                17 => *self.creepy = Creepy(d.value.into_boolean()?),
                18 => *self.stared_at = StaredAt(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Endermite;
impl Endermite {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct EndermiteQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl EndermiteQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct EvokerIsCelebrating(pub bool);
pub struct EvokerSpellCasting(pub u8);
pub struct Evoker;
impl Evoker {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(EvokerIsCelebrating(false))
            .add(EvokerSpellCasting(0))
            .build()
    }
}

#[derive(Query)]
struct EvokerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    evoker_is_celebrating: &'a mut EvokerIsCelebrating,
    evoker_spell_casting: &'a mut EvokerSpellCasting,
}
impl EvokerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.evoker_is_celebrating = EvokerIsCelebrating(d.value.into_boolean()?),
                17 => *self.evoker_spell_casting = EvokerSpellCasting(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

pub struct EvokerFangs;
impl EvokerFangs {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct EvokerFangsQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl EvokerFangsQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct ExperienceBottleItemStack(pub Slot);
pub struct ExperienceBottle;
impl ExperienceBottle {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(ExperienceBottleItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct ExperienceBottleQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    experience_bottle_item_stack: &'a mut ExperienceBottleItemStack,
}
impl ExperienceBottleQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    *self.experience_bottle_item_stack =
                        ExperienceBottleItemStack(d.value.into_item_stack()?)
                }
            }
        }
        Ok(())
    }
}

pub struct ExperienceOrb;
impl ExperienceOrb {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct ExperienceOrbQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl ExperienceOrbQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct EyeOfEnderItemStack(pub Slot);
pub struct EyeOfEnder;
impl EyeOfEnder {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(EyeOfEnderItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct EyeOfEnderQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    eye_of_ender_item_stack: &'a mut EyeOfEnderItemStack,
}
impl EyeOfEnderQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    *self.eye_of_ender_item_stack = EyeOfEnderItemStack(d.value.into_item_stack()?)
                }
            }
        }
        Ok(())
    }
}

pub struct StartPos(pub BlockPos);
pub struct FallingBlock;
impl FallingBlock {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(StartPos(BlockPos::new(0, 0, 0)))
            .build()
    }
}

#[derive(Query)]
struct FallingBlockQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    start_pos: &'a mut StartPos,
}
impl FallingBlockQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.start_pos = StartPos(d.value.into_block_pos()?),
            }
        }
        Ok(())
    }
}

pub struct FireballItemStack(pub Slot);
pub struct Fireball;
impl Fireball {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(FireballItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct FireballQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    fireball_item_stack: &'a mut FireballItemStack,
}
impl FireballQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.fireball_item_stack = FireballItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct FireworksItem(pub Slot);
pub struct AttachedToTarget(pub OptionalUnsignedInt);
pub struct ShotAtAngle(pub bool);
pub struct FireworkRocket;
impl FireworkRocket {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(FireworksItem(Slot::Empty))
            .add(AttachedToTarget(OptionalUnsignedInt(None)))
            .add(ShotAtAngle(false))
            .build()
    }
}

#[derive(Query)]
struct FireworkRocketQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    fireworks_item: &'a mut FireworksItem,
    attached_to_target: &'a mut AttachedToTarget,
    shot_at_angle: &'a mut ShotAtAngle,
}
impl FireworkRocketQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.fireworks_item = FireworksItem(d.value.into_item_stack()?),
                9 => {
                    *self.attached_to_target =
                        AttachedToTarget(d.value.into_optional_unsigned_int()?)
                }
                10 => *self.shot_at_angle = ShotAtAngle(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct HookedEntity(pub i32);
pub struct Biting(pub bool);
pub struct FishingBobber;
impl FishingBobber {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(HookedEntity(0))
            .add(Biting(false))
            .build()
    }
}

#[derive(Query)]
struct FishingBobberQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    hooked_entity: &'a mut HookedEntity,
    biting: &'a mut Biting,
}
impl FishingBobberQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.hooked_entity = HookedEntity(d.value.into_int()?),
                9 => *self.biting = Biting(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct FoxKind(pub i32);
pub struct FoxSitting(pub bool);
pub struct Faceplanted(pub bool);
pub struct Sleeping(pub bool);
pub struct Pouncing(pub bool);
pub struct Crouching(pub bool);
pub struct FoxInterested(pub bool);
pub struct TrustedId0(pub Option<Uuid>);
pub struct TrustedId1(pub Option<Uuid>);
pub struct Fox;
impl Fox {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(FoxKind(0))
            .add(FoxSitting(false))
            .add(Faceplanted(false))
            .add(Sleeping(false))
            .add(Pouncing(false))
            .add(Crouching(false))
            .add(FoxInterested(false))
            .add(TrustedId0(None))
            .add(TrustedId1(None))
            .build()
    }
}

#[derive(Query)]
struct FoxQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    fox_kind: &'a mut FoxKind,
    fox_sitting: &'a mut FoxSitting,
    faceplanted: &'a mut Faceplanted,
    sleeping: &'a mut Sleeping,
    pouncing: &'a mut Pouncing,
    crouching: &'a mut Crouching,
    fox_interested: &'a mut FoxInterested,
    trusted_id_0: &'a mut TrustedId0,
    trusted_id_1: &'a mut TrustedId1,
}
impl FoxQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.fox_kind = FoxKind(d.value.into_int()?),
                18 => {
                    let bitfield = d.value.into_byte()?;
                    *self.fox_sitting = FoxSitting(bitfield & 0x1 != 0);
                    *self.faceplanted = Faceplanted(bitfield & 0x40 != 0);
                    *self.sleeping = Sleeping(bitfield & 0x20 != 0);
                    *self.pouncing = Pouncing(bitfield & 0x10 != 0);
                    *self.crouching = Crouching(bitfield & 0x4 != 0);
                    *self.fox_interested = FoxInterested(bitfield & 0x8 != 0);
                }
                19 => *self.trusted_id_0 = TrustedId0(d.value.into_optional_uuid()?),
                20 => *self.trusted_id_1 = TrustedId1(d.value.into_optional_uuid()?),
            }
        }
        Ok(())
    }
}

pub struct FrogVariant(pub azalea_registry::FrogVariant);
pub struct TongueTarget(pub OptionalUnsignedInt);
pub struct Frog;
impl Frog {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(FrogVariant(azalea_registry::FrogVariant::Temperate))
            .add(TongueTarget(OptionalUnsignedInt(None)))
            .build()
    }
}

#[derive(Query)]
struct FrogQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    frog_variant: &'a mut FrogVariant,
    tongue_target: &'a mut TongueTarget,
}
impl FrogQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.frog_variant = FrogVariant(d.value.into_frog_variant()?),
                18 => *self.tongue_target = TongueTarget(d.value.into_optional_unsigned_int()?),
            }
        }
        Ok(())
    }
}

pub struct Fuel(pub bool);
pub struct FurnaceMinecart;
impl FurnaceMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .add(Fuel(false))
            .build()
    }
}

#[derive(Query)]
struct FurnaceMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
    fuel: &'a mut Fuel,
}
impl FurnaceMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
                14 => *self.fuel = Fuel(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct IsCharging(pub bool);
pub struct Ghast;
impl Ghast {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(IsCharging(false))
            .build()
    }
}

#[derive(Query)]
struct GhastQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    is_charging: &'a mut IsCharging,
}
impl GhastQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.is_charging = IsCharging(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Giant;
impl Giant {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct GiantQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl GiantQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct ItemFrameItem(pub Slot);
pub struct Rotation(pub i32);
pub struct GlowItemFrame;
impl GlowItemFrame {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(ItemFrameItem(Slot::Empty))
            .add(Rotation(0))
            .build()
    }
}

#[derive(Query)]
struct GlowItemFrameQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    item_frame_item: &'a mut ItemFrameItem,
    rotation: &'a mut Rotation,
}
impl GlowItemFrameQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.item_frame_item = ItemFrameItem(d.value.into_item_stack()?),
                9 => *self.rotation = Rotation(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct DarkTicksRemaining(pub i32);
pub struct GlowSquid;
impl GlowSquid {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(DarkTicksRemaining(0))
            .build()
    }
}

#[derive(Query)]
struct GlowSquidQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    dark_ticks_remaining: &'a mut DarkTicksRemaining,
}
impl GlowSquidQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.dark_ticks_remaining = DarkTicksRemaining(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct IsScreamingGoat(pub bool);
pub struct HasLeftHorn(pub bool);
pub struct HasRightHorn(pub bool);
pub struct Goat;
impl Goat {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(IsScreamingGoat(false))
            .add(HasLeftHorn(true))
            .add(HasRightHorn(true))
            .build()
    }
}

#[derive(Query)]
struct GoatQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    is_screaming_goat: &'a mut IsScreamingGoat,
    has_left_horn: &'a mut HasLeftHorn,
    has_right_horn: &'a mut HasRightHorn,
}
impl GoatQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.is_screaming_goat = IsScreamingGoat(d.value.into_boolean()?),
                18 => *self.has_left_horn = HasLeftHorn(d.value.into_boolean()?),
                19 => *self.has_right_horn = HasRightHorn(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Guardian;
impl Guardian {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Moving(false))
            .add(AttackTarget(0))
            .build()
    }
}

#[derive(Query)]
struct GuardianQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    moving: &'a mut Moving,
    attack_target: &'a mut AttackTarget,
}
impl GuardianQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.moving = Moving(d.value.into_boolean()?),
                17 => *self.attack_target = AttackTarget(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct HoglinImmuneToZombification(pub bool);
pub struct Hoglin;
impl Hoglin {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(HoglinImmuneToZombification(false))
            .build()
    }
}

#[derive(Query)]
struct HoglinQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    hoglin_immune_to_zombification: &'a mut HoglinImmuneToZombification,
}
impl HoglinQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    *self.hoglin_immune_to_zombification =
                        HoglinImmuneToZombification(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

pub struct HopperMinecart;
impl HopperMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .build()
    }
}

#[derive(Query)]
struct HopperMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
}
impl HopperMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct HorseTamed(pub bool);
pub struct HorseEating(pub bool);
pub struct HorseStanding(pub bool);
pub struct HorseBred(pub bool);
pub struct HorseSaddled(pub bool);
pub struct HorseOwnerUuid(pub Option<Uuid>);
pub struct HorseTypeVariant(pub i32);
pub struct Horse;
impl Horse {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(HorseTamed(false))
            .add(HorseEating(false))
            .add(HorseStanding(false))
            .add(HorseBred(false))
            .add(HorseSaddled(false))
            .add(HorseOwnerUuid(None))
            .add(HorseTypeVariant(0))
            .build()
    }
}

#[derive(Query)]
struct HorseQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    horse_tamed: &'a mut HorseTamed,
    horse_eating: &'a mut HorseEating,
    horse_standing: &'a mut HorseStanding,
    horse_bred: &'a mut HorseBred,
    horse_saddled: &'a mut HorseSaddled,
    horse_owner_uuid: &'a mut HorseOwnerUuid,
    horse_type_variant: &'a mut HorseTypeVariant,
}
impl HorseQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.horse_tamed = HorseTamed(bitfield & 0x2 != 0);
                    *self.horse_eating = HorseEating(bitfield & 0x10 != 0);
                    *self.horse_standing = HorseStanding(bitfield & 0x20 != 0);
                    *self.horse_bred = HorseBred(bitfield & 0x8 != 0);
                    *self.horse_saddled = HorseSaddled(bitfield & 0x4 != 0);
                }
                18 => *self.horse_owner_uuid = HorseOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.horse_type_variant = HorseTypeVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Husk;
impl Husk {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZombieBaby(false))
            .add(SpecialType(0))
            .add(DrownedConversion(false))
            .build()
    }
}

#[derive(Query)]
struct HuskQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zombie_baby: &'a mut ZombieBaby,
    special_type: &'a mut SpecialType,
    drowned_conversion: &'a mut DrownedConversion,
}
impl HuskQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => *self.special_type = SpecialType(d.value.into_int()?),
                18 => *self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct IllusionerIsCelebrating(pub bool);
pub struct IllusionerSpellCasting(pub u8);
pub struct Illusioner;
impl Illusioner {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(IllusionerIsCelebrating(false))
            .add(IllusionerSpellCasting(0))
            .build()
    }
}

#[derive(Query)]
struct IllusionerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    illusioner_is_celebrating: &'a mut IllusionerIsCelebrating,
    illusioner_spell_casting: &'a mut IllusionerSpellCasting,
}
impl IllusionerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.illusioner_is_celebrating =
                        IllusionerIsCelebrating(d.value.into_boolean()?)
                }
                17 => *self.illusioner_spell_casting = IllusionerSpellCasting(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

pub struct PlayerCreated(pub bool);
pub struct IronGolem;
impl IronGolem {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PlayerCreated(false))
            .build()
    }
}

#[derive(Query)]
struct IronGolemQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    player_created: &'a mut PlayerCreated,
}
impl IronGolemQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.player_created = PlayerCreated(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct ItemItem(pub Slot);
pub struct Item;
impl Item {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(ItemItem(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct ItemQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    item_item: &'a mut ItemItem,
}
impl ItemQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.item_item = ItemItem(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct ItemFrame;
impl ItemFrame {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(ItemFrameItem(Slot::Empty))
            .add(Rotation(0))
            .build()
    }
}

#[derive(Query)]
struct ItemFrameQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    item_frame_item: &'a mut ItemFrameItem,
    rotation: &'a mut Rotation,
}
impl ItemFrameQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.item_frame_item = ItemFrameItem(d.value.into_item_stack()?),
                9 => *self.rotation = Rotation(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct LeashKnot;
impl LeashKnot {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct LeashKnotQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl LeashKnotQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct LightningBolt;
impl LightningBolt {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct LightningBoltQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl LightningBoltQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct LlamaTamed(pub bool);
pub struct LlamaEating(pub bool);
pub struct LlamaStanding(pub bool);
pub struct LlamaBred(pub bool);
pub struct LlamaSaddled(pub bool);
pub struct LlamaOwnerUuid(pub Option<Uuid>);
pub struct LlamaChest(pub bool);
pub struct Strength(pub i32);
pub struct Swag(pub i32);
pub struct LlamaVariant(pub i32);
pub struct Llama;
impl Llama {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(LlamaTamed(false))
            .add(LlamaEating(false))
            .add(LlamaStanding(false))
            .add(LlamaBred(false))
            .add(LlamaSaddled(false))
            .add(LlamaOwnerUuid(None))
            .add(LlamaChest(false))
            .add(Strength(0))
            .add(Swag(-1))
            .add(LlamaVariant(0))
            .build()
    }
}

#[derive(Query)]
struct LlamaQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    llama_tamed: &'a mut LlamaTamed,
    llama_eating: &'a mut LlamaEating,
    llama_standing: &'a mut LlamaStanding,
    llama_bred: &'a mut LlamaBred,
    llama_saddled: &'a mut LlamaSaddled,
    llama_owner_uuid: &'a mut LlamaOwnerUuid,
    llama_chest: &'a mut LlamaChest,
    strength: &'a mut Strength,
    swag: &'a mut Swag,
    llama_variant: &'a mut LlamaVariant,
}
impl LlamaQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.llama_tamed = LlamaTamed(bitfield & 0x2 != 0);
                    *self.llama_eating = LlamaEating(bitfield & 0x10 != 0);
                    *self.llama_standing = LlamaStanding(bitfield & 0x20 != 0);
                    *self.llama_bred = LlamaBred(bitfield & 0x8 != 0);
                    *self.llama_saddled = LlamaSaddled(bitfield & 0x4 != 0);
                }
                18 => *self.llama_owner_uuid = LlamaOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.llama_chest = LlamaChest(d.value.into_boolean()?),
                20 => *self.strength = Strength(d.value.into_int()?),
                21 => *self.swag = Swag(d.value.into_int()?),
                22 => *self.llama_variant = LlamaVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct LlamaSpit;
impl LlamaSpit {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct LlamaSpitQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl LlamaSpitQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct SlimeSize(pub i32);
pub struct MagmaCube;
impl MagmaCube {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(SlimeSize(1))
            .build()
    }
}

#[derive(Query)]
struct MagmaCubeQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    slime_size: &'a mut SlimeSize,
}
impl MagmaCubeQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.slime_size = SlimeSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Marker;
impl Marker {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct MarkerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl MarkerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Minecart;
impl Minecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .build()
    }
}

#[derive(Query)]
struct MinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
}
impl MinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct MooshroomKind(pub String);
pub struct Mooshroom;
impl Mooshroom {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(MooshroomKind(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct MooshroomQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    mooshroom_kind: &'a mut MooshroomKind,
}
impl MooshroomQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.mooshroom_kind = MooshroomKind(d.value.into_string()?),
            }
        }
        Ok(())
    }
}

pub struct MuleTamed(pub bool);
pub struct MuleEating(pub bool);
pub struct MuleStanding(pub bool);
pub struct MuleBred(pub bool);
pub struct MuleSaddled(pub bool);
pub struct MuleOwnerUuid(pub Option<Uuid>);
pub struct MuleChest(pub bool);
pub struct Mule;
impl Mule {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(MuleTamed(false))
            .add(MuleEating(false))
            .add(MuleStanding(false))
            .add(MuleBred(false))
            .add(MuleSaddled(false))
            .add(MuleOwnerUuid(None))
            .add(MuleChest(false))
            .build()
    }
}

#[derive(Query)]
struct MuleQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    mule_tamed: &'a mut MuleTamed,
    mule_eating: &'a mut MuleEating,
    mule_standing: &'a mut MuleStanding,
    mule_bred: &'a mut MuleBred,
    mule_saddled: &'a mut MuleSaddled,
    mule_owner_uuid: &'a mut MuleOwnerUuid,
    mule_chest: &'a mut MuleChest,
}
impl MuleQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.mule_tamed = MuleTamed(bitfield & 0x2 != 0);
                    *self.mule_eating = MuleEating(bitfield & 0x10 != 0);
                    *self.mule_standing = MuleStanding(bitfield & 0x20 != 0);
                    *self.mule_bred = MuleBred(bitfield & 0x8 != 0);
                    *self.mule_saddled = MuleSaddled(bitfield & 0x4 != 0);
                }
                18 => *self.mule_owner_uuid = MuleOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.mule_chest = MuleChest(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Trusting(pub bool);
pub struct Ocelot;
impl Ocelot {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(Trusting(false))
            .build()
    }
}

#[derive(Query)]
struct OcelotQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    trusting: &'a mut Trusting,
}
impl OcelotQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.trusting = Trusting(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct PaintingVariant(pub azalea_registry::PaintingVariant);
pub struct Painting;
impl Painting {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(PaintingVariant(azalea_registry::PaintingVariant::Kebab))
            .build()
    }
}

#[derive(Query)]
struct PaintingQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    painting_variant: &'a mut PaintingVariant,
}
impl PaintingQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.painting_variant = PaintingVariant(d.value.into_painting_variant()?),
            }
        }
        Ok(())
    }
}

pub struct PandaUnhappyCounter(pub i32);
pub struct SneezeCounter(pub i32);
pub struct EatCounter(pub i32);
pub struct Sneezing(pub bool);
pub struct PandaSitting(pub bool);
pub struct OnBack(pub bool);
pub struct PandaRolling(pub bool);
pub struct HiddenGene(pub u8);
pub struct PandaFlags(pub u8);
pub struct Panda;
impl Panda {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(PandaUnhappyCounter(0))
            .add(SneezeCounter(0))
            .add(EatCounter(0))
            .add(Sneezing(false))
            .add(PandaSitting(false))
            .add(OnBack(false))
            .add(PandaRolling(false))
            .add(HiddenGene(0))
            .add(PandaFlags(0))
            .build()
    }
}

#[derive(Query)]
struct PandaQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    panda_unhappy_counter: &'a mut PandaUnhappyCounter,
    sneeze_counter: &'a mut SneezeCounter,
    eat_counter: &'a mut EatCounter,
    sneezing: &'a mut Sneezing,
    panda_sitting: &'a mut PandaSitting,
    on_back: &'a mut OnBack,
    panda_rolling: &'a mut PandaRolling,
    hidden_gene: &'a mut HiddenGene,
    panda_flags: &'a mut PandaFlags,
}
impl PandaQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.panda_unhappy_counter = PandaUnhappyCounter(d.value.into_int()?),
                18 => *self.sneeze_counter = SneezeCounter(d.value.into_int()?),
                19 => *self.eat_counter = EatCounter(d.value.into_int()?),
                20 => {
                    let bitfield = d.value.into_byte()?;
                    *self.sneezing = Sneezing(bitfield & 0x2 != 0);
                    *self.panda_sitting = PandaSitting(bitfield & 0x8 != 0);
                    *self.on_back = OnBack(bitfield & 0x10 != 0);
                    *self.panda_rolling = PandaRolling(bitfield & 0x4 != 0);
                }
                21 => *self.hidden_gene = HiddenGene(d.value.into_byte()?),
                22 => *self.panda_flags = PandaFlags(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

pub struct ParrotVariant(pub i32);
pub struct Parrot;
impl Parrot {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(Tame(false))
            .add(InSittingPose(false))
            .add(Owneruuid(None))
            .add(ParrotVariant(0))
            .build()
    }
}

#[derive(Query)]
struct ParrotQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    tame: &'a mut Tame,
    in_sitting_pose: &'a mut InSittingPose,
    owneruuid: &'a mut Owneruuid,
    parrot_variant: &'a mut ParrotVariant,
}
impl ParrotQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.tame = Tame(bitfield & 0x4 != 0);
                    *self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => *self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => *self.parrot_variant = ParrotVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct PhantomSize(pub i32);
pub struct Phantom;
impl Phantom {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PhantomSize(0))
            .build()
    }
}

#[derive(Query)]
struct PhantomQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    phantom_size: &'a mut PhantomSize,
}
impl PhantomQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.phantom_size = PhantomSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct PigSaddle(pub bool);
pub struct PigBoostTime(pub i32);
pub struct Pig;
impl Pig {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(PigSaddle(false))
            .add(PigBoostTime(0))
            .build()
    }
}

#[derive(Query)]
struct PigQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    pig_saddle: &'a mut PigSaddle,
    pig_boost_time: &'a mut PigBoostTime,
}
impl PigQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.pig_saddle = PigSaddle(d.value.into_boolean()?),
                18 => *self.pig_boost_time = PigBoostTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct PiglinImmuneToZombification(pub bool);
pub struct PiglinBaby(pub bool);
pub struct PiglinIsChargingCrossbow(pub bool);
pub struct IsDancing(pub bool);
pub struct Piglin;
impl Piglin {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PiglinImmuneToZombification(false))
            .add(PiglinBaby(false))
            .add(PiglinIsChargingCrossbow(false))
            .add(IsDancing(false))
            .build()
    }
}

#[derive(Query)]
struct PiglinQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    piglin_immune_to_zombification: &'a mut PiglinImmuneToZombification,
    piglin_baby: &'a mut PiglinBaby,
    piglin_is_charging_crossbow: &'a mut PiglinIsChargingCrossbow,
    is_dancing: &'a mut IsDancing,
}
impl PiglinQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.piglin_immune_to_zombification =
                        PiglinImmuneToZombification(d.value.into_boolean()?)
                }
                17 => *self.piglin_baby = PiglinBaby(d.value.into_boolean()?),
                18 => {
                    *self.piglin_is_charging_crossbow =
                        PiglinIsChargingCrossbow(d.value.into_boolean()?)
                }
                19 => *self.is_dancing = IsDancing(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct PiglinBruteImmuneToZombification(pub bool);
pub struct PiglinBrute;
impl PiglinBrute {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PiglinBruteImmuneToZombification(false))
            .build()
    }
}

#[derive(Query)]
struct PiglinBruteQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    piglin_brute_immune_to_zombification: &'a mut PiglinBruteImmuneToZombification,
}
impl PiglinBruteQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.piglin_brute_immune_to_zombification =
                        PiglinBruteImmuneToZombification(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

pub struct PillagerIsCelebrating(pub bool);
pub struct PillagerIsChargingCrossbow(pub bool);
pub struct Pillager;
impl Pillager {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PillagerIsCelebrating(false))
            .add(PillagerIsChargingCrossbow(false))
            .build()
    }
}

#[derive(Query)]
struct PillagerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    pillager_is_celebrating: &'a mut PillagerIsCelebrating,
    pillager_is_charging_crossbow: &'a mut PillagerIsChargingCrossbow,
}
impl PillagerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.pillager_is_celebrating = PillagerIsCelebrating(d.value.into_boolean()?)
                }
                17 => {
                    *self.pillager_is_charging_crossbow =
                        PillagerIsChargingCrossbow(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

pub struct PlayerAbsorption(pub f32);
pub struct Score(pub i32);
pub struct PlayerModeCustomisation(pub u8);
pub struct PlayerMainHand(pub u8);
pub struct ShoulderLeft(pub azalea_nbt::Tag);
pub struct ShoulderRight(pub azalea_nbt::Tag);
pub struct Player;
impl Player {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(PlayerAbsorption(0.0))
            .add(Score(0))
            .add(PlayerModeCustomisation(0))
            .add(PlayerMainHand(1))
            .add(ShoulderLeft(azalea_nbt::Tag::Compound(Default::default())))
            .add(ShoulderRight(azalea_nbt::Tag::Compound(Default::default())))
            .build()
    }
}

#[derive(Query)]
struct PlayerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    player_absorption: &'a mut PlayerAbsorption,
    score: &'a mut Score,
    player_mode_customisation: &'a mut PlayerModeCustomisation,
    player_main_hand: &'a mut PlayerMainHand,
    shoulder_left: &'a mut ShoulderLeft,
    shoulder_right: &'a mut ShoulderRight,
}
impl PlayerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => *self.player_absorption = PlayerAbsorption(d.value.into_float()?),
                16 => *self.score = Score(d.value.into_int()?),
                17 => {
                    *self.player_mode_customisation = PlayerModeCustomisation(d.value.into_byte()?)
                }
                18 => *self.player_main_hand = PlayerMainHand(d.value.into_byte()?),
                19 => *self.shoulder_left = ShoulderLeft(d.value.into_compound_tag()?),
                20 => *self.shoulder_right = ShoulderRight(d.value.into_compound_tag()?),
            }
        }
        Ok(())
    }
}

pub struct PolarBearStanding(pub bool);
pub struct PolarBear;
impl PolarBear {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(PolarBearStanding(false))
            .build()
    }
}

#[derive(Query)]
struct PolarBearQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    polar_bear_standing: &'a mut PolarBearStanding,
}
impl PolarBearQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.polar_bear_standing = PolarBearStanding(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct PotionItemStack(pub Slot);
pub struct Potion;
impl Potion {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(PotionItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct PotionQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    potion_item_stack: &'a mut PotionItemStack,
}
impl PotionQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.potion_item_stack = PotionItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct PufferfishFromBucket(pub bool);
pub struct PuffState(pub i32);
pub struct Pufferfish;
impl Pufferfish {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(PufferfishFromBucket(false))
            .add(PuffState(0))
            .build()
    }
}

#[derive(Query)]
struct PufferfishQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    pufferfish_from_bucket: &'a mut PufferfishFromBucket,
    puff_state: &'a mut PuffState,
}
impl PufferfishQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.pufferfish_from_bucket = PufferfishFromBucket(d.value.into_boolean()?),
                17 => *self.puff_state = PuffState(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct RabbitKind(pub i32);
pub struct Rabbit;
impl Rabbit {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(RabbitKind(Default::default()))
            .build()
    }
}

#[derive(Query)]
struct RabbitQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    rabbit_kind: &'a mut RabbitKind,
}
impl RabbitQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.rabbit_kind = RabbitKind(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct RavagerIsCelebrating(pub bool);
pub struct Ravager;
impl Ravager {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(RavagerIsCelebrating(false))
            .build()
    }
}

#[derive(Query)]
struct RavagerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    ravager_is_celebrating: &'a mut RavagerIsCelebrating,
}
impl RavagerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.ravager_is_celebrating = RavagerIsCelebrating(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct SalmonFromBucket(pub bool);
pub struct Salmon;
impl Salmon {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(SalmonFromBucket(false))
            .build()
    }
}

#[derive(Query)]
struct SalmonQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    salmon_from_bucket: &'a mut SalmonFromBucket,
}
impl SalmonQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.salmon_from_bucket = SalmonFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Sheared(pub bool);
pub struct Sheep;
impl Sheep {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(Sheared(false))
            .build()
    }
}

#[derive(Query)]
struct SheepQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    sheared: &'a mut Sheared,
}
impl SheepQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.sheared = Sheared(bitfield & 0x10 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct AttachFace(pub Direction);
pub struct Peek(pub u8);
pub struct ShulkerColor(pub u8);
pub struct Shulker;
impl Shulker {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AttachFace(Default::default()))
            .add(Peek(0))
            .add(ShulkerColor(16))
            .build()
    }
}

#[derive(Query)]
struct ShulkerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    attach_face: &'a mut AttachFace,
    peek: &'a mut Peek,
    shulker_color: &'a mut ShulkerColor,
}
impl ShulkerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.attach_face = AttachFace(d.value.into_direction()?),
                17 => *self.peek = Peek(d.value.into_byte()?),
                18 => *self.shulker_color = ShulkerColor(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

pub struct ShulkerBullet;
impl ShulkerBullet {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .build()
    }
}

#[derive(Query)]
struct ShulkerBulletQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
}
impl ShulkerBulletQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct Silverfish;
impl Silverfish {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct SilverfishQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl SilverfishQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct StrayConversion(pub bool);
pub struct Skeleton;
impl Skeleton {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(StrayConversion(false))
            .build()
    }
}

#[derive(Query)]
struct SkeletonQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    stray_conversion: &'a mut StrayConversion,
}
impl SkeletonQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.stray_conversion = StrayConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct SkeletonHorseTamed(pub bool);
pub struct SkeletonHorseEating(pub bool);
pub struct SkeletonHorseStanding(pub bool);
pub struct SkeletonHorseBred(pub bool);
pub struct SkeletonHorseSaddled(pub bool);
pub struct SkeletonHorseOwnerUuid(pub Option<Uuid>);
pub struct SkeletonHorse;
impl SkeletonHorse {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(SkeletonHorseTamed(false))
            .add(SkeletonHorseEating(false))
            .add(SkeletonHorseStanding(false))
            .add(SkeletonHorseBred(false))
            .add(SkeletonHorseSaddled(false))
            .add(SkeletonHorseOwnerUuid(None))
            .build()
    }
}

#[derive(Query)]
struct SkeletonHorseQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    skeleton_horse_tamed: &'a mut SkeletonHorseTamed,
    skeleton_horse_eating: &'a mut SkeletonHorseEating,
    skeleton_horse_standing: &'a mut SkeletonHorseStanding,
    skeleton_horse_bred: &'a mut SkeletonHorseBred,
    skeleton_horse_saddled: &'a mut SkeletonHorseSaddled,
    skeleton_horse_owner_uuid: &'a mut SkeletonHorseOwnerUuid,
}
impl SkeletonHorseQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.skeleton_horse_tamed = SkeletonHorseTamed(bitfield & 0x2 != 0);
                    *self.skeleton_horse_eating = SkeletonHorseEating(bitfield & 0x10 != 0);
                    *self.skeleton_horse_standing = SkeletonHorseStanding(bitfield & 0x20 != 0);
                    *self.skeleton_horse_bred = SkeletonHorseBred(bitfield & 0x8 != 0);
                    *self.skeleton_horse_saddled = SkeletonHorseSaddled(bitfield & 0x4 != 0);
                }
                18 => {
                    *self.skeleton_horse_owner_uuid =
                        SkeletonHorseOwnerUuid(d.value.into_optional_uuid()?)
                }
            }
        }
        Ok(())
    }
}

pub struct Slime;
impl Slime {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(SlimeSize(1))
            .build()
    }
}

#[derive(Query)]
struct SlimeQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    slime_size: &'a mut SlimeSize,
}
impl SlimeQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.slime_size = SlimeSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct SmallFireballItemStack(pub Slot);
pub struct SmallFireball;
impl SmallFireball {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(SmallFireballItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct SmallFireballQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    small_fireball_item_stack: &'a mut SmallFireballItemStack,
}
impl SmallFireballQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    *self.small_fireball_item_stack =
                        SmallFireballItemStack(d.value.into_item_stack()?)
                }
            }
        }
        Ok(())
    }
}

pub struct HasPumpkin(pub bool);
pub struct SnowGolem;
impl SnowGolem {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(HasPumpkin(true))
            .build()
    }
}

#[derive(Query)]
struct SnowGolemQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    has_pumpkin: &'a mut HasPumpkin,
}
impl SnowGolemQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.has_pumpkin = HasPumpkin(bitfield & 0x10 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct SnowballItemStack(pub Slot);
pub struct Snowball;
impl Snowball {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(SnowballItemStack(Slot::Empty))
            .build()
    }
}

#[derive(Query)]
struct SnowballQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    snowball_item_stack: &'a mut SnowballItemStack,
}
impl SnowballQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.snowball_item_stack = SnowballItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

pub struct SpawnerMinecart;
impl SpawnerMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .build()
    }
}

#[derive(Query)]
struct SpawnerMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
}
impl SpawnerMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct SpectralArrowCritArrow(pub bool);
pub struct SpectralArrowShotFromCrossbow(pub bool);
pub struct SpectralArrowNoPhysics(pub bool);
pub struct SpectralArrowPierceLevel(pub u8);
pub struct SpectralArrow;
impl SpectralArrow {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(SpectralArrowCritArrow(false))
            .add(SpectralArrowShotFromCrossbow(false))
            .add(SpectralArrowNoPhysics(false))
            .add(SpectralArrowPierceLevel(0))
            .build()
    }
}

#[derive(Query)]
struct SpectralArrowQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    spectral_arrow_crit_arrow: &'a mut SpectralArrowCritArrow,
    spectral_arrow_shot_from_crossbow: &'a mut SpectralArrowShotFromCrossbow,
    spectral_arrow_no_physics: &'a mut SpectralArrowNoPhysics,
    spectral_arrow_pierce_level: &'a mut SpectralArrowPierceLevel,
}
impl SpectralArrowQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.spectral_arrow_crit_arrow = SpectralArrowCritArrow(bitfield & 0x1 != 0);
                    *self.spectral_arrow_shot_from_crossbow =
                        SpectralArrowShotFromCrossbow(bitfield & 0x4 != 0);
                    *self.spectral_arrow_no_physics = SpectralArrowNoPhysics(bitfield & 0x2 != 0);
                }
                9 => {
                    *self.spectral_arrow_pierce_level =
                        SpectralArrowPierceLevel(d.value.into_byte()?)
                }
            }
        }
        Ok(())
    }
}

pub struct Spider;
impl Spider {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(Climbing(false))
            .build()
    }
}

#[derive(Query)]
struct SpiderQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    climbing: &'a mut Climbing,
}
impl SpiderQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    *self.climbing = Climbing(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct Squid;
impl Squid {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct SquidQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl SquidQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct Stray;
impl Stray {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct StrayQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl StrayQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct StriderBoostTime(pub i32);
pub struct Suffocating(pub bool);
pub struct StriderSaddle(pub bool);
pub struct Strider;
impl Strider {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(StriderBoostTime(0))
            .add(Suffocating(false))
            .add(StriderSaddle(false))
            .build()
    }
}

#[derive(Query)]
struct StriderQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    strider_boost_time: &'a mut StriderBoostTime,
    suffocating: &'a mut Suffocating,
    strider_saddle: &'a mut StriderSaddle,
}
impl StriderQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.strider_boost_time = StriderBoostTime(d.value.into_int()?),
                18 => *self.suffocating = Suffocating(d.value.into_boolean()?),
                19 => *self.strider_saddle = StriderSaddle(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct TadpoleFromBucket(pub bool);
pub struct Tadpole;
impl Tadpole {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(TadpoleFromBucket(false))
            .build()
    }
}

#[derive(Query)]
struct TadpoleQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    tadpole_from_bucket: &'a mut TadpoleFromBucket,
}
impl TadpoleQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.tadpole_from_bucket = TadpoleFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Fuse(pub i32);
pub struct Tnt;
impl Tnt {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(Fuse(80))
            .build()
    }
}

#[derive(Query)]
struct TntQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    fuse: &'a mut Fuse,
}
impl TntQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.fuse = Fuse(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct TntMinecart;
impl TntMinecart {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AbstractMinecartHurt(0))
            .add(AbstractMinecartHurtdir(1))
            .add(AbstractMinecartDamage(0.0))
            .add(DisplayBlock(Default::default()))
            .add(DisplayOffset(6))
            .add(CustomDisplay(false))
            .build()
    }
}

#[derive(Query)]
struct TntMinecartQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    abstract_minecart_hurt: &'a mut AbstractMinecartHurt,
    abstract_minecart_hurtdir: &'a mut AbstractMinecartHurtdir,
    abstract_minecart_damage: &'a mut AbstractMinecartDamage,
    display_block: &'a mut DisplayBlock,
    display_offset: &'a mut DisplayOffset,
    custom_display: &'a mut CustomDisplay,
}
impl TntMinecartQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => *self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => {
                    *self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?)
                }
                11 => *self.display_block = DisplayBlock(d.value.into_int()?),
                12 => *self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => *self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct TraderLlama;
impl TraderLlama {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(LlamaTamed(false))
            .add(LlamaEating(false))
            .add(LlamaStanding(false))
            .add(LlamaBred(false))
            .add(LlamaSaddled(false))
            .add(LlamaOwnerUuid(None))
            .add(LlamaChest(false))
            .add(Strength(0))
            .add(Swag(-1))
            .add(LlamaVariant(0))
            .build()
    }
}

#[derive(Query)]
struct TraderLlamaQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    llama_tamed: &'a mut LlamaTamed,
    llama_eating: &'a mut LlamaEating,
    llama_standing: &'a mut LlamaStanding,
    llama_bred: &'a mut LlamaBred,
    llama_saddled: &'a mut LlamaSaddled,
    llama_owner_uuid: &'a mut LlamaOwnerUuid,
    llama_chest: &'a mut LlamaChest,
    strength: &'a mut Strength,
    swag: &'a mut Swag,
    llama_variant: &'a mut LlamaVariant,
}
impl TraderLlamaQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.llama_tamed = LlamaTamed(bitfield & 0x2 != 0);
                    *self.llama_eating = LlamaEating(bitfield & 0x10 != 0);
                    *self.llama_standing = LlamaStanding(bitfield & 0x20 != 0);
                    *self.llama_bred = LlamaBred(bitfield & 0x8 != 0);
                    *self.llama_saddled = LlamaSaddled(bitfield & 0x4 != 0);
                }
                18 => *self.llama_owner_uuid = LlamaOwnerUuid(d.value.into_optional_uuid()?),
                19 => *self.llama_chest = LlamaChest(d.value.into_boolean()?),
                20 => *self.strength = Strength(d.value.into_int()?),
                21 => *self.swag = Swag(d.value.into_int()?),
                22 => *self.llama_variant = LlamaVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct TridentCritArrow(pub bool);
pub struct TridentShotFromCrossbow(pub bool);
pub struct TridentNoPhysics(pub bool);
pub struct TridentPierceLevel(pub u8);
pub struct Loyalty(pub u8);
pub struct Foil(pub bool);
pub struct Trident;
impl Trident {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(TridentCritArrow(false))
            .add(TridentShotFromCrossbow(false))
            .add(TridentNoPhysics(false))
            .add(TridentPierceLevel(0))
            .add(Loyalty(0))
            .add(Foil(false))
            .build()
    }
}

#[derive(Query)]
struct TridentQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    trident_crit_arrow: &'a mut TridentCritArrow,
    trident_shot_from_crossbow: &'a mut TridentShotFromCrossbow,
    trident_no_physics: &'a mut TridentNoPhysics,
    trident_pierce_level: &'a mut TridentPierceLevel,
    loyalty: &'a mut Loyalty,
    foil: &'a mut Foil,
}
impl TridentQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.trident_crit_arrow = TridentCritArrow(bitfield & 0x1 != 0);
                    *self.trident_shot_from_crossbow = TridentShotFromCrossbow(bitfield & 0x4 != 0);
                    *self.trident_no_physics = TridentNoPhysics(bitfield & 0x2 != 0);
                }
                9 => *self.trident_pierce_level = TridentPierceLevel(d.value.into_byte()?),
                10 => *self.loyalty = Loyalty(d.value.into_byte()?),
                11 => *self.foil = Foil(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct TropicalFishFromBucket(pub bool);
pub struct TropicalFishTypeVariant(pub i32);
pub struct TropicalFish;
impl TropicalFish {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(TropicalFishFromBucket(false))
            .add(TropicalFishTypeVariant(0))
            .build()
    }
}

#[derive(Query)]
struct TropicalFishQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    tropical_fish_from_bucket: &'a mut TropicalFishFromBucket,
    tropical_fish_type_variant: &'a mut TropicalFishTypeVariant,
}
impl TropicalFishQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.tropical_fish_from_bucket =
                        TropicalFishFromBucket(d.value.into_boolean()?)
                }
                17 => {
                    *self.tropical_fish_type_variant = TropicalFishTypeVariant(d.value.into_int()?)
                }
            }
        }
        Ok(())
    }
}

pub struct HomePos(pub BlockPos);
pub struct HasEgg(pub bool);
pub struct LayingEgg(pub bool);
pub struct TravelPos(pub BlockPos);
pub struct GoingHome(pub bool);
pub struct Travelling(pub bool);
pub struct Turtle;
impl Turtle {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(HomePos(BlockPos::new(0, 0, 0)))
            .add(HasEgg(false))
            .add(LayingEgg(false))
            .add(TravelPos(BlockPos::new(0, 0, 0)))
            .add(GoingHome(false))
            .add(Travelling(false))
            .build()
    }
}

#[derive(Query)]
struct TurtleQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    home_pos: &'a mut HomePos,
    has_egg: &'a mut HasEgg,
    laying_egg: &'a mut LayingEgg,
    travel_pos: &'a mut TravelPos,
    going_home: &'a mut GoingHome,
    travelling: &'a mut Travelling,
}
impl TurtleQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.home_pos = HomePos(d.value.into_block_pos()?),
                18 => *self.has_egg = HasEgg(d.value.into_boolean()?),
                19 => *self.laying_egg = LayingEgg(d.value.into_boolean()?),
                20 => *self.travel_pos = TravelPos(d.value.into_block_pos()?),
                21 => *self.going_home = GoingHome(d.value.into_boolean()?),
                22 => *self.travelling = Travelling(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct VexFlags(pub u8);
pub struct Vex;
impl Vex {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(VexFlags(0))
            .build()
    }
}

#[derive(Query)]
struct VexQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    vex_flags: &'a mut VexFlags,
}
impl VexQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.vex_flags = VexFlags(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

pub struct VillagerUnhappyCounter(pub i32);
pub struct VillagerVillagerData(pub VillagerData);
pub struct Villager;
impl Villager {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(VillagerUnhappyCounter(0))
            .add(VillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerType::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }))
            .build()
    }
}

#[derive(Query)]
struct VillagerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    villager_unhappy_counter: &'a mut VillagerUnhappyCounter,
    villager_villager_data: &'a mut VillagerVillagerData,
}
impl VillagerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => *self.villager_unhappy_counter = VillagerUnhappyCounter(d.value.into_int()?),
                18 => {
                    *self.villager_villager_data =
                        VillagerVillagerData(d.value.into_villager_data()?)
                }
            }
        }
        Ok(())
    }
}

pub struct VindicatorIsCelebrating(pub bool);
pub struct Vindicator;
impl Vindicator {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(VindicatorIsCelebrating(false))
            .build()
    }
}

#[derive(Query)]
struct VindicatorQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    vindicator_is_celebrating: &'a mut VindicatorIsCelebrating,
}
impl VindicatorQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    *self.vindicator_is_celebrating =
                        VindicatorIsCelebrating(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

pub struct WanderingTraderUnhappyCounter(pub i32);
pub struct WanderingTrader;
impl WanderingTrader {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(WanderingTraderUnhappyCounter(0))
            .build()
    }
}

#[derive(Query)]
struct WanderingTraderQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    wandering_trader_unhappy_counter: &'a mut WanderingTraderUnhappyCounter,
}
impl WanderingTraderQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    *self.wandering_trader_unhappy_counter =
                        WanderingTraderUnhappyCounter(d.value.into_int()?)
                }
            }
        }
        Ok(())
    }
}

pub struct ClientAngerLevel(pub i32);
pub struct Warden;
impl Warden {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ClientAngerLevel(0))
            .build()
    }
}

#[derive(Query)]
struct WardenQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    client_anger_level: &'a mut ClientAngerLevel,
}
impl WardenQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.client_anger_level = ClientAngerLevel(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct WitchIsCelebrating(pub bool);
pub struct WitchUsingItem(pub bool);
pub struct Witch;
impl Witch {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(WitchIsCelebrating(false))
            .add(WitchUsingItem(false))
            .build()
    }
}

#[derive(Query)]
struct WitchQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    witch_is_celebrating: &'a mut WitchIsCelebrating,
    witch_using_item: &'a mut WitchUsingItem,
}
impl WitchQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.witch_is_celebrating = WitchIsCelebrating(d.value.into_boolean()?),
                17 => *self.witch_using_item = WitchUsingItem(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct TargetA(pub i32);
pub struct TargetB(pub i32);
pub struct TargetC(pub i32);
pub struct Inv(pub i32);
pub struct Wither;
impl Wither {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(TargetA(0))
            .add(TargetB(0))
            .add(TargetC(0))
            .add(Inv(0))
            .build()
    }
}

#[derive(Query)]
struct WitherQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    target_a: &'a mut TargetA,
    target_b: &'a mut TargetB,
    target_c: &'a mut TargetC,
    inv: &'a mut Inv,
}
impl WitherQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.target_a = TargetA(d.value.into_int()?),
                17 => *self.target_b = TargetB(d.value.into_int()?),
                18 => *self.target_c = TargetC(d.value.into_int()?),
                19 => *self.inv = Inv(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct WitherSkeleton;
impl WitherSkeleton {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .build()
    }
}

#[derive(Query)]
struct WitherSkeletonQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
}
impl WitherSkeletonQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

pub struct Dangerous(pub bool);
pub struct WitherSkull;
impl WitherSkull {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(Dangerous(false))
            .build()
    }
}

#[derive(Query)]
struct WitherSkullQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    dangerous: &'a mut Dangerous,
}
impl WitherSkullQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => *self.dangerous = Dangerous(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct WolfInterested(pub bool);
pub struct WolfCollarColor(pub i32);
pub struct WolfRemainingAngerTime(pub i32);
pub struct Wolf;
impl Wolf {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(Tame(false))
            .add(InSittingPose(false))
            .add(Owneruuid(None))
            .add(WolfInterested(false))
            .add(WolfCollarColor(Default::default()))
            .add(WolfRemainingAngerTime(0))
            .build()
    }
}

#[derive(Query)]
struct WolfQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    tame: &'a mut Tame,
    in_sitting_pose: &'a mut InSittingPose,
    owneruuid: &'a mut Owneruuid,
    wolf_interested: &'a mut WolfInterested,
    wolf_collar_color: &'a mut WolfCollarColor,
    wolf_remaining_anger_time: &'a mut WolfRemainingAngerTime,
}
impl WolfQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.tame = Tame(bitfield & 0x4 != 0);
                    *self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => *self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => *self.wolf_interested = WolfInterested(d.value.into_boolean()?),
                20 => *self.wolf_collar_color = WolfCollarColor(d.value.into_int()?),
                21 => *self.wolf_remaining_anger_time = WolfRemainingAngerTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

pub struct ZoglinBaby(pub bool);
pub struct Zoglin;
impl Zoglin {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZoglinBaby(false))
            .build()
    }
}

#[derive(Query)]
struct ZoglinQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zoglin_baby: &'a mut ZoglinBaby,
}
impl ZoglinQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zoglin_baby = ZoglinBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct Zombie;
impl Zombie {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZombieBaby(false))
            .add(SpecialType(0))
            .add(DrownedConversion(false))
            .build()
    }
}

#[derive(Query)]
struct ZombieQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zombie_baby: &'a mut ZombieBaby,
    special_type: &'a mut SpecialType,
    drowned_conversion: &'a mut DrownedConversion,
}
impl ZombieQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => *self.special_type = SpecialType(d.value.into_int()?),
                18 => *self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

pub struct ZombieHorseTamed(pub bool);
pub struct ZombieHorseEating(pub bool);
pub struct ZombieHorseStanding(pub bool);
pub struct ZombieHorseBred(pub bool);
pub struct ZombieHorseSaddled(pub bool);
pub struct ZombieHorseOwnerUuid(pub Option<Uuid>);
pub struct ZombieHorse;
impl ZombieHorse {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(AbstractAgeableBaby(false))
            .add(ZombieHorseTamed(false))
            .add(ZombieHorseEating(false))
            .add(ZombieHorseStanding(false))
            .add(ZombieHorseBred(false))
            .add(ZombieHorseSaddled(false))
            .add(ZombieHorseOwnerUuid(None))
            .build()
    }
}

#[derive(Query)]
struct ZombieHorseQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    abstract_ageable_baby: &'a mut AbstractAgeableBaby,
    zombie_horse_tamed: &'a mut ZombieHorseTamed,
    zombie_horse_eating: &'a mut ZombieHorseEating,
    zombie_horse_standing: &'a mut ZombieHorseStanding,
    zombie_horse_bred: &'a mut ZombieHorseBred,
    zombie_horse_saddled: &'a mut ZombieHorseSaddled,
    zombie_horse_owner_uuid: &'a mut ZombieHorseOwnerUuid,
}
impl ZombieHorseQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    *self.zombie_horse_tamed = ZombieHorseTamed(bitfield & 0x2 != 0);
                    *self.zombie_horse_eating = ZombieHorseEating(bitfield & 0x10 != 0);
                    *self.zombie_horse_standing = ZombieHorseStanding(bitfield & 0x20 != 0);
                    *self.zombie_horse_bred = ZombieHorseBred(bitfield & 0x8 != 0);
                    *self.zombie_horse_saddled = ZombieHorseSaddled(bitfield & 0x4 != 0);
                }
                18 => {
                    *self.zombie_horse_owner_uuid =
                        ZombieHorseOwnerUuid(d.value.into_optional_uuid()?)
                }
            }
        }
        Ok(())
    }
}

pub struct Converting(pub bool);
pub struct ZombieVillagerVillagerData(pub VillagerData);
pub struct ZombieVillager;
impl ZombieVillager {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZombieBaby(false))
            .add(SpecialType(0))
            .add(DrownedConversion(false))
            .add(Converting(false))
            .add(ZombieVillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerType::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }))
            .build()
    }
}

#[derive(Query)]
struct ZombieVillagerQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zombie_baby: &'a mut ZombieBaby,
    special_type: &'a mut SpecialType,
    drowned_conversion: &'a mut DrownedConversion,
    converting: &'a mut Converting,
    zombie_villager_villager_data: &'a mut ZombieVillagerVillagerData,
}
impl ZombieVillagerQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => *self.special_type = SpecialType(d.value.into_int()?),
                18 => *self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
                19 => *self.converting = Converting(d.value.into_boolean()?),
                20 => {
                    *self.zombie_villager_villager_data =
                        ZombieVillagerVillagerData(d.value.into_villager_data()?)
                }
            }
        }
        Ok(())
    }
}

pub struct ZombifiedPiglin;
impl ZombifiedPiglin {
    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {
        builder
            .add(OnFire(false))
            .add(ShiftKeyDown(false))
            .add(Sprinting(false))
            .add(Swimming(false))
            .add(CurrentlyGlowing(false))
            .add(Invisible(false))
            .add(FallFlying(false))
            .add(AirSupply(Default::default()))
            .add(CustomName(None))
            .add(CustomNameVisible(false))
            .add(Silent(false))
            .add(NoGravity(false))
            .add(Pose::default())
            .add(TicksFrozen(0))
            .add(AutoSpinAttack(false))
            .add(AbstractLivingUsingItem(false))
            .add(Health(1.0))
            .add(AbstractLivingEffectColor(0))
            .add(EffectAmbience(false))
            .add(ArrowCount(0))
            .add(StingerCount(0))
            .add(SleepingPos(None))
            .add(NoAi(false))
            .add(LeftHanded(false))
            .add(Aggressive(false))
            .add(ZombieBaby(false))
            .add(SpecialType(0))
            .add(DrownedConversion(false))
            .build()
    }
}

#[derive(Query)]
struct ZombifiedPiglinQuery<'a> {
    on_fire: &'a mut OnFire,
    shift_key_down: &'a mut ShiftKeyDown,
    sprinting: &'a mut Sprinting,
    swimming: &'a mut Swimming,
    currently_glowing: &'a mut CurrentlyGlowing,
    invisible: &'a mut Invisible,
    fall_flying: &'a mut FallFlying,
    air_supply: &'a mut AirSupply,
    custom_name: &'a mut CustomName,
    custom_name_visible: &'a mut CustomNameVisible,
    silent: &'a mut Silent,
    no_gravity: &'a mut NoGravity,
    pose: &'a mut Pose,
    ticks_frozen: &'a mut TicksFrozen,
    auto_spin_attack: &'a mut AutoSpinAttack,
    abstract_living_using_item: &'a mut AbstractLivingUsingItem,
    health: &'a mut Health,
    abstract_living_effect_color: &'a mut AbstractLivingEffectColor,
    effect_ambience: &'a mut EffectAmbience,
    arrow_count: &'a mut ArrowCount,
    stinger_count: &'a mut StingerCount,
    sleeping_pos: &'a mut SleepingPos,
    no_ai: &'a mut NoAi,
    left_handed: &'a mut LeftHanded,
    aggressive: &'a mut Aggressive,
    zombie_baby: &'a mut ZombieBaby,
    special_type: &'a mut SpecialType,
    drowned_conversion: &'a mut DrownedConversion,
}
impl ZombifiedPiglinQuery<'_> {
    pub fn update_metadata(
        &mut self,
        world: hecs::World,
        entity: hecs::Entity,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    *self.on_fire = OnFire(bitfield & 0x1 != 0);
                    *self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    *self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    *self.swimming = Swimming(bitfield & 0x10 != 0);
                    *self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    *self.invisible = Invisible(bitfield & 0x20 != 0);
                    *self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => *self.air_supply = AirSupply(d.value.into_int()?),
                2 => *self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => *self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => *self.silent = Silent(d.value.into_boolean()?),
                5 => *self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => *self.pose = d.value.into_pose()?,
                7 => *self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    *self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    *self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => *self.health = Health(d.value.into_float()?),
                10 => {
                    *self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => *self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => *self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => *self.stinger_count = StingerCount(d.value.into_int()?),
                14 => *self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    *self.no_ai = NoAi(bitfield & 0x1 != 0);
                    *self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    *self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => *self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => *self.special_type = SpecialType(d.value.into_int()?),
                18 => *self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

fn update_metadata(
    world: hecs::World,
    entity: hecs::Entity,
    data: EntityMetadataItems,
) -> Result<(), UpdateMetadataError> {
    if let Ok(e) = world.query_one_mut::<AllayQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<AreaEffectCloudQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ArmorStandQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ArrowQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<AxolotlQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BatQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BeeQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BlazeQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BoatQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CamelQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CatQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CaveSpiderQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChestBoatQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChestMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChickenQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CodQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CommandBlockMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CowQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CreeperQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DolphinQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DonkeyQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DragonFireballQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DrownedQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EggQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ElderGuardianQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndCrystalQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EnderDragonQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EnderPearlQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndermanQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndermiteQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EvokerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EvokerFangsQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ExperienceBottleQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ExperienceOrbQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EyeOfEnderQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FallingBlockQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FireballQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FireworkRocketQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FishingBobberQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FoxQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FrogQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FurnaceMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GhastQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GiantQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GlowItemFrameQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GlowSquidQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GoatQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GuardianQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HoglinQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HopperMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HorseQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HuskQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<IllusionerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<IronGolemQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ItemQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ItemFrameQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LeashKnotQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LightningBoltQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LlamaQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LlamaSpitQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MagmaCubeQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MarkerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MooshroomQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MuleQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<OcelotQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PaintingQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PandaQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ParrotQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PhantomQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PigQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PiglinQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PiglinBruteQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PillagerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PlayerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PolarBearQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PotionQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PufferfishQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<RabbitQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<RavagerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SalmonQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SheepQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ShulkerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ShulkerBulletQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SilverfishQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SkeletonQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SkeletonHorseQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SlimeQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SmallFireballQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SnowGolemQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SnowballQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpawnerMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpectralArrowQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpiderQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SquidQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<StrayQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<StriderQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TadpoleQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TntQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TntMinecartQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TraderLlamaQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TridentQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TropicalFishQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TurtleQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VexQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VillagerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VindicatorQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WanderingTraderQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WardenQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitchQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherSkeletonQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherSkullQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WolfQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZoglinQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieHorseQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieVillagerQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombifiedPiglinQuery>(entity) {
        e.update_metadata(world, entity, data)?;
        return Ok(());
    }
    Ok(())
}
