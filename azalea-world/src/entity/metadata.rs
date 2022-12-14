// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

#![allow(clippy::clone_on_copy, clippy::derivable_impls)]
use super::{
    EntityDataValue, EntityMetadataItems, OptionalUnsignedInt, Pose, Rotations, VillagerData,
};
use azalea_block::BlockState;
use azalea_chat::FormattedText;
use azalea_core::{BlockPos, Direction, Particle, Slot};
use bevy_ecs::{bundle::Bundle, component::Component};
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

#[derive(Component)]
pub struct OnFire(pub bool);
#[derive(Component)]
pub struct ShiftKeyDown(pub bool);
#[derive(Component)]
pub struct Sprinting(pub bool);
#[derive(Component)]
pub struct Swimming(pub bool);
#[derive(Component)]
pub struct CurrentlyGlowing(pub bool);
#[derive(Component)]
pub struct Invisible(pub bool);
#[derive(Component)]
pub struct FallFlying(pub bool);
#[derive(Component)]
pub struct AirSupply(pub i32);
#[derive(Component)]
pub struct CustomName(pub Option<FormattedText>);
#[derive(Component)]
pub struct CustomNameVisible(pub bool);
#[derive(Component)]
pub struct Silent(pub bool);
#[derive(Component)]
pub struct NoGravity(pub bool);
#[derive(Component)]
pub struct TicksFrozen(pub i32);
#[derive(Component)]
pub struct AutoSpinAttack(pub bool);
#[derive(Component)]
pub struct AbstractLivingUsingItem(pub bool);
#[derive(Component)]
pub struct Health(pub f32);
#[derive(Component)]
pub struct AbstractLivingEffectColor(pub i32);
#[derive(Component)]
pub struct EffectAmbience(pub bool);
#[derive(Component)]
pub struct ArrowCount(pub i32);
#[derive(Component)]
pub struct StingerCount(pub i32);
#[derive(Component)]
pub struct SleepingPos(pub Option<BlockPos>);
#[derive(Component)]
pub struct NoAi(pub bool);
#[derive(Component)]
pub struct LeftHanded(pub bool);
#[derive(Component)]
pub struct Aggressive(pub bool);
#[derive(Component)]
pub struct Dancing(pub bool);
#[derive(Component)]
pub struct CanDuplicate(pub bool);
#[derive(Component)]
pub struct Allay;

#[derive(Bundle)]
struct AllayBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    dancing: Dancing,
    can_duplicate: CanDuplicate,
}
impl AllayBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.dancing = Dancing(d.value.into_boolean()?),
                17 => self.can_duplicate = CanDuplicate(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for AllayBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            dancing: Dancing(false),
            can_duplicate: CanDuplicate(true),
        }
    }
}

#[derive(Component)]
pub struct Radius(pub f32);
#[derive(Component)]
pub struct AreaEffectCloudColor(pub i32);
#[derive(Component)]
pub struct Waiting(pub bool);
#[derive(Component)]
pub struct AreaEffectCloud;

#[derive(Bundle)]
struct AreaEffectCloudBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    radius: Radius,
    area_effect_cloud_color: AreaEffectCloudColor,
    waiting: Waiting,
    particle: Particle,
}
impl AreaEffectCloudBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.radius = Radius(d.value.into_float()?),
                9 => self.area_effect_cloud_color = AreaEffectCloudColor(d.value.into_int()?),
                10 => self.waiting = Waiting(d.value.into_boolean()?),
                11 => self.particle = d.value.into_particle()?,
            }
        }
        Ok(())
    }
}

impl Default for AreaEffectCloudBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            radius: Radius(3.0),
            area_effect_cloud_color: AreaEffectCloudColor(0),
            waiting: Waiting(false),
            particle: Particle::default(),
        }
    }
}

#[derive(Component)]
pub struct Small(pub bool);
#[derive(Component)]
pub struct ShowArms(pub bool);
#[derive(Component)]
pub struct NoBasePlate(pub bool);
#[derive(Component)]
pub struct ArmorStandMarker(pub bool);
#[derive(Component)]
pub struct HeadPose(pub Rotations);
#[derive(Component)]
pub struct BodyPose(pub Rotations);
#[derive(Component)]
pub struct LeftArmPose(pub Rotations);
#[derive(Component)]
pub struct RightArmPose(pub Rotations);
#[derive(Component)]
pub struct LeftLegPose(pub Rotations);
#[derive(Component)]
pub struct RightLegPose(pub Rotations);
#[derive(Component)]
pub struct ArmorStand;

#[derive(Bundle)]
struct ArmorStandBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    small: Small,
    show_arms: ShowArms,
    no_base_plate: NoBasePlate,
    armor_stand_marker: ArmorStandMarker,
    head_pose: HeadPose,
    body_pose: BodyPose,
    left_arm_pose: LeftArmPose,
    right_arm_pose: RightArmPose,
    left_leg_pose: LeftLegPose,
    right_leg_pose: RightLegPose,
}
impl ArmorStandBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.small = Small(bitfield & 0x1 != 0);
                    self.show_arms = ShowArms(bitfield & 0x4 != 0);
                    self.no_base_plate = NoBasePlate(bitfield & 0x8 != 0);
                    self.armor_stand_marker = ArmorStandMarker(bitfield & 0x10 != 0);
                }
                16 => self.head_pose = HeadPose(d.value.into_rotations()?),
                17 => self.body_pose = BodyPose(d.value.into_rotations()?),
                18 => self.left_arm_pose = LeftArmPose(d.value.into_rotations()?),
                19 => self.right_arm_pose = RightArmPose(d.value.into_rotations()?),
                20 => self.left_leg_pose = LeftLegPose(d.value.into_rotations()?),
                21 => self.right_leg_pose = RightLegPose(d.value.into_rotations()?),
            }
        }
        Ok(())
    }
}

impl Default for ArmorStandBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            small: Small(false),
            show_arms: ShowArms(false),
            no_base_plate: NoBasePlate(false),
            armor_stand_marker: ArmorStandMarker(false),
            head_pose: HeadPose(Default::default()),
            body_pose: BodyPose(Default::default()),
            left_arm_pose: LeftArmPose(Default::default()),
            right_arm_pose: RightArmPose(Default::default()),
            left_leg_pose: LeftLegPose(Default::default()),
            right_leg_pose: RightLegPose(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct ArrowCritArrow(pub bool);
#[derive(Component)]
pub struct ArrowShotFromCrossbow(pub bool);
#[derive(Component)]
pub struct ArrowNoPhysics(pub bool);
#[derive(Component)]
pub struct ArrowPierceLevel(pub u8);
#[derive(Component)]
pub struct ArrowEffectColor(pub i32);
#[derive(Component)]
pub struct Arrow;

#[derive(Bundle)]
struct ArrowBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    arrow_crit_arrow: ArrowCritArrow,
    arrow_shot_from_crossbow: ArrowShotFromCrossbow,
    arrow_no_physics: ArrowNoPhysics,
    arrow_pierce_level: ArrowPierceLevel,
    arrow_effect_color: ArrowEffectColor,
}
impl ArrowBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.arrow_crit_arrow = ArrowCritArrow(bitfield & 0x1 != 0);
                    self.arrow_shot_from_crossbow = ArrowShotFromCrossbow(bitfield & 0x4 != 0);
                    self.arrow_no_physics = ArrowNoPhysics(bitfield & 0x2 != 0);
                }
                9 => self.arrow_pierce_level = ArrowPierceLevel(d.value.into_byte()?),
                10 => self.arrow_effect_color = ArrowEffectColor(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ArrowBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            arrow_crit_arrow: ArrowCritArrow(false),
            arrow_shot_from_crossbow: ArrowShotFromCrossbow(false),
            arrow_no_physics: ArrowNoPhysics(false),
            arrow_pierce_level: ArrowPierceLevel(0),
            arrow_effect_color: ArrowEffectColor(-1),
        }
    }
}

#[derive(Component)]
pub struct AbstractAgeableBaby(pub bool);
#[derive(Component)]
pub struct AxolotlVariant(pub i32);
#[derive(Component)]
pub struct PlayingDead(pub bool);
#[derive(Component)]
pub struct AxolotlFromBucket(pub bool);
#[derive(Component)]
pub struct Axolotl;

#[derive(Bundle)]
struct AxolotlBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    axolotl_variant: AxolotlVariant,
    playing_dead: PlayingDead,
    axolotl_from_bucket: AxolotlFromBucket,
}
impl AxolotlBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.axolotl_variant = AxolotlVariant(d.value.into_int()?),
                18 => self.playing_dead = PlayingDead(d.value.into_boolean()?),
                19 => self.axolotl_from_bucket = AxolotlFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for AxolotlBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            axolotl_variant: AxolotlVariant(0),
            playing_dead: PlayingDead(false),
            axolotl_from_bucket: AxolotlFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct Resting(pub bool);
#[derive(Component)]
pub struct Bat;

#[derive(Bundle)]
struct BatBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    resting: Resting,
}
impl BatBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.resting = Resting(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for BatBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            resting: Resting(false),
        }
    }
}

#[derive(Component)]
pub struct HasNectar(pub bool);
#[derive(Component)]
pub struct HasStung(pub bool);
#[derive(Component)]
pub struct BeeRolling(pub bool);
#[derive(Component)]
pub struct BeeRemainingAngerTime(pub i32);
#[derive(Component)]
pub struct Bee;

#[derive(Bundle)]
struct BeeBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    has_nectar: HasNectar,
    has_stung: HasStung,
    bee_rolling: BeeRolling,
    bee_remaining_anger_time: BeeRemainingAngerTime,
}
impl BeeBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.has_nectar = HasNectar(bitfield & 0x8 != 0);
                    self.has_stung = HasStung(bitfield & 0x4 != 0);
                    self.bee_rolling = BeeRolling(bitfield & 0x2 != 0);
                }
                18 => self.bee_remaining_anger_time = BeeRemainingAngerTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for BeeBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            has_nectar: HasNectar(false),
            has_stung: HasStung(false),
            bee_rolling: BeeRolling(false),
            bee_remaining_anger_time: BeeRemainingAngerTime(0),
        }
    }
}

#[derive(Component)]
pub struct Charged(pub bool);
#[derive(Component)]
pub struct Blaze;

#[derive(Bundle)]
struct BlazeBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    charged: Charged,
}
impl BlazeBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.charged = Charged(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for BlazeBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            charged: Charged(false),
        }
    }
}

#[derive(Component)]
pub struct BoatHurt(pub i32);
#[derive(Component)]
pub struct BoatHurtdir(pub i32);
#[derive(Component)]
pub struct BoatDamage(pub f32);
#[derive(Component)]
pub struct BoatKind(pub i32);
#[derive(Component)]
pub struct PaddleLeft(pub bool);
#[derive(Component)]
pub struct PaddleRight(pub bool);
#[derive(Component)]
pub struct BubbleTime(pub i32);
#[derive(Component)]
pub struct Boat;

#[derive(Bundle)]
struct BoatBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    boat_hurt: BoatHurt,
    boat_hurtdir: BoatHurtdir,
    boat_damage: BoatDamage,
    boat_kind: BoatKind,
    paddle_left: PaddleLeft,
    paddle_right: PaddleRight,
    bubble_time: BubbleTime,
}
impl BoatBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.boat_hurt = BoatHurt(d.value.into_int()?),
                9 => self.boat_hurtdir = BoatHurtdir(d.value.into_int()?),
                10 => self.boat_damage = BoatDamage(d.value.into_float()?),
                11 => self.boat_kind = BoatKind(d.value.into_int()?),
                12 => self.paddle_left = PaddleLeft(d.value.into_boolean()?),
                13 => self.paddle_right = PaddleRight(d.value.into_boolean()?),
                14 => self.bubble_time = BubbleTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for BoatBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            boat_hurt: BoatHurt(0),
            boat_hurtdir: BoatHurtdir(1),
            boat_damage: BoatDamage(0.0),
            boat_kind: BoatKind(Default::default()),
            paddle_left: PaddleLeft(false),
            paddle_right: PaddleRight(false),
            bubble_time: BubbleTime(0),
        }
    }
}

#[derive(Component)]
pub struct CamelTamed(pub bool);
#[derive(Component)]
pub struct CamelEating(pub bool);
#[derive(Component)]
pub struct CamelStanding(pub bool);
#[derive(Component)]
pub struct CamelBred(pub bool);
#[derive(Component)]
pub struct CamelSaddled(pub bool);
#[derive(Component)]
pub struct CamelOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct Dash(pub bool);
#[derive(Component)]
pub struct LastPoseChangeTick(pub i64);
#[derive(Component)]
pub struct Camel;

#[derive(Bundle)]
struct CamelBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    camel_tamed: CamelTamed,
    camel_eating: CamelEating,
    camel_standing: CamelStanding,
    camel_bred: CamelBred,
    camel_saddled: CamelSaddled,
    camel_owner_uuid: CamelOwnerUuid,
    dash: Dash,
    last_pose_change_tick: LastPoseChangeTick,
}
impl CamelBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.camel_tamed = CamelTamed(bitfield & 0x2 != 0);
                    self.camel_eating = CamelEating(bitfield & 0x10 != 0);
                    self.camel_standing = CamelStanding(bitfield & 0x20 != 0);
                    self.camel_bred = CamelBred(bitfield & 0x8 != 0);
                    self.camel_saddled = CamelSaddled(bitfield & 0x4 != 0);
                }
                18 => self.camel_owner_uuid = CamelOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.dash = Dash(d.value.into_boolean()?),
                20 => self.last_pose_change_tick = LastPoseChangeTick(d.value.into_long()?),
            }
        }
        Ok(())
    }
}

impl Default for CamelBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            camel_tamed: CamelTamed(false),
            camel_eating: CamelEating(false),
            camel_standing: CamelStanding(false),
            camel_bred: CamelBred(false),
            camel_saddled: CamelSaddled(false),
            camel_owner_uuid: CamelOwnerUuid(None),
            dash: Dash(false),
            last_pose_change_tick: LastPoseChangeTick(-52),
        }
    }
}

#[derive(Component)]
pub struct Tame(pub bool);
#[derive(Component)]
pub struct InSittingPose(pub bool);
#[derive(Component)]
pub struct Owneruuid(pub Option<Uuid>);
#[derive(Component)]
pub struct CatVariant(pub azalea_registry::CatVariant);
#[derive(Component)]
pub struct IsLying(pub bool);
#[derive(Component)]
pub struct RelaxStateOne(pub bool);
#[derive(Component)]
pub struct CatCollarColor(pub i32);
#[derive(Component)]
pub struct Cat;

#[derive(Bundle)]
struct CatBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
    cat_variant: CatVariant,
    is_lying: IsLying,
    relax_state_one: RelaxStateOne,
    cat_collar_color: CatCollarColor,
}
impl CatBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.tame = Tame(bitfield & 0x4 != 0);
                    self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => self.cat_variant = CatVariant(d.value.into_cat_variant()?),
                20 => self.is_lying = IsLying(d.value.into_boolean()?),
                21 => self.relax_state_one = RelaxStateOne(d.value.into_boolean()?),
                22 => self.cat_collar_color = CatCollarColor(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for CatBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
            cat_variant: CatVariant(azalea_registry::CatVariant::Tabby),
            is_lying: IsLying(false),
            relax_state_one: RelaxStateOne(false),
            cat_collar_color: CatCollarColor(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct Climbing(pub bool);
#[derive(Component)]
pub struct CaveSpider;

#[derive(Bundle)]
struct CaveSpiderBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    climbing: Climbing,
}
impl CaveSpiderBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.climbing = Climbing(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for CaveSpiderBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            climbing: Climbing(false),
        }
    }
}

#[derive(Component)]
pub struct ChestBoat;

#[derive(Bundle)]
struct ChestBoatBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    boat_hurt: BoatHurt,
    boat_hurtdir: BoatHurtdir,
    boat_damage: BoatDamage,
    boat_kind: BoatKind,
    paddle_left: PaddleLeft,
    paddle_right: PaddleRight,
    bubble_time: BubbleTime,
}
impl ChestBoatBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.boat_hurt = BoatHurt(d.value.into_int()?),
                9 => self.boat_hurtdir = BoatHurtdir(d.value.into_int()?),
                10 => self.boat_damage = BoatDamage(d.value.into_float()?),
                11 => self.boat_kind = BoatKind(d.value.into_int()?),
                12 => self.paddle_left = PaddleLeft(d.value.into_boolean()?),
                13 => self.paddle_right = PaddleRight(d.value.into_boolean()?),
                14 => self.bubble_time = BubbleTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ChestBoatBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            boat_hurt: BoatHurt(0),
            boat_hurtdir: BoatHurtdir(1),
            boat_damage: BoatDamage(0.0),
            boat_kind: BoatKind(Default::default()),
            paddle_left: PaddleLeft(false),
            paddle_right: PaddleRight(false),
            bubble_time: BubbleTime(0),
        }
    }
}

#[derive(Component)]
pub struct AbstractMinecartHurt(pub i32);
#[derive(Component)]
pub struct AbstractMinecartHurtdir(pub i32);
#[derive(Component)]
pub struct AbstractMinecartDamage(pub f32);
#[derive(Component)]
pub struct DisplayBlock(pub i32);
#[derive(Component)]
pub struct DisplayOffset(pub i32);
#[derive(Component)]
pub struct CustomDisplay(pub bool);
#[derive(Component)]
pub struct ChestMinecart;

#[derive(Bundle)]
struct ChestMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl ChestMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for ChestMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct Chicken;

#[derive(Bundle)]
struct ChickenBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
}
impl ChickenBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for ChickenBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
        }
    }
}

#[derive(Component)]
pub struct CodFromBucket(pub bool);
#[derive(Component)]
pub struct Cod;

#[derive(Bundle)]
struct CodBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    cod_from_bucket: CodFromBucket,
}
impl CodBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.cod_from_bucket = CodFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for CodBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            cod_from_bucket: CodFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct CommandName(pub String);
#[derive(Component)]
pub struct LastOutput(pub FormattedText);
#[derive(Component)]
pub struct CommandBlockMinecart;

#[derive(Bundle)]
struct CommandBlockMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
    command_name: CommandName,
    last_output: LastOutput,
}
impl CommandBlockMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
                14 => self.command_name = CommandName(d.value.into_string()?),
                15 => self.last_output = LastOutput(d.value.into_formatted_text()?),
            }
        }
        Ok(())
    }
}

impl Default for CommandBlockMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
            command_name: CommandName("".to_string()),
            last_output: LastOutput(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct Cow;

#[derive(Bundle)]
struct CowBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
}
impl CowBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for CowBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
        }
    }
}

#[derive(Component)]
pub struct SwellDir(pub i32);
#[derive(Component)]
pub struct IsPowered(pub bool);
#[derive(Component)]
pub struct IsIgnited(pub bool);
#[derive(Component)]
pub struct Creeper;

#[derive(Bundle)]
struct CreeperBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    swell_dir: SwellDir,
    is_powered: IsPowered,
    is_ignited: IsIgnited,
}
impl CreeperBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.swell_dir = SwellDir(d.value.into_int()?),
                17 => self.is_powered = IsPowered(d.value.into_boolean()?),
                18 => self.is_ignited = IsIgnited(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for CreeperBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            swell_dir: SwellDir(-1),
            is_powered: IsPowered(false),
            is_ignited: IsIgnited(false),
        }
    }
}

#[derive(Component)]
pub struct TreasurePos(pub BlockPos);
#[derive(Component)]
pub struct GotFish(pub bool);
#[derive(Component)]
pub struct MoistnessLevel(pub i32);
#[derive(Component)]
pub struct Dolphin;

#[derive(Bundle)]
struct DolphinBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    treasure_pos: TreasurePos,
    got_fish: GotFish,
    moistness_level: MoistnessLevel,
}
impl DolphinBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.treasure_pos = TreasurePos(d.value.into_block_pos()?),
                17 => self.got_fish = GotFish(d.value.into_boolean()?),
                18 => self.moistness_level = MoistnessLevel(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for DolphinBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            treasure_pos: TreasurePos(BlockPos::new(0, 0, 0)),
            got_fish: GotFish(false),
            moistness_level: MoistnessLevel(2400),
        }
    }
}

#[derive(Component)]
pub struct DonkeyTamed(pub bool);
#[derive(Component)]
pub struct DonkeyEating(pub bool);
#[derive(Component)]
pub struct DonkeyStanding(pub bool);
#[derive(Component)]
pub struct DonkeyBred(pub bool);
#[derive(Component)]
pub struct DonkeySaddled(pub bool);
#[derive(Component)]
pub struct DonkeyOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct DonkeyChest(pub bool);
#[derive(Component)]
pub struct Donkey;

#[derive(Bundle)]
struct DonkeyBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    donkey_tamed: DonkeyTamed,
    donkey_eating: DonkeyEating,
    donkey_standing: DonkeyStanding,
    donkey_bred: DonkeyBred,
    donkey_saddled: DonkeySaddled,
    donkey_owner_uuid: DonkeyOwnerUuid,
    donkey_chest: DonkeyChest,
}
impl DonkeyBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.donkey_tamed = DonkeyTamed(bitfield & 0x2 != 0);
                    self.donkey_eating = DonkeyEating(bitfield & 0x10 != 0);
                    self.donkey_standing = DonkeyStanding(bitfield & 0x20 != 0);
                    self.donkey_bred = DonkeyBred(bitfield & 0x8 != 0);
                    self.donkey_saddled = DonkeySaddled(bitfield & 0x4 != 0);
                }
                18 => self.donkey_owner_uuid = DonkeyOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.donkey_chest = DonkeyChest(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for DonkeyBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            donkey_tamed: DonkeyTamed(false),
            donkey_eating: DonkeyEating(false),
            donkey_standing: DonkeyStanding(false),
            donkey_bred: DonkeyBred(false),
            donkey_saddled: DonkeySaddled(false),
            donkey_owner_uuid: DonkeyOwnerUuid(None),
            donkey_chest: DonkeyChest(false),
        }
    }
}

#[derive(Component)]
pub struct DragonFireball;

#[derive(Bundle)]
struct DragonFireballBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl DragonFireballBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for DragonFireballBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct ZombieBaby(pub bool);
#[derive(Component)]
pub struct SpecialType(pub i32);
#[derive(Component)]
pub struct DrownedConversion(pub bool);
#[derive(Component)]
pub struct Drowned;

#[derive(Bundle)]
struct DrownedBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl DrownedBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => self.special_type = SpecialType(d.value.into_int()?),
                18 => self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for DrownedBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

#[derive(Component)]
pub struct EggItemStack(pub Slot);
#[derive(Component)]
pub struct Egg;

#[derive(Bundle)]
struct EggBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    egg_item_stack: EggItemStack,
}
impl EggBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.egg_item_stack = EggItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for EggBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            egg_item_stack: EggItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct Moving(pub bool);
#[derive(Component)]
pub struct AttackTarget(pub i32);
#[derive(Component)]
pub struct ElderGuardian;

#[derive(Bundle)]
struct ElderGuardianBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    moving: Moving,
    attack_target: AttackTarget,
}
impl ElderGuardianBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.moving = Moving(d.value.into_boolean()?),
                17 => self.attack_target = AttackTarget(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ElderGuardianBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            moving: Moving(false),
            attack_target: AttackTarget(0),
        }
    }
}

#[derive(Component)]
pub struct BeamTarget(pub Option<BlockPos>);
#[derive(Component)]
pub struct ShowBottom(pub bool);
#[derive(Component)]
pub struct EndCrystal;

#[derive(Bundle)]
struct EndCrystalBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    beam_target: BeamTarget,
    show_bottom: ShowBottom,
}
impl EndCrystalBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.beam_target = BeamTarget(d.value.into_optional_block_pos()?),
                9 => self.show_bottom = ShowBottom(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for EndCrystalBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            beam_target: BeamTarget(None),
            show_bottom: ShowBottom(true),
        }
    }
}

#[derive(Component)]
pub struct Phase(pub i32);
#[derive(Component)]
pub struct EnderDragon;

#[derive(Bundle)]
struct EnderDragonBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    phase: Phase,
}
impl EnderDragonBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.phase = Phase(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for EnderDragonBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            phase: Phase(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct EnderPearlItemStack(pub Slot);
#[derive(Component)]
pub struct EnderPearl;

#[derive(Bundle)]
struct EnderPearlBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    ender_pearl_item_stack: EnderPearlItemStack,
}
impl EnderPearlBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.ender_pearl_item_stack = EnderPearlItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for EnderPearlBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            ender_pearl_item_stack: EnderPearlItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct CarryState(pub BlockState);
#[derive(Component)]
pub struct Creepy(pub bool);
#[derive(Component)]
pub struct StaredAt(pub bool);
#[derive(Component)]
pub struct Enderman;

#[derive(Bundle)]
struct EndermanBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    carry_state: CarryState,
    creepy: Creepy,
    stared_at: StaredAt,
}
impl EndermanBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.carry_state = CarryState(d.value.into_block_state()?),
                17 => self.creepy = Creepy(d.value.into_boolean()?),
                18 => self.stared_at = StaredAt(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for EndermanBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            carry_state: CarryState(BlockState::Air),
            creepy: Creepy(false),
            stared_at: StaredAt(false),
        }
    }
}

#[derive(Component)]
pub struct Endermite;

#[derive(Bundle)]
struct EndermiteBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl EndermiteBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for EndermiteBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct EvokerIsCelebrating(pub bool);
#[derive(Component)]
pub struct EvokerSpellCasting(pub u8);
#[derive(Component)]
pub struct Evoker;

#[derive(Bundle)]
struct EvokerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    evoker_is_celebrating: EvokerIsCelebrating,
    evoker_spell_casting: EvokerSpellCasting,
}
impl EvokerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.evoker_is_celebrating = EvokerIsCelebrating(d.value.into_boolean()?),
                17 => self.evoker_spell_casting = EvokerSpellCasting(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

impl Default for EvokerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            evoker_is_celebrating: EvokerIsCelebrating(false),
            evoker_spell_casting: EvokerSpellCasting(0),
        }
    }
}

#[derive(Component)]
pub struct EvokerFangs;

#[derive(Bundle)]
struct EvokerFangsBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl EvokerFangsBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for EvokerFangsBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct ExperienceBottleItemStack(pub Slot);
#[derive(Component)]
pub struct ExperienceBottle;

#[derive(Bundle)]
struct ExperienceBottleBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    experience_bottle_item_stack: ExperienceBottleItemStack,
}
impl ExperienceBottleBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    self.experience_bottle_item_stack =
                        ExperienceBottleItemStack(d.value.into_item_stack()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for ExperienceBottleBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            experience_bottle_item_stack: ExperienceBottleItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct ExperienceOrb;

#[derive(Bundle)]
struct ExperienceOrbBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl ExperienceOrbBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ExperienceOrbBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct EyeOfEnderItemStack(pub Slot);
#[derive(Component)]
pub struct EyeOfEnder;

#[derive(Bundle)]
struct EyeOfEnderBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    eye_of_ender_item_stack: EyeOfEnderItemStack,
}
impl EyeOfEnderBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.eye_of_ender_item_stack = EyeOfEnderItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for EyeOfEnderBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            eye_of_ender_item_stack: EyeOfEnderItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct StartPos(pub BlockPos);
#[derive(Component)]
pub struct FallingBlock;

#[derive(Bundle)]
struct FallingBlockBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    start_pos: StartPos,
}
impl FallingBlockBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.start_pos = StartPos(d.value.into_block_pos()?),
            }
        }
        Ok(())
    }
}

impl Default for FallingBlockBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            start_pos: StartPos(BlockPos::new(0, 0, 0)),
        }
    }
}

#[derive(Component)]
pub struct FireballItemStack(pub Slot);
#[derive(Component)]
pub struct Fireball;

#[derive(Bundle)]
struct FireballBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    fireball_item_stack: FireballItemStack,
}
impl FireballBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.fireball_item_stack = FireballItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for FireballBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            fireball_item_stack: FireballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct FireworksItem(pub Slot);
#[derive(Component)]
pub struct AttachedToTarget(pub OptionalUnsignedInt);
#[derive(Component)]
pub struct ShotAtAngle(pub bool);
#[derive(Component)]
pub struct FireworkRocket;

#[derive(Bundle)]
struct FireworkRocketBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    fireworks_item: FireworksItem,
    attached_to_target: AttachedToTarget,
    shot_at_angle: ShotAtAngle,
}
impl FireworkRocketBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.fireworks_item = FireworksItem(d.value.into_item_stack()?),
                9 => {
                    self.attached_to_target =
                        AttachedToTarget(d.value.into_optional_unsigned_int()?)
                }
                10 => self.shot_at_angle = ShotAtAngle(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for FireworkRocketBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            fireworks_item: FireworksItem(Slot::Empty),
            attached_to_target: AttachedToTarget(OptionalUnsignedInt(None)),
            shot_at_angle: ShotAtAngle(false),
        }
    }
}

#[derive(Component)]
pub struct HookedEntity(pub i32);
#[derive(Component)]
pub struct Biting(pub bool);
#[derive(Component)]
pub struct FishingBobber;

#[derive(Bundle)]
struct FishingBobberBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    hooked_entity: HookedEntity,
    biting: Biting,
}
impl FishingBobberBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.hooked_entity = HookedEntity(d.value.into_int()?),
                9 => self.biting = Biting(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for FishingBobberBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            hooked_entity: HookedEntity(0),
            biting: Biting(false),
        }
    }
}

#[derive(Component)]
pub struct FoxKind(pub i32);
#[derive(Component)]
pub struct FoxSitting(pub bool);
#[derive(Component)]
pub struct Faceplanted(pub bool);
#[derive(Component)]
pub struct Sleeping(pub bool);
#[derive(Component)]
pub struct Pouncing(pub bool);
#[derive(Component)]
pub struct Crouching(pub bool);
#[derive(Component)]
pub struct FoxInterested(pub bool);
#[derive(Component)]
pub struct TrustedId0(pub Option<Uuid>);
#[derive(Component)]
pub struct TrustedId1(pub Option<Uuid>);
#[derive(Component)]
pub struct Fox;

#[derive(Bundle)]
struct FoxBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    fox_kind: FoxKind,
    fox_sitting: FoxSitting,
    faceplanted: Faceplanted,
    sleeping: Sleeping,
    pouncing: Pouncing,
    crouching: Crouching,
    fox_interested: FoxInterested,
    trusted_id_0: TrustedId0,
    trusted_id_1: TrustedId1,
}
impl FoxBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.fox_kind = FoxKind(d.value.into_int()?),
                18 => {
                    let bitfield = d.value.into_byte()?;
                    self.fox_sitting = FoxSitting(bitfield & 0x1 != 0);
                    self.faceplanted = Faceplanted(bitfield & 0x40 != 0);
                    self.sleeping = Sleeping(bitfield & 0x20 != 0);
                    self.pouncing = Pouncing(bitfield & 0x10 != 0);
                    self.crouching = Crouching(bitfield & 0x4 != 0);
                    self.fox_interested = FoxInterested(bitfield & 0x8 != 0);
                }
                19 => self.trusted_id_0 = TrustedId0(d.value.into_optional_uuid()?),
                20 => self.trusted_id_1 = TrustedId1(d.value.into_optional_uuid()?),
            }
        }
        Ok(())
    }
}

impl Default for FoxBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            fox_kind: FoxKind(0),
            fox_sitting: FoxSitting(false),
            faceplanted: Faceplanted(false),
            sleeping: Sleeping(false),
            pouncing: Pouncing(false),
            crouching: Crouching(false),
            fox_interested: FoxInterested(false),
            trusted_id_0: TrustedId0(None),
            trusted_id_1: TrustedId1(None),
        }
    }
}

#[derive(Component)]
pub struct FrogVariant(pub azalea_registry::FrogVariant);
#[derive(Component)]
pub struct TongueTarget(pub OptionalUnsignedInt);
#[derive(Component)]
pub struct Frog;

#[derive(Bundle)]
struct FrogBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    frog_variant: FrogVariant,
    tongue_target: TongueTarget,
}
impl FrogBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.frog_variant = FrogVariant(d.value.into_frog_variant()?),
                18 => self.tongue_target = TongueTarget(d.value.into_optional_unsigned_int()?),
            }
        }
        Ok(())
    }
}

impl Default for FrogBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            frog_variant: FrogVariant(azalea_registry::FrogVariant::Temperate),
            tongue_target: TongueTarget(OptionalUnsignedInt(None)),
        }
    }
}

#[derive(Component)]
pub struct Fuel(pub bool);
#[derive(Component)]
pub struct FurnaceMinecart;

#[derive(Bundle)]
struct FurnaceMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
    fuel: Fuel,
}
impl FurnaceMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
                14 => self.fuel = Fuel(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for FurnaceMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
            fuel: Fuel(false),
        }
    }
}

#[derive(Component)]
pub struct IsCharging(pub bool);
#[derive(Component)]
pub struct Ghast;

#[derive(Bundle)]
struct GhastBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    is_charging: IsCharging,
}
impl GhastBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.is_charging = IsCharging(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for GhastBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            is_charging: IsCharging(false),
        }
    }
}

#[derive(Component)]
pub struct Giant;

#[derive(Bundle)]
struct GiantBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl GiantBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for GiantBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct ItemFrameItem(pub Slot);
#[derive(Component)]
pub struct Rotation(pub i32);
#[derive(Component)]
pub struct GlowItemFrame;

#[derive(Bundle)]
struct GlowItemFrameBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    item_frame_item: ItemFrameItem,
    rotation: Rotation,
}
impl GlowItemFrameBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.item_frame_item = ItemFrameItem(d.value.into_item_stack()?),
                9 => self.rotation = Rotation(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for GlowItemFrameBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            item_frame_item: ItemFrameItem(Slot::Empty),
            rotation: Rotation(0),
        }
    }
}

#[derive(Component)]
pub struct DarkTicksRemaining(pub i32);
#[derive(Component)]
pub struct GlowSquid;

#[derive(Bundle)]
struct GlowSquidBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    dark_ticks_remaining: DarkTicksRemaining,
}
impl GlowSquidBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.dark_ticks_remaining = DarkTicksRemaining(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for GlowSquidBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            dark_ticks_remaining: DarkTicksRemaining(0),
        }
    }
}

#[derive(Component)]
pub struct IsScreamingGoat(pub bool);
#[derive(Component)]
pub struct HasLeftHorn(pub bool);
#[derive(Component)]
pub struct HasRightHorn(pub bool);
#[derive(Component)]
pub struct Goat;

#[derive(Bundle)]
struct GoatBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    is_screaming_goat: IsScreamingGoat,
    has_left_horn: HasLeftHorn,
    has_right_horn: HasRightHorn,
}
impl GoatBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.is_screaming_goat = IsScreamingGoat(d.value.into_boolean()?),
                18 => self.has_left_horn = HasLeftHorn(d.value.into_boolean()?),
                19 => self.has_right_horn = HasRightHorn(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for GoatBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            is_screaming_goat: IsScreamingGoat(false),
            has_left_horn: HasLeftHorn(true),
            has_right_horn: HasRightHorn(true),
        }
    }
}

#[derive(Component)]
pub struct Guardian;

#[derive(Bundle)]
struct GuardianBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    moving: Moving,
    attack_target: AttackTarget,
}
impl GuardianBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.moving = Moving(d.value.into_boolean()?),
                17 => self.attack_target = AttackTarget(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for GuardianBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            moving: Moving(false),
            attack_target: AttackTarget(0),
        }
    }
}

#[derive(Component)]
pub struct HoglinImmuneToZombification(pub bool);
#[derive(Component)]
pub struct Hoglin;

#[derive(Bundle)]
struct HoglinBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    hoglin_immune_to_zombification: HoglinImmuneToZombification,
}
impl HoglinBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    self.hoglin_immune_to_zombification =
                        HoglinImmuneToZombification(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for HoglinBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            hoglin_immune_to_zombification: HoglinImmuneToZombification(false),
        }
    }
}

#[derive(Component)]
pub struct HopperMinecart;

#[derive(Bundle)]
struct HopperMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl HopperMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for HopperMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct HorseTamed(pub bool);
#[derive(Component)]
pub struct HorseEating(pub bool);
#[derive(Component)]
pub struct HorseStanding(pub bool);
#[derive(Component)]
pub struct HorseBred(pub bool);
#[derive(Component)]
pub struct HorseSaddled(pub bool);
#[derive(Component)]
pub struct HorseOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct HorseTypeVariant(pub i32);
#[derive(Component)]
pub struct Horse;

#[derive(Bundle)]
struct HorseBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    horse_tamed: HorseTamed,
    horse_eating: HorseEating,
    horse_standing: HorseStanding,
    horse_bred: HorseBred,
    horse_saddled: HorseSaddled,
    horse_owner_uuid: HorseOwnerUuid,
    horse_type_variant: HorseTypeVariant,
}
impl HorseBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.horse_tamed = HorseTamed(bitfield & 0x2 != 0);
                    self.horse_eating = HorseEating(bitfield & 0x10 != 0);
                    self.horse_standing = HorseStanding(bitfield & 0x20 != 0);
                    self.horse_bred = HorseBred(bitfield & 0x8 != 0);
                    self.horse_saddled = HorseSaddled(bitfield & 0x4 != 0);
                }
                18 => self.horse_owner_uuid = HorseOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.horse_type_variant = HorseTypeVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for HorseBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            horse_tamed: HorseTamed(false),
            horse_eating: HorseEating(false),
            horse_standing: HorseStanding(false),
            horse_bred: HorseBred(false),
            horse_saddled: HorseSaddled(false),
            horse_owner_uuid: HorseOwnerUuid(None),
            horse_type_variant: HorseTypeVariant(0),
        }
    }
}

#[derive(Component)]
pub struct Husk;

#[derive(Bundle)]
struct HuskBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl HuskBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => self.special_type = SpecialType(d.value.into_int()?),
                18 => self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for HuskBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

#[derive(Component)]
pub struct IllusionerIsCelebrating(pub bool);
#[derive(Component)]
pub struct IllusionerSpellCasting(pub u8);
#[derive(Component)]
pub struct Illusioner;

#[derive(Bundle)]
struct IllusionerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    illusioner_is_celebrating: IllusionerIsCelebrating,
    illusioner_spell_casting: IllusionerSpellCasting,
}
impl IllusionerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    self.illusioner_is_celebrating =
                        IllusionerIsCelebrating(d.value.into_boolean()?)
                }
                17 => self.illusioner_spell_casting = IllusionerSpellCasting(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

impl Default for IllusionerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            illusioner_is_celebrating: IllusionerIsCelebrating(false),
            illusioner_spell_casting: IllusionerSpellCasting(0),
        }
    }
}

#[derive(Component)]
pub struct PlayerCreated(pub bool);
#[derive(Component)]
pub struct IronGolem;

#[derive(Bundle)]
struct IronGolemBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    player_created: PlayerCreated,
}
impl IronGolemBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.player_created = PlayerCreated(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for IronGolemBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            player_created: PlayerCreated(false),
        }
    }
}

#[derive(Component)]
pub struct ItemItem(pub Slot);
#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
struct ItemBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    item_item: ItemItem,
}
impl ItemBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.item_item = ItemItem(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            item_item: ItemItem(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct ItemFrame;

#[derive(Bundle)]
struct ItemFrameBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    item_frame_item: ItemFrameItem,
    rotation: Rotation,
}
impl ItemFrameBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.item_frame_item = ItemFrameItem(d.value.into_item_stack()?),
                9 => self.rotation = Rotation(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ItemFrameBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            item_frame_item: ItemFrameItem(Slot::Empty),
            rotation: Rotation(0),
        }
    }
}

#[derive(Component)]
pub struct LeashKnot;

#[derive(Bundle)]
struct LeashKnotBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl LeashKnotBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for LeashKnotBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct LightningBolt;

#[derive(Bundle)]
struct LightningBoltBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl LightningBoltBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for LightningBoltBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct LlamaTamed(pub bool);
#[derive(Component)]
pub struct LlamaEating(pub bool);
#[derive(Component)]
pub struct LlamaStanding(pub bool);
#[derive(Component)]
pub struct LlamaBred(pub bool);
#[derive(Component)]
pub struct LlamaSaddled(pub bool);
#[derive(Component)]
pub struct LlamaOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct LlamaChest(pub bool);
#[derive(Component)]
pub struct Strength(pub i32);
#[derive(Component)]
pub struct Swag(pub i32);
#[derive(Component)]
pub struct LlamaVariant(pub i32);
#[derive(Component)]
pub struct Llama;

#[derive(Bundle)]
struct LlamaBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    llama_tamed: LlamaTamed,
    llama_eating: LlamaEating,
    llama_standing: LlamaStanding,
    llama_bred: LlamaBred,
    llama_saddled: LlamaSaddled,
    llama_owner_uuid: LlamaOwnerUuid,
    llama_chest: LlamaChest,
    strength: Strength,
    swag: Swag,
    llama_variant: LlamaVariant,
}
impl LlamaBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.llama_tamed = LlamaTamed(bitfield & 0x2 != 0);
                    self.llama_eating = LlamaEating(bitfield & 0x10 != 0);
                    self.llama_standing = LlamaStanding(bitfield & 0x20 != 0);
                    self.llama_bred = LlamaBred(bitfield & 0x8 != 0);
                    self.llama_saddled = LlamaSaddled(bitfield & 0x4 != 0);
                }
                18 => self.llama_owner_uuid = LlamaOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.llama_chest = LlamaChest(d.value.into_boolean()?),
                20 => self.strength = Strength(d.value.into_int()?),
                21 => self.swag = Swag(d.value.into_int()?),
                22 => self.llama_variant = LlamaVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for LlamaBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            llama_tamed: LlamaTamed(false),
            llama_eating: LlamaEating(false),
            llama_standing: LlamaStanding(false),
            llama_bred: LlamaBred(false),
            llama_saddled: LlamaSaddled(false),
            llama_owner_uuid: LlamaOwnerUuid(None),
            llama_chest: LlamaChest(false),
            strength: Strength(0),
            swag: Swag(-1),
            llama_variant: LlamaVariant(0),
        }
    }
}

#[derive(Component)]
pub struct LlamaSpit;

#[derive(Bundle)]
struct LlamaSpitBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl LlamaSpitBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for LlamaSpitBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct SlimeSize(pub i32);
#[derive(Component)]
pub struct MagmaCube;

#[derive(Bundle)]
struct MagmaCubeBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    slime_size: SlimeSize,
}
impl MagmaCubeBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.slime_size = SlimeSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for MagmaCubeBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            slime_size: SlimeSize(1),
        }
    }
}

#[derive(Component)]
pub struct Marker;

#[derive(Bundle)]
struct MarkerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl MarkerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for MarkerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct Minecart;

#[derive(Bundle)]
struct MinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl MinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for MinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct MooshroomKind(pub String);
#[derive(Component)]
pub struct Mooshroom;

#[derive(Bundle)]
struct MooshroomBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    mooshroom_kind: MooshroomKind,
}
impl MooshroomBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.mooshroom_kind = MooshroomKind(d.value.into_string()?),
            }
        }
        Ok(())
    }
}

impl Default for MooshroomBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            mooshroom_kind: MooshroomKind(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct MuleTamed(pub bool);
#[derive(Component)]
pub struct MuleEating(pub bool);
#[derive(Component)]
pub struct MuleStanding(pub bool);
#[derive(Component)]
pub struct MuleBred(pub bool);
#[derive(Component)]
pub struct MuleSaddled(pub bool);
#[derive(Component)]
pub struct MuleOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct MuleChest(pub bool);
#[derive(Component)]
pub struct Mule;

#[derive(Bundle)]
struct MuleBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    mule_tamed: MuleTamed,
    mule_eating: MuleEating,
    mule_standing: MuleStanding,
    mule_bred: MuleBred,
    mule_saddled: MuleSaddled,
    mule_owner_uuid: MuleOwnerUuid,
    mule_chest: MuleChest,
}
impl MuleBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.mule_tamed = MuleTamed(bitfield & 0x2 != 0);
                    self.mule_eating = MuleEating(bitfield & 0x10 != 0);
                    self.mule_standing = MuleStanding(bitfield & 0x20 != 0);
                    self.mule_bred = MuleBred(bitfield & 0x8 != 0);
                    self.mule_saddled = MuleSaddled(bitfield & 0x4 != 0);
                }
                18 => self.mule_owner_uuid = MuleOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.mule_chest = MuleChest(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for MuleBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            mule_tamed: MuleTamed(false),
            mule_eating: MuleEating(false),
            mule_standing: MuleStanding(false),
            mule_bred: MuleBred(false),
            mule_saddled: MuleSaddled(false),
            mule_owner_uuid: MuleOwnerUuid(None),
            mule_chest: MuleChest(false),
        }
    }
}

#[derive(Component)]
pub struct Trusting(pub bool);
#[derive(Component)]
pub struct Ocelot;

#[derive(Bundle)]
struct OcelotBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    trusting: Trusting,
}
impl OcelotBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.trusting = Trusting(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for OcelotBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            trusting: Trusting(false),
        }
    }
}

#[derive(Component)]
pub struct PaintingVariant(pub azalea_registry::PaintingVariant);
#[derive(Component)]
pub struct Painting;

#[derive(Bundle)]
struct PaintingBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    painting_variant: PaintingVariant,
}
impl PaintingBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.painting_variant = PaintingVariant(d.value.into_painting_variant()?),
            }
        }
        Ok(())
    }
}

impl Default for PaintingBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            painting_variant: PaintingVariant(azalea_registry::PaintingVariant::Kebab),
        }
    }
}

#[derive(Component)]
pub struct PandaUnhappyCounter(pub i32);
#[derive(Component)]
pub struct SneezeCounter(pub i32);
#[derive(Component)]
pub struct EatCounter(pub i32);
#[derive(Component)]
pub struct Sneezing(pub bool);
#[derive(Component)]
pub struct PandaSitting(pub bool);
#[derive(Component)]
pub struct OnBack(pub bool);
#[derive(Component)]
pub struct PandaRolling(pub bool);
#[derive(Component)]
pub struct HiddenGene(pub u8);
#[derive(Component)]
pub struct PandaFlags(pub u8);
#[derive(Component)]
pub struct Panda;

#[derive(Bundle)]
struct PandaBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    panda_unhappy_counter: PandaUnhappyCounter,
    sneeze_counter: SneezeCounter,
    eat_counter: EatCounter,
    sneezing: Sneezing,
    panda_sitting: PandaSitting,
    on_back: OnBack,
    panda_rolling: PandaRolling,
    hidden_gene: HiddenGene,
    panda_flags: PandaFlags,
}
impl PandaBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.panda_unhappy_counter = PandaUnhappyCounter(d.value.into_int()?),
                18 => self.sneeze_counter = SneezeCounter(d.value.into_int()?),
                19 => self.eat_counter = EatCounter(d.value.into_int()?),
                20 => {
                    let bitfield = d.value.into_byte()?;
                    self.sneezing = Sneezing(bitfield & 0x2 != 0);
                    self.panda_sitting = PandaSitting(bitfield & 0x8 != 0);
                    self.on_back = OnBack(bitfield & 0x10 != 0);
                    self.panda_rolling = PandaRolling(bitfield & 0x4 != 0);
                }
                21 => self.hidden_gene = HiddenGene(d.value.into_byte()?),
                22 => self.panda_flags = PandaFlags(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

impl Default for PandaBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            panda_unhappy_counter: PandaUnhappyCounter(0),
            sneeze_counter: SneezeCounter(0),
            eat_counter: EatCounter(0),
            sneezing: Sneezing(false),
            panda_sitting: PandaSitting(false),
            on_back: OnBack(false),
            panda_rolling: PandaRolling(false),
            hidden_gene: HiddenGene(0),
            panda_flags: PandaFlags(0),
        }
    }
}

#[derive(Component)]
pub struct ParrotVariant(pub i32);
#[derive(Component)]
pub struct Parrot;

#[derive(Bundle)]
struct ParrotBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
    parrot_variant: ParrotVariant,
}
impl ParrotBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.tame = Tame(bitfield & 0x4 != 0);
                    self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => self.parrot_variant = ParrotVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ParrotBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
            parrot_variant: ParrotVariant(0),
        }
    }
}

#[derive(Component)]
pub struct PhantomSize(pub i32);
#[derive(Component)]
pub struct Phantom;

#[derive(Bundle)]
struct PhantomBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    phantom_size: PhantomSize,
}
impl PhantomBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.phantom_size = PhantomSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for PhantomBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            phantom_size: PhantomSize(0),
        }
    }
}

#[derive(Component)]
pub struct PigSaddle(pub bool);
#[derive(Component)]
pub struct PigBoostTime(pub i32);
#[derive(Component)]
pub struct Pig;

#[derive(Bundle)]
struct PigBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    pig_saddle: PigSaddle,
    pig_boost_time: PigBoostTime,
}
impl PigBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.pig_saddle = PigSaddle(d.value.into_boolean()?),
                18 => self.pig_boost_time = PigBoostTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for PigBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            pig_saddle: PigSaddle(false),
            pig_boost_time: PigBoostTime(0),
        }
    }
}

#[derive(Component)]
pub struct PiglinImmuneToZombification(pub bool);
#[derive(Component)]
pub struct PiglinBaby(pub bool);
#[derive(Component)]
pub struct PiglinIsChargingCrossbow(pub bool);
#[derive(Component)]
pub struct IsDancing(pub bool);
#[derive(Component)]
pub struct Piglin;

#[derive(Bundle)]
struct PiglinBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    piglin_immune_to_zombification: PiglinImmuneToZombification,
    piglin_baby: PiglinBaby,
    piglin_is_charging_crossbow: PiglinIsChargingCrossbow,
    is_dancing: IsDancing,
}
impl PiglinBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    self.piglin_immune_to_zombification =
                        PiglinImmuneToZombification(d.value.into_boolean()?)
                }
                17 => self.piglin_baby = PiglinBaby(d.value.into_boolean()?),
                18 => {
                    self.piglin_is_charging_crossbow =
                        PiglinIsChargingCrossbow(d.value.into_boolean()?)
                }
                19 => self.is_dancing = IsDancing(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for PiglinBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            piglin_immune_to_zombification: PiglinImmuneToZombification(false),
            piglin_baby: PiglinBaby(false),
            piglin_is_charging_crossbow: PiglinIsChargingCrossbow(false),
            is_dancing: IsDancing(false),
        }
    }
}

#[derive(Component)]
pub struct PiglinBruteImmuneToZombification(pub bool);
#[derive(Component)]
pub struct PiglinBrute;

#[derive(Bundle)]
struct PiglinBruteBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    piglin_brute_immune_to_zombification: PiglinBruteImmuneToZombification,
}
impl PiglinBruteBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    self.piglin_brute_immune_to_zombification =
                        PiglinBruteImmuneToZombification(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for PiglinBruteBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            piglin_brute_immune_to_zombification: PiglinBruteImmuneToZombification(false),
        }
    }
}

#[derive(Component)]
pub struct PillagerIsCelebrating(pub bool);
#[derive(Component)]
pub struct PillagerIsChargingCrossbow(pub bool);
#[derive(Component)]
pub struct Pillager;

#[derive(Bundle)]
struct PillagerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    pillager_is_celebrating: PillagerIsCelebrating,
    pillager_is_charging_crossbow: PillagerIsChargingCrossbow,
}
impl PillagerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.pillager_is_celebrating = PillagerIsCelebrating(d.value.into_boolean()?),
                17 => {
                    self.pillager_is_charging_crossbow =
                        PillagerIsChargingCrossbow(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for PillagerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            pillager_is_celebrating: PillagerIsCelebrating(false),
            pillager_is_charging_crossbow: PillagerIsChargingCrossbow(false),
        }
    }
}

#[derive(Component)]
pub struct PlayerAbsorption(pub f32);
#[derive(Component)]
pub struct Score(pub i32);
#[derive(Component)]
pub struct PlayerModeCustomisation(pub u8);
#[derive(Component)]
pub struct PlayerMainHand(pub u8);
#[derive(Component)]
pub struct ShoulderLeft(pub azalea_nbt::Tag);
#[derive(Component)]
pub struct ShoulderRight(pub azalea_nbt::Tag);
#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    player_absorption: PlayerAbsorption,
    score: Score,
    player_mode_customisation: PlayerModeCustomisation,
    player_main_hand: PlayerMainHand,
    shoulder_left: ShoulderLeft,
    shoulder_right: ShoulderRight,
}
impl PlayerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => self.player_absorption = PlayerAbsorption(d.value.into_float()?),
                16 => self.score = Score(d.value.into_int()?),
                17 => {
                    self.player_mode_customisation = PlayerModeCustomisation(d.value.into_byte()?)
                }
                18 => self.player_main_hand = PlayerMainHand(d.value.into_byte()?),
                19 => self.shoulder_left = ShoulderLeft(d.value.into_compound_tag()?),
                20 => self.shoulder_right = ShoulderRight(d.value.into_compound_tag()?),
            }
        }
        Ok(())
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            player_absorption: PlayerAbsorption(0.0),
            score: Score(0),
            player_mode_customisation: PlayerModeCustomisation(0),
            player_main_hand: PlayerMainHand(1),
            shoulder_left: ShoulderLeft(azalea_nbt::Tag::Compound(Default::default())),
            shoulder_right: ShoulderRight(azalea_nbt::Tag::Compound(Default::default())),
        }
    }
}

#[derive(Component)]
pub struct PolarBearStanding(pub bool);
#[derive(Component)]
pub struct PolarBear;

#[derive(Bundle)]
struct PolarBearBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    polar_bear_standing: PolarBearStanding,
}
impl PolarBearBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.polar_bear_standing = PolarBearStanding(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for PolarBearBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            polar_bear_standing: PolarBearStanding(false),
        }
    }
}

#[derive(Component)]
pub struct PotionItemStack(pub Slot);
#[derive(Component)]
pub struct Potion;

#[derive(Bundle)]
struct PotionBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    potion_item_stack: PotionItemStack,
}
impl PotionBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.potion_item_stack = PotionItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for PotionBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            potion_item_stack: PotionItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct PufferfishFromBucket(pub bool);
#[derive(Component)]
pub struct PuffState(pub i32);
#[derive(Component)]
pub struct Pufferfish;

#[derive(Bundle)]
struct PufferfishBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    pufferfish_from_bucket: PufferfishFromBucket,
    puff_state: PuffState,
}
impl PufferfishBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.pufferfish_from_bucket = PufferfishFromBucket(d.value.into_boolean()?),
                17 => self.puff_state = PuffState(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for PufferfishBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            pufferfish_from_bucket: PufferfishFromBucket(false),
            puff_state: PuffState(0),
        }
    }
}

#[derive(Component)]
pub struct RabbitKind(pub i32);
#[derive(Component)]
pub struct Rabbit;

#[derive(Bundle)]
struct RabbitBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    rabbit_kind: RabbitKind,
}
impl RabbitBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.rabbit_kind = RabbitKind(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for RabbitBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            rabbit_kind: RabbitKind(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct RavagerIsCelebrating(pub bool);
#[derive(Component)]
pub struct Ravager;

#[derive(Bundle)]
struct RavagerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    ravager_is_celebrating: RavagerIsCelebrating,
}
impl RavagerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.ravager_is_celebrating = RavagerIsCelebrating(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for RavagerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            ravager_is_celebrating: RavagerIsCelebrating(false),
        }
    }
}

#[derive(Component)]
pub struct SalmonFromBucket(pub bool);
#[derive(Component)]
pub struct Salmon;

#[derive(Bundle)]
struct SalmonBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    salmon_from_bucket: SalmonFromBucket,
}
impl SalmonBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.salmon_from_bucket = SalmonFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for SalmonBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            salmon_from_bucket: SalmonFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct Sheared(pub bool);
#[derive(Component)]
pub struct Sheep;

#[derive(Bundle)]
struct SheepBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    sheared: Sheared,
}
impl SheepBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.sheared = Sheared(bitfield & 0x10 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for SheepBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            sheared: Sheared(false),
        }
    }
}

#[derive(Component)]
pub struct AttachFace(pub Direction);
#[derive(Component)]
pub struct Peek(pub u8);
#[derive(Component)]
pub struct ShulkerColor(pub u8);
#[derive(Component)]
pub struct Shulker;

#[derive(Bundle)]
struct ShulkerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    attach_face: AttachFace,
    peek: Peek,
    shulker_color: ShulkerColor,
}
impl ShulkerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.attach_face = AttachFace(d.value.into_direction()?),
                17 => self.peek = Peek(d.value.into_byte()?),
                18 => self.shulker_color = ShulkerColor(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

impl Default for ShulkerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            attach_face: AttachFace(Default::default()),
            peek: Peek(0),
            shulker_color: ShulkerColor(16),
        }
    }
}

#[derive(Component)]
pub struct ShulkerBullet;

#[derive(Bundle)]
struct ShulkerBulletBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl ShulkerBulletBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for ShulkerBulletBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct Silverfish;

#[derive(Bundle)]
struct SilverfishBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl SilverfishBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for SilverfishBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct StrayConversion(pub bool);
#[derive(Component)]
pub struct Skeleton;

#[derive(Bundle)]
struct SkeletonBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    stray_conversion: StrayConversion,
}
impl SkeletonBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.stray_conversion = StrayConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for SkeletonBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            stray_conversion: StrayConversion(false),
        }
    }
}

#[derive(Component)]
pub struct SkeletonHorseTamed(pub bool);
#[derive(Component)]
pub struct SkeletonHorseEating(pub bool);
#[derive(Component)]
pub struct SkeletonHorseStanding(pub bool);
#[derive(Component)]
pub struct SkeletonHorseBred(pub bool);
#[derive(Component)]
pub struct SkeletonHorseSaddled(pub bool);
#[derive(Component)]
pub struct SkeletonHorseOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct SkeletonHorse;

#[derive(Bundle)]
struct SkeletonHorseBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    skeleton_horse_tamed: SkeletonHorseTamed,
    skeleton_horse_eating: SkeletonHorseEating,
    skeleton_horse_standing: SkeletonHorseStanding,
    skeleton_horse_bred: SkeletonHorseBred,
    skeleton_horse_saddled: SkeletonHorseSaddled,
    skeleton_horse_owner_uuid: SkeletonHorseOwnerUuid,
}
impl SkeletonHorseBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.skeleton_horse_tamed = SkeletonHorseTamed(bitfield & 0x2 != 0);
                    self.skeleton_horse_eating = SkeletonHorseEating(bitfield & 0x10 != 0);
                    self.skeleton_horse_standing = SkeletonHorseStanding(bitfield & 0x20 != 0);
                    self.skeleton_horse_bred = SkeletonHorseBred(bitfield & 0x8 != 0);
                    self.skeleton_horse_saddled = SkeletonHorseSaddled(bitfield & 0x4 != 0);
                }
                18 => {
                    self.skeleton_horse_owner_uuid =
                        SkeletonHorseOwnerUuid(d.value.into_optional_uuid()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for SkeletonHorseBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            skeleton_horse_tamed: SkeletonHorseTamed(false),
            skeleton_horse_eating: SkeletonHorseEating(false),
            skeleton_horse_standing: SkeletonHorseStanding(false),
            skeleton_horse_bred: SkeletonHorseBred(false),
            skeleton_horse_saddled: SkeletonHorseSaddled(false),
            skeleton_horse_owner_uuid: SkeletonHorseOwnerUuid(None),
        }
    }
}

#[derive(Component)]
pub struct Slime;

#[derive(Bundle)]
struct SlimeBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    slime_size: SlimeSize,
}
impl SlimeBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.slime_size = SlimeSize(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for SlimeBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            slime_size: SlimeSize(1),
        }
    }
}

#[derive(Component)]
pub struct SmallFireballItemStack(pub Slot);
#[derive(Component)]
pub struct SmallFireball;

#[derive(Bundle)]
struct SmallFireballBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    small_fireball_item_stack: SmallFireballItemStack,
}
impl SmallFireballBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    self.small_fireball_item_stack =
                        SmallFireballItemStack(d.value.into_item_stack()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for SmallFireballBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            small_fireball_item_stack: SmallFireballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct HasPumpkin(pub bool);
#[derive(Component)]
pub struct SnowGolem;

#[derive(Bundle)]
struct SnowGolemBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    has_pumpkin: HasPumpkin,
}
impl SnowGolemBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.has_pumpkin = HasPumpkin(bitfield & 0x10 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for SnowGolemBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            has_pumpkin: HasPumpkin(true),
        }
    }
}

#[derive(Component)]
pub struct SnowballItemStack(pub Slot);
#[derive(Component)]
pub struct Snowball;

#[derive(Bundle)]
struct SnowballBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    snowball_item_stack: SnowballItemStack,
}
impl SnowballBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.snowball_item_stack = SnowballItemStack(d.value.into_item_stack()?),
            }
        }
        Ok(())
    }
}

impl Default for SnowballBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            snowball_item_stack: SnowballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct SpawnerMinecart;

#[derive(Bundle)]
struct SpawnerMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl SpawnerMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for SpawnerMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct SpectralArrowCritArrow(pub bool);
#[derive(Component)]
pub struct SpectralArrowShotFromCrossbow(pub bool);
#[derive(Component)]
pub struct SpectralArrowNoPhysics(pub bool);
#[derive(Component)]
pub struct SpectralArrowPierceLevel(pub u8);
#[derive(Component)]
pub struct SpectralArrow;

#[derive(Bundle)]
struct SpectralArrowBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    spectral_arrow_crit_arrow: SpectralArrowCritArrow,
    spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow,
    spectral_arrow_no_physics: SpectralArrowNoPhysics,
    spectral_arrow_pierce_level: SpectralArrowPierceLevel,
}
impl SpectralArrowBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.spectral_arrow_crit_arrow = SpectralArrowCritArrow(bitfield & 0x1 != 0);
                    self.spectral_arrow_shot_from_crossbow =
                        SpectralArrowShotFromCrossbow(bitfield & 0x4 != 0);
                    self.spectral_arrow_no_physics = SpectralArrowNoPhysics(bitfield & 0x2 != 0);
                }
                9 => {
                    self.spectral_arrow_pierce_level =
                        SpectralArrowPierceLevel(d.value.into_byte()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for SpectralArrowBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            spectral_arrow_crit_arrow: SpectralArrowCritArrow(false),
            spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow(false),
            spectral_arrow_no_physics: SpectralArrowNoPhysics(false),
            spectral_arrow_pierce_level: SpectralArrowPierceLevel(0),
        }
    }
}

#[derive(Component)]
pub struct Spider;

#[derive(Bundle)]
struct SpiderBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    climbing: Climbing,
}
impl SpiderBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    let bitfield = d.value.into_byte()?;
                    self.climbing = Climbing(bitfield & 0x1 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for SpiderBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            climbing: Climbing(false),
        }
    }
}

#[derive(Component)]
pub struct Squid;

#[derive(Bundle)]
struct SquidBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl SquidBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for SquidBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct Stray;

#[derive(Bundle)]
struct StrayBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl StrayBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for StrayBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct StriderBoostTime(pub i32);
#[derive(Component)]
pub struct Suffocating(pub bool);
#[derive(Component)]
pub struct StriderSaddle(pub bool);
#[derive(Component)]
pub struct Strider;

#[derive(Bundle)]
struct StriderBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    strider_boost_time: StriderBoostTime,
    suffocating: Suffocating,
    strider_saddle: StriderSaddle,
}
impl StriderBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.strider_boost_time = StriderBoostTime(d.value.into_int()?),
                18 => self.suffocating = Suffocating(d.value.into_boolean()?),
                19 => self.strider_saddle = StriderSaddle(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for StriderBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            strider_boost_time: StriderBoostTime(0),
            suffocating: Suffocating(false),
            strider_saddle: StriderSaddle(false),
        }
    }
}

#[derive(Component)]
pub struct TadpoleFromBucket(pub bool);
#[derive(Component)]
pub struct Tadpole;

#[derive(Bundle)]
struct TadpoleBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    tadpole_from_bucket: TadpoleFromBucket,
}
impl TadpoleBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.tadpole_from_bucket = TadpoleFromBucket(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for TadpoleBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            tadpole_from_bucket: TadpoleFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct Fuse(pub i32);
#[derive(Component)]
pub struct Tnt;

#[derive(Bundle)]
struct TntBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    fuse: Fuse,
}
impl TntBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.fuse = Fuse(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for TntBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            fuse: Fuse(80),
        }
    }
}

#[derive(Component)]
pub struct TntMinecart;

#[derive(Bundle)]
struct TntMinecartBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl TntMinecartBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.abstract_minecart_hurt = AbstractMinecartHurt(d.value.into_int()?),
                9 => self.abstract_minecart_hurtdir = AbstractMinecartHurtdir(d.value.into_int()?),
                10 => self.abstract_minecart_damage = AbstractMinecartDamage(d.value.into_float()?),
                11 => self.display_block = DisplayBlock(d.value.into_int()?),
                12 => self.display_offset = DisplayOffset(d.value.into_int()?),
                13 => self.custom_display = CustomDisplay(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for TntMinecartBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct TraderLlama;

#[derive(Bundle)]
struct TraderLlamaBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    llama_tamed: LlamaTamed,
    llama_eating: LlamaEating,
    llama_standing: LlamaStanding,
    llama_bred: LlamaBred,
    llama_saddled: LlamaSaddled,
    llama_owner_uuid: LlamaOwnerUuid,
    llama_chest: LlamaChest,
    strength: Strength,
    swag: Swag,
    llama_variant: LlamaVariant,
}
impl TraderLlamaBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.llama_tamed = LlamaTamed(bitfield & 0x2 != 0);
                    self.llama_eating = LlamaEating(bitfield & 0x10 != 0);
                    self.llama_standing = LlamaStanding(bitfield & 0x20 != 0);
                    self.llama_bred = LlamaBred(bitfield & 0x8 != 0);
                    self.llama_saddled = LlamaSaddled(bitfield & 0x4 != 0);
                }
                18 => self.llama_owner_uuid = LlamaOwnerUuid(d.value.into_optional_uuid()?),
                19 => self.llama_chest = LlamaChest(d.value.into_boolean()?),
                20 => self.strength = Strength(d.value.into_int()?),
                21 => self.swag = Swag(d.value.into_int()?),
                22 => self.llama_variant = LlamaVariant(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for TraderLlamaBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            llama_tamed: LlamaTamed(false),
            llama_eating: LlamaEating(false),
            llama_standing: LlamaStanding(false),
            llama_bred: LlamaBred(false),
            llama_saddled: LlamaSaddled(false),
            llama_owner_uuid: LlamaOwnerUuid(None),
            llama_chest: LlamaChest(false),
            strength: Strength(0),
            swag: Swag(-1),
            llama_variant: LlamaVariant(0),
        }
    }
}

#[derive(Component)]
pub struct TridentCritArrow(pub bool);
#[derive(Component)]
pub struct TridentShotFromCrossbow(pub bool);
#[derive(Component)]
pub struct TridentNoPhysics(pub bool);
#[derive(Component)]
pub struct TridentPierceLevel(pub u8);
#[derive(Component)]
pub struct Loyalty(pub u8);
#[derive(Component)]
pub struct Foil(pub bool);
#[derive(Component)]
pub struct Trident;

#[derive(Bundle)]
struct TridentBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    trident_crit_arrow: TridentCritArrow,
    trident_shot_from_crossbow: TridentShotFromCrossbow,
    trident_no_physics: TridentNoPhysics,
    trident_pierce_level: TridentPierceLevel,
    loyalty: Loyalty,
    foil: Foil,
}
impl TridentBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.trident_crit_arrow = TridentCritArrow(bitfield & 0x1 != 0);
                    self.trident_shot_from_crossbow = TridentShotFromCrossbow(bitfield & 0x4 != 0);
                    self.trident_no_physics = TridentNoPhysics(bitfield & 0x2 != 0);
                }
                9 => self.trident_pierce_level = TridentPierceLevel(d.value.into_byte()?),
                10 => self.loyalty = Loyalty(d.value.into_byte()?),
                11 => self.foil = Foil(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for TridentBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            trident_crit_arrow: TridentCritArrow(false),
            trident_shot_from_crossbow: TridentShotFromCrossbow(false),
            trident_no_physics: TridentNoPhysics(false),
            trident_pierce_level: TridentPierceLevel(0),
            loyalty: Loyalty(0),
            foil: Foil(false),
        }
    }
}

#[derive(Component)]
pub struct TropicalFishFromBucket(pub bool);
#[derive(Component)]
pub struct TropicalFishTypeVariant(pub i32);
#[derive(Component)]
pub struct TropicalFish;

#[derive(Bundle)]
struct TropicalFishBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    tropical_fish_from_bucket: TropicalFishFromBucket,
    tropical_fish_type_variant: TropicalFishTypeVariant,
}
impl TropicalFishBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    self.tropical_fish_from_bucket = TropicalFishFromBucket(d.value.into_boolean()?)
                }
                17 => {
                    self.tropical_fish_type_variant = TropicalFishTypeVariant(d.value.into_int()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for TropicalFishBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            tropical_fish_from_bucket: TropicalFishFromBucket(false),
            tropical_fish_type_variant: TropicalFishTypeVariant(0),
        }
    }
}

#[derive(Component)]
pub struct HomePos(pub BlockPos);
#[derive(Component)]
pub struct HasEgg(pub bool);
#[derive(Component)]
pub struct LayingEgg(pub bool);
#[derive(Component)]
pub struct TravelPos(pub BlockPos);
#[derive(Component)]
pub struct GoingHome(pub bool);
#[derive(Component)]
pub struct Travelling(pub bool);
#[derive(Component)]
pub struct Turtle;

#[derive(Bundle)]
struct TurtleBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    home_pos: HomePos,
    has_egg: HasEgg,
    laying_egg: LayingEgg,
    travel_pos: TravelPos,
    going_home: GoingHome,
    travelling: Travelling,
}
impl TurtleBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.home_pos = HomePos(d.value.into_block_pos()?),
                18 => self.has_egg = HasEgg(d.value.into_boolean()?),
                19 => self.laying_egg = LayingEgg(d.value.into_boolean()?),
                20 => self.travel_pos = TravelPos(d.value.into_block_pos()?),
                21 => self.going_home = GoingHome(d.value.into_boolean()?),
                22 => self.travelling = Travelling(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for TurtleBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            home_pos: HomePos(BlockPos::new(0, 0, 0)),
            has_egg: HasEgg(false),
            laying_egg: LayingEgg(false),
            travel_pos: TravelPos(BlockPos::new(0, 0, 0)),
            going_home: GoingHome(false),
            travelling: Travelling(false),
        }
    }
}

#[derive(Component)]
pub struct VexFlags(pub u8);
#[derive(Component)]
pub struct Vex;

#[derive(Bundle)]
struct VexBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    vex_flags: VexFlags,
}
impl VexBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.vex_flags = VexFlags(d.value.into_byte()?),
            }
        }
        Ok(())
    }
}

impl Default for VexBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            vex_flags: VexFlags(0),
        }
    }
}

#[derive(Component)]
pub struct VillagerUnhappyCounter(pub i32);
#[derive(Component)]
pub struct VillagerVillagerData(pub VillagerData);
#[derive(Component)]
pub struct Villager;

#[derive(Bundle)]
struct VillagerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    villager_unhappy_counter: VillagerUnhappyCounter,
    villager_villager_data: VillagerVillagerData,
}
impl VillagerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => self.villager_unhappy_counter = VillagerUnhappyCounter(d.value.into_int()?),
                18 => {
                    self.villager_villager_data =
                        VillagerVillagerData(d.value.into_villager_data()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for VillagerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            villager_unhappy_counter: VillagerUnhappyCounter(0),
            villager_villager_data: VillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerKind::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

#[derive(Component)]
pub struct VindicatorIsCelebrating(pub bool);
#[derive(Component)]
pub struct Vindicator;

#[derive(Bundle)]
struct VindicatorBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    vindicator_is_celebrating: VindicatorIsCelebrating,
}
impl VindicatorBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => {
                    self.vindicator_is_celebrating =
                        VindicatorIsCelebrating(d.value.into_boolean()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for VindicatorBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            vindicator_is_celebrating: VindicatorIsCelebrating(false),
        }
    }
}

#[derive(Component)]
pub struct WanderingTraderUnhappyCounter(pub i32);
#[derive(Component)]
pub struct WanderingTrader;

#[derive(Bundle)]
struct WanderingTraderBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter,
}
impl WanderingTraderBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    self.wandering_trader_unhappy_counter =
                        WanderingTraderUnhappyCounter(d.value.into_int()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for WanderingTraderBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter(0),
        }
    }
}

#[derive(Component)]
pub struct ClientAngerLevel(pub i32);
#[derive(Component)]
pub struct Warden;

#[derive(Bundle)]
struct WardenBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    client_anger_level: ClientAngerLevel,
}
impl WardenBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.client_anger_level = ClientAngerLevel(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for WardenBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            client_anger_level: ClientAngerLevel(0),
        }
    }
}

#[derive(Component)]
pub struct WitchIsCelebrating(pub bool);
#[derive(Component)]
pub struct WitchUsingItem(pub bool);
#[derive(Component)]
pub struct Witch;

#[derive(Bundle)]
struct WitchBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    witch_is_celebrating: WitchIsCelebrating,
    witch_using_item: WitchUsingItem,
}
impl WitchBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.witch_is_celebrating = WitchIsCelebrating(d.value.into_boolean()?),
                17 => self.witch_using_item = WitchUsingItem(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for WitchBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            witch_is_celebrating: WitchIsCelebrating(false),
            witch_using_item: WitchUsingItem(false),
        }
    }
}

#[derive(Component)]
pub struct TargetA(pub i32);
#[derive(Component)]
pub struct TargetB(pub i32);
#[derive(Component)]
pub struct TargetC(pub i32);
#[derive(Component)]
pub struct Inv(pub i32);
#[derive(Component)]
pub struct Wither;

#[derive(Bundle)]
struct WitherBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    target_a: TargetA,
    target_b: TargetB,
    target_c: TargetC,
    inv: Inv,
}
impl WitherBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.target_a = TargetA(d.value.into_int()?),
                17 => self.target_b = TargetB(d.value.into_int()?),
                18 => self.target_c = TargetC(d.value.into_int()?),
                19 => self.inv = Inv(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for WitherBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            target_a: TargetA(0),
            target_b: TargetB(0),
            target_c: TargetC(0),
            inv: Inv(0),
        }
    }
}

#[derive(Component)]
pub struct WitherSkeleton;

#[derive(Bundle)]
struct WitherSkeletonBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl WitherSkeletonBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
            }
        }
        Ok(())
    }
}

impl Default for WitherSkeletonBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct Dangerous(pub bool);
#[derive(Component)]
pub struct WitherSkull;

#[derive(Bundle)]
struct WitherSkullBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    dangerous: Dangerous,
}
impl WitherSkullBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => self.dangerous = Dangerous(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for WitherSkullBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            dangerous: Dangerous(false),
        }
    }
}

#[derive(Component)]
pub struct WolfInterested(pub bool);
#[derive(Component)]
pub struct WolfCollarColor(pub i32);
#[derive(Component)]
pub struct WolfRemainingAngerTime(pub i32);
#[derive(Component)]
pub struct Wolf;

#[derive(Bundle)]
struct WolfBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
    wolf_interested: WolfInterested,
    wolf_collar_color: WolfCollarColor,
    wolf_remaining_anger_time: WolfRemainingAngerTime,
}
impl WolfBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.tame = Tame(bitfield & 0x4 != 0);
                    self.in_sitting_pose = InSittingPose(bitfield & 0x1 != 0);
                }
                18 => self.owneruuid = Owneruuid(d.value.into_optional_uuid()?),
                19 => self.wolf_interested = WolfInterested(d.value.into_boolean()?),
                20 => self.wolf_collar_color = WolfCollarColor(d.value.into_int()?),
                21 => self.wolf_remaining_anger_time = WolfRemainingAngerTime(d.value.into_int()?),
            }
        }
        Ok(())
    }
}

impl Default for WolfBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
            wolf_interested: WolfInterested(false),
            wolf_collar_color: WolfCollarColor(Default::default()),
            wolf_remaining_anger_time: WolfRemainingAngerTime(0),
        }
    }
}

#[derive(Component)]
pub struct ZoglinBaby(pub bool);
#[derive(Component)]
pub struct Zoglin;

#[derive(Bundle)]
struct ZoglinBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zoglin_baby: ZoglinBaby,
}
impl ZoglinBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zoglin_baby = ZoglinBaby(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for ZoglinBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zoglin_baby: ZoglinBaby(false),
        }
    }
}

#[derive(Component)]
pub struct Zombie;

#[derive(Bundle)]
struct ZombieBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl ZombieBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => self.special_type = SpecialType(d.value.into_int()?),
                18 => self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for ZombieBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

#[derive(Component)]
pub struct ZombieHorseTamed(pub bool);
#[derive(Component)]
pub struct ZombieHorseEating(pub bool);
#[derive(Component)]
pub struct ZombieHorseStanding(pub bool);
#[derive(Component)]
pub struct ZombieHorseBred(pub bool);
#[derive(Component)]
pub struct ZombieHorseSaddled(pub bool);
#[derive(Component)]
pub struct ZombieHorseOwnerUuid(pub Option<Uuid>);
#[derive(Component)]
pub struct ZombieHorse;

#[derive(Bundle)]
struct ZombieHorseBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    abstract_ageable_baby: AbstractAgeableBaby,
    zombie_horse_tamed: ZombieHorseTamed,
    zombie_horse_eating: ZombieHorseEating,
    zombie_horse_standing: ZombieHorseStanding,
    zombie_horse_bred: ZombieHorseBred,
    zombie_horse_saddled: ZombieHorseSaddled,
    zombie_horse_owner_uuid: ZombieHorseOwnerUuid,
}
impl ZombieHorseBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.abstract_ageable_baby = AbstractAgeableBaby(d.value.into_boolean()?),
                17 => {
                    let bitfield = d.value.into_byte()?;
                    self.zombie_horse_tamed = ZombieHorseTamed(bitfield & 0x2 != 0);
                    self.zombie_horse_eating = ZombieHorseEating(bitfield & 0x10 != 0);
                    self.zombie_horse_standing = ZombieHorseStanding(bitfield & 0x20 != 0);
                    self.zombie_horse_bred = ZombieHorseBred(bitfield & 0x8 != 0);
                    self.zombie_horse_saddled = ZombieHorseSaddled(bitfield & 0x4 != 0);
                }
                18 => {
                    self.zombie_horse_owner_uuid =
                        ZombieHorseOwnerUuid(d.value.into_optional_uuid()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for ZombieHorseBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            abstract_ageable_baby: AbstractAgeableBaby(false),
            zombie_horse_tamed: ZombieHorseTamed(false),
            zombie_horse_eating: ZombieHorseEating(false),
            zombie_horse_standing: ZombieHorseStanding(false),
            zombie_horse_bred: ZombieHorseBred(false),
            zombie_horse_saddled: ZombieHorseSaddled(false),
            zombie_horse_owner_uuid: ZombieHorseOwnerUuid(None),
        }
    }
}

#[derive(Component)]
pub struct Converting(pub bool);
#[derive(Component)]
pub struct ZombieVillagerVillagerData(pub VillagerData);
#[derive(Component)]
pub struct ZombieVillager;

#[derive(Bundle)]
struct ZombieVillagerBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
    converting: Converting,
    zombie_villager_villager_data: ZombieVillagerVillagerData,
}
impl ZombieVillagerBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => self.special_type = SpecialType(d.value.into_int()?),
                18 => self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
                19 => self.converting = Converting(d.value.into_boolean()?),
                20 => {
                    self.zombie_villager_villager_data =
                        ZombieVillagerVillagerData(d.value.into_villager_data()?)
                }
            }
        }
        Ok(())
    }
}

impl Default for ZombieVillagerBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
            converting: Converting(false),
            zombie_villager_villager_data: ZombieVillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerKind::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

#[derive(Component)]
pub struct ZombifiedPiglin;

#[derive(Bundle)]
struct ZombifiedPiglinBundle {
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl ZombifiedPiglinBundle {
    pub fn update_metadata(
        &mut self,
        ecs: bevy_ecs::world::World,
        entity: bevy_ecs::world::EntityMut,
        data: EntityMetadataItems,
    ) -> Result<(), UpdateMetadataError> {
        for d in data.0 {
            match d.index {
                0 => {
                    let bitfield = d.value.into_byte()?;
                    self.on_fire = OnFire(bitfield & 0x1 != 0);
                    self.shift_key_down = ShiftKeyDown(bitfield & 0x2 != 0);
                    self.sprinting = Sprinting(bitfield & 0x8 != 0);
                    self.swimming = Swimming(bitfield & 0x10 != 0);
                    self.currently_glowing = CurrentlyGlowing(bitfield & 0x40 != 0);
                    self.invisible = Invisible(bitfield & 0x20 != 0);
                    self.fall_flying = FallFlying(bitfield & 0x80 != 0);
                }
                1 => self.air_supply = AirSupply(d.value.into_int()?),
                2 => self.custom_name = CustomName(d.value.into_optional_component()?),
                3 => self.custom_name_visible = CustomNameVisible(d.value.into_boolean()?),
                4 => self.silent = Silent(d.value.into_boolean()?),
                5 => self.no_gravity = NoGravity(d.value.into_boolean()?),
                6 => self.pose = d.value.into_pose()?,
                7 => self.ticks_frozen = TicksFrozen(d.value.into_int()?),
                8 => {
                    let bitfield = d.value.into_byte()?;
                    self.auto_spin_attack = AutoSpinAttack(bitfield & 0x4 != 0);
                    self.abstract_living_using_item = AbstractLivingUsingItem(bitfield & 0x1 != 0);
                }
                9 => self.health = Health(d.value.into_float()?),
                10 => {
                    self.abstract_living_effect_color =
                        AbstractLivingEffectColor(d.value.into_int()?)
                }
                11 => self.effect_ambience = EffectAmbience(d.value.into_boolean()?),
                12 => self.arrow_count = ArrowCount(d.value.into_int()?),
                13 => self.stinger_count = StingerCount(d.value.into_int()?),
                14 => self.sleeping_pos = SleepingPos(d.value.into_optional_block_pos()?),
                15 => {
                    let bitfield = d.value.into_byte()?;
                    self.no_ai = NoAi(bitfield & 0x1 != 0);
                    self.left_handed = LeftHanded(bitfield & 0x2 != 0);
                    self.aggressive = Aggressive(bitfield & 0x4 != 0);
                }
                16 => self.zombie_baby = ZombieBaby(d.value.into_boolean()?),
                17 => self.special_type = SpecialType(d.value.into_int()?),
                18 => self.drowned_conversion = DrownedConversion(d.value.into_boolean()?),
            }
        }
        Ok(())
    }
}

impl Default for ZombifiedPiglinBundle {
    fn default() -> Self {
        Self {
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

fn update_metadata(
    ecs: bevy_ecs::world::World,
    entity: bevy_ecs::world::EntityMut,
    data: EntityMetadataItems,
) -> Result<(), UpdateMetadataError> {
    if let Ok(e) = world.query_one_mut::<AllayBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<AreaEffectCloudBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ArmorStandBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ArrowBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<AxolotlBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BatBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BeeBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BlazeBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<BoatBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CamelBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CatBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CaveSpiderBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChestBoatBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChestMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ChickenBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CodBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CommandBlockMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CowBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<CreeperBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DolphinBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DonkeyBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DragonFireballBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<DrownedBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EggBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ElderGuardianBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndCrystalBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EnderDragonBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EnderPearlBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndermanBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EndermiteBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EvokerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EvokerFangsBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ExperienceBottleBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ExperienceOrbBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<EyeOfEnderBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FallingBlockBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FireballBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FireworkRocketBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FishingBobberBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FoxBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FrogBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<FurnaceMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GhastBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GiantBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GlowItemFrameBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GlowSquidBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GoatBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<GuardianBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HoglinBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HopperMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HorseBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<HuskBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<IllusionerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<IronGolemBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ItemBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ItemFrameBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LeashKnotBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LightningBoltBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LlamaBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<LlamaSpitBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MagmaCubeBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MarkerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MooshroomBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<MuleBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<OcelotBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PaintingBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PandaBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ParrotBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PhantomBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PigBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PiglinBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PiglinBruteBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PillagerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PlayerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PolarBearBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PotionBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<PufferfishBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<RabbitBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<RavagerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SalmonBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SheepBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ShulkerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ShulkerBulletBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SilverfishBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SkeletonBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SkeletonHorseBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SlimeBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SmallFireballBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SnowGolemBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SnowballBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpawnerMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpectralArrowBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SpiderBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<SquidBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<StrayBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<StriderBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TadpoleBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TntBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TntMinecartBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TraderLlamaBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TridentBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TropicalFishBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<TurtleBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VexBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VillagerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<VindicatorBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WanderingTraderBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WardenBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitchBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherSkeletonBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WitherSkullBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<WolfBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZoglinBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieHorseBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombieVillagerBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    if let Ok(e) = world.query_one_mut::<ZombifiedPiglinBundle>(entity) {
        e.update_metadata(ecs, entity, data)?;
        return Ok(());
    }
    Ok(())
}
