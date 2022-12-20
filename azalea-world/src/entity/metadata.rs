// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

#![allow(clippy::clone_on_copy, clippy::derivable_impls)]
use super::{EntityDataItem, EntityDataValue, OptionalUnsignedInt, Pose, Rotations, VillagerData};
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
impl Allay {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(Dancing(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(CanDuplicate(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AllayBundle {
    parent: AbstractCreatureBundle,
    dancing: Dancing,
    can_duplicate: CanDuplicate,
}
impl Default for AllayBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
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
impl AreaEffectCloud {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(Radius(d.value.into_float()?));
            }
            9 => {
                entity.insert(AreaEffectCloudColor(d.value.into_int()?));
            }
            10 => {
                entity.insert(Waiting(d.value.into_boolean()?));
            }
            11 => {
                entity.insert(d.value.into_particle()?);
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AreaEffectCloudBundle {
    parent: AbstractEntityBundle,
    radius: Radius,
    area_effect_cloud_color: AreaEffectCloudColor,
    waiting: Waiting,
    particle: Particle,
}
impl Default for AreaEffectCloudBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl ArmorStand {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::update_metadata(ecs, entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Small(bitfield & 0x1 != 0));
                entity.insert(ShowArms(bitfield & 0x4 != 0));
                entity.insert(NoBasePlate(bitfield & 0x8 != 0));
                entity.insert(ArmorStandMarker(bitfield & 0x10 != 0));
            }
            16 => {
                entity.insert(HeadPose(d.value.into_rotations()?));
            }
            17 => {
                entity.insert(BodyPose(d.value.into_rotations()?));
            }
            18 => {
                entity.insert(LeftArmPose(d.value.into_rotations()?));
            }
            19 => {
                entity.insert(RightArmPose(d.value.into_rotations()?));
            }
            20 => {
                entity.insert(LeftLegPose(d.value.into_rotations()?));
            }
            21 => {
                entity.insert(RightLegPose(d.value.into_rotations()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ArmorStandBundle {
    parent: AbstractLivingBundle,
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
impl Default for ArmorStandBundle {
    fn default() -> Self {
        Self {
            parent: AbstractLivingBundle {
                parent: AbstractEntityBundle {
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
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
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
impl Arrow {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(ArrowCritArrow(bitfield & 0x1 != 0));
                entity.insert(ArrowShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(ArrowNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(ArrowPierceLevel(d.value.into_byte()?));
            }
            10 => {
                entity.insert(ArrowEffectColor(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ArrowBundle {
    parent: AbstractEntityBundle,
    arrow_crit_arrow: ArrowCritArrow,
    arrow_shot_from_crossbow: ArrowShotFromCrossbow,
    arrow_no_physics: ArrowNoPhysics,
    arrow_pierce_level: ArrowPierceLevel,
    arrow_effect_color: ArrowEffectColor,
}
impl Default for ArrowBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Axolotl {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(AxolotlVariant(d.value.into_int()?));
            }
            18 => {
                entity.insert(PlayingDead(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(AxolotlFromBucket(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AxolotlBundle {
    parent: AbstractAnimalBundle,
    axolotl_variant: AxolotlVariant,
    playing_dead: PlayingDead,
    axolotl_from_bucket: AxolotlFromBucket,
}
impl Default for AxolotlBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Bat {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Resting(bitfield & 0x1 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct BatBundle {
    parent: AbstractInsentientBundle,
    resting: Resting,
}
impl Default for BatBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
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
impl Bee {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasNectar(bitfield & 0x8 != 0));
                entity.insert(HasStung(bitfield & 0x4 != 0));
                entity.insert(BeeRolling(bitfield & 0x2 != 0));
            }
            18 => {
                entity.insert(BeeRemainingAngerTime(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct BeeBundle {
    parent: AbstractAnimalBundle,
    has_nectar: HasNectar,
    has_stung: HasStung,
    bee_rolling: BeeRolling,
    bee_remaining_anger_time: BeeRemainingAngerTime,
}
impl Default for BeeBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Blaze {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Charged(bitfield & 0x1 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct BlazeBundle {
    parent: AbstractMonsterBundle,
    charged: Charged,
}
impl Default for BlazeBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Boat {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(BoatHurt(d.value.into_int()?));
            }
            9 => {
                entity.insert(BoatHurtdir(d.value.into_int()?));
            }
            10 => {
                entity.insert(BoatDamage(d.value.into_float()?));
            }
            11 => {
                entity.insert(BoatKind(d.value.into_int()?));
            }
            12 => {
                entity.insert(PaddleLeft(d.value.into_boolean()?));
            }
            13 => {
                entity.insert(PaddleRight(d.value.into_boolean()?));
            }
            14 => {
                entity.insert(BubbleTime(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct BoatBundle {
    parent: AbstractEntityBundle,
    boat_hurt: BoatHurt,
    boat_hurtdir: BoatHurtdir,
    boat_damage: BoatDamage,
    boat_kind: BoatKind,
    paddle_left: PaddleLeft,
    paddle_right: PaddleRight,
    bubble_time: BubbleTime,
}
impl Default for BoatBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Camel {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(CamelTamed(bitfield & 0x2 != 0));
                entity.insert(CamelEating(bitfield & 0x10 != 0));
                entity.insert(CamelStanding(bitfield & 0x20 != 0));
                entity.insert(CamelBred(bitfield & 0x8 != 0));
                entity.insert(CamelSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(CamelOwnerUuid(d.value.into_optional_uuid()?));
            }
            19 => {
                entity.insert(Dash(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(LastPoseChangeTick(d.value.into_long()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CamelBundle {
    parent: AbstractAnimalBundle,
    camel_tamed: CamelTamed,
    camel_eating: CamelEating,
    camel_standing: CamelStanding,
    camel_bred: CamelBred,
    camel_saddled: CamelSaddled,
    camel_owner_uuid: CamelOwnerUuid,
    dash: Dash,
    last_pose_change_tick: LastPoseChangeTick,
}
impl Default for CamelBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Cat {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::update_metadata(ecs, entity, d)?,
            19 => {
                entity.insert(CatVariant(d.value.into_cat_variant()?));
            }
            20 => {
                entity.insert(IsLying(d.value.into_boolean()?));
            }
            21 => {
                entity.insert(RelaxStateOne(d.value.into_boolean()?));
            }
            22 => {
                entity.insert(CatCollarColor(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CatBundle {
    parent: AbstractTameableBundle,
    cat_variant: CatVariant,
    is_lying: IsLying,
    relax_state_one: RelaxStateOne,
    cat_collar_color: CatCollarColor,
}
impl Default for CatBundle {
    fn default() -> Self {
        Self {
            parent: AbstractTameableBundle {
                parent: AbstractAnimalBundle {
                    parent: AbstractAgeableBundle {
                        parent: AbstractCreatureBundle {
                            parent: AbstractInsentientBundle {
                                parent: AbstractLivingBundle {
                                    parent: AbstractEntityBundle {
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
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
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
impl CaveSpider {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Spider::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CaveSpiderBundle {
    parent: SpiderBundle,
}
impl Default for CaveSpiderBundle {
    fn default() -> Self {
        Self {
            parent: SpiderBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                climbing: Climbing(false),
            },
        }
    }
}

#[derive(Component)]
pub struct ChestBoat;
impl ChestBoat {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => Boat::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ChestBoatBundle {
    parent: BoatBundle,
}
impl Default for ChestBoatBundle {
    fn default() -> Self {
        Self {
            parent: BoatBundle {
                parent: AbstractEntityBundle {
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
                },
                boat_hurt: BoatHurt(0),
                boat_hurtdir: BoatHurtdir(1),
                boat_damage: BoatDamage(0.0),
                boat_kind: BoatKind(Default::default()),
                paddle_left: PaddleLeft(false),
                paddle_right: PaddleRight(false),
                bubble_time: BubbleTime(0),
            },
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
impl ChestMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ChestMinecartBundle {
    parent: AbstractMinecartBundle,
}
impl Default for ChestMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component)]
pub struct Chicken;
impl Chicken {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ChickenBundle {
    parent: AbstractAnimalBundle,
}
impl Default for ChickenBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
        }
    }
}

#[derive(Component)]
pub struct CodFromBucket(pub bool);
#[derive(Component)]
pub struct Cod;
impl Cod {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(CodFromBucket(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CodBundle {
    parent: AbstractCreatureBundle,
    cod_from_bucket: CodFromBucket,
}
impl Default for CodBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
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
impl CommandBlockMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
            14 => {
                entity.insert(CommandName(d.value.into_string()?));
            }
            15 => {
                entity.insert(LastOutput(d.value.into_formatted_text()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CommandBlockMinecartBundle {
    parent: AbstractMinecartBundle,
    command_name: CommandName,
    last_output: LastOutput,
}
impl Default for CommandBlockMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
            command_name: CommandName("".to_string()),
            last_output: LastOutput(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct Cow;
impl Cow {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CowBundle {
    parent: AbstractAnimalBundle,
}
impl Default for CowBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Creeper {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(SwellDir(d.value.into_int()?));
            }
            17 => {
                entity.insert(IsPowered(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(IsIgnited(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct CreeperBundle {
    parent: AbstractMonsterBundle,
    swell_dir: SwellDir,
    is_powered: IsPowered,
    is_ignited: IsIgnited,
}
impl Default for CreeperBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Dolphin {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(TreasurePos(d.value.into_block_pos()?));
            }
            17 => {
                entity.insert(GotFish(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(MoistnessLevel(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct DolphinBundle {
    parent: AbstractCreatureBundle,
    treasure_pos: TreasurePos,
    got_fish: GotFish,
    moistness_level: MoistnessLevel,
}
impl Default for DolphinBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
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
impl Donkey {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(DonkeyTamed(bitfield & 0x2 != 0));
                entity.insert(DonkeyEating(bitfield & 0x10 != 0));
                entity.insert(DonkeyStanding(bitfield & 0x20 != 0));
                entity.insert(DonkeyBred(bitfield & 0x8 != 0));
                entity.insert(DonkeySaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(DonkeyOwnerUuid(d.value.into_optional_uuid()?));
            }
            19 => {
                entity.insert(DonkeyChest(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct DonkeyBundle {
    parent: AbstractAnimalBundle,
    donkey_tamed: DonkeyTamed,
    donkey_eating: DonkeyEating,
    donkey_standing: DonkeyStanding,
    donkey_bred: DonkeyBred,
    donkey_saddled: DonkeySaddled,
    donkey_owner_uuid: DonkeyOwnerUuid,
    donkey_chest: DonkeyChest,
}
impl Default for DonkeyBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl DragonFireball {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct DragonFireballBundle {
    parent: AbstractEntityBundle,
}
impl Default for DragonFireballBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Drowned {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct DrownedBundle {
    parent: ZombieBundle,
}
impl Default for DrownedBundle {
    fn default() -> Self {
        Self {
            parent: ZombieBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component)]
pub struct EggItemStack(pub Slot);
#[derive(Component)]
pub struct Egg;
impl Egg {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(EggItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EggBundle {
    parent: AbstractEntityBundle,
    egg_item_stack: EggItemStack,
}
impl Default for EggBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl ElderGuardian {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => Guardian::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ElderGuardianBundle {
    parent: GuardianBundle,
}
impl Default for ElderGuardianBundle {
    fn default() -> Self {
        Self {
            parent: GuardianBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                moving: Moving(false),
                attack_target: AttackTarget(0),
            },
        }
    }
}

#[derive(Component)]
pub struct BeamTarget(pub Option<BlockPos>);
#[derive(Component)]
pub struct ShowBottom(pub bool);
#[derive(Component)]
pub struct EndCrystal;
impl EndCrystal {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(BeamTarget(d.value.into_optional_block_pos()?));
            }
            9 => {
                entity.insert(ShowBottom(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EndCrystalBundle {
    parent: AbstractEntityBundle,
    beam_target: BeamTarget,
    show_bottom: ShowBottom,
}
impl Default for EndCrystalBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            beam_target: BeamTarget(None),
            show_bottom: ShowBottom(true),
        }
    }
}

#[derive(Component)]
pub struct Phase(pub i32);
#[derive(Component)]
pub struct EnderDragon;
impl EnderDragon {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(Phase(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EnderDragonBundle {
    parent: AbstractInsentientBundle,
    phase: Phase,
}
impl Default for EnderDragonBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            phase: Phase(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct EnderPearlItemStack(pub Slot);
#[derive(Component)]
pub struct EnderPearl;
impl EnderPearl {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(EnderPearlItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EnderPearlBundle {
    parent: AbstractEntityBundle,
    ender_pearl_item_stack: EnderPearlItemStack,
}
impl Default for EnderPearlBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Enderman {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(CarryState(d.value.into_block_state()?));
            }
            17 => {
                entity.insert(Creepy(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(StaredAt(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EndermanBundle {
    parent: AbstractMonsterBundle,
    carry_state: CarryState,
    creepy: Creepy,
    stared_at: StaredAt,
}
impl Default for EndermanBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            carry_state: CarryState(BlockState::Air),
            creepy: Creepy(false),
            stared_at: StaredAt(false),
        }
    }
}

#[derive(Component)]
pub struct Endermite;
impl Endermite {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EndermiteBundle {
    parent: AbstractMonsterBundle,
}
impl Default for EndermiteBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component)]
pub struct EvokerIsCelebrating(pub bool);
#[derive(Component)]
pub struct EvokerSpellCasting(pub u8);
#[derive(Component)]
pub struct Evoker;
impl Evoker {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(EvokerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(EvokerSpellCasting(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EvokerBundle {
    parent: AbstractMonsterBundle,
    evoker_is_celebrating: EvokerIsCelebrating,
    evoker_spell_casting: EvokerSpellCasting,
}
impl Default for EvokerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            evoker_is_celebrating: EvokerIsCelebrating(false),
            evoker_spell_casting: EvokerSpellCasting(0),
        }
    }
}

#[derive(Component)]
pub struct EvokerFangs;
impl EvokerFangs {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EvokerFangsBundle {
    parent: AbstractEntityBundle,
}
impl Default for EvokerFangsBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct ExperienceBottleItemStack(pub Slot);
#[derive(Component)]
pub struct ExperienceBottle;
impl ExperienceBottle {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(ExperienceBottleItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ExperienceBottleBundle {
    parent: AbstractEntityBundle,
    experience_bottle_item_stack: ExperienceBottleItemStack,
}
impl Default for ExperienceBottleBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            experience_bottle_item_stack: ExperienceBottleItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct ExperienceOrb;
impl ExperienceOrb {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ExperienceOrbBundle {
    parent: AbstractEntityBundle,
}
impl Default for ExperienceOrbBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct EyeOfEnderItemStack(pub Slot);
#[derive(Component)]
pub struct EyeOfEnder;
impl EyeOfEnder {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(EyeOfEnderItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct EyeOfEnderBundle {
    parent: AbstractEntityBundle,
    eye_of_ender_item_stack: EyeOfEnderItemStack,
}
impl Default for EyeOfEnderBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            eye_of_ender_item_stack: EyeOfEnderItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct StartPos(pub BlockPos);
#[derive(Component)]
pub struct FallingBlock;
impl FallingBlock {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(StartPos(d.value.into_block_pos()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FallingBlockBundle {
    parent: AbstractEntityBundle,
    start_pos: StartPos,
}
impl Default for FallingBlockBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            start_pos: StartPos(BlockPos::new(0, 0, 0)),
        }
    }
}

#[derive(Component)]
pub struct FireballItemStack(pub Slot);
#[derive(Component)]
pub struct Fireball;
impl Fireball {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(FireballItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FireballBundle {
    parent: AbstractEntityBundle,
    fireball_item_stack: FireballItemStack,
}
impl Default for FireballBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl FireworkRocket {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(FireworksItem(d.value.into_item_stack()?));
            }
            9 => {
                entity.insert(AttachedToTarget(d.value.into_optional_unsigned_int()?));
            }
            10 => {
                entity.insert(ShotAtAngle(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FireworkRocketBundle {
    parent: AbstractEntityBundle,
    fireworks_item: FireworksItem,
    attached_to_target: AttachedToTarget,
    shot_at_angle: ShotAtAngle,
}
impl Default for FireworkRocketBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl FishingBobber {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(HookedEntity(d.value.into_int()?));
            }
            9 => {
                entity.insert(Biting(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FishingBobberBundle {
    parent: AbstractEntityBundle,
    hooked_entity: HookedEntity,
    biting: Biting,
}
impl Default for FishingBobberBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Fox {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(FoxKind(d.value.into_int()?));
            }
            18 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(FoxSitting(bitfield & 0x1 != 0));
                entity.insert(Faceplanted(bitfield & 0x40 != 0));
                entity.insert(Sleeping(bitfield & 0x20 != 0));
                entity.insert(Pouncing(bitfield & 0x10 != 0));
                entity.insert(Crouching(bitfield & 0x4 != 0));
                entity.insert(FoxInterested(bitfield & 0x8 != 0));
            }
            19 => {
                entity.insert(TrustedId0(d.value.into_optional_uuid()?));
            }
            20 => {
                entity.insert(TrustedId1(d.value.into_optional_uuid()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FoxBundle {
    parent: AbstractAnimalBundle,
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
impl Default for FoxBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Frog {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(FrogVariant(d.value.into_frog_variant()?));
            }
            18 => {
                entity.insert(TongueTarget(d.value.into_optional_unsigned_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FrogBundle {
    parent: AbstractAnimalBundle,
    frog_variant: FrogVariant,
    tongue_target: TongueTarget,
}
impl Default for FrogBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            frog_variant: FrogVariant(azalea_registry::FrogVariant::Temperate),
            tongue_target: TongueTarget(OptionalUnsignedInt(None)),
        }
    }
}

#[derive(Component)]
pub struct Fuel(pub bool);
#[derive(Component)]
pub struct FurnaceMinecart;
impl FurnaceMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
            14 => {
                entity.insert(Fuel(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct FurnaceMinecartBundle {
    parent: AbstractMinecartBundle,
    fuel: Fuel,
}
impl Default for FurnaceMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
            fuel: Fuel(false),
        }
    }
}

#[derive(Component)]
pub struct IsCharging(pub bool);
#[derive(Component)]
pub struct Ghast;
impl Ghast {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(IsCharging(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GhastBundle {
    parent: AbstractInsentientBundle,
    is_charging: IsCharging,
}
impl Default for GhastBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            is_charging: IsCharging(false),
        }
    }
}

#[derive(Component)]
pub struct Giant;
impl Giant {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GiantBundle {
    parent: AbstractMonsterBundle,
}
impl Default for GiantBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component)]
pub struct ItemFrameItem(pub Slot);
#[derive(Component)]
pub struct Rotation(pub i32);
#[derive(Component)]
pub struct GlowItemFrame;
impl GlowItemFrame {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=9 => ItemFrame::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GlowItemFrameBundle {
    parent: ItemFrameBundle,
}
impl Default for GlowItemFrameBundle {
    fn default() -> Self {
        Self {
            parent: ItemFrameBundle {
                parent: AbstractEntityBundle {
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
                },
                item_frame_item: ItemFrameItem(Slot::Empty),
                rotation: Rotation(0),
            },
        }
    }
}

#[derive(Component)]
pub struct DarkTicksRemaining(pub i32);
#[derive(Component)]
pub struct GlowSquid;
impl GlowSquid {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => Squid::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(DarkTicksRemaining(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GlowSquidBundle {
    parent: SquidBundle,
    dark_ticks_remaining: DarkTicksRemaining,
}
impl Default for GlowSquidBundle {
    fn default() -> Self {
        Self {
            parent: SquidBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Goat {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(IsScreamingGoat(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(HasLeftHorn(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(HasRightHorn(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GoatBundle {
    parent: AbstractAnimalBundle,
    is_screaming_goat: IsScreamingGoat,
    has_left_horn: HasLeftHorn,
    has_right_horn: HasRightHorn,
}
impl Default for GoatBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            is_screaming_goat: IsScreamingGoat(false),
            has_left_horn: HasLeftHorn(true),
            has_right_horn: HasRightHorn(true),
        }
    }
}

#[derive(Component)]
pub struct Guardian;
impl Guardian {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(Moving(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(AttackTarget(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct GuardianBundle {
    parent: AbstractMonsterBundle,
    moving: Moving,
    attack_target: AttackTarget,
}
impl Default for GuardianBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            moving: Moving(false),
            attack_target: AttackTarget(0),
        }
    }
}

#[derive(Component)]
pub struct HoglinImmuneToZombification(pub bool);
#[derive(Component)]
pub struct Hoglin;
impl Hoglin {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(HoglinImmuneToZombification(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct HoglinBundle {
    parent: AbstractAnimalBundle,
    hoglin_immune_to_zombification: HoglinImmuneToZombification,
}
impl Default for HoglinBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            hoglin_immune_to_zombification: HoglinImmuneToZombification(false),
        }
    }
}

#[derive(Component)]
pub struct HopperMinecart;
impl HopperMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct HopperMinecartBundle {
    parent: AbstractMinecartBundle,
}
impl Default for HopperMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
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
impl Horse {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HorseTamed(bitfield & 0x2 != 0));
                entity.insert(HorseEating(bitfield & 0x10 != 0));
                entity.insert(HorseStanding(bitfield & 0x20 != 0));
                entity.insert(HorseBred(bitfield & 0x8 != 0));
                entity.insert(HorseSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(HorseOwnerUuid(d.value.into_optional_uuid()?));
            }
            19 => {
                entity.insert(HorseTypeVariant(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct HorseBundle {
    parent: AbstractAnimalBundle,
    horse_tamed: HorseTamed,
    horse_eating: HorseEating,
    horse_standing: HorseStanding,
    horse_bred: HorseBred,
    horse_saddled: HorseSaddled,
    horse_owner_uuid: HorseOwnerUuid,
    horse_type_variant: HorseTypeVariant,
}
impl Default for HorseBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Husk {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct HuskBundle {
    parent: ZombieBundle,
}
impl Default for HuskBundle {
    fn default() -> Self {
        Self {
            parent: ZombieBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component)]
pub struct IllusionerIsCelebrating(pub bool);
#[derive(Component)]
pub struct IllusionerSpellCasting(pub u8);
#[derive(Component)]
pub struct Illusioner;
impl Illusioner {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(IllusionerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(IllusionerSpellCasting(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct IllusionerBundle {
    parent: AbstractMonsterBundle,
    illusioner_is_celebrating: IllusionerIsCelebrating,
    illusioner_spell_casting: IllusionerSpellCasting,
}
impl Default for IllusionerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            illusioner_is_celebrating: IllusionerIsCelebrating(false),
            illusioner_spell_casting: IllusionerSpellCasting(0),
        }
    }
}

#[derive(Component)]
pub struct PlayerCreated(pub bool);
#[derive(Component)]
pub struct IronGolem;
impl IronGolem {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(PlayerCreated(bitfield & 0x1 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct IronGolemBundle {
    parent: AbstractCreatureBundle,
    player_created: PlayerCreated,
}
impl Default for IronGolemBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            player_created: PlayerCreated(false),
        }
    }
}

#[derive(Component)]
pub struct ItemItem(pub Slot);
#[derive(Component)]
pub struct Item;
impl Item {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(ItemItem(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ItemBundle {
    parent: AbstractEntityBundle,
    item_item: ItemItem,
}
impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            item_item: ItemItem(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct ItemFrame;
impl ItemFrame {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(ItemFrameItem(d.value.into_item_stack()?));
            }
            9 => {
                entity.insert(Rotation(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ItemFrameBundle {
    parent: AbstractEntityBundle,
    item_frame_item: ItemFrameItem,
    rotation: Rotation,
}
impl Default for ItemFrameBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            item_frame_item: ItemFrameItem(Slot::Empty),
            rotation: Rotation(0),
        }
    }
}

#[derive(Component)]
pub struct LeashKnot;
impl LeashKnot {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct LeashKnotBundle {
    parent: AbstractEntityBundle,
}
impl Default for LeashKnotBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct LightningBolt;
impl LightningBolt {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct LightningBoltBundle {
    parent: AbstractEntityBundle,
}
impl Default for LightningBoltBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Llama {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(LlamaTamed(bitfield & 0x2 != 0));
                entity.insert(LlamaEating(bitfield & 0x10 != 0));
                entity.insert(LlamaStanding(bitfield & 0x20 != 0));
                entity.insert(LlamaBred(bitfield & 0x8 != 0));
                entity.insert(LlamaSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(LlamaOwnerUuid(d.value.into_optional_uuid()?));
            }
            19 => {
                entity.insert(LlamaChest(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(Strength(d.value.into_int()?));
            }
            21 => {
                entity.insert(Swag(d.value.into_int()?));
            }
            22 => {
                entity.insert(LlamaVariant(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct LlamaBundle {
    parent: AbstractAnimalBundle,
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
impl Default for LlamaBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl LlamaSpit {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct LlamaSpitBundle {
    parent: AbstractEntityBundle,
}
impl Default for LlamaSpitBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct SlimeSize(pub i32);
#[derive(Component)]
pub struct MagmaCube;
impl MagmaCube {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Slime::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct MagmaCubeBundle {
    parent: SlimeBundle,
}
impl Default for MagmaCubeBundle {
    fn default() -> Self {
        Self {
            parent: SlimeBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
                slime_size: SlimeSize(1),
            },
        }
    }
}

#[derive(Component)]
pub struct Marker;
impl Marker {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct MarkerBundle {
    parent: AbstractEntityBundle,
}
impl Default for MarkerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct Minecart;
impl Minecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct MinecartBundle {
    parent: AbstractMinecartBundle,
}
impl Default for MinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component)]
pub struct MooshroomKind(pub String);
#[derive(Component)]
pub struct Mooshroom;
impl Mooshroom {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Cow::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(MooshroomKind(d.value.into_string()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct MooshroomBundle {
    parent: CowBundle,
    mooshroom_kind: MooshroomKind,
}
impl Default for MooshroomBundle {
    fn default() -> Self {
        Self {
            parent: CowBundle {
                parent: AbstractAnimalBundle {
                    parent: AbstractAgeableBundle {
                        parent: AbstractCreatureBundle {
                            parent: AbstractInsentientBundle {
                                parent: AbstractLivingBundle {
                                    parent: AbstractEntityBundle {
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
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
            },
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
impl Mule {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(MuleTamed(bitfield & 0x2 != 0));
                entity.insert(MuleEating(bitfield & 0x10 != 0));
                entity.insert(MuleStanding(bitfield & 0x20 != 0));
                entity.insert(MuleBred(bitfield & 0x8 != 0));
                entity.insert(MuleSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(MuleOwnerUuid(d.value.into_optional_uuid()?));
            }
            19 => {
                entity.insert(MuleChest(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct MuleBundle {
    parent: AbstractAnimalBundle,
    mule_tamed: MuleTamed,
    mule_eating: MuleEating,
    mule_standing: MuleStanding,
    mule_bred: MuleBred,
    mule_saddled: MuleSaddled,
    mule_owner_uuid: MuleOwnerUuid,
    mule_chest: MuleChest,
}
impl Default for MuleBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Ocelot {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(Trusting(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct OcelotBundle {
    parent: AbstractAnimalBundle,
    trusting: Trusting,
}
impl Default for OcelotBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            trusting: Trusting(false),
        }
    }
}

#[derive(Component)]
pub struct PaintingVariant(pub azalea_registry::PaintingVariant);
#[derive(Component)]
pub struct Painting;
impl Painting {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(PaintingVariant(d.value.into_painting_variant()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PaintingBundle {
    parent: AbstractEntityBundle,
    painting_variant: PaintingVariant,
}
impl Default for PaintingBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Panda {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(PandaUnhappyCounter(d.value.into_int()?));
            }
            18 => {
                entity.insert(SneezeCounter(d.value.into_int()?));
            }
            19 => {
                entity.insert(EatCounter(d.value.into_int()?));
            }
            20 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Sneezing(bitfield & 0x2 != 0));
                entity.insert(PandaSitting(bitfield & 0x8 != 0));
                entity.insert(OnBack(bitfield & 0x10 != 0));
                entity.insert(PandaRolling(bitfield & 0x4 != 0));
            }
            21 => {
                entity.insert(HiddenGene(d.value.into_byte()?));
            }
            22 => {
                entity.insert(PandaFlags(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PandaBundle {
    parent: AbstractAnimalBundle,
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
impl Default for PandaBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Parrot {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::update_metadata(ecs, entity, d)?,
            19 => {
                entity.insert(ParrotVariant(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ParrotBundle {
    parent: AbstractTameableBundle,
    parrot_variant: ParrotVariant,
}
impl Default for ParrotBundle {
    fn default() -> Self {
        Self {
            parent: AbstractTameableBundle {
                parent: AbstractAnimalBundle {
                    parent: AbstractAgeableBundle {
                        parent: AbstractCreatureBundle {
                            parent: AbstractInsentientBundle {
                                parent: AbstractLivingBundle {
                                    parent: AbstractEntityBundle {
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
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
            parrot_variant: ParrotVariant(0),
        }
    }
}

#[derive(Component)]
pub struct PhantomSize(pub i32);
#[derive(Component)]
pub struct Phantom;
impl Phantom {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(PhantomSize(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PhantomBundle {
    parent: AbstractInsentientBundle,
    phantom_size: PhantomSize,
}
impl Default for PhantomBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
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
impl Pig {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(PigSaddle(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(PigBoostTime(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PigBundle {
    parent: AbstractAnimalBundle,
    pig_saddle: PigSaddle,
    pig_boost_time: PigBoostTime,
}
impl Default for PigBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Piglin {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(PiglinImmuneToZombification(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PiglinBaby(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(PiglinIsChargingCrossbow(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(IsDancing(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PiglinBundle {
    parent: AbstractMonsterBundle,
    piglin_immune_to_zombification: PiglinImmuneToZombification,
    piglin_baby: PiglinBaby,
    piglin_is_charging_crossbow: PiglinIsChargingCrossbow,
    is_dancing: IsDancing,
}
impl Default for PiglinBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl PiglinBrute {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(PiglinBruteImmuneToZombification(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PiglinBruteBundle {
    parent: AbstractMonsterBundle,
    piglin_brute_immune_to_zombification: PiglinBruteImmuneToZombification,
}
impl Default for PiglinBruteBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Pillager {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(PillagerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PillagerIsChargingCrossbow(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PillagerBundle {
    parent: AbstractMonsterBundle,
    pillager_is_celebrating: PillagerIsCelebrating,
    pillager_is_charging_crossbow: PillagerIsChargingCrossbow,
}
impl Default for PillagerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Player {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::update_metadata(ecs, entity, d)?,
            15 => {
                entity.insert(PlayerAbsorption(d.value.into_float()?));
            }
            16 => {
                entity.insert(Score(d.value.into_int()?));
            }
            17 => {
                entity.insert(PlayerModeCustomisation(d.value.into_byte()?));
            }
            18 => {
                entity.insert(PlayerMainHand(d.value.into_byte()?));
            }
            19 => {
                entity.insert(ShoulderLeft(d.value.into_compound_tag()?));
            }
            20 => {
                entity.insert(ShoulderRight(d.value.into_compound_tag()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    parent: AbstractLivingBundle,
    player_absorption: PlayerAbsorption,
    score: Score,
    player_mode_customisation: PlayerModeCustomisation,
    player_main_hand: PlayerMainHand,
    shoulder_left: ShoulderLeft,
    shoulder_right: ShoulderRight,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractLivingBundle {
                parent: AbstractEntityBundle {
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
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
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
impl PolarBear {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(PolarBearStanding(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PolarBearBundle {
    parent: AbstractAnimalBundle,
    polar_bear_standing: PolarBearStanding,
}
impl Default for PolarBearBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            polar_bear_standing: PolarBearStanding(false),
        }
    }
}

#[derive(Component)]
pub struct PotionItemStack(pub Slot);
#[derive(Component)]
pub struct Potion;
impl Potion {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(PotionItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PotionBundle {
    parent: AbstractEntityBundle,
    potion_item_stack: PotionItemStack,
}
impl Default for PotionBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Pufferfish {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(PufferfishFromBucket(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PuffState(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PufferfishBundle {
    parent: AbstractCreatureBundle,
    pufferfish_from_bucket: PufferfishFromBucket,
    puff_state: PuffState,
}
impl Default for PufferfishBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            pufferfish_from_bucket: PufferfishFromBucket(false),
            puff_state: PuffState(0),
        }
    }
}

#[derive(Component)]
pub struct RabbitKind(pub i32);
#[derive(Component)]
pub struct Rabbit;
impl Rabbit {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(RabbitKind(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct RabbitBundle {
    parent: AbstractAnimalBundle,
    rabbit_kind: RabbitKind,
}
impl Default for RabbitBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            rabbit_kind: RabbitKind(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct RavagerIsCelebrating(pub bool);
#[derive(Component)]
pub struct Ravager;
impl Ravager {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(RavagerIsCelebrating(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct RavagerBundle {
    parent: AbstractMonsterBundle,
    ravager_is_celebrating: RavagerIsCelebrating,
}
impl Default for RavagerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            ravager_is_celebrating: RavagerIsCelebrating(false),
        }
    }
}

#[derive(Component)]
pub struct SalmonFromBucket(pub bool);
#[derive(Component)]
pub struct Salmon;
impl Salmon {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(SalmonFromBucket(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SalmonBundle {
    parent: AbstractCreatureBundle,
    salmon_from_bucket: SalmonFromBucket,
}
impl Default for SalmonBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            salmon_from_bucket: SalmonFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct Sheared(pub bool);
#[derive(Component)]
pub struct Sheep;
impl Sheep {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Sheared(bitfield & 0x10 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SheepBundle {
    parent: AbstractAnimalBundle,
    sheared: Sheared,
}
impl Default for SheepBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Shulker {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(AttachFace(d.value.into_direction()?));
            }
            17 => {
                entity.insert(Peek(d.value.into_byte()?));
            }
            18 => {
                entity.insert(ShulkerColor(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ShulkerBundle {
    parent: AbstractCreatureBundle,
    attach_face: AttachFace,
    peek: Peek,
    shulker_color: ShulkerColor,
}
impl Default for ShulkerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            attach_face: AttachFace(Default::default()),
            peek: Peek(0),
            shulker_color: ShulkerColor(16),
        }
    }
}

#[derive(Component)]
pub struct ShulkerBullet;
impl ShulkerBullet {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ShulkerBulletBundle {
    parent: AbstractEntityBundle,
}
impl Default for ShulkerBulletBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
        }
    }
}

#[derive(Component)]
pub struct Silverfish;
impl Silverfish {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SilverfishBundle {
    parent: AbstractMonsterBundle,
}
impl Default for SilverfishBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component)]
pub struct StrayConversion(pub bool);
#[derive(Component)]
pub struct Skeleton;
impl Skeleton {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(StrayConversion(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SkeletonBundle {
    parent: AbstractMonsterBundle,
    stray_conversion: StrayConversion,
}
impl Default for SkeletonBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl SkeletonHorse {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(SkeletonHorseTamed(bitfield & 0x2 != 0));
                entity.insert(SkeletonHorseEating(bitfield & 0x10 != 0));
                entity.insert(SkeletonHorseStanding(bitfield & 0x20 != 0));
                entity.insert(SkeletonHorseBred(bitfield & 0x8 != 0));
                entity.insert(SkeletonHorseSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(SkeletonHorseOwnerUuid(d.value.into_optional_uuid()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SkeletonHorseBundle {
    parent: AbstractAnimalBundle,
    skeleton_horse_tamed: SkeletonHorseTamed,
    skeleton_horse_eating: SkeletonHorseEating,
    skeleton_horse_standing: SkeletonHorseStanding,
    skeleton_horse_bred: SkeletonHorseBred,
    skeleton_horse_saddled: SkeletonHorseSaddled,
    skeleton_horse_owner_uuid: SkeletonHorseOwnerUuid,
}
impl Default for SkeletonHorseBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Slime {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(SlimeSize(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SlimeBundle {
    parent: AbstractInsentientBundle,
    slime_size: SlimeSize,
}
impl Default for SlimeBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            slime_size: SlimeSize(1),
        }
    }
}

#[derive(Component)]
pub struct SmallFireballItemStack(pub Slot);
#[derive(Component)]
pub struct SmallFireball;
impl SmallFireball {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(SmallFireballItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SmallFireballBundle {
    parent: AbstractEntityBundle,
    small_fireball_item_stack: SmallFireballItemStack,
}
impl Default for SmallFireballBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            small_fireball_item_stack: SmallFireballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct HasPumpkin(pub bool);
#[derive(Component)]
pub struct SnowGolem;
impl SnowGolem {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasPumpkin(bitfield & 0x10 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SnowGolemBundle {
    parent: AbstractCreatureBundle,
    has_pumpkin: HasPumpkin,
}
impl Default for SnowGolemBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            has_pumpkin: HasPumpkin(true),
        }
    }
}

#[derive(Component)]
pub struct SnowballItemStack(pub Slot);
#[derive(Component)]
pub struct Snowball;
impl Snowball {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(SnowballItemStack(d.value.into_item_stack()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SnowballBundle {
    parent: AbstractEntityBundle,
    snowball_item_stack: SnowballItemStack,
}
impl Default for SnowballBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            snowball_item_stack: SnowballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct SpawnerMinecart;
impl SpawnerMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SpawnerMinecartBundle {
    parent: AbstractMinecartBundle,
}
impl Default for SpawnerMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
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
impl SpectralArrow {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(SpectralArrowCritArrow(bitfield & 0x1 != 0));
                entity.insert(SpectralArrowShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(SpectralArrowNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(SpectralArrowPierceLevel(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SpectralArrowBundle {
    parent: AbstractEntityBundle,
    spectral_arrow_crit_arrow: SpectralArrowCritArrow,
    spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow,
    spectral_arrow_no_physics: SpectralArrowNoPhysics,
    spectral_arrow_pierce_level: SpectralArrowPierceLevel,
}
impl Default for SpectralArrowBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            spectral_arrow_crit_arrow: SpectralArrowCritArrow(false),
            spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow(false),
            spectral_arrow_no_physics: SpectralArrowNoPhysics(false),
            spectral_arrow_pierce_level: SpectralArrowPierceLevel(0),
        }
    }
}

#[derive(Component)]
pub struct Spider;
impl Spider {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Climbing(bitfield & 0x1 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SpiderBundle {
    parent: AbstractMonsterBundle,
    climbing: Climbing,
}
impl Default for SpiderBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            climbing: Climbing(false),
        }
    }
}

#[derive(Component)]
pub struct Squid;
impl Squid {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct SquidBundle {
    parent: AbstractCreatureBundle,
}
impl Default for SquidBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
        }
    }
}

#[derive(Component)]
pub struct Stray;
impl Stray {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct StrayBundle {
    parent: AbstractMonsterBundle,
}
impl Default for StrayBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Strider {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(StriderBoostTime(d.value.into_int()?));
            }
            18 => {
                entity.insert(Suffocating(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(StriderSaddle(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct StriderBundle {
    parent: AbstractAnimalBundle,
    strider_boost_time: StriderBoostTime,
    suffocating: Suffocating,
    strider_saddle: StriderSaddle,
}
impl Default for StriderBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Tadpole {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(TadpoleFromBucket(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TadpoleBundle {
    parent: AbstractCreatureBundle,
    tadpole_from_bucket: TadpoleFromBucket,
}
impl Default for TadpoleBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            tadpole_from_bucket: TadpoleFromBucket(false),
        }
    }
}

#[derive(Component)]
pub struct Fuse(pub i32);
#[derive(Component)]
pub struct Tnt;
impl Tnt {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(Fuse(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TntBundle {
    parent: AbstractEntityBundle,
    fuse: Fuse,
}
impl Default for TntBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            fuse: Fuse(80),
        }
    }
}

#[derive(Component)]
pub struct TntMinecart;
impl TntMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TntMinecartBundle {
    parent: AbstractMinecartBundle,
}
impl Default for TntMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMinecartBundle {
                parent: AbstractEntityBundle {
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
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component)]
pub struct TraderLlama;
impl TraderLlama {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=22 => Llama::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TraderLlamaBundle {
    parent: LlamaBundle,
}
impl Default for TraderLlamaBundle {
    fn default() -> Self {
        Self {
            parent: LlamaBundle {
                parent: AbstractAnimalBundle {
                    parent: AbstractAgeableBundle {
                        parent: AbstractCreatureBundle {
                            parent: AbstractInsentientBundle {
                                parent: AbstractLivingBundle {
                                    parent: AbstractEntityBundle {
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
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
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
            },
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
impl Trident {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(TridentCritArrow(bitfield & 0x1 != 0));
                entity.insert(TridentShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(TridentNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(TridentPierceLevel(d.value.into_byte()?));
            }
            10 => {
                entity.insert(Loyalty(d.value.into_byte()?));
            }
            11 => {
                entity.insert(Foil(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TridentBundle {
    parent: AbstractEntityBundle,
    trident_crit_arrow: TridentCritArrow,
    trident_shot_from_crossbow: TridentShotFromCrossbow,
    trident_no_physics: TridentNoPhysics,
    trident_pierce_level: TridentPierceLevel,
    loyalty: Loyalty,
    foil: Foil,
}
impl Default for TridentBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl TropicalFish {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(TropicalFishFromBucket(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(TropicalFishTypeVariant(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TropicalFishBundle {
    parent: AbstractCreatureBundle,
    tropical_fish_from_bucket: TropicalFishFromBucket,
    tropical_fish_type_variant: TropicalFishTypeVariant,
}
impl Default for TropicalFishBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
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
impl Turtle {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(HomePos(d.value.into_block_pos()?));
            }
            18 => {
                entity.insert(HasEgg(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(LayingEgg(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(TravelPos(d.value.into_block_pos()?));
            }
            21 => {
                entity.insert(GoingHome(d.value.into_boolean()?));
            }
            22 => {
                entity.insert(Travelling(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct TurtleBundle {
    parent: AbstractAnimalBundle,
    home_pos: HomePos,
    has_egg: HasEgg,
    laying_egg: LayingEgg,
    travel_pos: TravelPos,
    going_home: GoingHome,
    travelling: Travelling,
}
impl Default for TurtleBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl Vex {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(VexFlags(d.value.into_byte()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct VexBundle {
    parent: AbstractMonsterBundle,
    vex_flags: VexFlags,
}
impl Default for VexBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Villager {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(VillagerUnhappyCounter(d.value.into_int()?));
            }
            18 => {
                entity.insert(VillagerVillagerData(d.value.into_villager_data()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct VillagerBundle {
    parent: AbstractAgeableBundle,
    villager_unhappy_counter: VillagerUnhappyCounter,
    villager_villager_data: VillagerVillagerData,
}
impl Default for VillagerBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAgeableBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
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
impl Vindicator {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(VindicatorIsCelebrating(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct VindicatorBundle {
    parent: AbstractMonsterBundle,
    vindicator_is_celebrating: VindicatorIsCelebrating,
}
impl Default for VindicatorBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            vindicator_is_celebrating: VindicatorIsCelebrating(false),
        }
    }
}

#[derive(Component)]
pub struct WanderingTraderUnhappyCounter(pub i32);
#[derive(Component)]
pub struct WanderingTrader;
impl WanderingTrader {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::update_metadata(ecs, entity, d)?,
            17 => {
                entity.insert(WanderingTraderUnhappyCounter(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WanderingTraderBundle {
    parent: AbstractAgeableBundle,
    wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter,
}
impl Default for WanderingTraderBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAgeableBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
            wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter(0),
        }
    }
}

#[derive(Component)]
pub struct ClientAngerLevel(pub i32);
#[derive(Component)]
pub struct Warden;
impl Warden {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(ClientAngerLevel(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WardenBundle {
    parent: AbstractMonsterBundle,
    client_anger_level: ClientAngerLevel,
}
impl Default for WardenBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Witch {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(WitchIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(WitchUsingItem(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WitchBundle {
    parent: AbstractMonsterBundle,
    witch_is_celebrating: WitchIsCelebrating,
    witch_using_item: WitchUsingItem,
}
impl Default for WitchBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl Wither {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(TargetA(d.value.into_int()?));
            }
            17 => {
                entity.insert(TargetB(d.value.into_int()?));
            }
            18 => {
                entity.insert(TargetC(d.value.into_int()?));
            }
            19 => {
                entity.insert(Inv(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WitherBundle {
    parent: AbstractMonsterBundle,
    target_a: TargetA,
    target_b: TargetB,
    target_c: TargetC,
    inv: Inv,
}
impl Default for WitherBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            target_a: TargetA(0),
            target_b: TargetB(0),
            target_c: TargetC(0),
            inv: Inv(0),
        }
    }
}

#[derive(Component)]
pub struct WitherSkeleton;
impl WitherSkeleton {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WitherSkeletonBundle {
    parent: AbstractMonsterBundle,
}
impl Default for WitherSkeletonBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component)]
pub struct Dangerous(pub bool);
#[derive(Component)]
pub struct WitherSkull;
impl WitherSkull {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(Dangerous(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WitherSkullBundle {
    parent: AbstractEntityBundle,
    dangerous: Dangerous,
}
impl Default for WitherSkullBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
impl Wolf {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::update_metadata(ecs, entity, d)?,
            19 => {
                entity.insert(WolfInterested(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(WolfCollarColor(d.value.into_int()?));
            }
            21 => {
                entity.insert(WolfRemainingAngerTime(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct WolfBundle {
    parent: AbstractTameableBundle,
    wolf_interested: WolfInterested,
    wolf_collar_color: WolfCollarColor,
    wolf_remaining_anger_time: WolfRemainingAngerTime,
}
impl Default for WolfBundle {
    fn default() -> Self {
        Self {
            parent: AbstractTameableBundle {
                parent: AbstractAnimalBundle {
                    parent: AbstractAgeableBundle {
                        parent: AbstractCreatureBundle {
                            parent: AbstractInsentientBundle {
                                parent: AbstractLivingBundle {
                                    parent: AbstractEntityBundle {
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
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
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
impl Zoglin {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(ZoglinBaby(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ZoglinBundle {
    parent: AbstractMonsterBundle,
    zoglin_baby: ZoglinBaby,
}
impl Default for ZoglinBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            zoglin_baby: ZoglinBaby(false),
        }
    }
}

#[derive(Component)]
pub struct Zombie;
impl Zombie {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(ZombieBaby(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(SpecialType(d.value.into_int()?));
            }
            18 => {
                entity.insert(DrownedConversion(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ZombieBundle {
    parent: AbstractMonsterBundle,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl Default for ZombieBundle {
    fn default() -> Self {
        Self {
            parent: AbstractMonsterBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
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
impl ZombieHorse {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(ZombieHorseTamed(bitfield & 0x2 != 0));
                entity.insert(ZombieHorseEating(bitfield & 0x10 != 0));
                entity.insert(ZombieHorseStanding(bitfield & 0x20 != 0));
                entity.insert(ZombieHorseBred(bitfield & 0x8 != 0));
                entity.insert(ZombieHorseSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(ZombieHorseOwnerUuid(d.value.into_optional_uuid()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ZombieHorseBundle {
    parent: AbstractAnimalBundle,
    zombie_horse_tamed: ZombieHorseTamed,
    zombie_horse_eating: ZombieHorseEating,
    zombie_horse_standing: ZombieHorseStanding,
    zombie_horse_bred: ZombieHorseBred,
    zombie_horse_saddled: ZombieHorseSaddled,
    zombie_horse_owner_uuid: ZombieHorseOwnerUuid,
}
impl Default for ZombieHorseBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
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
impl ZombieVillager {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::update_metadata(ecs, entity, d)?,
            19 => {
                entity.insert(Converting(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(ZombieVillagerVillagerData(d.value.into_villager_data()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ZombieVillagerBundle {
    parent: ZombieBundle,
    converting: Converting,
    zombie_villager_villager_data: ZombieVillagerVillagerData,
}
impl Default for ZombieVillagerBundle {
    fn default() -> Self {
        Self {
            parent: ZombieBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
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
impl ZombifiedPiglin {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct ZombifiedPiglinBundle {
    parent: ZombieBundle,
}
impl Default for ZombifiedPiglinBundle {
    fn default() -> Self {
        Self {
            parent: ZombieBundle {
                parent: AbstractMonsterBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractAgeable;
impl AbstractAgeable {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
            16 => {
                entity.insert(AbstractAgeableBaby(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractAgeableBundle {
    parent: AbstractCreatureBundle,
    abstract_ageable_baby: AbstractAgeableBaby,
}
impl Default for AbstractAgeableBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            abstract_ageable_baby: AbstractAgeableBaby(false),
        }
    }
}

#[derive(Component)]
pub struct AbstractAnimal;
impl AbstractAnimal {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractAnimalBundle {
    parent: AbstractAgeableBundle,
}
impl Default for AbstractAnimalBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAgeableBundle {
                parent: AbstractCreatureBundle {
                    parent: AbstractInsentientBundle {
                        parent: AbstractLivingBundle {
                            parent: AbstractEntityBundle {
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
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractCreature;
impl AbstractCreature {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractCreatureBundle {
    parent: AbstractInsentientBundle,
}
impl Default for AbstractCreatureBundle {
    fn default() -> Self {
        Self {
            parent: AbstractInsentientBundle {
                parent: AbstractLivingBundle {
                    parent: AbstractEntityBundle {
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
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractEntity;
impl AbstractEntity {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(OnFire(bitfield & 0x1 != 0));
                entity.insert(ShiftKeyDown(bitfield & 0x2 != 0));
                entity.insert(Sprinting(bitfield & 0x8 != 0));
                entity.insert(Swimming(bitfield & 0x10 != 0));
                entity.insert(CurrentlyGlowing(bitfield & 0x40 != 0));
                entity.insert(Invisible(bitfield & 0x20 != 0));
                entity.insert(FallFlying(bitfield & 0x80 != 0));
            }
            1 => {
                entity.insert(AirSupply(d.value.into_int()?));
            }
            2 => {
                entity.insert(CustomName(d.value.into_optional_formatted_text()?));
            }
            3 => {
                entity.insert(CustomNameVisible(d.value.into_boolean()?));
            }
            4 => {
                entity.insert(Silent(d.value.into_boolean()?));
            }
            5 => {
                entity.insert(NoGravity(d.value.into_boolean()?));
            }
            6 => {
                entity.insert(d.value.into_pose()?);
            }
            7 => {
                entity.insert(TicksFrozen(d.value.into_int()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractEntityBundle {
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
impl Default for AbstractEntityBundle {
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
pub struct AbstractInsentient;
impl AbstractInsentient {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::update_metadata(ecs, entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(NoAi(bitfield & 0x1 != 0));
                entity.insert(LeftHanded(bitfield & 0x2 != 0));
                entity.insert(Aggressive(bitfield & 0x4 != 0));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractInsentientBundle {
    parent: AbstractLivingBundle,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl Default for AbstractInsentientBundle {
    fn default() -> Self {
        Self {
            parent: AbstractLivingBundle {
                parent: AbstractEntityBundle {
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
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct AbstractLiving;
impl AbstractLiving {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(AutoSpinAttack(bitfield & 0x4 != 0));
                entity.insert(AbstractLivingUsingItem(bitfield & 0x1 != 0));
            }
            9 => {
                entity.insert(Health(d.value.into_float()?));
            }
            10 => {
                entity.insert(AbstractLivingEffectColor(d.value.into_int()?));
            }
            11 => {
                entity.insert(EffectAmbience(d.value.into_boolean()?));
            }
            12 => {
                entity.insert(ArrowCount(d.value.into_int()?));
            }
            13 => {
                entity.insert(StingerCount(d.value.into_int()?));
            }
            14 => {
                entity.insert(SleepingPos(d.value.into_optional_block_pos()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractLivingBundle {
    parent: AbstractEntityBundle,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
}
impl Default for AbstractLivingBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
        }
    }
}

#[derive(Component)]
pub struct AbstractMinecart;
impl AbstractMinecart {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::update_metadata(ecs, entity, d)?,
            8 => {
                entity.insert(AbstractMinecartHurt(d.value.into_int()?));
            }
            9 => {
                entity.insert(AbstractMinecartHurtdir(d.value.into_int()?));
            }
            10 => {
                entity.insert(AbstractMinecartDamage(d.value.into_float()?));
            }
            11 => {
                entity.insert(DisplayBlock(d.value.into_int()?));
            }
            12 => {
                entity.insert(DisplayOffset(d.value.into_int()?));
            }
            13 => {
                entity.insert(CustomDisplay(d.value.into_boolean()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractMinecartBundle {
    parent: AbstractEntityBundle,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl Default for AbstractMinecartBundle {
    fn default() -> Self {
        Self {
            parent: AbstractEntityBundle {
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
            },
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
pub struct AbstractMonster;
impl AbstractMonster {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::update_metadata(ecs, entity, d)?,
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractMonsterBundle {
    parent: AbstractCreatureBundle,
}
impl Default for AbstractMonsterBundle {
    fn default() -> Self {
        Self {
            parent: AbstractCreatureBundle {
                parent: AbstractInsentientBundle {
                    parent: AbstractLivingBundle {
                        parent: AbstractEntityBundle {
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
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractTameable;
impl AbstractTameable {
    pub fn update_metadata(
        ecs: bevy_ecs::world::World,
        entity: &mut bevy_ecs::world::EntityMut,
        d: &EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::update_metadata(ecs, entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Tame(bitfield & 0x4 != 0));
                entity.insert(InSittingPose(bitfield & 0x1 != 0));
            }
            18 => {
                entity.insert(Owneruuid(d.value.into_optional_uuid()?));
            }
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct AbstractTameableBundle {
    parent: AbstractAnimalBundle,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
}
impl Default for AbstractTameableBundle {
    fn default() -> Self {
        Self {
            parent: AbstractAnimalBundle {
                parent: AbstractAgeableBundle {
                    parent: AbstractCreatureBundle {
                        parent: AbstractInsentientBundle {
                            parent: AbstractLivingBundle {
                                parent: AbstractEntityBundle {
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
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
        }
    }
}

pub fn update_metadatas(
    ecs: bevy_ecs::world::World,
    entity: bevy_ecs::world::EntityMut,
    items: &Vec<EntityDataItem>,
) -> Result<(), UpdateMetadataError> {
    if entity.contains::<Allay>() {
        for d in items {
            Allay::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<AreaEffectCloud>() {
        for d in items {
            AreaEffectCloud::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ArmorStand>() {
        for d in items {
            ArmorStand::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Arrow>() {
        for d in items {
            Arrow::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Axolotl>() {
        for d in items {
            Axolotl::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Bat>() {
        for d in items {
            Bat::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Bee>() {
        for d in items {
            Bee::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Blaze>() {
        for d in items {
            Blaze::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Boat>() {
        for d in items {
            Boat::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Camel>() {
        for d in items {
            Camel::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Cat>() {
        for d in items {
            Cat::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<CaveSpider>() {
        for d in items {
            CaveSpider::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ChestBoat>() {
        for d in items {
            ChestBoat::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ChestMinecart>() {
        for d in items {
            ChestMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Chicken>() {
        for d in items {
            Chicken::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Cod>() {
        for d in items {
            Cod::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<CommandBlockMinecart>() {
        for d in items {
            CommandBlockMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Cow>() {
        for d in items {
            Cow::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Creeper>() {
        for d in items {
            Creeper::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Dolphin>() {
        for d in items {
            Dolphin::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Donkey>() {
        for d in items {
            Donkey::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<DragonFireball>() {
        for d in items {
            DragonFireball::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Drowned>() {
        for d in items {
            Drowned::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Egg>() {
        for d in items {
            Egg::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ElderGuardian>() {
        for d in items {
            ElderGuardian::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<EndCrystal>() {
        for d in items {
            EndCrystal::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<EnderDragon>() {
        for d in items {
            EnderDragon::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<EnderPearl>() {
        for d in items {
            EnderPearl::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Enderman>() {
        for d in items {
            Enderman::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Endermite>() {
        for d in items {
            Endermite::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Evoker>() {
        for d in items {
            Evoker::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<EvokerFangs>() {
        for d in items {
            EvokerFangs::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ExperienceBottle>() {
        for d in items {
            ExperienceBottle::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ExperienceOrb>() {
        for d in items {
            ExperienceOrb::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<EyeOfEnder>() {
        for d in items {
            EyeOfEnder::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<FallingBlock>() {
        for d in items {
            FallingBlock::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Fireball>() {
        for d in items {
            Fireball::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<FireworkRocket>() {
        for d in items {
            FireworkRocket::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<FishingBobber>() {
        for d in items {
            FishingBobber::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Fox>() {
        for d in items {
            Fox::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Frog>() {
        for d in items {
            Frog::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<FurnaceMinecart>() {
        for d in items {
            FurnaceMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Ghast>() {
        for d in items {
            Ghast::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Giant>() {
        for d in items {
            Giant::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<GlowItemFrame>() {
        for d in items {
            GlowItemFrame::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<GlowSquid>() {
        for d in items {
            GlowSquid::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Goat>() {
        for d in items {
            Goat::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Guardian>() {
        for d in items {
            Guardian::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Hoglin>() {
        for d in items {
            Hoglin::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<HopperMinecart>() {
        for d in items {
            HopperMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Horse>() {
        for d in items {
            Horse::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Husk>() {
        for d in items {
            Husk::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Illusioner>() {
        for d in items {
            Illusioner::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<IronGolem>() {
        for d in items {
            IronGolem::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Item>() {
        for d in items {
            Item::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ItemFrame>() {
        for d in items {
            ItemFrame::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<LeashKnot>() {
        for d in items {
            LeashKnot::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<LightningBolt>() {
        for d in items {
            LightningBolt::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Llama>() {
        for d in items {
            Llama::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<LlamaSpit>() {
        for d in items {
            LlamaSpit::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<MagmaCube>() {
        for d in items {
            MagmaCube::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Marker>() {
        for d in items {
            Marker::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Minecart>() {
        for d in items {
            Minecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Mooshroom>() {
        for d in items {
            Mooshroom::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Mule>() {
        for d in items {
            Mule::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Ocelot>() {
        for d in items {
            Ocelot::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Painting>() {
        for d in items {
            Painting::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Panda>() {
        for d in items {
            Panda::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Parrot>() {
        for d in items {
            Parrot::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Phantom>() {
        for d in items {
            Phantom::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Pig>() {
        for d in items {
            Pig::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Piglin>() {
        for d in items {
            Piglin::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<PiglinBrute>() {
        for d in items {
            PiglinBrute::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Pillager>() {
        for d in items {
            Pillager::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Player>() {
        for d in items {
            Player::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<PolarBear>() {
        for d in items {
            PolarBear::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Potion>() {
        for d in items {
            Potion::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Pufferfish>() {
        for d in items {
            Pufferfish::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Rabbit>() {
        for d in items {
            Rabbit::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Ravager>() {
        for d in items {
            Ravager::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Salmon>() {
        for d in items {
            Salmon::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Sheep>() {
        for d in items {
            Sheep::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Shulker>() {
        for d in items {
            Shulker::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ShulkerBullet>() {
        for d in items {
            ShulkerBullet::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Silverfish>() {
        for d in items {
            Silverfish::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Skeleton>() {
        for d in items {
            Skeleton::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<SkeletonHorse>() {
        for d in items {
            SkeletonHorse::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Slime>() {
        for d in items {
            Slime::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<SmallFireball>() {
        for d in items {
            SmallFireball::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<SnowGolem>() {
        for d in items {
            SnowGolem::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Snowball>() {
        for d in items {
            Snowball::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<SpawnerMinecart>() {
        for d in items {
            SpawnerMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<SpectralArrow>() {
        for d in items {
            SpectralArrow::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Spider>() {
        for d in items {
            Spider::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Squid>() {
        for d in items {
            Squid::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Stray>() {
        for d in items {
            Stray::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Strider>() {
        for d in items {
            Strider::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Tadpole>() {
        for d in items {
            Tadpole::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Tnt>() {
        for d in items {
            Tnt::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<TntMinecart>() {
        for d in items {
            TntMinecart::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<TraderLlama>() {
        for d in items {
            TraderLlama::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Trident>() {
        for d in items {
            Trident::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<TropicalFish>() {
        for d in items {
            TropicalFish::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Turtle>() {
        for d in items {
            Turtle::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Vex>() {
        for d in items {
            Vex::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Villager>() {
        for d in items {
            Villager::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Vindicator>() {
        for d in items {
            Vindicator::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<WanderingTrader>() {
        for d in items {
            WanderingTrader::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Warden>() {
        for d in items {
            Warden::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Witch>() {
        for d in items {
            Witch::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Wither>() {
        for d in items {
            Wither::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<WitherSkeleton>() {
        for d in items {
            WitherSkeleton::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<WitherSkull>() {
        for d in items {
            WitherSkull::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Wolf>() {
        for d in items {
            Wolf::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Zoglin>() {
        for d in items {
            Zoglin::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<Zombie>() {
        for d in items {
            Zombie::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ZombieHorse>() {
        for d in items {
            ZombieHorse::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ZombieVillager>() {
        for d in items {
            ZombieVillager::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    if entity.contains::<ZombifiedPiglin>() {
        for d in items {
            ZombifiedPiglin::update_metadata(ecs, &mut entity, d)?;
        }
        return Ok(());
    }
    Ok(())
}
