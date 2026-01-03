// This file is @generated from codegen/lib/code/entity.py.
// Don't change it manually!

//! Metadata fields stored on entities.
//!
//! Also see the [protocol wiki documentation](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata).
//!
//! # Entities
//!
//! Azalea creates a marker ECS component for every entity and abstract entity.
//! You can use these to check if an entity is of a given type with an ECS
//! filter, such as `With<AbstractMonster>`.
//!
//! All marker components are shown as a tree structure below:
//!
//! - [AbstractEntity]
//!   - [AreaEffectCloud]
//!   - [BreezeWindCharge]
//!   - [DragonFireball]
//!   - [EndCrystal]
//!   - [EvokerFangs]
//!   - [ExperienceOrb]
//!   - [EyeOfEnder]
//!   - [FallingBlock]
//!   - [Fireball]
//!   - [FireworkRocket]
//!   - [FishingBobber]
//!   - [Interaction]
//!   - [Item]
//!   - [ItemFrame]
//!     - [GlowItemFrame]
//!   - [LeashKnot]
//!   - [LightningBolt]
//!   - [LlamaSpit]
//!   - [Marker]
//!   - [OminousItemSpawner]
//!   - [Painting]
//!   - [ShulkerBullet]
//!   - [SmallFireball]
//!   - [Tnt]
//!   - [WindCharge]
//!   - [WitherSkull]
//!   - [AbstractArrow]
//!     - [Arrow]
//!     - [SpectralArrow]
//!     - [Trident]
//!   - [AbstractDisplay]
//!     - [BlockDisplay]
//!     - [ItemDisplay]
//!     - [TextDisplay]
//!   - [AbstractLiving]
//!     - [ArmorStand]
//!     - [AbstractAvatar]
//!       - [Mannequin]
//!       - [Player]
//!     - [AbstractInsentient]
//!       - [Bat]
//!       - [EnderDragon]
//!       - [Ghast]
//!       - [Phantom]
//!       - [Slime]
//!         - [MagmaCube]
//!       - [AbstractCreature]
//!         - [Allay]
//!         - [CopperGolem]
//!         - [IronGolem]
//!         - [Pufferfish]
//!         - [Shulker]
//!         - [SnowGolem]
//!         - [Tadpole]
//!         - [AbstractAgeable]
//!           - [Dolphin]
//!           - [Squid]
//!             - [GlowSquid]
//!           - [AbstractAnimal]
//!             - [Armadillo]
//!             - [Axolotl]
//!             - [Bee]
//!             - [Chicken]
//!             - [Cow]
//!             - [Fox]
//!             - [Frog]
//!             - [Goat]
//!             - [HappyGhast]
//!             - [Hoglin]
//!             - [Mooshroom]
//!             - [Ocelot]
//!             - [Panda]
//!             - [Pig]
//!             - [PolarBear]
//!             - [Rabbit]
//!             - [Sheep]
//!             - [Sniffer]
//!             - [Strider]
//!             - [Turtle]
//!             - [AbstractHorse]
//!               - [Camel]
//!                 - [CamelHusk]
//!               - [Horse]
//!               - [SkeletonHorse]
//!               - [ZombieHorse]
//!               - [AbstractChestedHorse]
//!                 - [Donkey]
//!                 - [Llama]
//!                   - [TraderLlama]
//!                 - [Mule]
//!             - [AbstractTameable]
//!               - [Cat]
//!               - [Nautilus]
//!               - [Parrot]
//!               - [Wolf]
//!               - [ZombieNautilus]
//!           - [AbstractVillager]
//!             - [Villager]
//!             - [WanderingTrader]
//!         - [AbstractFish]
//!           - [Cod]
//!           - [Salmon]
//!           - [TropicalFish]
//!         - [AbstractMonster]
//!           - [Blaze]
//!           - [Bogged]
//!           - [Breeze]
//!           - [Creaking]
//!           - [Creeper]
//!           - [Enderman]
//!           - [Endermite]
//!           - [Giant]
//!           - [Guardian]
//!             - [ElderGuardian]
//!           - [Parched]
//!           - [Silverfish]
//!           - [Skeleton]
//!           - [Spider]
//!             - [CaveSpider]
//!           - [Stray]
//!           - [Vex]
//!           - [Warden]
//!           - [Wither]
//!           - [WitherSkeleton]
//!           - [Zoglin]
//!           - [Zombie]
//!             - [Drowned]
//!             - [Husk]
//!             - [ZombieVillager]
//!             - [ZombifiedPiglin]
//!           - [AbstractPiglin]
//!             - [Piglin]
//!             - [PiglinBrute]
//!           - [AbstractRaider]
//!             - [Pillager]
//!             - [Ravager]
//!             - [Vindicator]
//!             - [Witch]
//!             - [AbstractSpellcasterIllager]
//!               - [Evoker]
//!               - [Illusioner]
//!   - [AbstractThrownItemProjectile]
//!     - [Egg]
//!     - [EnderPearl]
//!     - [ExperienceBottle]
//!     - [LingeringPotion]
//!     - [Snowball]
//!     - [SplashPotion]
//!   - [AbstractVehicle]
//!     - [AbstractBoat]
//!       - [AcaciaBoat]
//!       - [AcaciaChestBoat]
//!       - [BambooChestRaft]
//!       - [BambooRaft]
//!       - [BirchBoat]
//!       - [BirchChestBoat]
//!       - [CherryBoat]
//!       - [CherryChestBoat]
//!       - [DarkOakBoat]
//!       - [DarkOakChestBoat]
//!       - [JungleBoat]
//!       - [JungleChestBoat]
//!       - [MangroveBoat]
//!       - [MangroveChestBoat]
//!       - [OakBoat]
//!       - [OakChestBoat]
//!       - [PaleOakBoat]
//!       - [PaleOakChestBoat]
//!       - [SpruceBoat]
//!       - [SpruceChestBoat]
//!     - [AbstractMinecart]
//!       - [ChestMinecart]
//!       - [CommandBlockMinecart]
//!       - [FurnaceMinecart]
//!       - [HopperMinecart]
//!       - [Minecart]
//!       - [SpawnerMinecart]
//!       - [TntMinecart]

#![allow(clippy::single_match)]

use azalea_chat::FormattedText;
use azalea_core::{
    direction::Direction,
    position::{BlockPos, Vec3f32},
};
use azalea_inventory::{ItemStack, components};
use azalea_registry::{DataRegistry, builtin::EntityKind};
use bevy_ecs::{bundle::Bundle, component::Component};
use derive_more::{Deref, DerefMut};
use thiserror::Error;
use uuid::Uuid;

use super::{
    ArmadilloStateKind, CopperGolemStateKind, EntityDataItem, EntityDataValue, OptionalUnsignedInt,
    Pose, Quaternion, Rotations, SnifferStateKind, VillagerData, WeatheringCopperStateKind,
};
use crate::{HumanoidArm, particle::Particle};

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

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct OnFire(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct AbstractEntityShiftKeyDown(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct Sprinting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct Swimming(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct CurrentlyGlowing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct Invisible(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractEntity].
pub struct FallFlying(pub bool);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AirSupply(pub i32);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CustomName(pub Option<Box<FormattedText>>);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CustomNameVisible(pub bool);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Silent(pub bool);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct NoGravity(pub bool);
/// A metadata field for [AbstractEntity].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TicksFrozen(pub i32);
/// The root entity marker component.
///
/// All entities that have had their metadata sent by the server will have this
/// component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractEntity` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [OnFire]
/// - [AbstractEntityShiftKeyDown]
/// - [Sprinting]
/// - [Swimming]
/// - [CurrentlyGlowing]
/// - [Invisible]
/// - [FallFlying]
/// - [AirSupply]
/// - [CustomName]
/// - [CustomNameVisible]
/// - [Silent]
/// - [NoGravity]
/// - [TicksFrozen]
///
/// # Children
///
/// - [AreaEffectCloud]
/// - [BreezeWindCharge]
/// - [DragonFireball]
/// - [EndCrystal]
/// - [EvokerFangs]
/// - [ExperienceOrb]
/// - [EyeOfEnder]
/// - [FallingBlock]
/// - [Fireball]
/// - [FireworkRocket]
/// - [FishingBobber]
/// - [Interaction]
/// - [Item]
/// - [ItemFrame]
///   - [GlowItemFrame]
/// - [LeashKnot]
/// - [LightningBolt]
/// - [LlamaSpit]
/// - [Marker]
/// - [OminousItemSpawner]
/// - [Painting]
/// - [ShulkerBullet]
/// - [SmallFireball]
/// - [Tnt]
/// - [WindCharge]
/// - [WitherSkull]
/// - [AbstractArrow]
///   - [Arrow]
///   - [SpectralArrow]
///   - [Trident]
/// - [AbstractDisplay]
///   - [BlockDisplay]
///   - [ItemDisplay]
///   - [TextDisplay]
/// - [AbstractLiving]
///   - [ArmorStand]
///   - [AbstractAvatar]
///     - [Mannequin]
///     - [Player]
///   - [AbstractInsentient]
///     - [Bat]
///     - [EnderDragon]
///     - [Ghast]
///     - [Phantom]
///     - [Slime]
///       - [MagmaCube]
///     - [AbstractCreature]
///       - [Allay]
///       - [CopperGolem]
///       - [IronGolem]
///       - [Pufferfish]
///       - [Shulker]
///       - [SnowGolem]
///       - [Tadpole]
///       - [AbstractAgeable]
///         - [Dolphin]
///         - [Squid]
///           - [GlowSquid]
///         - [AbstractAnimal]
///           - [Armadillo]
///           - [Axolotl]
///           - [Bee]
///           - [Chicken]
///           - [Cow]
///           - [Fox]
///           - [Frog]
///           - [Goat]
///           - [HappyGhast]
///           - [Hoglin]
///           - [Mooshroom]
///           - [Ocelot]
///           - [Panda]
///           - [Pig]
///           - [PolarBear]
///           - [Rabbit]
///           - [Sheep]
///           - [Sniffer]
///           - [Strider]
///           - [Turtle]
///           - [AbstractHorse]
///             - [Camel]
///               - [CamelHusk]
///             - [Horse]
///             - [SkeletonHorse]
///             - [ZombieHorse]
///             - [AbstractChestedHorse]
///               - [Donkey]
///               - [Llama]
///                 - [TraderLlama]
///               - [Mule]
///           - [AbstractTameable]
///             - [Cat]
///             - [Nautilus]
///             - [Parrot]
///             - [Wolf]
///             - [ZombieNautilus]
///         - [AbstractVillager]
///           - [Villager]
///           - [WanderingTrader]
///       - [AbstractFish]
///         - [Cod]
///         - [Salmon]
///         - [TropicalFish]
///       - [AbstractMonster]
///         - [Blaze]
///         - [Bogged]
///         - [Breeze]
///         - [Creaking]
///         - [Creeper]
///         - [Enderman]
///         - [Endermite]
///         - [Giant]
///         - [Guardian]
///           - [ElderGuardian]
///         - [Parched]
///         - [Silverfish]
///         - [Skeleton]
///         - [Spider]
///           - [CaveSpider]
///         - [Stray]
///         - [Vex]
///         - [Warden]
///         - [Wither]
///         - [WitherSkeleton]
///         - [Zoglin]
///         - [Zombie]
///           - [Drowned]
///           - [Husk]
///           - [ZombieVillager]
///           - [ZombifiedPiglin]
///         - [AbstractPiglin]
///           - [Piglin]
///           - [PiglinBrute]
///         - [AbstractRaider]
///           - [Pillager]
///           - [Ravager]
///           - [Vindicator]
///           - [Witch]
///           - [AbstractSpellcasterIllager]
///             - [Evoker]
///             - [Illusioner]
/// - [AbstractThrownItemProjectile]
///   - [Egg]
///   - [EnderPearl]
///   - [ExperienceBottle]
///   - [LingeringPotion]
///   - [Snowball]
///   - [SplashPotion]
/// - [AbstractVehicle]
///   - [AbstractBoat]
///     - [AcaciaBoat]
///     - [AcaciaChestBoat]
///     - [BambooChestRaft]
///     - [BambooRaft]
///     - [BirchBoat]
///     - [BirchChestBoat]
///     - [CherryBoat]
///     - [CherryChestBoat]
///     - [DarkOakBoat]
///     - [DarkOakChestBoat]
///     - [JungleBoat]
///     - [JungleChestBoat]
///     - [MangroveBoat]
///     - [MangroveChestBoat]
///     - [OakBoat]
///     - [OakChestBoat]
///     - [PaleOakBoat]
///     - [PaleOakChestBoat]
///     - [SpruceBoat]
///     - [SpruceChestBoat]
///   - [AbstractMinecart]
///     - [ChestMinecart]
///     - [CommandBlockMinecart]
///     - [FurnaceMinecart]
///     - [HopperMinecart]
///     - [Minecart]
///     - [SpawnerMinecart]
///     - [TntMinecart]
#[derive(Component)]
pub struct AbstractEntity;
impl AbstractEntity {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(OnFire(bitfield & 0x1 != 0));
                entity.insert(AbstractEntityShiftKeyDown(bitfield & 0x2 != 0));
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractEntity].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractEntityMetadataBundle {
    _marker: AbstractEntity,
    on_fire: OnFire,
    abstract_entity_shift_key_down: AbstractEntityShiftKeyDown,
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
impl Default for AbstractEntityMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractEntity,
            on_fire: OnFire(false),
            abstract_entity_shift_key_down: AbstractEntityShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(Default::default()),
            custom_name_visible: CustomNameVisible(Default::default()),
            silent: Silent(Default::default()),
            no_gravity: NoGravity(Default::default()),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(Default::default()),
        }
    }
}

/// A metadata field for [AreaEffectCloud].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Radius(pub f32);
/// A metadata field for [AreaEffectCloud].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Waiting(pub bool);
/// The marker component for entities of type `minecraft:area_effect_cloud`.
///
/// # Metadata
///
/// These are the metadata components that all `AreaEffectCloud` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Radius]
/// - [Waiting]
///
/// # Parents
///
/// Entities with `AreaEffectCloud` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct AreaEffectCloud;
impl AreaEffectCloud {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Radius(d.value.into_float()?));
            }
            9 => {
                entity.insert(Waiting(d.value.into_boolean()?));
            }
            10 => {
                entity.insert(d.value.into_particle()?);
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AreaEffectCloud].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AreaEffectCloudMetadataBundle {
    _marker: AreaEffectCloud,
    parent: AbstractEntityMetadataBundle,
    radius: Radius,
    waiting: Waiting,
    particle: Particle,
}
impl Default for AreaEffectCloudMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AreaEffectCloud,
            parent: Default::default(),
            radius: Radius(3.0),
            waiting: Waiting(false),
            particle: Particle::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:breeze_wind_charge`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `BreezeWindCharge` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BreezeWindCharge;
impl BreezeWindCharge {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BreezeWindCharge].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BreezeWindChargeMetadataBundle {
    _marker: BreezeWindCharge,
    parent: AbstractEntityMetadataBundle,
}
impl Default for BreezeWindChargeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BreezeWindCharge,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:dragon_fireball`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `DragonFireball` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct DragonFireball;
impl DragonFireball {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [DragonFireball].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DragonFireballMetadataBundle {
    _marker: DragonFireball,
    parent: AbstractEntityMetadataBundle,
}
impl Default for DragonFireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: DragonFireball,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [EndCrystal].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BeamTarget(pub Option<BlockPos>);
/// A metadata field for [EndCrystal].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShowBottom(pub bool);
/// The marker component for entities of type `minecraft:end_crystal`.
///
/// # Metadata
///
/// These are the metadata components that all `EndCrystal` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [BeamTarget]
/// - [ShowBottom]
///
/// # Parents
///
/// Entities with `EndCrystal` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct EndCrystal;
impl EndCrystal {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(BeamTarget(d.value.into_optional_block_pos()?));
            }
            9 => {
                entity.insert(ShowBottom(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [EndCrystal].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EndCrystalMetadataBundle {
    _marker: EndCrystal,
    parent: AbstractEntityMetadataBundle,
    beam_target: BeamTarget,
    show_bottom: ShowBottom,
}
impl Default for EndCrystalMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EndCrystal,
            parent: Default::default(),
            beam_target: BeamTarget(None),
            show_bottom: ShowBottom(true),
        }
    }
}

/// The marker component for entities of type `minecraft:evoker_fangs`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `EvokerFangs` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct EvokerFangs;
impl EvokerFangs {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [EvokerFangs].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EvokerFangsMetadataBundle {
    _marker: EvokerFangs,
    parent: AbstractEntityMetadataBundle,
}
impl Default for EvokerFangsMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EvokerFangs,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [ExperienceOrb].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Value(pub i32);
/// The marker component for entities of type `minecraft:experience_orb`.
///
/// # Metadata
///
/// These are the metadata components that all `ExperienceOrb` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Value]
///
/// # Parents
///
/// Entities with `ExperienceOrb` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ExperienceOrb;
impl ExperienceOrb {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Value(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ExperienceOrb].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ExperienceOrbMetadataBundle {
    _marker: ExperienceOrb,
    parent: AbstractEntityMetadataBundle,
    value: Value,
}
impl Default for ExperienceOrbMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ExperienceOrb,
            parent: Default::default(),
            value: Value(0),
        }
    }
}

/// A metadata field for [EyeOfEnder].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct EyeOfEnderItemStack(pub ItemStack);
/// The marker component for entities of type `minecraft:eye_of_ender`.
///
/// # Metadata
///
/// These are the metadata components that all `EyeOfEnder` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [EyeOfEnderItemStack]
///
/// # Parents
///
/// Entities with `EyeOfEnder` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct EyeOfEnder;
impl EyeOfEnder {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(EyeOfEnderItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [EyeOfEnder].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EyeOfEnderMetadataBundle {
    _marker: EyeOfEnder,
    parent: AbstractEntityMetadataBundle,
    eye_of_ender_item_stack: EyeOfEnderItemStack,
}
impl Default for EyeOfEnderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EyeOfEnder,
            parent: Default::default(),
            eye_of_ender_item_stack: EyeOfEnderItemStack(Default::default()),
        }
    }
}

/// A metadata field for [FallingBlock].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StartPos(pub BlockPos);
/// The marker component for entities of type `minecraft:falling_block`.
///
/// # Metadata
///
/// These are the metadata components that all `FallingBlock` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [StartPos]
///
/// # Parents
///
/// Entities with `FallingBlock` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct FallingBlock;
impl FallingBlock {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(StartPos(d.value.into_block_pos()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [FallingBlock].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FallingBlockMetadataBundle {
    _marker: FallingBlock,
    parent: AbstractEntityMetadataBundle,
    start_pos: StartPos,
}
impl Default for FallingBlockMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FallingBlock,
            parent: Default::default(),
            start_pos: StartPos(BlockPos::new(0, 0, 0)),
        }
    }
}

/// A metadata field for [Fireball].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct FireballItemStack(pub ItemStack);
/// The marker component for entities of type `minecraft:fireball`.
///
/// # Metadata
///
/// These are the metadata components that all `Fireball` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [FireballItemStack]
///
/// # Parents
///
/// Entities with `Fireball` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Fireball;
impl Fireball {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(FireballItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Fireball].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FireballMetadataBundle {
    _marker: Fireball,
    parent: AbstractEntityMetadataBundle,
    fireball_item_stack: FireballItemStack,
}
impl Default for FireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Fireball,
            parent: Default::default(),
            fireball_item_stack: FireballItemStack(Default::default()),
        }
    }
}

/// A metadata field for [FireworkRocket].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct FireworksItem(pub ItemStack);
/// A metadata field for [FireworkRocket].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AttachedToTarget(pub OptionalUnsignedInt);
/// A metadata field for [FireworkRocket].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShotAtAngle(pub bool);
/// The marker component for entities of type `minecraft:firework_rocket`.
///
/// # Metadata
///
/// These are the metadata components that all `FireworkRocket` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [FireworksItem]
/// - [AttachedToTarget]
/// - [ShotAtAngle]
///
/// # Parents
///
/// Entities with `FireworkRocket` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct FireworkRocket;
impl FireworkRocket {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(FireworksItem(d.value.into_item_stack()?));
            }
            9 => {
                entity.insert(AttachedToTarget(d.value.into_optional_unsigned_int()?));
            }
            10 => {
                entity.insert(ShotAtAngle(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [FireworkRocket].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FireworkRocketMetadataBundle {
    _marker: FireworkRocket,
    parent: AbstractEntityMetadataBundle,
    fireworks_item: FireworksItem,
    attached_to_target: AttachedToTarget,
    shot_at_angle: ShotAtAngle,
}
impl Default for FireworkRocketMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FireworkRocket,
            parent: Default::default(),
            fireworks_item: FireworksItem(Default::default()),
            attached_to_target: AttachedToTarget(OptionalUnsignedInt(None)),
            shot_at_angle: ShotAtAngle(false),
        }
    }
}

/// A metadata field for [FishingBobber].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HookedEntity(pub i32);
/// A metadata field for [FishingBobber].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Biting(pub bool);
/// The marker component for entities of type `minecraft:fishing_bobber`.
///
/// # Metadata
///
/// These are the metadata components that all `FishingBobber` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [HookedEntity]
/// - [Biting]
///
/// # Parents
///
/// Entities with `FishingBobber` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct FishingBobber;
impl FishingBobber {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(HookedEntity(d.value.into_int()?));
            }
            9 => {
                entity.insert(Biting(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [FishingBobber].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FishingBobberMetadataBundle {
    _marker: FishingBobber,
    parent: AbstractEntityMetadataBundle,
    hooked_entity: HookedEntity,
    biting: Biting,
}
impl Default for FishingBobberMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FishingBobber,
            parent: Default::default(),
            hooked_entity: HookedEntity(0),
            biting: Biting(false),
        }
    }
}

/// A metadata field for [Interaction].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct InteractionWidth(pub f32);
/// A metadata field for [Interaction].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct InteractionHeight(pub f32);
/// A metadata field for [Interaction].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Response(pub bool);
/// The marker component for entities of type `minecraft:interaction`.
///
/// # Metadata
///
/// These are the metadata components that all `Interaction` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [InteractionWidth]
/// - [InteractionHeight]
/// - [Response]
///
/// # Parents
///
/// Entities with `Interaction` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Interaction;
impl Interaction {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(InteractionWidth(d.value.into_float()?));
            }
            9 => {
                entity.insert(InteractionHeight(d.value.into_float()?));
            }
            10 => {
                entity.insert(Response(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Interaction].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct InteractionMetadataBundle {
    _marker: Interaction,
    parent: AbstractEntityMetadataBundle,
    interaction_width: InteractionWidth,
    interaction_height: InteractionHeight,
    response: Response,
}
impl Default for InteractionMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Interaction,
            parent: Default::default(),
            interaction_width: InteractionWidth(1.0),
            interaction_height: InteractionHeight(1.0),
            response: Response(false),
        }
    }
}

/// A metadata field for [Item].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ItemItem(pub ItemStack);
/// The marker component for entities of type `minecraft:item`.
///
/// # Metadata
///
/// These are the metadata components that all `Item` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [ItemItem]
///
/// # Parents
///
/// Entities with `Item` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Item;
impl Item {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ItemItem(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Item].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ItemMetadataBundle {
    _marker: Item,
    parent: AbstractEntityMetadataBundle,
    item_item: ItemItem,
}
impl Default for ItemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Item,
            parent: Default::default(),
            item_item: ItemItem(Default::default()),
        }
    }
}

/// A metadata field for [ItemFrame].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ItemFrameDirection(pub Direction);
/// A metadata field for [ItemFrame].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ItemFrameItem(pub ItemStack);
/// A metadata field for [ItemFrame].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Rotation(pub i32);
/// The marker component for entities of type `minecraft:item_frame`.
///
/// # Metadata
///
/// These are the metadata components that all `ItemFrame` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [ItemFrameDirection]
/// - [ItemFrameItem]
/// - [Rotation]
///
/// # Parents
///
/// Entities with `ItemFrame` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [GlowItemFrame]
#[derive(Component)]
pub struct ItemFrame;
impl ItemFrame {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ItemFrameDirection(d.value.into_direction()?));
            }
            9 => {
                entity.insert(ItemFrameItem(d.value.into_item_stack()?));
            }
            10 => {
                entity.insert(Rotation(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ItemFrame].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ItemFrameMetadataBundle {
    _marker: ItemFrame,
    parent: AbstractEntityMetadataBundle,
    item_frame_direction: ItemFrameDirection,
    item_frame_item: ItemFrameItem,
    rotation: Rotation,
}
impl Default for ItemFrameMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ItemFrame,
            parent: Default::default(),
            item_frame_direction: ItemFrameDirection(Default::default()),
            item_frame_item: ItemFrameItem(Default::default()),
            rotation: Rotation(0),
        }
    }
}

/// The marker component for entities of type `minecraft:glow_item_frame`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `GlowItemFrame` will also have the following marker components
/// and their metadata fields:
///
/// - [ItemFrame]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct GlowItemFrame;
impl GlowItemFrame {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => ItemFrame::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [GlowItemFrame].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GlowItemFrameMetadataBundle {
    _marker: GlowItemFrame,
    parent: ItemFrameMetadataBundle,
}
impl Default for GlowItemFrameMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: GlowItemFrame,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:leash_knot`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `LeashKnot` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct LeashKnot;
impl LeashKnot {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [LeashKnot].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct LeashKnotMetadataBundle {
    _marker: LeashKnot,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LeashKnotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LeashKnot,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:lightning_bolt`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `LightningBolt` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct LightningBolt;
impl LightningBolt {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [LightningBolt].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct LightningBoltMetadataBundle {
    _marker: LightningBolt,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LightningBoltMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LightningBolt,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:llama_spit`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `LlamaSpit` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct LlamaSpit;
impl LlamaSpit {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [LlamaSpit].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct LlamaSpitMetadataBundle {
    _marker: LlamaSpit,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LlamaSpitMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LlamaSpit,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:marker`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Marker` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Marker;
impl Marker {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Marker].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MarkerMetadataBundle {
    _marker: Marker,
    parent: AbstractEntityMetadataBundle,
}
impl Default for MarkerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Marker,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [OminousItemSpawner].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct OminousItemSpawnerItem(pub ItemStack);
/// The marker component for entities of type `minecraft:ominous_item_spawner`.
///
/// # Metadata
///
/// These are the metadata components that all `OminousItemSpawner` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [OminousItemSpawnerItem]
///
/// # Parents
///
/// Entities with `OminousItemSpawner` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct OminousItemSpawner;
impl OminousItemSpawner {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(OminousItemSpawnerItem(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [OminousItemSpawner].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct OminousItemSpawnerMetadataBundle {
    _marker: OminousItemSpawner,
    parent: AbstractEntityMetadataBundle,
    ominous_item_spawner_item: OminousItemSpawnerItem,
}
impl Default for OminousItemSpawnerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: OminousItemSpawner,
            parent: Default::default(),
            ominous_item_spawner_item: OminousItemSpawnerItem(Default::default()),
        }
    }
}

/// A metadata field for [Painting].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PaintingDirection(pub Direction);
/// A metadata field for [Painting].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PaintingVariant(pub azalea_registry::data::PaintingVariant);
/// The marker component for entities of type `minecraft:painting`.
///
/// # Metadata
///
/// These are the metadata components that all `Painting` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PaintingDirection]
/// - [PaintingVariant]
///
/// # Parents
///
/// Entities with `Painting` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Painting;
impl Painting {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(PaintingDirection(d.value.into_direction()?));
            }
            9 => {
                entity.insert(PaintingVariant(d.value.into_painting_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Painting].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PaintingMetadataBundle {
    _marker: Painting,
    parent: AbstractEntityMetadataBundle,
    painting_direction: PaintingDirection,
    painting_variant: PaintingVariant,
}
impl Default for PaintingMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Painting,
            parent: Default::default(),
            painting_direction: PaintingDirection(Default::default()),
            painting_variant: PaintingVariant(azalea_registry::data::PaintingVariant::new_raw(0)),
        }
    }
}

/// The marker component for entities of type `minecraft:shulker_bullet`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ShulkerBullet` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ShulkerBullet;
impl ShulkerBullet {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ShulkerBullet].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ShulkerBulletMetadataBundle {
    _marker: ShulkerBullet,
    parent: AbstractEntityMetadataBundle,
}
impl Default for ShulkerBulletMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ShulkerBullet,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [SmallFireball].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SmallFireballItemStack(pub ItemStack);
/// The marker component for entities of type `minecraft:small_fireball`.
///
/// # Metadata
///
/// These are the metadata components that all `SmallFireball` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [SmallFireballItemStack]
///
/// # Parents
///
/// Entities with `SmallFireball` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SmallFireball;
impl SmallFireball {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(SmallFireballItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SmallFireball].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SmallFireballMetadataBundle {
    _marker: SmallFireball,
    parent: AbstractEntityMetadataBundle,
    small_fireball_item_stack: SmallFireballItemStack,
}
impl Default for SmallFireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SmallFireball,
            parent: Default::default(),
            small_fireball_item_stack: SmallFireballItemStack(Default::default()),
        }
    }
}

/// A metadata field for [Tnt].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Fuse(pub i32);
/// A metadata field for [Tnt].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TntBlockState(pub azalea_block::BlockState);
/// The marker component for entities of type `minecraft:tnt`.
///
/// # Metadata
///
/// These are the metadata components that all `Tnt` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [Fuse]
/// - [TntBlockState]
///
/// # Parents
///
/// Entities with `Tnt` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Tnt;
impl Tnt {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Fuse(d.value.into_int()?));
            }
            9 => {
                entity.insert(TntBlockState(d.value.into_block_state()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Tnt].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TntMetadataBundle {
    _marker: Tnt,
    parent: AbstractEntityMetadataBundle,
    fuse: Fuse,
    tnt_block_state: TntBlockState,
}
impl Default for TntMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Tnt,
            parent: Default::default(),
            fuse: Fuse(80),
            tnt_block_state: TntBlockState(Default::default()),
        }
    }
}

/// The marker component for entities of type `minecraft:wind_charge`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `WindCharge` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct WindCharge;
impl WindCharge {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [WindCharge].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WindChargeMetadataBundle {
    _marker: WindCharge,
    parent: AbstractEntityMetadataBundle,
}
impl Default for WindChargeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WindCharge,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [WitherSkull].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Dangerous(pub bool);
/// The marker component for entities of type `minecraft:wither_skull`.
///
/// # Metadata
///
/// These are the metadata components that all `WitherSkull` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Dangerous]
///
/// # Parents
///
/// Entities with `WitherSkull` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct WitherSkull;
impl WitherSkull {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Dangerous(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [WitherSkull].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WitherSkullMetadataBundle {
    _marker: WitherSkull,
    parent: AbstractEntityMetadataBundle,
    dangerous: Dangerous,
}
impl Default for WitherSkullMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WitherSkull,
            parent: Default::default(),
            dangerous: Dangerous(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractArrow].
pub struct CritArrow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractArrow].
pub struct NoPhysics(pub bool);
/// A metadata field for [AbstractArrow].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PierceLevel(pub u8);
/// A metadata field for [AbstractArrow].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct InGround(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractArrow` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [CritArrow]
/// - [NoPhysics]
/// - [PierceLevel]
/// - [InGround]
///
/// # Parents
///
/// Entities with `AbstractArrow` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [Arrow]
/// - [SpectralArrow]
/// - [Trident]
#[derive(Component)]
pub struct AbstractArrow;
impl AbstractArrow {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(CritArrow(bitfield & 0x1 != 0));
                entity.insert(NoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(PierceLevel(d.value.into_byte()?));
            }
            10 => {
                entity.insert(InGround(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractArrow].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractArrowMetadataBundle {
    _marker: AbstractArrow,
    parent: AbstractEntityMetadataBundle,
    crit_arrow: CritArrow,
    no_physics: NoPhysics,
    pierce_level: PierceLevel,
    in_ground: InGround,
}
impl Default for AbstractArrowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractArrow,
            parent: Default::default(),
            crit_arrow: CritArrow(false),
            no_physics: NoPhysics(false),
            pierce_level: PierceLevel(0),
            in_ground: InGround(false),
        }
    }
}

/// A metadata field for [Arrow].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct EffectColor(pub i32);
/// The marker component for entities of type `minecraft:arrow`.
///
/// # Metadata
///
/// These are the metadata components that all `Arrow` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [EffectColor]
///
/// # Parents
///
/// Entities with `Arrow` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractArrow]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Arrow;
impl Arrow {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => AbstractArrow::apply_metadata(entity, d)?,
            11 => {
                entity.insert(EffectColor(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Arrow].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ArrowMetadataBundle {
    _marker: Arrow,
    parent: AbstractArrowMetadataBundle,
    effect_color: EffectColor,
}
impl Default for ArrowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Arrow,
            parent: Default::default(),
            effect_color: EffectColor(-1),
        }
    }
}

/// The marker component for entities of type `minecraft:spectral_arrow`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SpectralArrow` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractArrow]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SpectralArrow;
impl SpectralArrow {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => AbstractArrow::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SpectralArrow].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SpectralArrowMetadataBundle {
    _marker: SpectralArrow,
    parent: AbstractArrowMetadataBundle,
}
impl Default for SpectralArrowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpectralArrow,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Trident].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Loyalty(pub u8);
/// A metadata field for [Trident].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Foil(pub bool);
/// The marker component for entities of type `minecraft:trident`.
///
/// # Metadata
///
/// These are the metadata components that all `Trident` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Loyalty]
/// - [Foil]
///
/// # Parents
///
/// Entities with `Trident` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractArrow]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Trident;
impl Trident {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => AbstractArrow::apply_metadata(entity, d)?,
            11 => {
                entity.insert(Loyalty(d.value.into_byte()?));
            }
            12 => {
                entity.insert(Foil(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Trident].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TridentMetadataBundle {
    _marker: Trident,
    parent: AbstractArrowMetadataBundle,
    loyalty: Loyalty,
    foil: Foil,
}
impl Default for TridentMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Trident,
            parent: Default::default(),
            loyalty: Loyalty(0),
            foil: Foil(false),
        }
    }
}

/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TransformationInterpolationStartDeltaTicks(pub i32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TransformationInterpolationDuration(pub i32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PosRotInterpolationDuration(pub i32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Translation(pub Vec3f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Scale(pub Vec3f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LeftRotation(pub Quaternion);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct RightRotation(pub Quaternion);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BillboardRenderConstraints(pub u8);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BrightnessOverride(pub i32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ViewRange(pub f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShadowRadius(pub f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShadowStrength(pub f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractDisplayWidth(pub f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractDisplayHeight(pub f32);
/// A metadata field for [AbstractDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct GlowColorOverride(pub i32);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractDisplay` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [TransformationInterpolationStartDeltaTicks]
/// - [TransformationInterpolationDuration]
/// - [PosRotInterpolationDuration]
/// - [Translation]
/// - [Scale]
/// - [LeftRotation]
/// - [RightRotation]
/// - [BillboardRenderConstraints]
/// - [BrightnessOverride]
/// - [ViewRange]
/// - [ShadowRadius]
/// - [ShadowStrength]
/// - [AbstractDisplayWidth]
/// - [AbstractDisplayHeight]
/// - [GlowColorOverride]
///
/// # Parents
///
/// Entities with `AbstractDisplay` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [BlockDisplay]
/// - [ItemDisplay]
/// - [TextDisplay]
#[derive(Component)]
pub struct AbstractDisplay;
impl AbstractDisplay {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(TransformationInterpolationStartDeltaTicks(
                    d.value.into_int()?,
                ));
            }
            9 => {
                entity.insert(TransformationInterpolationDuration(d.value.into_int()?));
            }
            10 => {
                entity.insert(PosRotInterpolationDuration(d.value.into_int()?));
            }
            11 => {
                entity.insert(Translation(d.value.into_vector3()?));
            }
            12 => {
                entity.insert(Scale(d.value.into_vector3()?));
            }
            13 => {
                entity.insert(LeftRotation(d.value.into_quaternion()?));
            }
            14 => {
                entity.insert(RightRotation(d.value.into_quaternion()?));
            }
            15 => {
                entity.insert(BillboardRenderConstraints(d.value.into_byte()?));
            }
            16 => {
                entity.insert(BrightnessOverride(d.value.into_int()?));
            }
            17 => {
                entity.insert(ViewRange(d.value.into_float()?));
            }
            18 => {
                entity.insert(ShadowRadius(d.value.into_float()?));
            }
            19 => {
                entity.insert(ShadowStrength(d.value.into_float()?));
            }
            20 => {
                entity.insert(AbstractDisplayWidth(d.value.into_float()?));
            }
            21 => {
                entity.insert(AbstractDisplayHeight(d.value.into_float()?));
            }
            22 => {
                entity.insert(GlowColorOverride(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractDisplay].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractDisplayMetadataBundle {
    _marker: AbstractDisplay,
    parent: AbstractEntityMetadataBundle,
    transformation_interpolation_start_delta_ticks: TransformationInterpolationStartDeltaTicks,
    transformation_interpolation_duration: TransformationInterpolationDuration,
    pos_rot_interpolation_duration: PosRotInterpolationDuration,
    translation: Translation,
    scale: Scale,
    left_rotation: LeftRotation,
    right_rotation: RightRotation,
    billboard_render_constraints: BillboardRenderConstraints,
    brightness_override: BrightnessOverride,
    view_range: ViewRange,
    shadow_radius: ShadowRadius,
    shadow_strength: ShadowStrength,
    abstract_display_width: AbstractDisplayWidth,
    abstract_display_height: AbstractDisplayHeight,
    glow_color_override: GlowColorOverride,
}
impl Default for AbstractDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractDisplay,
            parent: Default::default(),
            transformation_interpolation_start_delta_ticks:
                TransformationInterpolationStartDeltaTicks(0),
            transformation_interpolation_duration: TransformationInterpolationDuration(0),
            pos_rot_interpolation_duration: PosRotInterpolationDuration(0),
            translation: Translation(Vec3f32 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            scale: Scale(Vec3f32 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            left_rotation: LeftRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            right_rotation: RightRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            billboard_render_constraints: BillboardRenderConstraints(Default::default()),
            brightness_override: BrightnessOverride(-1),
            view_range: ViewRange(1.0),
            shadow_radius: ShadowRadius(0.0),
            shadow_strength: ShadowStrength(1.0),
            abstract_display_width: AbstractDisplayWidth(0.0),
            abstract_display_height: AbstractDisplayHeight(0.0),
            glow_color_override: GlowColorOverride(-1),
        }
    }
}

/// A metadata field for [BlockDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BlockDisplayBlockState(pub azalea_block::BlockState);
/// The marker component for entities of type `minecraft:block_display`.
///
/// # Metadata
///
/// These are the metadata components that all `BlockDisplay` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [BlockDisplayBlockState]
///
/// # Parents
///
/// Entities with `BlockDisplay` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractDisplay]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BlockDisplay;
impl BlockDisplay {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=22 => AbstractDisplay::apply_metadata(entity, d)?,
            23 => {
                entity.insert(BlockDisplayBlockState(d.value.into_block_state()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BlockDisplay].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BlockDisplayMetadataBundle {
    _marker: BlockDisplay,
    parent: AbstractDisplayMetadataBundle,
    block_display_block_state: BlockDisplayBlockState,
}
impl Default for BlockDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BlockDisplay,
            parent: Default::default(),
            block_display_block_state: BlockDisplayBlockState(Default::default()),
        }
    }
}

/// A metadata field for [ItemDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ItemDisplayItemStack(pub ItemStack);
/// A metadata field for [ItemDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ItemDisplayItemDisplay(pub u8);
/// The marker component for entities of type `minecraft:item_display`.
///
/// # Metadata
///
/// These are the metadata components that all `ItemDisplay` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [ItemDisplayItemStack]
/// - [ItemDisplayItemDisplay]
///
/// # Parents
///
/// Entities with `ItemDisplay` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractDisplay]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ItemDisplay;
impl ItemDisplay {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=22 => AbstractDisplay::apply_metadata(entity, d)?,
            23 => {
                entity.insert(ItemDisplayItemStack(d.value.into_item_stack()?));
            }
            24 => {
                entity.insert(ItemDisplayItemDisplay(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ItemDisplay].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ItemDisplayMetadataBundle {
    _marker: ItemDisplay,
    parent: AbstractDisplayMetadataBundle,
    item_display_item_stack: ItemDisplayItemStack,
    item_display_item_display: ItemDisplayItemDisplay,
}
impl Default for ItemDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ItemDisplay,
            parent: Default::default(),
            item_display_item_stack: ItemDisplayItemStack(Default::default()),
            item_display_item_display: ItemDisplayItemDisplay(Default::default()),
        }
    }
}

/// A metadata field for [TextDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Text(pub Box<FormattedText>);
/// A metadata field for [TextDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LineWidth(pub i32);
/// A metadata field for [TextDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BackgroundColor(pub i32);
/// A metadata field for [TextDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TextOpacity(pub u8);
/// A metadata field for [TextDisplay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StyleFlags(pub u8);
/// The marker component for entities of type `minecraft:text_display`.
///
/// # Metadata
///
/// These are the metadata components that all `TextDisplay` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Text]
/// - [LineWidth]
/// - [BackgroundColor]
/// - [TextOpacity]
/// - [StyleFlags]
///
/// # Parents
///
/// Entities with `TextDisplay` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractDisplay]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct TextDisplay;
impl TextDisplay {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=22 => AbstractDisplay::apply_metadata(entity, d)?,
            23 => {
                entity.insert(Text(d.value.into_formatted_text()?));
            }
            24 => {
                entity.insert(LineWidth(d.value.into_int()?));
            }
            25 => {
                entity.insert(BackgroundColor(d.value.into_int()?));
            }
            26 => {
                entity.insert(TextOpacity(d.value.into_byte()?));
            }
            27 => {
                entity.insert(StyleFlags(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [TextDisplay].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TextDisplayMetadataBundle {
    _marker: TextDisplay,
    parent: AbstractDisplayMetadataBundle,
    text: Text,
    line_width: LineWidth,
    background_color: BackgroundColor,
    text_opacity: TextOpacity,
    style_flags: StyleFlags,
}
impl Default for TextDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TextDisplay,
            parent: Default::default(),
            text: Text(Default::default()),
            line_width: LineWidth(200),
            background_color: BackgroundColor(1073741824),
            text_opacity: TextOpacity(127),
            style_flags: StyleFlags(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractLiving].
pub struct AutoSpinAttack(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractLiving].
pub struct AbstractLivingUsingItem(pub bool);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Health(pub f32);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct EffectParticles(pub Box<[Particle]>);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct EffectAmbience(pub bool);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ArrowCount(pub i32);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StingerCount(pub i32);
/// A metadata field for [AbstractLiving].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SleepingPos(pub Option<BlockPos>);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractLiving` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [AutoSpinAttack]
/// - [AbstractLivingUsingItem]
/// - [Health]
/// - [EffectParticles]
/// - [EffectAmbience]
/// - [ArrowCount]
/// - [StingerCount]
/// - [SleepingPos]
///
/// # Parents
///
/// Entities with `AbstractLiving` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [ArmorStand]
/// - [AbstractAvatar]
///   - [Mannequin]
///   - [Player]
/// - [AbstractInsentient]
///   - [Bat]
///   - [EnderDragon]
///   - [Ghast]
///   - [Phantom]
///   - [Slime]
///     - [MagmaCube]
///   - [AbstractCreature]
///     - [Allay]
///     - [CopperGolem]
///     - [IronGolem]
///     - [Pufferfish]
///     - [Shulker]
///     - [SnowGolem]
///     - [Tadpole]
///     - [AbstractAgeable]
///       - [Dolphin]
///       - [Squid]
///         - [GlowSquid]
///       - [AbstractAnimal]
///         - [Armadillo]
///         - [Axolotl]
///         - [Bee]
///         - [Chicken]
///         - [Cow]
///         - [Fox]
///         - [Frog]
///         - [Goat]
///         - [HappyGhast]
///         - [Hoglin]
///         - [Mooshroom]
///         - [Ocelot]
///         - [Panda]
///         - [Pig]
///         - [PolarBear]
///         - [Rabbit]
///         - [Sheep]
///         - [Sniffer]
///         - [Strider]
///         - [Turtle]
///         - [AbstractHorse]
///           - [Camel]
///             - [CamelHusk]
///           - [Horse]
///           - [SkeletonHorse]
///           - [ZombieHorse]
///           - [AbstractChestedHorse]
///             - [Donkey]
///             - [Llama]
///               - [TraderLlama]
///             - [Mule]
///         - [AbstractTameable]
///           - [Cat]
///           - [Nautilus]
///           - [Parrot]
///           - [Wolf]
///           - [ZombieNautilus]
///       - [AbstractVillager]
///         - [Villager]
///         - [WanderingTrader]
///     - [AbstractFish]
///       - [Cod]
///       - [Salmon]
///       - [TropicalFish]
///     - [AbstractMonster]
///       - [Blaze]
///       - [Bogged]
///       - [Breeze]
///       - [Creaking]
///       - [Creeper]
///       - [Enderman]
///       - [Endermite]
///       - [Giant]
///       - [Guardian]
///         - [ElderGuardian]
///       - [Parched]
///       - [Silverfish]
///       - [Skeleton]
///       - [Spider]
///         - [CaveSpider]
///       - [Stray]
///       - [Vex]
///       - [Warden]
///       - [Wither]
///       - [WitherSkeleton]
///       - [Zoglin]
///       - [Zombie]
///         - [Drowned]
///         - [Husk]
///         - [ZombieVillager]
///         - [ZombifiedPiglin]
///       - [AbstractPiglin]
///         - [Piglin]
///         - [PiglinBrute]
///       - [AbstractRaider]
///         - [Pillager]
///         - [Ravager]
///         - [Vindicator]
///         - [Witch]
///         - [AbstractSpellcasterIllager]
///           - [Evoker]
///           - [Illusioner]
#[derive(Component)]
pub struct AbstractLiving;
impl AbstractLiving {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(AutoSpinAttack(bitfield & 0x4 != 0));
                entity.insert(AbstractLivingUsingItem(bitfield & 0x1 != 0));
            }
            9 => {
                entity.insert(Health(d.value.into_float()?));
            }
            10 => {
                entity.insert(EffectParticles(d.value.into_particles()?));
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractLiving].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractLivingMetadataBundle {
    _marker: AbstractLiving,
    parent: AbstractEntityMetadataBundle,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    effect_particles: EffectParticles,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
}
impl Default for AbstractLivingMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractLiving,
            parent: Default::default(),
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            effect_particles: EffectParticles(Default::default()),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [ArmorStand].
pub struct Small(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [ArmorStand].
pub struct ShowArms(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [ArmorStand].
pub struct ShowBasePlate(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [ArmorStand].
pub struct ArmorStandMarker(pub bool);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HeadPose(pub Rotations);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BodyPose(pub Rotations);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LeftArmPose(pub Rotations);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct RightArmPose(pub Rotations);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LeftLegPose(pub Rotations);
/// A metadata field for [ArmorStand].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct RightLegPose(pub Rotations);
/// The marker component for entities of type `minecraft:armor_stand`.
///
/// # Metadata
///
/// These are the metadata components that all `ArmorStand` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Small]
/// - [ShowArms]
/// - [ShowBasePlate]
/// - [ArmorStandMarker]
/// - [HeadPose]
/// - [BodyPose]
/// - [LeftArmPose]
/// - [RightArmPose]
/// - [LeftLegPose]
/// - [RightLegPose]
///
/// # Parents
///
/// Entities with `ArmorStand` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ArmorStand;
impl ArmorStand {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Small(bitfield & 0x1 != 0));
                entity.insert(ShowArms(bitfield & 0x4 != 0));
                entity.insert(ShowBasePlate(bitfield & 0x8 != 0));
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ArmorStand].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ArmorStandMetadataBundle {
    _marker: ArmorStand,
    parent: AbstractLivingMetadataBundle,
    small: Small,
    show_arms: ShowArms,
    show_base_plate: ShowBasePlate,
    armor_stand_marker: ArmorStandMarker,
    head_pose: HeadPose,
    body_pose: BodyPose,
    left_arm_pose: LeftArmPose,
    right_arm_pose: RightArmPose,
    left_leg_pose: LeftLegPose,
    right_leg_pose: RightLegPose,
}
impl Default for ArmorStandMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ArmorStand,
            parent: Default::default(),
            small: Small(false),
            show_arms: ShowArms(false),
            show_base_plate: ShowBasePlate(false),
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

/// A metadata field for [AbstractAvatar].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PlayerMainHand(pub HumanoidArm);
/// A metadata field for [AbstractAvatar].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PlayerModeCustomisation(pub u8);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractAvatar` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PlayerMainHand]
/// - [PlayerModeCustomisation]
///
/// # Parents
///
/// Entities with `AbstractAvatar` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Mannequin]
/// - [Player]
#[derive(Component)]
pub struct AbstractAvatar;
impl AbstractAvatar {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                entity.insert(PlayerMainHand(d.value.into_humanoid_arm()?));
            }
            16 => {
                entity.insert(PlayerModeCustomisation(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractAvatar].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractAvatarMetadataBundle {
    _marker: AbstractAvatar,
    parent: AbstractLivingMetadataBundle,
    player_main_hand: PlayerMainHand,
    player_mode_customisation: PlayerModeCustomisation,
}
impl Default for AbstractAvatarMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractAvatar,
            parent: Default::default(),
            player_main_hand: PlayerMainHand(Default::default()),
            player_mode_customisation: PlayerModeCustomisation(0),
        }
    }
}

/// A metadata field for [Mannequin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Profile(pub components::Profile);
/// A metadata field for [Mannequin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Immovable(pub bool);
/// A metadata field for [Mannequin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Description(pub Option<Box<FormattedText>>);
/// The marker component for entities of type `minecraft:mannequin`.
///
/// # Metadata
///
/// These are the metadata components that all `Mannequin` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Profile]
/// - [Immovable]
/// - [Description]
///
/// # Parents
///
/// Entities with `Mannequin` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAvatar]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Mannequin;
impl Mannequin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAvatar::apply_metadata(entity, d)?,
            17 => {
                entity.insert(Profile(d.value.into_resolvable_profile()?));
            }
            18 => {
                entity.insert(Immovable(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(Description(d.value.into_optional_formatted_text()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Mannequin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MannequinMetadataBundle {
    _marker: Mannequin,
    parent: AbstractAvatarMetadataBundle,
    profile: Profile,
    immovable: Immovable,
    description: Description,
}
impl Default for MannequinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Mannequin,
            parent: Default::default(),
            profile: Profile(Default::default()),
            immovable: Immovable(false),
            description: Description(Default::default()),
        }
    }
}

/// A metadata field for [Player].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PlayerAbsorption(pub f32);
/// A metadata field for [Player].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Score(pub i32);
/// A metadata field for [Player].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShoulderParrotLeft(pub OptionalUnsignedInt);
/// A metadata field for [Player].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ShoulderParrotRight(pub OptionalUnsignedInt);
/// The marker component for entities of type `minecraft:player`.
///
/// # Metadata
///
/// These are the metadata components that all `Player` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [PlayerAbsorption]
/// - [Score]
/// - [ShoulderParrotLeft]
/// - [ShoulderParrotRight]
///
/// # Parents
///
/// Entities with `Player` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAvatar]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Player;
impl Player {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAvatar::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PlayerAbsorption(d.value.into_float()?));
            }
            18 => {
                entity.insert(Score(d.value.into_int()?));
            }
            19 => {
                entity.insert(ShoulderParrotLeft(d.value.into_optional_unsigned_int()?));
            }
            20 => {
                entity.insert(ShoulderParrotRight(d.value.into_optional_unsigned_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Player].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PlayerMetadataBundle {
    _marker: Player,
    parent: AbstractAvatarMetadataBundle,
    player_absorption: PlayerAbsorption,
    score: Score,
    shoulder_parrot_left: ShoulderParrotLeft,
    shoulder_parrot_right: ShoulderParrotRight,
}
impl Default for PlayerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Player,
            parent: Default::default(),
            player_absorption: PlayerAbsorption(0.0),
            score: Score(0),
            shoulder_parrot_left: ShoulderParrotLeft(OptionalUnsignedInt(None)),
            shoulder_parrot_right: ShoulderParrotRight(OptionalUnsignedInt(None)),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractInsentient].
pub struct NoAi(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractInsentient].
pub struct LeftHanded(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractInsentient].
pub struct Aggressive(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractInsentient` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [NoAi]
/// - [LeftHanded]
/// - [Aggressive]
///
/// # Parents
///
/// Entities with `AbstractInsentient` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Bat]
/// - [EnderDragon]
/// - [Ghast]
/// - [Phantom]
/// - [Slime]
///   - [MagmaCube]
/// - [AbstractCreature]
///   - [Allay]
///   - [CopperGolem]
///   - [IronGolem]
///   - [Pufferfish]
///   - [Shulker]
///   - [SnowGolem]
///   - [Tadpole]
///   - [AbstractAgeable]
///     - [Dolphin]
///     - [Squid]
///       - [GlowSquid]
///     - [AbstractAnimal]
///       - [Armadillo]
///       - [Axolotl]
///       - [Bee]
///       - [Chicken]
///       - [Cow]
///       - [Fox]
///       - [Frog]
///       - [Goat]
///       - [HappyGhast]
///       - [Hoglin]
///       - [Mooshroom]
///       - [Ocelot]
///       - [Panda]
///       - [Pig]
///       - [PolarBear]
///       - [Rabbit]
///       - [Sheep]
///       - [Sniffer]
///       - [Strider]
///       - [Turtle]
///       - [AbstractHorse]
///         - [Camel]
///           - [CamelHusk]
///         - [Horse]
///         - [SkeletonHorse]
///         - [ZombieHorse]
///         - [AbstractChestedHorse]
///           - [Donkey]
///           - [Llama]
///             - [TraderLlama]
///           - [Mule]
///       - [AbstractTameable]
///         - [Cat]
///         - [Nautilus]
///         - [Parrot]
///         - [Wolf]
///         - [ZombieNautilus]
///     - [AbstractVillager]
///       - [Villager]
///       - [WanderingTrader]
///   - [AbstractFish]
///     - [Cod]
///     - [Salmon]
///     - [TropicalFish]
///   - [AbstractMonster]
///     - [Blaze]
///     - [Bogged]
///     - [Breeze]
///     - [Creaking]
///     - [Creeper]
///     - [Enderman]
///     - [Endermite]
///     - [Giant]
///     - [Guardian]
///       - [ElderGuardian]
///     - [Parched]
///     - [Silverfish]
///     - [Skeleton]
///     - [Spider]
///       - [CaveSpider]
///     - [Stray]
///     - [Vex]
///     - [Warden]
///     - [Wither]
///     - [WitherSkeleton]
///     - [Zoglin]
///     - [Zombie]
///       - [Drowned]
///       - [Husk]
///       - [ZombieVillager]
///       - [ZombifiedPiglin]
///     - [AbstractPiglin]
///       - [Piglin]
///       - [PiglinBrute]
///     - [AbstractRaider]
///       - [Pillager]
///       - [Ravager]
///       - [Vindicator]
///       - [Witch]
///       - [AbstractSpellcasterIllager]
///         - [Evoker]
///         - [Illusioner]
#[derive(Component)]
pub struct AbstractInsentient;
impl AbstractInsentient {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(NoAi(bitfield & 0x1 != 0));
                entity.insert(LeftHanded(bitfield & 0x2 != 0));
                entity.insert(Aggressive(bitfield & 0x4 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractInsentient].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractInsentientMetadataBundle {
    _marker: AbstractInsentient,
    parent: AbstractLivingMetadataBundle,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl Default for AbstractInsentientMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractInsentient,
            parent: Default::default(),
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Bat].
pub struct Resting(pub bool);
/// The marker component for entities of type `minecraft:bat`.
///
/// # Metadata
///
/// These are the metadata components that all `Bat` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [Resting]
///
/// # Parents
///
/// Entities with `Bat` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Bat;
impl Bat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Resting(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Bat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BatMetadataBundle {
    _marker: Bat,
    parent: AbstractInsentientMetadataBundle,
    resting: Resting,
}
impl Default for BatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Bat,
            parent: Default::default(),
            resting: Resting(false),
        }
    }
}

/// A metadata field for [EnderDragon].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Phase(pub i32);
/// The marker component for entities of type `minecraft:ender_dragon`.
///
/// # Metadata
///
/// These are the metadata components that all `EnderDragon` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Phase]
///
/// # Parents
///
/// Entities with `EnderDragon` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct EnderDragon;
impl EnderDragon {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Phase(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [EnderDragon].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EnderDragonMetadataBundle {
    _marker: EnderDragon,
    parent: AbstractInsentientMetadataBundle,
    phase: Phase,
}
impl Default for EnderDragonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EnderDragon,
            parent: Default::default(),
            phase: Phase(Default::default()),
        }
    }
}

/// A metadata field for [Ghast].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsCharging(pub bool);
/// The marker component for entities of type `minecraft:ghast`.
///
/// # Metadata
///
/// These are the metadata components that all `Ghast` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [IsCharging]
///
/// # Parents
///
/// Entities with `Ghast` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Ghast;
impl Ghast {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(IsCharging(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Ghast].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GhastMetadataBundle {
    _marker: Ghast,
    parent: AbstractInsentientMetadataBundle,
    is_charging: IsCharging,
}
impl Default for GhastMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ghast,
            parent: Default::default(),
            is_charging: IsCharging(false),
        }
    }
}

/// A metadata field for [Phantom].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PhantomSize(pub i32);
/// The marker component for entities of type `minecraft:phantom`.
///
/// # Metadata
///
/// These are the metadata components that all `Phantom` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [PhantomSize]
///
/// # Parents
///
/// Entities with `Phantom` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Phantom;
impl Phantom {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PhantomSize(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Phantom].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PhantomMetadataBundle {
    _marker: Phantom,
    parent: AbstractInsentientMetadataBundle,
    phantom_size: PhantomSize,
}
impl Default for PhantomMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Phantom,
            parent: Default::default(),
            phantom_size: PhantomSize(0),
        }
    }
}

/// A metadata field for [Slime].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SlimeSize(pub i32);
/// The marker component for entities of type `minecraft:slime`.
///
/// # Metadata
///
/// These are the metadata components that all `Slime` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [SlimeSize]
///
/// # Parents
///
/// Entities with `Slime` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [MagmaCube]
#[derive(Component)]
pub struct Slime;
impl Slime {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(SlimeSize(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Slime].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SlimeMetadataBundle {
    _marker: Slime,
    parent: AbstractInsentientMetadataBundle,
    slime_size: SlimeSize,
}
impl Default for SlimeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Slime,
            parent: Default::default(),
            slime_size: SlimeSize(1),
        }
    }
}

/// The marker component for entities of type `minecraft:magma_cube`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `MagmaCube` will also have the following marker components and
/// their metadata fields:
///
/// - [Slime]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct MagmaCube;
impl MagmaCube {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Slime::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [MagmaCube].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MagmaCubeMetadataBundle {
    _marker: MagmaCube,
    parent: SlimeMetadataBundle,
}
impl Default for MagmaCubeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: MagmaCube,
            parent: Default::default(),
        }
    }
}

/// An abstract entity marker component.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `AbstractCreature` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Allay]
/// - [CopperGolem]
/// - [IronGolem]
/// - [Pufferfish]
/// - [Shulker]
/// - [SnowGolem]
/// - [Tadpole]
/// - [AbstractAgeable]
///   - [Dolphin]
///   - [Squid]
///     - [GlowSquid]
///   - [AbstractAnimal]
///     - [Armadillo]
///     - [Axolotl]
///     - [Bee]
///     - [Chicken]
///     - [Cow]
///     - [Fox]
///     - [Frog]
///     - [Goat]
///     - [HappyGhast]
///     - [Hoglin]
///     - [Mooshroom]
///     - [Ocelot]
///     - [Panda]
///     - [Pig]
///     - [PolarBear]
///     - [Rabbit]
///     - [Sheep]
///     - [Sniffer]
///     - [Strider]
///     - [Turtle]
///     - [AbstractHorse]
///       - [Camel]
///         - [CamelHusk]
///       - [Horse]
///       - [SkeletonHorse]
///       - [ZombieHorse]
///       - [AbstractChestedHorse]
///         - [Donkey]
///         - [Llama]
///           - [TraderLlama]
///         - [Mule]
///     - [AbstractTameable]
///       - [Cat]
///       - [Nautilus]
///       - [Parrot]
///       - [Wolf]
///       - [ZombieNautilus]
///   - [AbstractVillager]
///     - [Villager]
///     - [WanderingTrader]
/// - [AbstractFish]
///   - [Cod]
///   - [Salmon]
///   - [TropicalFish]
/// - [AbstractMonster]
///   - [Blaze]
///   - [Bogged]
///   - [Breeze]
///   - [Creaking]
///   - [Creeper]
///   - [Enderman]
///   - [Endermite]
///   - [Giant]
///   - [Guardian]
///     - [ElderGuardian]
///   - [Parched]
///   - [Silverfish]
///   - [Skeleton]
///   - [Spider]
///     - [CaveSpider]
///   - [Stray]
///   - [Vex]
///   - [Warden]
///   - [Wither]
///   - [WitherSkeleton]
///   - [Zoglin]
///   - [Zombie]
///     - [Drowned]
///     - [Husk]
///     - [ZombieVillager]
///     - [ZombifiedPiglin]
///   - [AbstractPiglin]
///     - [Piglin]
///     - [PiglinBrute]
///   - [AbstractRaider]
///     - [Pillager]
///     - [Ravager]
///     - [Vindicator]
///     - [Witch]
///     - [AbstractSpellcasterIllager]
///       - [Evoker]
///       - [Illusioner]
#[derive(Component)]
pub struct AbstractCreature;
impl AbstractCreature {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractCreature].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractCreatureMetadataBundle {
    _marker: AbstractCreature,
    parent: AbstractInsentientMetadataBundle,
}
impl Default for AbstractCreatureMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractCreature,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Allay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Dancing(pub bool);
/// A metadata field for [Allay].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CanDuplicate(pub bool);
/// The marker component for entities of type `minecraft:allay`.
///
/// # Metadata
///
/// These are the metadata components that all `Allay` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Dancing]
/// - [CanDuplicate]
///
/// # Parents
///
/// Entities with `Allay` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Allay;
impl Allay {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Dancing(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(CanDuplicate(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Allay].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AllayMetadataBundle {
    _marker: Allay,
    parent: AbstractCreatureMetadataBundle,
    dancing: Dancing,
    can_duplicate: CanDuplicate,
}
impl Default for AllayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Allay,
            parent: Default::default(),
            dancing: Dancing(false),
            can_duplicate: CanDuplicate(true),
        }
    }
}

/// A metadata field for [CopperGolem].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WeatherState(pub WeatheringCopperStateKind);
/// A metadata field for [CopperGolem].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CopperGolemState(pub CopperGolemStateKind);
/// The marker component for entities of type `minecraft:copper_golem`.
///
/// # Metadata
///
/// These are the metadata components that all `CopperGolem` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [WeatherState]
/// - [CopperGolemState]
///
/// # Parents
///
/// Entities with `CopperGolem` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CopperGolem;
impl CopperGolem {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(WeatherState(d.value.into_weathering_copper_state()?));
            }
            17 => {
                entity.insert(CopperGolemState(d.value.into_copper_golem_state()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CopperGolem].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CopperGolemMetadataBundle {
    _marker: CopperGolem,
    parent: AbstractCreatureMetadataBundle,
    weather_state: WeatherState,
    copper_golem_state: CopperGolemState,
}
impl Default for CopperGolemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CopperGolem,
            parent: Default::default(),
            weather_state: WeatherState(Default::default()),
            copper_golem_state: CopperGolemState(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [IronGolem].
pub struct PlayerCreated(pub bool);
/// The marker component for entities of type `minecraft:iron_golem`.
///
/// # Metadata
///
/// These are the metadata components that all `IronGolem` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PlayerCreated]
///
/// # Parents
///
/// Entities with `IronGolem` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct IronGolem;
impl IronGolem {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(PlayerCreated(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [IronGolem].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct IronGolemMetadataBundle {
    _marker: IronGolem,
    parent: AbstractCreatureMetadataBundle,
    player_created: PlayerCreated,
}
impl Default for IronGolemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: IronGolem,
            parent: Default::default(),
            player_created: PlayerCreated(false),
        }
    }
}

/// A metadata field for [Pufferfish].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PufferfishFromBucket(pub bool);
/// A metadata field for [Pufferfish].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PuffState(pub i32);
/// The marker component for entities of type `minecraft:pufferfish`.
///
/// # Metadata
///
/// These are the metadata components that all `Pufferfish` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PufferfishFromBucket]
/// - [PuffState]
///
/// # Parents
///
/// Entities with `Pufferfish` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Pufferfish;
impl Pufferfish {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PufferfishFromBucket(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PuffState(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Pufferfish].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PufferfishMetadataBundle {
    _marker: Pufferfish,
    parent: AbstractCreatureMetadataBundle,
    pufferfish_from_bucket: PufferfishFromBucket,
    puff_state: PuffState,
}
impl Default for PufferfishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pufferfish,
            parent: Default::default(),
            pufferfish_from_bucket: PufferfishFromBucket(false),
            puff_state: PuffState(0),
        }
    }
}

/// A metadata field for [Shulker].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AttachFace(pub Direction);
/// A metadata field for [Shulker].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Peek(pub u8);
/// A metadata field for [Shulker].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Color(pub u8);
/// The marker component for entities of type `minecraft:shulker`.
///
/// # Metadata
///
/// These are the metadata components that all `Shulker` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [AttachFace]
/// - [Peek]
/// - [Color]
///
/// # Parents
///
/// Entities with `Shulker` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Shulker;
impl Shulker {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AttachFace(d.value.into_direction()?));
            }
            17 => {
                entity.insert(Peek(d.value.into_byte()?));
            }
            18 => {
                entity.insert(Color(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Shulker].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ShulkerMetadataBundle {
    _marker: Shulker,
    parent: AbstractCreatureMetadataBundle,
    attach_face: AttachFace,
    peek: Peek,
    color: Color,
}
impl Default for ShulkerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Shulker,
            parent: Default::default(),
            attach_face: AttachFace(Default::default()),
            peek: Peek(0),
            color: Color(16),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [SnowGolem].
pub struct HasPumpkin(pub bool);
/// The marker component for entities of type `minecraft:snow_golem`.
///
/// # Metadata
///
/// These are the metadata components that all `SnowGolem` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [HasPumpkin]
///
/// # Parents
///
/// Entities with `SnowGolem` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SnowGolem;
impl SnowGolem {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasPumpkin(bitfield & 0x10 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SnowGolem].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SnowGolemMetadataBundle {
    _marker: SnowGolem,
    parent: AbstractCreatureMetadataBundle,
    has_pumpkin: HasPumpkin,
}
impl Default for SnowGolemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SnowGolem,
            parent: Default::default(),
            has_pumpkin: HasPumpkin(true),
        }
    }
}

/// A metadata field for [Tadpole].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TadpoleFromBucket(pub bool);
/// The marker component for entities of type `minecraft:tadpole`.
///
/// # Metadata
///
/// These are the metadata components that all `Tadpole` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [TadpoleFromBucket]
///
/// # Parents
///
/// Entities with `Tadpole` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Tadpole;
impl Tadpole {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(TadpoleFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Tadpole].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TadpoleMetadataBundle {
    _marker: Tadpole,
    parent: AbstractCreatureMetadataBundle,
    tadpole_from_bucket: TadpoleFromBucket,
}
impl Default for TadpoleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Tadpole,
            parent: Default::default(),
            tadpole_from_bucket: TadpoleFromBucket(false),
        }
    }
}

/// A metadata field for [AbstractAgeable].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractAgeableBaby(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractAgeable` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [AbstractAgeableBaby]
///
/// # Parents
///
/// Entities with `AbstractAgeable` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Dolphin]
/// - [Squid]
///   - [GlowSquid]
/// - [AbstractAnimal]
///   - [Armadillo]
///   - [Axolotl]
///   - [Bee]
///   - [Chicken]
///   - [Cow]
///   - [Fox]
///   - [Frog]
///   - [Goat]
///   - [HappyGhast]
///   - [Hoglin]
///   - [Mooshroom]
///   - [Ocelot]
///   - [Panda]
///   - [Pig]
///   - [PolarBear]
///   - [Rabbit]
///   - [Sheep]
///   - [Sniffer]
///   - [Strider]
///   - [Turtle]
///   - [AbstractHorse]
///     - [Camel]
///       - [CamelHusk]
///     - [Horse]
///     - [SkeletonHorse]
///     - [ZombieHorse]
///     - [AbstractChestedHorse]
///       - [Donkey]
///       - [Llama]
///         - [TraderLlama]
///       - [Mule]
///   - [AbstractTameable]
///     - [Cat]
///     - [Nautilus]
///     - [Parrot]
///     - [Wolf]
///     - [ZombieNautilus]
/// - [AbstractVillager]
///   - [Villager]
///   - [WanderingTrader]
#[derive(Component)]
pub struct AbstractAgeable;
impl AbstractAgeable {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AbstractAgeableBaby(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractAgeable].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractAgeableMetadataBundle {
    _marker: AbstractAgeable,
    parent: AbstractCreatureMetadataBundle,
    abstract_ageable_baby: AbstractAgeableBaby,
}
impl Default for AbstractAgeableMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractAgeable,
            parent: Default::default(),
            abstract_ageable_baby: AbstractAgeableBaby(false),
        }
    }
}

/// A metadata field for [Dolphin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct GotFish(pub bool);
/// A metadata field for [Dolphin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct MoistnessLevel(pub i32);
/// The marker component for entities of type `minecraft:dolphin`.
///
/// # Metadata
///
/// These are the metadata components that all `Dolphin` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [GotFish]
/// - [MoistnessLevel]
///
/// # Parents
///
/// Entities with `Dolphin` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Dolphin;
impl Dolphin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            17 => {
                entity.insert(GotFish(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(MoistnessLevel(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Dolphin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DolphinMetadataBundle {
    _marker: Dolphin,
    parent: AbstractAgeableMetadataBundle,
    got_fish: GotFish,
    moistness_level: MoistnessLevel,
}
impl Default for DolphinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Dolphin,
            parent: Default::default(),
            got_fish: GotFish(false),
            moistness_level: MoistnessLevel(2400),
        }
    }
}

/// The marker component for entities of type `minecraft:squid`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Squid` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [GlowSquid]
#[derive(Component)]
pub struct Squid;
impl Squid {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Squid].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SquidMetadataBundle {
    _marker: Squid,
    parent: AbstractAgeableMetadataBundle,
}
impl Default for SquidMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Squid,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [GlowSquid].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct DarkTicksRemaining(pub i32);
/// The marker component for entities of type `minecraft:glow_squid`.
///
/// # Metadata
///
/// These are the metadata components that all `GlowSquid` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [DarkTicksRemaining]
///
/// # Parents
///
/// Entities with `GlowSquid` will also have the following marker components and
/// their metadata fields:
///
/// - [Squid]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct GlowSquid;
impl GlowSquid {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Squid::apply_metadata(entity, d)?,
            17 => {
                entity.insert(DarkTicksRemaining(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [GlowSquid].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GlowSquidMetadataBundle {
    _marker: GlowSquid,
    parent: SquidMetadataBundle,
    dark_ticks_remaining: DarkTicksRemaining,
}
impl Default for GlowSquidMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: GlowSquid,
            parent: Default::default(),
            dark_ticks_remaining: DarkTicksRemaining(0),
        }
    }
}

/// An abstract entity marker component.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `AbstractAnimal` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Armadillo]
/// - [Axolotl]
/// - [Bee]
/// - [Chicken]
/// - [Cow]
/// - [Fox]
/// - [Frog]
/// - [Goat]
/// - [HappyGhast]
/// - [Hoglin]
/// - [Mooshroom]
/// - [Ocelot]
/// - [Panda]
/// - [Pig]
/// - [PolarBear]
/// - [Rabbit]
/// - [Sheep]
/// - [Sniffer]
/// - [Strider]
/// - [Turtle]
/// - [AbstractHorse]
///   - [Camel]
///     - [CamelHusk]
///   - [Horse]
///   - [SkeletonHorse]
///   - [ZombieHorse]
///   - [AbstractChestedHorse]
///     - [Donkey]
///     - [Llama]
///       - [TraderLlama]
///     - [Mule]
/// - [AbstractTameable]
///   - [Cat]
///   - [Nautilus]
///   - [Parrot]
///   - [Wolf]
///   - [ZombieNautilus]
#[derive(Component)]
pub struct AbstractAnimal;
impl AbstractAnimal {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractAnimal].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractAnimalMetadataBundle {
    _marker: AbstractAnimal,
    parent: AbstractAgeableMetadataBundle,
}
impl Default for AbstractAnimalMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractAnimal,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Armadillo].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ArmadilloState(pub ArmadilloStateKind);
/// The marker component for entities of type `minecraft:armadillo`.
///
/// # Metadata
///
/// These are the metadata components that all `Armadillo` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [ArmadilloState]
///
/// # Parents
///
/// Entities with `Armadillo` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Armadillo;
impl Armadillo {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(ArmadilloState(d.value.into_armadillo_state()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Armadillo].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ArmadilloMetadataBundle {
    _marker: Armadillo,
    parent: AbstractAnimalMetadataBundle,
    armadillo_state: ArmadilloState,
}
impl Default for ArmadilloMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Armadillo,
            parent: Default::default(),
            armadillo_state: ArmadilloState(Default::default()),
        }
    }
}

/// A metadata field for [Axolotl].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AxolotlVariant(pub i32);
/// A metadata field for [Axolotl].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PlayingDead(pub bool);
/// A metadata field for [Axolotl].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AxolotlFromBucket(pub bool);
/// The marker component for entities of type `minecraft:axolotl`.
///
/// # Metadata
///
/// These are the metadata components that all `Axolotl` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [AxolotlVariant]
/// - [PlayingDead]
/// - [AxolotlFromBucket]
///
/// # Parents
///
/// Entities with `Axolotl` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Axolotl;
impl Axolotl {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(AxolotlVariant(d.value.into_int()?));
            }
            18 => {
                entity.insert(PlayingDead(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(AxolotlFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Axolotl].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AxolotlMetadataBundle {
    _marker: Axolotl,
    parent: AbstractAnimalMetadataBundle,
    axolotl_variant: AxolotlVariant,
    playing_dead: PlayingDead,
    axolotl_from_bucket: AxolotlFromBucket,
}
impl Default for AxolotlMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Axolotl,
            parent: Default::default(),
            axolotl_variant: AxolotlVariant(0),
            playing_dead: PlayingDead(false),
            axolotl_from_bucket: AxolotlFromBucket(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Bee].
pub struct HasNectar(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Bee].
pub struct HasStung(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Bee].
pub struct BeeRolling(pub bool);
/// A metadata field for [Bee].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BeeAngerEndTime(pub i64);
/// The marker component for entities of type `minecraft:bee`.
///
/// # Metadata
///
/// These are the metadata components that all `Bee` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [HasNectar]
/// - [HasStung]
/// - [BeeRolling]
/// - [BeeAngerEndTime]
///
/// # Parents
///
/// Entities with `Bee` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Bee;
impl Bee {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasNectar(bitfield & 0x8 != 0));
                entity.insert(HasStung(bitfield & 0x4 != 0));
                entity.insert(BeeRolling(bitfield & 0x2 != 0));
            }
            18 => {
                entity.insert(BeeAngerEndTime(d.value.into_long()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Bee].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BeeMetadataBundle {
    _marker: Bee,
    parent: AbstractAnimalMetadataBundle,
    has_nectar: HasNectar,
    has_stung: HasStung,
    bee_rolling: BeeRolling,
    bee_anger_end_time: BeeAngerEndTime,
}
impl Default for BeeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Bee,
            parent: Default::default(),
            has_nectar: HasNectar(false),
            has_stung: HasStung(false),
            bee_rolling: BeeRolling(false),
            bee_anger_end_time: BeeAngerEndTime(-1),
        }
    }
}

/// A metadata field for [Chicken].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ChickenVariant(pub azalea_registry::data::ChickenVariant);
/// The marker component for entities of type `minecraft:chicken`.
///
/// # Metadata
///
/// These are the metadata components that all `Chicken` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [ChickenVariant]
///
/// # Parents
///
/// Entities with `Chicken` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Chicken;
impl Chicken {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(ChickenVariant(d.value.into_chicken_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Chicken].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ChickenMetadataBundle {
    _marker: Chicken,
    parent: AbstractAnimalMetadataBundle,
    chicken_variant: ChickenVariant,
}
impl Default for ChickenMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Chicken,
            parent: Default::default(),
            chicken_variant: ChickenVariant(azalea_registry::data::ChickenVariant::new_raw(0)),
        }
    }
}

/// A metadata field for [Cow].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CowVariant(pub azalea_registry::data::CowVariant);
/// The marker component for entities of type `minecraft:cow`.
///
/// # Metadata
///
/// These are the metadata components that all `Cow` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [CowVariant]
///
/// # Parents
///
/// Entities with `Cow` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Cow;
impl Cow {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(CowVariant(d.value.into_cow_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Cow].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CowMetadataBundle {
    _marker: Cow,
    parent: AbstractAnimalMetadataBundle,
    cow_variant: CowVariant,
}
impl Default for CowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cow,
            parent: Default::default(),
            cow_variant: CowVariant(azalea_registry::data::CowVariant::new_raw(0)),
        }
    }
}

/// A metadata field for [Fox].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct FoxKind(pub i32);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct FoxSitting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct Faceplanted(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct Defending(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct Sleeping(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct Pouncing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct FoxCrouching(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Fox].
pub struct FoxInterested(pub bool);
/// A metadata field for [Fox].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TrustedId0(pub Option<Uuid>);
/// A metadata field for [Fox].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TrustedId1(pub Option<Uuid>);
/// The marker component for entities of type `minecraft:fox`.
///
/// # Metadata
///
/// These are the metadata components that all `Fox` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [FoxKind]
/// - [FoxSitting]
/// - [Faceplanted]
/// - [Defending]
/// - [Sleeping]
/// - [Pouncing]
/// - [FoxCrouching]
/// - [FoxInterested]
/// - [TrustedId0]
/// - [TrustedId1]
///
/// # Parents
///
/// Entities with `Fox` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Fox;
impl Fox {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(FoxKind(d.value.into_int()?));
            }
            18 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(FoxSitting(bitfield & 0x1 != 0));
                entity.insert(Faceplanted(bitfield & 0x40 != 0));
                entity.insert(Defending(bitfield & 0x80 != 0));
                entity.insert(Sleeping(bitfield & 0x20 != 0));
                entity.insert(Pouncing(bitfield & 0x10 != 0));
                entity.insert(FoxCrouching(bitfield & 0x4 != 0));
                entity.insert(FoxInterested(bitfield & 0x8 != 0));
            }
            19 => {
                entity.insert(TrustedId0(d.value.into_optional_living_entity_reference()?));
            }
            20 => {
                entity.insert(TrustedId1(d.value.into_optional_living_entity_reference()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Fox].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FoxMetadataBundle {
    _marker: Fox,
    parent: AbstractAnimalMetadataBundle,
    fox_kind: FoxKind,
    fox_sitting: FoxSitting,
    faceplanted: Faceplanted,
    defending: Defending,
    sleeping: Sleeping,
    pouncing: Pouncing,
    fox_crouching: FoxCrouching,
    fox_interested: FoxInterested,
    trusted_id_0: TrustedId0,
    trusted_id_1: TrustedId1,
}
impl Default for FoxMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Fox,
            parent: Default::default(),
            fox_kind: FoxKind(Default::default()),
            fox_sitting: FoxSitting(false),
            faceplanted: Faceplanted(false),
            defending: Defending(false),
            sleeping: Sleeping(false),
            pouncing: Pouncing(false),
            fox_crouching: FoxCrouching(false),
            fox_interested: FoxInterested(false),
            trusted_id_0: TrustedId0(None),
            trusted_id_1: TrustedId1(None),
        }
    }
}

/// A metadata field for [Frog].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct FrogVariant(pub azalea_registry::data::FrogVariant);
/// A metadata field for [Frog].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TongueTarget(pub OptionalUnsignedInt);
/// The marker component for entities of type `minecraft:frog`.
///
/// # Metadata
///
/// These are the metadata components that all `Frog` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [FrogVariant]
/// - [TongueTarget]
///
/// # Parents
///
/// Entities with `Frog` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Frog;
impl Frog {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(FrogVariant(d.value.into_frog_variant()?));
            }
            18 => {
                entity.insert(TongueTarget(d.value.into_optional_unsigned_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Frog].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FrogMetadataBundle {
    _marker: Frog,
    parent: AbstractAnimalMetadataBundle,
    frog_variant: FrogVariant,
    tongue_target: TongueTarget,
}
impl Default for FrogMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Frog,
            parent: Default::default(),
            frog_variant: FrogVariant(azalea_registry::data::FrogVariant::new_raw(0)),
            tongue_target: TongueTarget(OptionalUnsignedInt(None)),
        }
    }
}

/// A metadata field for [Goat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsScreamingGoat(pub bool);
/// A metadata field for [Goat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HasLeftHorn(pub bool);
/// A metadata field for [Goat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HasRightHorn(pub bool);
/// The marker component for entities of type `minecraft:goat`.
///
/// # Metadata
///
/// These are the metadata components that all `Goat` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [IsScreamingGoat]
/// - [HasLeftHorn]
/// - [HasRightHorn]
///
/// # Parents
///
/// Entities with `Goat` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Goat;
impl Goat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(IsScreamingGoat(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(HasLeftHorn(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(HasRightHorn(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Goat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GoatMetadataBundle {
    _marker: Goat,
    parent: AbstractAnimalMetadataBundle,
    is_screaming_goat: IsScreamingGoat,
    has_left_horn: HasLeftHorn,
    has_right_horn: HasRightHorn,
}
impl Default for GoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Goat,
            parent: Default::default(),
            is_screaming_goat: IsScreamingGoat(false),
            has_left_horn: HasLeftHorn(true),
            has_right_horn: HasRightHorn(true),
        }
    }
}

/// A metadata field for [HappyGhast].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsLeashHolder(pub bool);
/// A metadata field for [HappyGhast].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StaysStill(pub bool);
/// The marker component for entities of type `minecraft:happy_ghast`.
///
/// # Metadata
///
/// These are the metadata components that all `HappyGhast` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [IsLeashHolder]
/// - [StaysStill]
///
/// # Parents
///
/// Entities with `HappyGhast` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct HappyGhast;
impl HappyGhast {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(IsLeashHolder(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(StaysStill(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [HappyGhast].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct HappyGhastMetadataBundle {
    _marker: HappyGhast,
    parent: AbstractAnimalMetadataBundle,
    is_leash_holder: IsLeashHolder,
    stays_still: StaysStill,
}
impl Default for HappyGhastMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: HappyGhast,
            parent: Default::default(),
            is_leash_holder: IsLeashHolder(false),
            stays_still: StaysStill(false),
        }
    }
}

/// A metadata field for [Hoglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HoglinImmuneToZombification(pub bool);
/// The marker component for entities of type `minecraft:hoglin`.
///
/// # Metadata
///
/// These are the metadata components that all `Hoglin` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [HoglinImmuneToZombification]
///
/// # Parents
///
/// Entities with `Hoglin` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Hoglin;
impl Hoglin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(HoglinImmuneToZombification(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Hoglin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct HoglinMetadataBundle {
    _marker: Hoglin,
    parent: AbstractAnimalMetadataBundle,
    hoglin_immune_to_zombification: HoglinImmuneToZombification,
}
impl Default for HoglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Hoglin,
            parent: Default::default(),
            hoglin_immune_to_zombification: HoglinImmuneToZombification(false),
        }
    }
}

/// A metadata field for [Mooshroom].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct MooshroomKind(pub i32);
/// The marker component for entities of type `minecraft:mooshroom`.
///
/// # Metadata
///
/// These are the metadata components that all `Mooshroom` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [MooshroomKind]
///
/// # Parents
///
/// Entities with `Mooshroom` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Mooshroom;
impl Mooshroom {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(MooshroomKind(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Mooshroom].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MooshroomMetadataBundle {
    _marker: Mooshroom,
    parent: AbstractAnimalMetadataBundle,
    mooshroom_kind: MooshroomKind,
}
impl Default for MooshroomMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Mooshroom,
            parent: Default::default(),
            mooshroom_kind: MooshroomKind(Default::default()),
        }
    }
}

/// A metadata field for [Ocelot].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Trusting(pub bool);
/// The marker component for entities of type `minecraft:ocelot`.
///
/// # Metadata
///
/// These are the metadata components that all `Ocelot` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Trusting]
///
/// # Parents
///
/// Entities with `Ocelot` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Ocelot;
impl Ocelot {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(Trusting(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Ocelot].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct OcelotMetadataBundle {
    _marker: Ocelot,
    parent: AbstractAnimalMetadataBundle,
    trusting: Trusting,
}
impl Default for OcelotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ocelot,
            parent: Default::default(),
            trusting: Trusting(false),
        }
    }
}

/// A metadata field for [Panda].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PandaUnhappyCounter(pub i32);
/// A metadata field for [Panda].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SneezeCounter(pub i32);
/// A metadata field for [Panda].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct EatCounter(pub i32);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Panda].
pub struct Sneezing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Panda].
pub struct PandaSitting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Panda].
pub struct OnBack(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Panda].
pub struct PandaRolling(pub bool);
/// A metadata field for [Panda].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HiddenGene(pub u8);
/// A metadata field for [Panda].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PandaFlags(pub u8);
/// The marker component for entities of type `minecraft:panda`.
///
/// # Metadata
///
/// These are the metadata components that all `Panda` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [PandaUnhappyCounter]
/// - [SneezeCounter]
/// - [EatCounter]
/// - [Sneezing]
/// - [PandaSitting]
/// - [OnBack]
/// - [PandaRolling]
/// - [HiddenGene]
/// - [PandaFlags]
///
/// # Parents
///
/// Entities with `Panda` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Panda;
impl Panda {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Panda].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PandaMetadataBundle {
    _marker: Panda,
    parent: AbstractAnimalMetadataBundle,
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
impl Default for PandaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Panda,
            parent: Default::default(),
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

/// A metadata field for [Pig].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PigBoostTime(pub i32);
/// A metadata field for [Pig].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PigVariant(pub azalea_registry::data::PigVariant);
/// The marker component for entities of type `minecraft:pig`.
///
/// # Metadata
///
/// These are the metadata components that all `Pig` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [PigBoostTime]
/// - [PigVariant]
///
/// # Parents
///
/// Entities with `Pig` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Pig;
impl Pig {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PigBoostTime(d.value.into_int()?));
            }
            18 => {
                entity.insert(PigVariant(d.value.into_pig_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Pig].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PigMetadataBundle {
    _marker: Pig,
    parent: AbstractAnimalMetadataBundle,
    pig_boost_time: PigBoostTime,
    pig_variant: PigVariant,
}
impl Default for PigMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pig,
            parent: Default::default(),
            pig_boost_time: PigBoostTime(0),
            pig_variant: PigVariant(azalea_registry::data::PigVariant::new_raw(0)),
        }
    }
}

/// A metadata field for [PolarBear].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PolarBearStanding(pub bool);
/// The marker component for entities of type `minecraft:polar_bear`.
///
/// # Metadata
///
/// These are the metadata components that all `PolarBear` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PolarBearStanding]
///
/// # Parents
///
/// Entities with `PolarBear` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct PolarBear;
impl PolarBear {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PolarBearStanding(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [PolarBear].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PolarBearMetadataBundle {
    _marker: PolarBear,
    parent: AbstractAnimalMetadataBundle,
    polar_bear_standing: PolarBearStanding,
}
impl Default for PolarBearMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PolarBear,
            parent: Default::default(),
            polar_bear_standing: PolarBearStanding(false),
        }
    }
}

/// A metadata field for [Rabbit].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct RabbitKind(pub i32);
/// The marker component for entities of type `minecraft:rabbit`.
///
/// # Metadata
///
/// These are the metadata components that all `Rabbit` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [RabbitKind]
///
/// # Parents
///
/// Entities with `Rabbit` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Rabbit;
impl Rabbit {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(RabbitKind(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Rabbit].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct RabbitMetadataBundle {
    _marker: Rabbit,
    parent: AbstractAnimalMetadataBundle,
    rabbit_kind: RabbitKind,
}
impl Default for RabbitMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Rabbit,
            parent: Default::default(),
            rabbit_kind: RabbitKind(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Sheep].
pub struct SheepSheared(pub bool);
/// The marker component for entities of type `minecraft:sheep`.
///
/// # Metadata
///
/// These are the metadata components that all `Sheep` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [SheepSheared]
///
/// # Parents
///
/// Entities with `Sheep` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Sheep;
impl Sheep {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(SheepSheared(bitfield & 0x10 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Sheep].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SheepMetadataBundle {
    _marker: Sheep,
    parent: AbstractAnimalMetadataBundle,
    sheep_sheared: SheepSheared,
}
impl Default for SheepMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Sheep,
            parent: Default::default(),
            sheep_sheared: SheepSheared(false),
        }
    }
}

/// A metadata field for [Sniffer].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SnifferState(pub SnifferStateKind);
/// A metadata field for [Sniffer].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct DropSeedAtTick(pub i32);
/// The marker component for entities of type `minecraft:sniffer`.
///
/// # Metadata
///
/// These are the metadata components that all `Sniffer` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [SnifferState]
/// - [DropSeedAtTick]
///
/// # Parents
///
/// Entities with `Sniffer` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Sniffer;
impl Sniffer {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(SnifferState(d.value.into_sniffer_state()?));
            }
            18 => {
                entity.insert(DropSeedAtTick(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Sniffer].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SnifferMetadataBundle {
    _marker: Sniffer,
    parent: AbstractAnimalMetadataBundle,
    sniffer_state: SnifferState,
    drop_seed_at_tick: DropSeedAtTick,
}
impl Default for SnifferMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Sniffer,
            parent: Default::default(),
            sniffer_state: SnifferState(Default::default()),
            drop_seed_at_tick: DropSeedAtTick(0),
        }
    }
}

/// A metadata field for [Strider].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StriderBoostTime(pub i32);
/// A metadata field for [Strider].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Suffocating(pub bool);
/// The marker component for entities of type `minecraft:strider`.
///
/// # Metadata
///
/// These are the metadata components that all `Strider` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [StriderBoostTime]
/// - [Suffocating]
///
/// # Parents
///
/// Entities with `Strider` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Strider;
impl Strider {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(StriderBoostTime(d.value.into_int()?));
            }
            18 => {
                entity.insert(Suffocating(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Strider].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct StriderMetadataBundle {
    _marker: Strider,
    parent: AbstractAnimalMetadataBundle,
    strider_boost_time: StriderBoostTime,
    suffocating: Suffocating,
}
impl Default for StriderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Strider,
            parent: Default::default(),
            strider_boost_time: StriderBoostTime(0),
            suffocating: Suffocating(false),
        }
    }
}

/// A metadata field for [Turtle].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HasEgg(pub bool);
/// A metadata field for [Turtle].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LayingEgg(pub bool);
/// The marker component for entities of type `minecraft:turtle`.
///
/// # Metadata
///
/// These are the metadata components that all `Turtle` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [HasEgg]
/// - [LayingEgg]
///
/// # Parents
///
/// Entities with `Turtle` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Turtle;
impl Turtle {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(HasEgg(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(LayingEgg(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Turtle].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TurtleMetadataBundle {
    _marker: Turtle,
    parent: AbstractAnimalMetadataBundle,
    has_egg: HasEgg,
    laying_egg: LayingEgg,
}
impl Default for TurtleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Turtle,
            parent: Default::default(),
            has_egg: HasEgg(false),
            laying_egg: LayingEgg(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractHorse].
pub struct Tamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractHorse].
pub struct Eating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractHorse].
pub struct AbstractHorseStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractHorse].
pub struct Bred(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractHorse` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Tamed]
/// - [Eating]
/// - [AbstractHorseStanding]
/// - [Bred]
///
/// # Parents
///
/// Entities with `AbstractHorse` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Camel]
///   - [CamelHusk]
/// - [Horse]
/// - [SkeletonHorse]
/// - [ZombieHorse]
/// - [AbstractChestedHorse]
///   - [Donkey]
///   - [Llama]
///     - [TraderLlama]
///   - [Mule]
#[derive(Component)]
pub struct AbstractHorse;
impl AbstractHorse {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Tamed(bitfield & 0x2 != 0));
                entity.insert(Eating(bitfield & 0x10 != 0));
                entity.insert(AbstractHorseStanding(bitfield & 0x20 != 0));
                entity.insert(Bred(bitfield & 0x8 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractHorse].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractHorseMetadataBundle {
    _marker: AbstractHorse,
    parent: AbstractAnimalMetadataBundle,
    tamed: Tamed,
    eating: Eating,
    abstract_horse_standing: AbstractHorseStanding,
    bred: Bred,
}
impl Default for AbstractHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractHorse,
            parent: Default::default(),
            tamed: Tamed(false),
            eating: Eating(false),
            abstract_horse_standing: AbstractHorseStanding(false),
            bred: Bred(false),
        }
    }
}

/// A metadata field for [Camel].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CamelDash(pub bool);
/// A metadata field for [Camel].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LastPoseChangeTick(pub i64);
/// The marker component for entities of type `minecraft:camel`.
///
/// # Metadata
///
/// These are the metadata components that all `Camel` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [CamelDash]
/// - [LastPoseChangeTick]
///
/// # Parents
///
/// Entities with `Camel` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [CamelHusk]
#[derive(Component)]
pub struct Camel;
impl Camel {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractHorse::apply_metadata(entity, d)?,
            18 => {
                entity.insert(CamelDash(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(LastPoseChangeTick(d.value.into_long()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Camel].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CamelMetadataBundle {
    _marker: Camel,
    parent: AbstractHorseMetadataBundle,
    camel_dash: CamelDash,
    last_pose_change_tick: LastPoseChangeTick,
}
impl Default for CamelMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Camel,
            parent: Default::default(),
            camel_dash: CamelDash(false),
            last_pose_change_tick: LastPoseChangeTick(0),
        }
    }
}

/// The marker component for entities of type `minecraft:camel_husk`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `CamelHusk` will also have the following marker components and
/// their metadata fields:
///
/// - [Camel]
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CamelHusk;
impl CamelHusk {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=19 => Camel::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CamelHusk].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CamelHuskMetadataBundle {
    _marker: CamelHusk,
    parent: CamelMetadataBundle,
}
impl Default for CamelHuskMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CamelHusk,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Horse].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HorseTypeVariant(pub i32);
/// The marker component for entities of type `minecraft:horse`.
///
/// # Metadata
///
/// These are the metadata components that all `Horse` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [HorseTypeVariant]
///
/// # Parents
///
/// Entities with `Horse` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Horse;
impl Horse {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractHorse::apply_metadata(entity, d)?,
            18 => {
                entity.insert(HorseTypeVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Horse].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct HorseMetadataBundle {
    _marker: Horse,
    parent: AbstractHorseMetadataBundle,
    horse_type_variant: HorseTypeVariant,
}
impl Default for HorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Horse,
            parent: Default::default(),
            horse_type_variant: HorseTypeVariant(0),
        }
    }
}

/// The marker component for entities of type `minecraft:skeleton_horse`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SkeletonHorse` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SkeletonHorse;
impl SkeletonHorse {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractHorse::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SkeletonHorse].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SkeletonHorseMetadataBundle {
    _marker: SkeletonHorse,
    parent: AbstractHorseMetadataBundle,
}
impl Default for SkeletonHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SkeletonHorse,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:zombie_horse`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ZombieHorse` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ZombieHorse;
impl ZombieHorse {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractHorse::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ZombieHorse].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZombieHorseMetadataBundle {
    _marker: ZombieHorse,
    parent: AbstractHorseMetadataBundle,
}
impl Default for ZombieHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombieHorse,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractChestedHorse].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Chest(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractChestedHorse` entities
/// are guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Chest]
///
/// # Parents
///
/// Entities with `AbstractChestedHorse` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Donkey]
/// - [Llama]
///   - [TraderLlama]
/// - [Mule]
#[derive(Component)]
pub struct AbstractChestedHorse;
impl AbstractChestedHorse {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractHorse::apply_metadata(entity, d)?,
            18 => {
                entity.insert(Chest(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractChestedHorse].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractChestedHorseMetadataBundle {
    _marker: AbstractChestedHorse,
    parent: AbstractHorseMetadataBundle,
    chest: Chest,
}
impl Default for AbstractChestedHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractChestedHorse,
            parent: Default::default(),
            chest: Chest(false),
        }
    }
}

/// The marker component for entities of type `minecraft:donkey`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Donkey` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractChestedHorse]
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Donkey;
impl Donkey {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractChestedHorse::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Donkey].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DonkeyMetadataBundle {
    _marker: Donkey,
    parent: AbstractChestedHorseMetadataBundle,
}
impl Default for DonkeyMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Donkey,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Llama].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Strength(pub i32);
/// A metadata field for [Llama].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LlamaVariant(pub i32);
/// The marker component for entities of type `minecraft:llama`.
///
/// # Metadata
///
/// These are the metadata components that all `Llama` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Strength]
/// - [LlamaVariant]
///
/// # Parents
///
/// Entities with `Llama` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractChestedHorse]
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [TraderLlama]
#[derive(Component)]
pub struct Llama;
impl Llama {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractChestedHorse::apply_metadata(entity, d)?,
            19 => {
                entity.insert(Strength(d.value.into_int()?));
            }
            20 => {
                entity.insert(LlamaVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Llama].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct LlamaMetadataBundle {
    _marker: Llama,
    parent: AbstractChestedHorseMetadataBundle,
    strength: Strength,
    llama_variant: LlamaVariant,
}
impl Default for LlamaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Llama,
            parent: Default::default(),
            strength: Strength(0),
            llama_variant: LlamaVariant(0),
        }
    }
}

/// The marker component for entities of type `minecraft:trader_llama`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `TraderLlama` will also have the following marker components
/// and their metadata fields:
///
/// - [Llama]
/// - [AbstractChestedHorse]
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct TraderLlama;
impl TraderLlama {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=20 => Llama::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [TraderLlama].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TraderLlamaMetadataBundle {
    _marker: TraderLlama,
    parent: LlamaMetadataBundle,
}
impl Default for TraderLlamaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TraderLlama,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:mule`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Mule` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractChestedHorse]
/// - [AbstractHorse]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Mule;
impl Mule {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractChestedHorse::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Mule].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MuleMetadataBundle {
    _marker: Mule,
    parent: AbstractChestedHorseMetadataBundle,
}
impl Default for MuleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Mule,
            parent: Default::default(),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractTameable].
pub struct Tame(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [AbstractTameable].
pub struct InSittingPose(pub bool);
/// A metadata field for [AbstractTameable].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Owneruuid(pub Option<Uuid>);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractTameable` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Tame]
/// - [InSittingPose]
/// - [Owneruuid]
///
/// # Parents
///
/// Entities with `AbstractTameable` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Cat]
/// - [Nautilus]
/// - [Parrot]
/// - [Wolf]
/// - [ZombieNautilus]
#[derive(Component)]
pub struct AbstractTameable;
impl AbstractTameable {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Tame(bitfield & 0x4 != 0));
                entity.insert(InSittingPose(bitfield & 0x1 != 0));
            }
            18 => {
                entity.insert(Owneruuid(d.value.into_optional_living_entity_reference()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractTameable].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractTameableMetadataBundle {
    _marker: AbstractTameable,
    parent: AbstractAnimalMetadataBundle,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
}
impl Default for AbstractTameableMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractTameable,
            parent: Default::default(),
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
        }
    }
}

/// A metadata field for [Cat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CatVariant(pub azalea_registry::data::CatVariant);
/// A metadata field for [Cat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsLying(pub bool);
/// A metadata field for [Cat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct RelaxStateOne(pub bool);
/// A metadata field for [Cat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CatCollarColor(pub i32);
/// The marker component for entities of type `minecraft:cat`.
///
/// # Metadata
///
/// These are the metadata components that all `Cat` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [CatVariant]
/// - [IsLying]
/// - [RelaxStateOne]
/// - [CatCollarColor]
///
/// # Parents
///
/// Entities with `Cat` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractTameable]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Cat;
impl Cat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Cat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CatMetadataBundle {
    _marker: Cat,
    parent: AbstractTameableMetadataBundle,
    cat_variant: CatVariant,
    is_lying: IsLying,
    relax_state_one: RelaxStateOne,
    cat_collar_color: CatCollarColor,
}
impl Default for CatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cat,
            parent: Default::default(),
            cat_variant: CatVariant(azalea_registry::data::CatVariant::new_raw(0)),
            is_lying: IsLying(false),
            relax_state_one: RelaxStateOne(false),
            cat_collar_color: CatCollarColor(Default::default()),
        }
    }
}

/// A metadata field for [Nautilus].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct NautilusDash(pub bool);
/// The marker component for entities of type `minecraft:nautilus`.
///
/// # Metadata
///
/// These are the metadata components that all `Nautilus` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [NautilusDash]
///
/// # Parents
///
/// Entities with `Nautilus` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractTameable]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Nautilus;
impl Nautilus {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(NautilusDash(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Nautilus].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct NautilusMetadataBundle {
    _marker: Nautilus,
    parent: AbstractTameableMetadataBundle,
    nautilus_dash: NautilusDash,
}
impl Default for NautilusMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Nautilus,
            parent: Default::default(),
            nautilus_dash: NautilusDash(false),
        }
    }
}

/// A metadata field for [Parrot].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ParrotVariant(pub i32);
/// The marker component for entities of type `minecraft:parrot`.
///
/// # Metadata
///
/// These are the metadata components that all `Parrot` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [ParrotVariant]
///
/// # Parents
///
/// Entities with `Parrot` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractTameable]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Parrot;
impl Parrot {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(ParrotVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Parrot].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ParrotMetadataBundle {
    _marker: Parrot,
    parent: AbstractTameableMetadataBundle,
    parrot_variant: ParrotVariant,
}
impl Default for ParrotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Parrot,
            parent: Default::default(),
            parrot_variant: ParrotVariant(Default::default()),
        }
    }
}

/// A metadata field for [Wolf].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WolfInterested(pub bool);
/// A metadata field for [Wolf].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WolfCollarColor(pub i32);
/// A metadata field for [Wolf].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WolfAngerEndTime(pub i64);
/// A metadata field for [Wolf].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WolfVariant(pub azalea_registry::data::WolfVariant);
/// A metadata field for [Wolf].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SoundVariant(pub azalea_registry::data::WolfSoundVariant);
/// The marker component for entities of type `minecraft:wolf`.
///
/// # Metadata
///
/// These are the metadata components that all `Wolf` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [WolfInterested]
/// - [WolfCollarColor]
/// - [WolfAngerEndTime]
/// - [WolfVariant]
/// - [SoundVariant]
///
/// # Parents
///
/// Entities with `Wolf` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractTameable]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Wolf;
impl Wolf {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(WolfInterested(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(WolfCollarColor(d.value.into_int()?));
            }
            21 => {
                entity.insert(WolfAngerEndTime(d.value.into_long()?));
            }
            22 => {
                entity.insert(WolfVariant(d.value.into_wolf_variant()?));
            }
            23 => {
                entity.insert(SoundVariant(d.value.into_wolf_sound_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Wolf].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WolfMetadataBundle {
    _marker: Wolf,
    parent: AbstractTameableMetadataBundle,
    wolf_interested: WolfInterested,
    wolf_collar_color: WolfCollarColor,
    wolf_anger_end_time: WolfAngerEndTime,
    wolf_variant: WolfVariant,
    sound_variant: SoundVariant,
}
impl Default for WolfMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Wolf,
            parent: Default::default(),
            wolf_interested: WolfInterested(false),
            wolf_collar_color: WolfCollarColor(Default::default()),
            wolf_anger_end_time: WolfAngerEndTime(-1),
            wolf_variant: WolfVariant(azalea_registry::data::WolfVariant::new_raw(0)),
            sound_variant: SoundVariant(azalea_registry::data::WolfSoundVariant::new_raw(0)),
        }
    }
}

/// A metadata field for [ZombieNautilus].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ZombieNautilusDash(pub bool);
/// A metadata field for [ZombieNautilus].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ZombieNautilusVariant(pub azalea_registry::data::ZombieNautilusVariant);
/// The marker component for entities of type `minecraft:zombie_nautilus`.
///
/// # Metadata
///
/// These are the metadata components that all `ZombieNautilus` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [ZombieNautilusDash]
/// - [ZombieNautilusVariant]
///
/// # Parents
///
/// Entities with `ZombieNautilus` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractTameable]
/// - [AbstractAnimal]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ZombieNautilus;
impl ZombieNautilus {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(ZombieNautilusDash(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(ZombieNautilusVariant(
                    d.value.into_zombie_nautilus_variant()?,
                ));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ZombieNautilus].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZombieNautilusMetadataBundle {
    _marker: ZombieNautilus,
    parent: AbstractTameableMetadataBundle,
    zombie_nautilus_dash: ZombieNautilusDash,
    zombie_nautilus_variant: ZombieNautilusVariant,
}
impl Default for ZombieNautilusMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombieNautilus,
            parent: Default::default(),
            zombie_nautilus_dash: ZombieNautilusDash(false),
            zombie_nautilus_variant: ZombieNautilusVariant(
                azalea_registry::data::ZombieNautilusVariant::new_raw(0),
            ),
        }
    }
}

/// A metadata field for [AbstractVillager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractVillagerUnhappyCounter(pub i32);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractVillager` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [AbstractVillagerUnhappyCounter]
///
/// # Parents
///
/// Entities with `AbstractVillager` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Villager]
/// - [WanderingTrader]
#[derive(Component)]
pub struct AbstractVillager;
impl AbstractVillager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            17 => {
                entity.insert(AbstractVillagerUnhappyCounter(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractVillager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractVillagerMetadataBundle {
    _marker: AbstractVillager,
    parent: AbstractAgeableMetadataBundle,
    abstract_villager_unhappy_counter: AbstractVillagerUnhappyCounter,
}
impl Default for AbstractVillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractVillager,
            parent: Default::default(),
            abstract_villager_unhappy_counter: AbstractVillagerUnhappyCounter(0),
        }
    }
}

/// A metadata field for [Villager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct VillagerVillagerData(pub VillagerData);
/// The marker component for entities of type `minecraft:villager`.
///
/// # Metadata
///
/// These are the metadata components that all `Villager` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [VillagerVillagerData]
///
/// # Parents
///
/// Entities with `Villager` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractVillager]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Villager;
impl Villager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractVillager::apply_metadata(entity, d)?,
            18 => {
                entity.insert(VillagerVillagerData(d.value.into_villager_data()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Villager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct VillagerMetadataBundle {
    _marker: Villager,
    parent: AbstractVillagerMetadataBundle,
    villager_villager_data: VillagerVillagerData,
}
impl Default for VillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Villager,
            parent: Default::default(),
            villager_villager_data: VillagerVillagerData(VillagerData {
                kind: azalea_registry::builtin::VillagerKind::Plains,
                profession: azalea_registry::builtin::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

/// The marker component for entities of type `minecraft:wandering_trader`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `WanderingTrader` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractVillager]
/// - [AbstractAgeable]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct WanderingTrader;
impl WanderingTrader {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractVillager::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [WanderingTrader].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WanderingTraderMetadataBundle {
    _marker: WanderingTrader,
    parent: AbstractVillagerMetadataBundle,
}
impl Default for WanderingTraderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WanderingTrader,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractFish].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractFishFromBucket(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractFish` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [AbstractFishFromBucket]
///
/// # Parents
///
/// Entities with `AbstractFish` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Cod]
/// - [Salmon]
/// - [TropicalFish]
#[derive(Component)]
pub struct AbstractFish;
impl AbstractFish {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AbstractFishFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractFish].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractFishMetadataBundle {
    _marker: AbstractFish,
    parent: AbstractCreatureMetadataBundle,
    abstract_fish_from_bucket: AbstractFishFromBucket,
}
impl Default for AbstractFishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractFish,
            parent: Default::default(),
            abstract_fish_from_bucket: AbstractFishFromBucket(false),
        }
    }
}

/// The marker component for entities of type `minecraft:cod`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Cod` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractFish]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Cod;
impl Cod {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractFish::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Cod].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CodMetadataBundle {
    _marker: Cod,
    parent: AbstractFishMetadataBundle,
}
impl Default for CodMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cod,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Salmon].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SalmonKind(pub i32);
/// The marker component for entities of type `minecraft:salmon`.
///
/// # Metadata
///
/// These are the metadata components that all `Salmon` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [SalmonKind]
///
/// # Parents
///
/// Entities with `Salmon` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractFish]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Salmon;
impl Salmon {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractFish::apply_metadata(entity, d)?,
            17 => {
                entity.insert(SalmonKind(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Salmon].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SalmonMetadataBundle {
    _marker: Salmon,
    parent: AbstractFishMetadataBundle,
    salmon_kind: SalmonKind,
}
impl Default for SalmonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Salmon,
            parent: Default::default(),
            salmon_kind: SalmonKind(Default::default()),
        }
    }
}

/// A metadata field for [TropicalFish].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TropicalFishTypeVariant(pub i32);
/// The marker component for entities of type `minecraft:tropical_fish`.
///
/// # Metadata
///
/// These are the metadata components that all `TropicalFish` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [TropicalFishTypeVariant]
///
/// # Parents
///
/// Entities with `TropicalFish` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractFish]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct TropicalFish;
impl TropicalFish {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractFish::apply_metadata(entity, d)?,
            17 => {
                entity.insert(TropicalFishTypeVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [TropicalFish].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TropicalFishMetadataBundle {
    _marker: TropicalFish,
    parent: AbstractFishMetadataBundle,
    tropical_fish_type_variant: TropicalFishTypeVariant,
}
impl Default for TropicalFishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TropicalFish,
            parent: Default::default(),
            tropical_fish_type_variant: TropicalFishTypeVariant(Default::default()),
        }
    }
}

/// An abstract entity marker component.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `AbstractMonster` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Blaze]
/// - [Bogged]
/// - [Breeze]
/// - [Creaking]
/// - [Creeper]
/// - [Enderman]
/// - [Endermite]
/// - [Giant]
/// - [Guardian]
///   - [ElderGuardian]
/// - [Parched]
/// - [Silverfish]
/// - [Skeleton]
/// - [Spider]
///   - [CaveSpider]
/// - [Stray]
/// - [Vex]
/// - [Warden]
/// - [Wither]
/// - [WitherSkeleton]
/// - [Zoglin]
/// - [Zombie]
///   - [Drowned]
///   - [Husk]
///   - [ZombieVillager]
///   - [ZombifiedPiglin]
/// - [AbstractPiglin]
///   - [Piglin]
///   - [PiglinBrute]
/// - [AbstractRaider]
///   - [Pillager]
///   - [Ravager]
///   - [Vindicator]
///   - [Witch]
///   - [AbstractSpellcasterIllager]
///     - [Evoker]
///     - [Illusioner]
#[derive(Component)]
pub struct AbstractMonster;
impl AbstractMonster {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractMonster].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractMonsterMetadataBundle {
    _marker: AbstractMonster,
    parent: AbstractCreatureMetadataBundle,
}
impl Default for AbstractMonsterMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractMonster,
            parent: Default::default(),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Blaze].
pub struct Charged(pub bool);
/// The marker component for entities of type `minecraft:blaze`.
///
/// # Metadata
///
/// These are the metadata components that all `Blaze` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Charged]
///
/// # Parents
///
/// Entities with `Blaze` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Blaze;
impl Blaze {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Charged(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Blaze].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BlazeMetadataBundle {
    _marker: Blaze,
    parent: AbstractMonsterMetadataBundle,
    charged: Charged,
}
impl Default for BlazeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Blaze,
            parent: Default::default(),
            charged: Charged(false),
        }
    }
}

/// A metadata field for [Bogged].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BoggedSheared(pub bool);
/// The marker component for entities of type `minecraft:bogged`.
///
/// # Metadata
///
/// These are the metadata components that all `Bogged` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [BoggedSheared]
///
/// # Parents
///
/// Entities with `Bogged` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Bogged;
impl Bogged {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(BoggedSheared(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Bogged].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BoggedMetadataBundle {
    _marker: Bogged,
    parent: AbstractMonsterMetadataBundle,
    bogged_sheared: BoggedSheared,
}
impl Default for BoggedMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Bogged,
            parent: Default::default(),
            bogged_sheared: BoggedSheared(false),
        }
    }
}

/// The marker component for entities of type `minecraft:breeze`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Breeze` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Breeze;
impl Breeze {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Breeze].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BreezeMetadataBundle {
    _marker: Breeze,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for BreezeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Breeze,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Creaking].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CanMove(pub bool);
/// A metadata field for [Creaking].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsActive(pub bool);
/// A metadata field for [Creaking].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsTearingDown(pub bool);
/// A metadata field for [Creaking].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct HomePos(pub Option<BlockPos>);
/// The marker component for entities of type `minecraft:creaking`.
///
/// # Metadata
///
/// These are the metadata components that all `Creaking` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [CanMove]
/// - [IsActive]
/// - [IsTearingDown]
/// - [HomePos]
///
/// # Parents
///
/// Entities with `Creaking` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Creaking;
impl Creaking {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(CanMove(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(IsActive(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(IsTearingDown(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(HomePos(d.value.into_optional_block_pos()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Creaking].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CreakingMetadataBundle {
    _marker: Creaking,
    parent: AbstractMonsterMetadataBundle,
    can_move: CanMove,
    is_active: IsActive,
    is_tearing_down: IsTearingDown,
    home_pos: HomePos,
}
impl Default for CreakingMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Creaking,
            parent: Default::default(),
            can_move: CanMove(true),
            is_active: IsActive(false),
            is_tearing_down: IsTearingDown(false),
            home_pos: HomePos(None),
        }
    }
}

/// A metadata field for [Creeper].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SwellDir(pub i32);
/// A metadata field for [Creeper].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsPowered(pub bool);
/// A metadata field for [Creeper].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsIgnited(pub bool);
/// The marker component for entities of type `minecraft:creeper`.
///
/// # Metadata
///
/// These are the metadata components that all `Creeper` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [SwellDir]
/// - [IsPowered]
/// - [IsIgnited]
///
/// # Parents
///
/// Entities with `Creeper` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Creeper;
impl Creeper {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(SwellDir(d.value.into_int()?));
            }
            17 => {
                entity.insert(IsPowered(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(IsIgnited(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Creeper].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CreeperMetadataBundle {
    _marker: Creeper,
    parent: AbstractMonsterMetadataBundle,
    swell_dir: SwellDir,
    is_powered: IsPowered,
    is_ignited: IsIgnited,
}
impl Default for CreeperMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Creeper,
            parent: Default::default(),
            swell_dir: SwellDir(-1),
            is_powered: IsPowered(false),
            is_ignited: IsIgnited(false),
        }
    }
}

/// A metadata field for [Enderman].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CarryState(pub azalea_block::BlockState);
/// A metadata field for [Enderman].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Creepy(pub bool);
/// A metadata field for [Enderman].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StaredAt(pub bool);
/// The marker component for entities of type `minecraft:enderman`.
///
/// # Metadata
///
/// These are the metadata components that all `Enderman` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [CarryState]
/// - [Creepy]
/// - [StaredAt]
///
/// # Parents
///
/// Entities with `Enderman` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Enderman;
impl Enderman {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(CarryState(d.value.into_optional_block_state()?));
            }
            17 => {
                entity.insert(Creepy(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(StaredAt(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Enderman].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EndermanMetadataBundle {
    _marker: Enderman,
    parent: AbstractMonsterMetadataBundle,
    carry_state: CarryState,
    creepy: Creepy,
    stared_at: StaredAt,
}
impl Default for EndermanMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Enderman,
            parent: Default::default(),
            carry_state: CarryState(azalea_block::BlockState::AIR),
            creepy: Creepy(false),
            stared_at: StaredAt(false),
        }
    }
}

/// The marker component for entities of type `minecraft:endermite`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Endermite` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Endermite;
impl Endermite {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Endermite].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EndermiteMetadataBundle {
    _marker: Endermite,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for EndermiteMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Endermite,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:giant`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Giant` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Giant;
impl Giant {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Giant].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GiantMetadataBundle {
    _marker: Giant,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for GiantMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Giant,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Guardian].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Moving(pub bool);
/// A metadata field for [Guardian].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AttackTarget(pub i32);
/// The marker component for entities of type `minecraft:guardian`.
///
/// # Metadata
///
/// These are the metadata components that all `Guardian` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Moving]
/// - [AttackTarget]
///
/// # Parents
///
/// Entities with `Guardian` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [ElderGuardian]
#[derive(Component)]
pub struct Guardian;
impl Guardian {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Moving(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(AttackTarget(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Guardian].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct GuardianMetadataBundle {
    _marker: Guardian,
    parent: AbstractMonsterMetadataBundle,
    moving: Moving,
    attack_target: AttackTarget,
}
impl Default for GuardianMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Guardian,
            parent: Default::default(),
            moving: Moving(false),
            attack_target: AttackTarget(0),
        }
    }
}

/// The marker component for entities of type `minecraft:elder_guardian`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ElderGuardian` will also have the following marker components
/// and their metadata fields:
///
/// - [Guardian]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ElderGuardian;
impl ElderGuardian {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => Guardian::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ElderGuardian].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ElderGuardianMetadataBundle {
    _marker: ElderGuardian,
    parent: GuardianMetadataBundle,
}
impl Default for ElderGuardianMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ElderGuardian,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:parched`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Parched` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Parched;
impl Parched {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Parched].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ParchedMetadataBundle {
    _marker: Parched,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for ParchedMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Parched,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:silverfish`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Silverfish` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Silverfish;
impl Silverfish {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Silverfish].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SilverfishMetadataBundle {
    _marker: Silverfish,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for SilverfishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Silverfish,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Skeleton].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct StrayConversion(pub bool);
/// The marker component for entities of type `minecraft:skeleton`.
///
/// # Metadata
///
/// These are the metadata components that all `Skeleton` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [StrayConversion]
///
/// # Parents
///
/// Entities with `Skeleton` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Skeleton;
impl Skeleton {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(StrayConversion(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Skeleton].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SkeletonMetadataBundle {
    _marker: Skeleton,
    parent: AbstractMonsterMetadataBundle,
    stray_conversion: StrayConversion,
}
impl Default for SkeletonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Skeleton,
            parent: Default::default(),
            stray_conversion: StrayConversion(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
/// A metadata field for [Spider].
pub struct Climbing(pub bool);
/// The marker component for entities of type `minecraft:spider`.
///
/// # Metadata
///
/// These are the metadata components that all `Spider` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [Climbing]
///
/// # Parents
///
/// Entities with `Spider` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [CaveSpider]
#[derive(Component)]
pub struct Spider;
impl Spider {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Climbing(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Spider].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SpiderMetadataBundle {
    _marker: Spider,
    parent: AbstractMonsterMetadataBundle,
    climbing: Climbing,
}
impl Default for SpiderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Spider,
            parent: Default::default(),
            climbing: Climbing(false),
        }
    }
}

/// The marker component for entities of type `minecraft:cave_spider`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `CaveSpider` will also have the following marker components
/// and their metadata fields:
///
/// - [Spider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CaveSpider;
impl CaveSpider {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Spider::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CaveSpider].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CaveSpiderMetadataBundle {
    _marker: CaveSpider,
    parent: SpiderMetadataBundle,
}
impl Default for CaveSpiderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CaveSpider,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:stray`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Stray` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Stray;
impl Stray {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Stray].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct StrayMetadataBundle {
    _marker: Stray,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for StrayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Stray,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Vex].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct VexFlags(pub u8);
/// The marker component for entities of type `minecraft:vex`.
///
/// # Metadata
///
/// These are the metadata components that all `Vex` entities are guaranteed to
/// have, in addition to the metadata components from parent types:
///
/// - [VexFlags]
///
/// # Parents
///
/// Entities with `Vex` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Vex;
impl Vex {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(VexFlags(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Vex].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct VexMetadataBundle {
    _marker: Vex,
    parent: AbstractMonsterMetadataBundle,
    vex_flags: VexFlags,
}
impl Default for VexMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Vex,
            parent: Default::default(),
            vex_flags: VexFlags(0),
        }
    }
}

/// A metadata field for [Warden].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ClientAngerLevel(pub i32);
/// The marker component for entities of type `minecraft:warden`.
///
/// # Metadata
///
/// These are the metadata components that all `Warden` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [ClientAngerLevel]
///
/// # Parents
///
/// Entities with `Warden` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Warden;
impl Warden {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ClientAngerLevel(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Warden].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WardenMetadataBundle {
    _marker: Warden,
    parent: AbstractMonsterMetadataBundle,
    client_anger_level: ClientAngerLevel,
}
impl Default for WardenMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Warden,
            parent: Default::default(),
            client_anger_level: ClientAngerLevel(0),
        }
    }
}

/// A metadata field for [Wither].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TargetA(pub i32);
/// A metadata field for [Wither].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TargetB(pub i32);
/// A metadata field for [Wither].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct TargetC(pub i32);
/// A metadata field for [Wither].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Inv(pub i32);
/// The marker component for entities of type `minecraft:wither`.
///
/// # Metadata
///
/// These are the metadata components that all `Wither` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [TargetA]
/// - [TargetB]
/// - [TargetC]
/// - [Inv]
///
/// # Parents
///
/// Entities with `Wither` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Wither;
impl Wither {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
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
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Wither].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WitherMetadataBundle {
    _marker: Wither,
    parent: AbstractMonsterMetadataBundle,
    target_a: TargetA,
    target_b: TargetB,
    target_c: TargetC,
    inv: Inv,
}
impl Default for WitherMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Wither,
            parent: Default::default(),
            target_a: TargetA(0),
            target_b: TargetB(0),
            target_c: TargetC(0),
            inv: Inv(0),
        }
    }
}

/// The marker component for entities of type `minecraft:wither_skeleton`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `WitherSkeleton` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct WitherSkeleton;
impl WitherSkeleton {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [WitherSkeleton].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WitherSkeletonMetadataBundle {
    _marker: WitherSkeleton,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for WitherSkeletonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WitherSkeleton,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Zoglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ZoglinBaby(pub bool);
/// The marker component for entities of type `minecraft:zoglin`.
///
/// # Metadata
///
/// These are the metadata components that all `Zoglin` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [ZoglinBaby]
///
/// # Parents
///
/// Entities with `Zoglin` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Zoglin;
impl Zoglin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ZoglinBaby(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Zoglin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZoglinMetadataBundle {
    _marker: Zoglin,
    parent: AbstractMonsterMetadataBundle,
    zoglin_baby: ZoglinBaby,
}
impl Default for ZoglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Zoglin,
            parent: Default::default(),
            zoglin_baby: ZoglinBaby(false),
        }
    }
}

/// A metadata field for [Zombie].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ZombieBaby(pub bool);
/// A metadata field for [Zombie].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SpecialType(pub i32);
/// A metadata field for [Zombie].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct DrownedConversion(pub bool);
/// The marker component for entities of type `minecraft:zombie`.
///
/// # Metadata
///
/// These are the metadata components that all `Zombie` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [ZombieBaby]
/// - [SpecialType]
/// - [DrownedConversion]
///
/// # Parents
///
/// Entities with `Zombie` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Drowned]
/// - [Husk]
/// - [ZombieVillager]
/// - [ZombifiedPiglin]
#[derive(Component)]
pub struct Zombie;
impl Zombie {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ZombieBaby(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(SpecialType(d.value.into_int()?));
            }
            18 => {
                entity.insert(DrownedConversion(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Zombie].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZombieMetadataBundle {
    _marker: Zombie,
    parent: AbstractMonsterMetadataBundle,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl Default for ZombieMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Zombie,
            parent: Default::default(),
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

/// The marker component for entities of type `minecraft:drowned`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Drowned` will also have the following marker components and
/// their metadata fields:
///
/// - [Zombie]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Drowned;
impl Drowned {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Drowned].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DrownedMetadataBundle {
    _marker: Drowned,
    parent: ZombieMetadataBundle,
}
impl Default for DrownedMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Drowned,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:husk`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Husk` will also have the following marker components and
/// their metadata fields:
///
/// - [Zombie]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Husk;
impl Husk {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Husk].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct HuskMetadataBundle {
    _marker: Husk,
    parent: ZombieMetadataBundle,
}
impl Default for HuskMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Husk,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [ZombieVillager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Converting(pub bool);
/// A metadata field for [ZombieVillager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct ZombieVillagerVillagerData(pub VillagerData);
/// The marker component for entities of type `minecraft:zombie_villager`.
///
/// # Metadata
///
/// These are the metadata components that all `ZombieVillager` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Converting]
/// - [ZombieVillagerVillagerData]
///
/// # Parents
///
/// Entities with `ZombieVillager` will also have the following marker
/// components and their metadata fields:
///
/// - [Zombie]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ZombieVillager;
impl ZombieVillager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            19 => {
                entity.insert(Converting(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(ZombieVillagerVillagerData(d.value.into_villager_data()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ZombieVillager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZombieVillagerMetadataBundle {
    _marker: ZombieVillager,
    parent: ZombieMetadataBundle,
    converting: Converting,
    zombie_villager_villager_data: ZombieVillagerVillagerData,
}
impl Default for ZombieVillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombieVillager,
            parent: Default::default(),
            converting: Converting(false),
            zombie_villager_villager_data: ZombieVillagerVillagerData(VillagerData {
                kind: azalea_registry::builtin::VillagerKind::Plains,
                profession: azalea_registry::builtin::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

/// The marker component for entities of type `minecraft:zombified_piglin`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ZombifiedPiglin` will also have the following marker
/// components and their metadata fields:
///
/// - [Zombie]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ZombifiedPiglin;
impl ZombifiedPiglin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ZombifiedPiglin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ZombifiedPiglinMetadataBundle {
    _marker: ZombifiedPiglin,
    parent: ZombieMetadataBundle,
}
impl Default for ZombifiedPiglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombifiedPiglin,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractPiglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractPiglinImmuneToZombification(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractPiglin` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [AbstractPiglinImmuneToZombification]
///
/// # Parents
///
/// Entities with `AbstractPiglin` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Piglin]
/// - [PiglinBrute]
#[derive(Component)]
pub struct AbstractPiglin;
impl AbstractPiglin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AbstractPiglinImmuneToZombification(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractPiglin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractPiglinMetadataBundle {
    _marker: AbstractPiglin,
    parent: AbstractMonsterMetadataBundle,
    abstract_piglin_immune_to_zombification: AbstractPiglinImmuneToZombification,
}
impl Default for AbstractPiglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractPiglin,
            parent: Default::default(),
            abstract_piglin_immune_to_zombification: AbstractPiglinImmuneToZombification(false),
        }
    }
}

/// A metadata field for [Piglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PiglinBaby(pub bool);
/// A metadata field for [Piglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PiglinIsChargingCrossbow(pub bool);
/// A metadata field for [Piglin].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsDancing(pub bool);
/// The marker component for entities of type `minecraft:piglin`.
///
/// # Metadata
///
/// These are the metadata components that all `Piglin` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [PiglinBaby]
/// - [PiglinIsChargingCrossbow]
/// - [IsDancing]
///
/// # Parents
///
/// Entities with `Piglin` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractPiglin]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Piglin;
impl Piglin {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractPiglin::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PiglinBaby(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(PiglinIsChargingCrossbow(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(IsDancing(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Piglin].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PiglinMetadataBundle {
    _marker: Piglin,
    parent: AbstractPiglinMetadataBundle,
    piglin_baby: PiglinBaby,
    piglin_is_charging_crossbow: PiglinIsChargingCrossbow,
    is_dancing: IsDancing,
}
impl Default for PiglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Piglin,
            parent: Default::default(),
            piglin_baby: PiglinBaby(false),
            piglin_is_charging_crossbow: PiglinIsChargingCrossbow(false),
            is_dancing: IsDancing(false),
        }
    }
}

/// The marker component for entities of type `minecraft:piglin_brute`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `PiglinBrute` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractPiglin]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct PiglinBrute;
impl PiglinBrute {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractPiglin::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [PiglinBrute].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PiglinBruteMetadataBundle {
    _marker: PiglinBrute,
    parent: AbstractPiglinMetadataBundle,
}
impl Default for PiglinBruteMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PiglinBrute,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractRaider].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct IsCelebrating(pub bool);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractRaider` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [IsCelebrating]
///
/// # Parents
///
/// Entities with `AbstractRaider` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Pillager]
/// - [Ravager]
/// - [Vindicator]
/// - [Witch]
/// - [AbstractSpellcasterIllager]
///   - [Evoker]
///   - [Illusioner]
#[derive(Component)]
pub struct AbstractRaider;
impl AbstractRaider {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(IsCelebrating(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractRaider].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractRaiderMetadataBundle {
    _marker: AbstractRaider,
    parent: AbstractMonsterMetadataBundle,
    is_celebrating: IsCelebrating,
}
impl Default for AbstractRaiderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractRaider,
            parent: Default::default(),
            is_celebrating: IsCelebrating(false),
        }
    }
}

/// A metadata field for [Pillager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PillagerIsChargingCrossbow(pub bool);
/// The marker component for entities of type `minecraft:pillager`.
///
/// # Metadata
///
/// These are the metadata components that all `Pillager` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PillagerIsChargingCrossbow]
///
/// # Parents
///
/// Entities with `Pillager` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Pillager;
impl Pillager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractRaider::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PillagerIsChargingCrossbow(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Pillager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PillagerMetadataBundle {
    _marker: Pillager,
    parent: AbstractRaiderMetadataBundle,
    pillager_is_charging_crossbow: PillagerIsChargingCrossbow,
}
impl Default for PillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pillager,
            parent: Default::default(),
            pillager_is_charging_crossbow: PillagerIsChargingCrossbow(false),
        }
    }
}

/// The marker component for entities of type `minecraft:ravager`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Ravager` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Ravager;
impl Ravager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractRaider::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Ravager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct RavagerMetadataBundle {
    _marker: Ravager,
    parent: AbstractRaiderMetadataBundle,
}
impl Default for RavagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ravager,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:vindicator`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Vindicator` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Vindicator;
impl Vindicator {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractRaider::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Vindicator].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct VindicatorMetadataBundle {
    _marker: Vindicator,
    parent: AbstractRaiderMetadataBundle,
}
impl Default for VindicatorMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Vindicator,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [Witch].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct WitchUsingItem(pub bool);
/// The marker component for entities of type `minecraft:witch`.
///
/// # Metadata
///
/// These are the metadata components that all `Witch` entities are guaranteed
/// to have, in addition to the metadata components from parent types:
///
/// - [WitchUsingItem]
///
/// # Parents
///
/// Entities with `Witch` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Witch;
impl Witch {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractRaider::apply_metadata(entity, d)?,
            17 => {
                entity.insert(WitchUsingItem(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Witch].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct WitchMetadataBundle {
    _marker: Witch,
    parent: AbstractRaiderMetadataBundle,
    witch_using_item: WitchUsingItem,
}
impl Default for WitchMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Witch,
            parent: Default::default(),
            witch_using_item: WitchUsingItem(false),
        }
    }
}

/// A metadata field for [AbstractSpellcasterIllager].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct SpellCasting(pub u8);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractSpellcasterIllager`
/// entities are guaranteed to have, in addition to the metadata components from
/// parent types:
///
/// - [SpellCasting]
///
/// # Parents
///
/// Entities with `AbstractSpellcasterIllager` will also have the following
/// marker components and their metadata fields:
///
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// - [Evoker]
/// - [Illusioner]
#[derive(Component)]
pub struct AbstractSpellcasterIllager;
impl AbstractSpellcasterIllager {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractRaider::apply_metadata(entity, d)?,
            17 => {
                entity.insert(SpellCasting(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractSpellcasterIllager].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractSpellcasterIllagerMetadataBundle {
    _marker: AbstractSpellcasterIllager,
    parent: AbstractRaiderMetadataBundle,
    spell_casting: SpellCasting,
}
impl Default for AbstractSpellcasterIllagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractSpellcasterIllager,
            parent: Default::default(),
            spell_casting: SpellCasting(0),
        }
    }
}

/// The marker component for entities of type `minecraft:evoker`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Evoker` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractSpellcasterIllager]
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Evoker;
impl Evoker {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractSpellcasterIllager::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Evoker].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EvokerMetadataBundle {
    _marker: Evoker,
    parent: AbstractSpellcasterIllagerMetadataBundle,
}
impl Default for EvokerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Evoker,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:illusioner`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Illusioner` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractSpellcasterIllager]
/// - [AbstractRaider]
/// - [AbstractMonster]
/// - [AbstractCreature]
/// - [AbstractInsentient]
/// - [AbstractLiving]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Illusioner;
impl Illusioner {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => AbstractSpellcasterIllager::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Illusioner].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct IllusionerMetadataBundle {
    _marker: Illusioner,
    parent: AbstractSpellcasterIllagerMetadataBundle,
}
impl Default for IllusionerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Illusioner,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractThrownItemProjectile].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct AbstractThrownItemProjectileItemStack(pub ItemStack);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractThrownItemProjectile`
/// entities are guaranteed to have, in addition to the metadata components from
/// parent types:
///
/// - [AbstractThrownItemProjectileItemStack]
///
/// # Parents
///
/// Entities with `AbstractThrownItemProjectile` will also have the following
/// marker components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [Egg]
/// - [EnderPearl]
/// - [ExperienceBottle]
/// - [LingeringPotion]
/// - [Snowball]
/// - [SplashPotion]
#[derive(Component)]
pub struct AbstractThrownItemProjectile;
impl AbstractThrownItemProjectile {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(AbstractThrownItemProjectileItemStack(
                    d.value.into_item_stack()?,
                ));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractThrownItemProjectile].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractThrownItemProjectileMetadataBundle {
    _marker: AbstractThrownItemProjectile,
    parent: AbstractEntityMetadataBundle,
    abstract_thrown_item_projectile_item_stack: AbstractThrownItemProjectileItemStack,
}
impl Default for AbstractThrownItemProjectileMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractThrownItemProjectile,
            parent: Default::default(),
            abstract_thrown_item_projectile_item_stack: AbstractThrownItemProjectileItemStack(
                Default::default(),
            ),
        }
    }
}

/// The marker component for entities of type `minecraft:egg`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Egg` will also have the following marker components and their
/// metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Egg;
impl Egg {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Egg].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EggMetadataBundle {
    _marker: Egg,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for EggMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Egg,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:ender_pearl`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `EnderPearl` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct EnderPearl;
impl EnderPearl {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [EnderPearl].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct EnderPearlMetadataBundle {
    _marker: EnderPearl,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for EnderPearlMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EnderPearl,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:experience_bottle`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ExperienceBottle` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ExperienceBottle;
impl ExperienceBottle {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ExperienceBottle].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ExperienceBottleMetadataBundle {
    _marker: ExperienceBottle,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for ExperienceBottleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ExperienceBottle,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:lingering_potion`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `LingeringPotion` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct LingeringPotion;
impl LingeringPotion {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [LingeringPotion].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct LingeringPotionMetadataBundle {
    _marker: LingeringPotion,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for LingeringPotionMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LingeringPotion,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:snowball`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Snowball` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Snowball;
impl Snowball {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Snowball].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SnowballMetadataBundle {
    _marker: Snowball,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for SnowballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Snowball,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:splash_potion`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SplashPotion` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractThrownItemProjectile]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SplashPotion;
impl SplashPotion {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=8 => AbstractThrownItemProjectile::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SplashPotion].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SplashPotionMetadataBundle {
    _marker: SplashPotion,
    parent: AbstractThrownItemProjectileMetadataBundle,
}
impl Default for SplashPotionMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SplashPotion,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractVehicle].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Hurt(pub i32);
/// A metadata field for [AbstractVehicle].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Hurtdir(pub i32);
/// A metadata field for [AbstractVehicle].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Damage(pub f32);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractVehicle` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Hurt]
/// - [Hurtdir]
/// - [Damage]
///
/// # Parents
///
/// Entities with `AbstractVehicle` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractEntity]
///
/// # Children
///
/// - [AbstractBoat]
///   - [AcaciaBoat]
///   - [AcaciaChestBoat]
///   - [BambooChestRaft]
///   - [BambooRaft]
///   - [BirchBoat]
///   - [BirchChestBoat]
///   - [CherryBoat]
///   - [CherryChestBoat]
///   - [DarkOakBoat]
///   - [DarkOakChestBoat]
///   - [JungleBoat]
///   - [JungleChestBoat]
///   - [MangroveBoat]
///   - [MangroveChestBoat]
///   - [OakBoat]
///   - [OakChestBoat]
///   - [PaleOakBoat]
///   - [PaleOakChestBoat]
///   - [SpruceBoat]
///   - [SpruceChestBoat]
/// - [AbstractMinecart]
///   - [ChestMinecart]
///   - [CommandBlockMinecart]
///   - [FurnaceMinecart]
///   - [HopperMinecart]
///   - [Minecart]
///   - [SpawnerMinecart]
///   - [TntMinecart]
#[derive(Component)]
pub struct AbstractVehicle;
impl AbstractVehicle {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Hurt(d.value.into_int()?));
            }
            9 => {
                entity.insert(Hurtdir(d.value.into_int()?));
            }
            10 => {
                entity.insert(Damage(d.value.into_float()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractVehicle].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractVehicleMetadataBundle {
    _marker: AbstractVehicle,
    parent: AbstractEntityMetadataBundle,
    hurt: Hurt,
    hurtdir: Hurtdir,
    damage: Damage,
}
impl Default for AbstractVehicleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractVehicle,
            parent: Default::default(),
            hurt: Hurt(0),
            hurtdir: Hurtdir(1),
            damage: Damage(0.0),
        }
    }
}

/// A metadata field for [AbstractBoat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PaddleLeft(pub bool);
/// A metadata field for [AbstractBoat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct PaddleRight(pub bool);
/// A metadata field for [AbstractBoat].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct BubbleTime(pub i32);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractBoat` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [PaddleLeft]
/// - [PaddleRight]
/// - [BubbleTime]
///
/// # Parents
///
/// Entities with `AbstractBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// - [AcaciaBoat]
/// - [AcaciaChestBoat]
/// - [BambooChestRaft]
/// - [BambooRaft]
/// - [BirchBoat]
/// - [BirchChestBoat]
/// - [CherryBoat]
/// - [CherryChestBoat]
/// - [DarkOakBoat]
/// - [DarkOakChestBoat]
/// - [JungleBoat]
/// - [JungleChestBoat]
/// - [MangroveBoat]
/// - [MangroveChestBoat]
/// - [OakBoat]
/// - [OakChestBoat]
/// - [PaleOakBoat]
/// - [PaleOakChestBoat]
/// - [SpruceBoat]
/// - [SpruceChestBoat]
#[derive(Component)]
pub struct AbstractBoat;
impl AbstractBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => AbstractVehicle::apply_metadata(entity, d)?,
            11 => {
                entity.insert(PaddleLeft(d.value.into_boolean()?));
            }
            12 => {
                entity.insert(PaddleRight(d.value.into_boolean()?));
            }
            13 => {
                entity.insert(BubbleTime(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractBoatMetadataBundle {
    _marker: AbstractBoat,
    parent: AbstractVehicleMetadataBundle,
    paddle_left: PaddleLeft,
    paddle_right: PaddleRight,
    bubble_time: BubbleTime,
}
impl Default for AbstractBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractBoat,
            parent: Default::default(),
            paddle_left: PaddleLeft(false),
            paddle_right: PaddleRight(false),
            bubble_time: BubbleTime(0),
        }
    }
}

/// The marker component for entities of type `minecraft:acacia_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `AcaciaBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct AcaciaBoat;
impl AcaciaBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AcaciaBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AcaciaBoatMetadataBundle {
    _marker: AcaciaBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for AcaciaBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AcaciaBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:acacia_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `AcaciaChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct AcaciaChestBoat;
impl AcaciaChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AcaciaChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AcaciaChestBoatMetadataBundle {
    _marker: AcaciaChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for AcaciaChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AcaciaChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:bamboo_chest_raft`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `BambooChestRaft` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BambooChestRaft;
impl BambooChestRaft {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BambooChestRaft].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BambooChestRaftMetadataBundle {
    _marker: BambooChestRaft,
    parent: AbstractBoatMetadataBundle,
}
impl Default for BambooChestRaftMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BambooChestRaft,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:bamboo_raft`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `BambooRaft` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BambooRaft;
impl BambooRaft {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BambooRaft].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BambooRaftMetadataBundle {
    _marker: BambooRaft,
    parent: AbstractBoatMetadataBundle,
}
impl Default for BambooRaftMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BambooRaft,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:birch_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `BirchBoat` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BirchBoat;
impl BirchBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BirchBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BirchBoatMetadataBundle {
    _marker: BirchBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for BirchBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BirchBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:birch_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `BirchChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct BirchChestBoat;
impl BirchChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [BirchChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct BirchChestBoatMetadataBundle {
    _marker: BirchChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for BirchChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BirchChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:cherry_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `CherryBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CherryBoat;
impl CherryBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CherryBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CherryBoatMetadataBundle {
    _marker: CherryBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for CherryBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CherryBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:cherry_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `CherryChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CherryChestBoat;
impl CherryChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CherryChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CherryChestBoatMetadataBundle {
    _marker: CherryChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for CherryChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CherryChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:dark_oak_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `DarkOakBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct DarkOakBoat;
impl DarkOakBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [DarkOakBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DarkOakBoatMetadataBundle {
    _marker: DarkOakBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for DarkOakBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: DarkOakBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:dark_oak_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `DarkOakChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct DarkOakChestBoat;
impl DarkOakChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [DarkOakChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct DarkOakChestBoatMetadataBundle {
    _marker: DarkOakChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for DarkOakChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: DarkOakChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:jungle_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `JungleBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct JungleBoat;
impl JungleBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [JungleBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct JungleBoatMetadataBundle {
    _marker: JungleBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for JungleBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: JungleBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:jungle_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `JungleChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct JungleChestBoat;
impl JungleChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [JungleChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct JungleChestBoatMetadataBundle {
    _marker: JungleChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for JungleChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: JungleChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:mangrove_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `MangroveBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct MangroveBoat;
impl MangroveBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [MangroveBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MangroveBoatMetadataBundle {
    _marker: MangroveBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for MangroveBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: MangroveBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:mangrove_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `MangroveChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct MangroveChestBoat;
impl MangroveChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [MangroveChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MangroveChestBoatMetadataBundle {
    _marker: MangroveChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for MangroveChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: MangroveChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:oak_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `OakBoat` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct OakBoat;
impl OakBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [OakBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct OakBoatMetadataBundle {
    _marker: OakBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for OakBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: OakBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:oak_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `OakChestBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct OakChestBoat;
impl OakChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [OakChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct OakChestBoatMetadataBundle {
    _marker: OakChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for OakChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: OakChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:pale_oak_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `PaleOakBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct PaleOakBoat;
impl PaleOakBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [PaleOakBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PaleOakBoatMetadataBundle {
    _marker: PaleOakBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for PaleOakBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PaleOakBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:pale_oak_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `PaleOakChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct PaleOakChestBoat;
impl PaleOakChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [PaleOakChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct PaleOakChestBoatMetadataBundle {
    _marker: PaleOakChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for PaleOakChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PaleOakChestBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:spruce_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SpruceBoat` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SpruceBoat;
impl SpruceBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SpruceBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SpruceBoatMetadataBundle {
    _marker: SpruceBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for SpruceBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpruceBoat,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:spruce_chest_boat`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SpruceChestBoat` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractBoat]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SpruceChestBoat;
impl SpruceChestBoat {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractBoat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SpruceChestBoat].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SpruceChestBoatMetadataBundle {
    _marker: SpruceChestBoat,
    parent: AbstractBoatMetadataBundle,
}
impl Default for SpruceChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpruceChestBoat,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [AbstractMinecart].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CustomDisplayBlock(pub azalea_block::BlockState);
/// A metadata field for [AbstractMinecart].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct DisplayOffset(pub i32);
/// An abstract entity marker component.
///
/// # Metadata
///
/// These are the metadata components that all `AbstractMinecart` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [CustomDisplayBlock]
/// - [DisplayOffset]
///
/// # Parents
///
/// Entities with `AbstractMinecart` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// - [ChestMinecart]
/// - [CommandBlockMinecart]
/// - [FurnaceMinecart]
/// - [HopperMinecart]
/// - [Minecart]
/// - [SpawnerMinecart]
/// - [TntMinecart]
#[derive(Component)]
pub struct AbstractMinecart;
impl AbstractMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=10 => AbstractVehicle::apply_metadata(entity, d)?,
            11 => {
                entity.insert(CustomDisplayBlock(d.value.into_optional_block_state()?));
            }
            12 => {
                entity.insert(DisplayOffset(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [AbstractMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct AbstractMinecartMetadataBundle {
    _marker: AbstractMinecart,
    parent: AbstractVehicleMetadataBundle,
    custom_display_block: CustomDisplayBlock,
    display_offset: DisplayOffset,
}
impl Default for AbstractMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractMinecart,
            parent: Default::default(),
            custom_display_block: CustomDisplayBlock(azalea_block::BlockState::AIR),
            display_offset: DisplayOffset(Default::default()),
        }
    }
}

/// The marker component for entities of type `minecraft:chest_minecart`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `ChestMinecart` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct ChestMinecart;
impl ChestMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [ChestMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct ChestMinecartMetadataBundle {
    _marker: ChestMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for ChestMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ChestMinecart,
            parent: Default::default(),
        }
    }
}

/// A metadata field for [CommandBlockMinecart].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct CommandName(pub Box<str>);
/// A metadata field for [CommandBlockMinecart].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct LastOutput(pub Box<FormattedText>);
/// The marker component for entities of type
/// `minecraft:command_block_minecart`.
///
/// # Metadata
///
/// These are the metadata components that all `CommandBlockMinecart` entities
/// are guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [CommandName]
/// - [LastOutput]
///
/// # Parents
///
/// Entities with `CommandBlockMinecart` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct CommandBlockMinecart;
impl CommandBlockMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            13 => {
                entity.insert(CommandName(d.value.into_string()?));
            }
            14 => {
                entity.insert(LastOutput(d.value.into_formatted_text()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [CommandBlockMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct CommandBlockMinecartMetadataBundle {
    _marker: CommandBlockMinecart,
    parent: AbstractMinecartMetadataBundle,
    command_name: CommandName,
    last_output: LastOutput,
}
impl Default for CommandBlockMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CommandBlockMinecart,
            parent: Default::default(),
            command_name: CommandName("".into()),
            last_output: LastOutput(Default::default()),
        }
    }
}

/// A metadata field for [FurnaceMinecart].
#[derive(Component, Deref, DerefMut, Clone, PartialEq)]
pub struct Fuel(pub bool);
/// The marker component for entities of type `minecraft:furnace_minecart`.
///
/// # Metadata
///
/// These are the metadata components that all `FurnaceMinecart` entities are
/// guaranteed to have, in addition to the metadata components from parent
/// types:
///
/// - [Fuel]
///
/// # Parents
///
/// Entities with `FurnaceMinecart` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct FurnaceMinecart;
impl FurnaceMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            13 => {
                entity.insert(Fuel(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [FurnaceMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct FurnaceMinecartMetadataBundle {
    _marker: FurnaceMinecart,
    parent: AbstractMinecartMetadataBundle,
    fuel: Fuel,
}
impl Default for FurnaceMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FurnaceMinecart,
            parent: Default::default(),
            fuel: Fuel(false),
        }
    }
}

/// The marker component for entities of type `minecraft:hopper_minecart`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `HopperMinecart` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct HopperMinecart;
impl HopperMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [HopperMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct HopperMinecartMetadataBundle {
    _marker: HopperMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for HopperMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: HopperMinecart,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:minecart`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `Minecart` will also have the following marker components and
/// their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct Minecart;
impl Minecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [Minecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct MinecartMetadataBundle {
    _marker: Minecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for MinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Minecart,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:spawner_minecart`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `SpawnerMinecart` will also have the following marker
/// components and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct SpawnerMinecart;
impl SpawnerMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [SpawnerMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct SpawnerMinecartMetadataBundle {
    _marker: SpawnerMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for SpawnerMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpawnerMinecart,
            parent: Default::default(),
        }
    }
}

/// The marker component for entities of type `minecraft:tnt_minecart`.
///
/// # Metadata
///
/// This entity type does not add any additional metadata. It will still have
/// metadata from parent types.
///
/// # Parents
///
/// Entities with `TntMinecart` will also have the following marker components
/// and their metadata fields:
///
/// - [AbstractMinecart]
/// - [AbstractVehicle]
/// - [AbstractEntity]
///
/// # Children
///
/// This entity type has no children types.
#[derive(Component)]
pub struct TntMinecart;
impl TntMinecart {
    fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=12 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

/// The metadata bundle for [TntMinecart].
///
/// This type should generally not be used directly.
#[derive(Bundle)]
pub struct TntMinecartMetadataBundle {
    _marker: TntMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for TntMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TntMinecart,
            parent: Default::default(),
        }
    }
}

pub fn apply_metadata(
    entity: &mut bevy_ecs::system::EntityCommands,
    entity_kind: EntityKind,
    items: Vec<EntityDataItem>,
) -> Result<(), UpdateMetadataError> {
    match entity_kind {
        EntityKind::AcaciaBoat => {
            for d in items {
                AcaciaBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::AcaciaChestBoat => {
            for d in items {
                AcaciaChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Allay => {
            for d in items {
                Allay::apply_metadata(entity, d)?;
            }
        }
        EntityKind::AreaEffectCloud => {
            for d in items {
                AreaEffectCloud::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Armadillo => {
            for d in items {
                Armadillo::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ArmorStand => {
            for d in items {
                ArmorStand::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Arrow => {
            for d in items {
                Arrow::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Axolotl => {
            for d in items {
                Axolotl::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BambooChestRaft => {
            for d in items {
                BambooChestRaft::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BambooRaft => {
            for d in items {
                BambooRaft::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Bat => {
            for d in items {
                Bat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Bee => {
            for d in items {
                Bee::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BirchBoat => {
            for d in items {
                BirchBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BirchChestBoat => {
            for d in items {
                BirchChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Blaze => {
            for d in items {
                Blaze::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BlockDisplay => {
            for d in items {
                BlockDisplay::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Bogged => {
            for d in items {
                Bogged::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Breeze => {
            for d in items {
                Breeze::apply_metadata(entity, d)?;
            }
        }
        EntityKind::BreezeWindCharge => {
            for d in items {
                BreezeWindCharge::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Camel => {
            for d in items {
                Camel::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CamelHusk => {
            for d in items {
                CamelHusk::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Cat => {
            for d in items {
                Cat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CaveSpider => {
            for d in items {
                CaveSpider::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CherryBoat => {
            for d in items {
                CherryBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CherryChestBoat => {
            for d in items {
                CherryChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ChestMinecart => {
            for d in items {
                ChestMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Chicken => {
            for d in items {
                Chicken::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Cod => {
            for d in items {
                Cod::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CommandBlockMinecart => {
            for d in items {
                CommandBlockMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::CopperGolem => {
            for d in items {
                CopperGolem::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Cow => {
            for d in items {
                Cow::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Creaking => {
            for d in items {
                Creaking::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Creeper => {
            for d in items {
                Creeper::apply_metadata(entity, d)?;
            }
        }
        EntityKind::DarkOakBoat => {
            for d in items {
                DarkOakBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::DarkOakChestBoat => {
            for d in items {
                DarkOakChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Dolphin => {
            for d in items {
                Dolphin::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Donkey => {
            for d in items {
                Donkey::apply_metadata(entity, d)?;
            }
        }
        EntityKind::DragonFireball => {
            for d in items {
                DragonFireball::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Drowned => {
            for d in items {
                Drowned::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Egg => {
            for d in items {
                Egg::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ElderGuardian => {
            for d in items {
                ElderGuardian::apply_metadata(entity, d)?;
            }
        }
        EntityKind::EndCrystal => {
            for d in items {
                EndCrystal::apply_metadata(entity, d)?;
            }
        }
        EntityKind::EnderDragon => {
            for d in items {
                EnderDragon::apply_metadata(entity, d)?;
            }
        }
        EntityKind::EnderPearl => {
            for d in items {
                EnderPearl::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Enderman => {
            for d in items {
                Enderman::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Endermite => {
            for d in items {
                Endermite::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Evoker => {
            for d in items {
                Evoker::apply_metadata(entity, d)?;
            }
        }
        EntityKind::EvokerFangs => {
            for d in items {
                EvokerFangs::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ExperienceBottle => {
            for d in items {
                ExperienceBottle::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ExperienceOrb => {
            for d in items {
                ExperienceOrb::apply_metadata(entity, d)?;
            }
        }
        EntityKind::EyeOfEnder => {
            for d in items {
                EyeOfEnder::apply_metadata(entity, d)?;
            }
        }
        EntityKind::FallingBlock => {
            for d in items {
                FallingBlock::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Fireball => {
            for d in items {
                Fireball::apply_metadata(entity, d)?;
            }
        }
        EntityKind::FireworkRocket => {
            for d in items {
                FireworkRocket::apply_metadata(entity, d)?;
            }
        }
        EntityKind::FishingBobber => {
            for d in items {
                FishingBobber::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Fox => {
            for d in items {
                Fox::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Frog => {
            for d in items {
                Frog::apply_metadata(entity, d)?;
            }
        }
        EntityKind::FurnaceMinecart => {
            for d in items {
                FurnaceMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Ghast => {
            for d in items {
                Ghast::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Giant => {
            for d in items {
                Giant::apply_metadata(entity, d)?;
            }
        }
        EntityKind::GlowItemFrame => {
            for d in items {
                GlowItemFrame::apply_metadata(entity, d)?;
            }
        }
        EntityKind::GlowSquid => {
            for d in items {
                GlowSquid::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Goat => {
            for d in items {
                Goat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Guardian => {
            for d in items {
                Guardian::apply_metadata(entity, d)?;
            }
        }
        EntityKind::HappyGhast => {
            for d in items {
                HappyGhast::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Hoglin => {
            for d in items {
                Hoglin::apply_metadata(entity, d)?;
            }
        }
        EntityKind::HopperMinecart => {
            for d in items {
                HopperMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Horse => {
            for d in items {
                Horse::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Husk => {
            for d in items {
                Husk::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Illusioner => {
            for d in items {
                Illusioner::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Interaction => {
            for d in items {
                Interaction::apply_metadata(entity, d)?;
            }
        }
        EntityKind::IronGolem => {
            for d in items {
                IronGolem::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Item => {
            for d in items {
                Item::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ItemDisplay => {
            for d in items {
                ItemDisplay::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ItemFrame => {
            for d in items {
                ItemFrame::apply_metadata(entity, d)?;
            }
        }
        EntityKind::JungleBoat => {
            for d in items {
                JungleBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::JungleChestBoat => {
            for d in items {
                JungleChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::LeashKnot => {
            for d in items {
                LeashKnot::apply_metadata(entity, d)?;
            }
        }
        EntityKind::LightningBolt => {
            for d in items {
                LightningBolt::apply_metadata(entity, d)?;
            }
        }
        EntityKind::LingeringPotion => {
            for d in items {
                LingeringPotion::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Llama => {
            for d in items {
                Llama::apply_metadata(entity, d)?;
            }
        }
        EntityKind::LlamaSpit => {
            for d in items {
                LlamaSpit::apply_metadata(entity, d)?;
            }
        }
        EntityKind::MagmaCube => {
            for d in items {
                MagmaCube::apply_metadata(entity, d)?;
            }
        }
        EntityKind::MangroveBoat => {
            for d in items {
                MangroveBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::MangroveChestBoat => {
            for d in items {
                MangroveChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Mannequin => {
            for d in items {
                Mannequin::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Marker => {
            for d in items {
                Marker::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Minecart => {
            for d in items {
                Minecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Mooshroom => {
            for d in items {
                Mooshroom::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Mule => {
            for d in items {
                Mule::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Nautilus => {
            for d in items {
                Nautilus::apply_metadata(entity, d)?;
            }
        }
        EntityKind::OakBoat => {
            for d in items {
                OakBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::OakChestBoat => {
            for d in items {
                OakChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Ocelot => {
            for d in items {
                Ocelot::apply_metadata(entity, d)?;
            }
        }
        EntityKind::OminousItemSpawner => {
            for d in items {
                OminousItemSpawner::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Painting => {
            for d in items {
                Painting::apply_metadata(entity, d)?;
            }
        }
        EntityKind::PaleOakBoat => {
            for d in items {
                PaleOakBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::PaleOakChestBoat => {
            for d in items {
                PaleOakChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Panda => {
            for d in items {
                Panda::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Parched => {
            for d in items {
                Parched::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Parrot => {
            for d in items {
                Parrot::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Phantom => {
            for d in items {
                Phantom::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Pig => {
            for d in items {
                Pig::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Piglin => {
            for d in items {
                Piglin::apply_metadata(entity, d)?;
            }
        }
        EntityKind::PiglinBrute => {
            for d in items {
                PiglinBrute::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Pillager => {
            for d in items {
                Pillager::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Player => {
            for d in items {
                Player::apply_metadata(entity, d)?;
            }
        }
        EntityKind::PolarBear => {
            for d in items {
                PolarBear::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Pufferfish => {
            for d in items {
                Pufferfish::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Rabbit => {
            for d in items {
                Rabbit::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Ravager => {
            for d in items {
                Ravager::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Salmon => {
            for d in items {
                Salmon::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Sheep => {
            for d in items {
                Sheep::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Shulker => {
            for d in items {
                Shulker::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ShulkerBullet => {
            for d in items {
                ShulkerBullet::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Silverfish => {
            for d in items {
                Silverfish::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Skeleton => {
            for d in items {
                Skeleton::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SkeletonHorse => {
            for d in items {
                SkeletonHorse::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Slime => {
            for d in items {
                Slime::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SmallFireball => {
            for d in items {
                SmallFireball::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Sniffer => {
            for d in items {
                Sniffer::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SnowGolem => {
            for d in items {
                SnowGolem::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Snowball => {
            for d in items {
                Snowball::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SpawnerMinecart => {
            for d in items {
                SpawnerMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SpectralArrow => {
            for d in items {
                SpectralArrow::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Spider => {
            for d in items {
                Spider::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SplashPotion => {
            for d in items {
                SplashPotion::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SpruceBoat => {
            for d in items {
                SpruceBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::SpruceChestBoat => {
            for d in items {
                SpruceChestBoat::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Squid => {
            for d in items {
                Squid::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Stray => {
            for d in items {
                Stray::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Strider => {
            for d in items {
                Strider::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Tadpole => {
            for d in items {
                Tadpole::apply_metadata(entity, d)?;
            }
        }
        EntityKind::TextDisplay => {
            for d in items {
                TextDisplay::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Tnt => {
            for d in items {
                Tnt::apply_metadata(entity, d)?;
            }
        }
        EntityKind::TntMinecart => {
            for d in items {
                TntMinecart::apply_metadata(entity, d)?;
            }
        }
        EntityKind::TraderLlama => {
            for d in items {
                TraderLlama::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Trident => {
            for d in items {
                Trident::apply_metadata(entity, d)?;
            }
        }
        EntityKind::TropicalFish => {
            for d in items {
                TropicalFish::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Turtle => {
            for d in items {
                Turtle::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Vex => {
            for d in items {
                Vex::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Villager => {
            for d in items {
                Villager::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Vindicator => {
            for d in items {
                Vindicator::apply_metadata(entity, d)?;
            }
        }
        EntityKind::WanderingTrader => {
            for d in items {
                WanderingTrader::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Warden => {
            for d in items {
                Warden::apply_metadata(entity, d)?;
            }
        }
        EntityKind::WindCharge => {
            for d in items {
                WindCharge::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Witch => {
            for d in items {
                Witch::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Wither => {
            for d in items {
                Wither::apply_metadata(entity, d)?;
            }
        }
        EntityKind::WitherSkeleton => {
            for d in items {
                WitherSkeleton::apply_metadata(entity, d)?;
            }
        }
        EntityKind::WitherSkull => {
            for d in items {
                WitherSkull::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Wolf => {
            for d in items {
                Wolf::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Zoglin => {
            for d in items {
                Zoglin::apply_metadata(entity, d)?;
            }
        }
        EntityKind::Zombie => {
            for d in items {
                Zombie::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ZombieHorse => {
            for d in items {
                ZombieHorse::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ZombieNautilus => {
            for d in items {
                ZombieNautilus::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ZombieVillager => {
            for d in items {
                ZombieVillager::apply_metadata(entity, d)?;
            }
        }
        EntityKind::ZombifiedPiglin => {
            for d in items {
                ZombifiedPiglin::apply_metadata(entity, d)?;
            }
        }
    }
    Ok(())
}

pub fn apply_default_metadata(entity: &mut bevy_ecs::system::EntityCommands, kind: EntityKind) {
    match kind {
        EntityKind::AcaciaBoat => {
            entity.insert(AcaciaBoatMetadataBundle::default());
        }
        EntityKind::AcaciaChestBoat => {
            entity.insert(AcaciaChestBoatMetadataBundle::default());
        }
        EntityKind::Allay => {
            entity.insert(AllayMetadataBundle::default());
        }
        EntityKind::AreaEffectCloud => {
            entity.insert(AreaEffectCloudMetadataBundle::default());
        }
        EntityKind::Armadillo => {
            entity.insert(ArmadilloMetadataBundle::default());
        }
        EntityKind::ArmorStand => {
            entity.insert(ArmorStandMetadataBundle::default());
        }
        EntityKind::Arrow => {
            entity.insert(ArrowMetadataBundle::default());
        }
        EntityKind::Axolotl => {
            entity.insert(AxolotlMetadataBundle::default());
        }
        EntityKind::BambooChestRaft => {
            entity.insert(BambooChestRaftMetadataBundle::default());
        }
        EntityKind::BambooRaft => {
            entity.insert(BambooRaftMetadataBundle::default());
        }
        EntityKind::Bat => {
            entity.insert(BatMetadataBundle::default());
        }
        EntityKind::Bee => {
            entity.insert(BeeMetadataBundle::default());
        }
        EntityKind::BirchBoat => {
            entity.insert(BirchBoatMetadataBundle::default());
        }
        EntityKind::BirchChestBoat => {
            entity.insert(BirchChestBoatMetadataBundle::default());
        }
        EntityKind::Blaze => {
            entity.insert(BlazeMetadataBundle::default());
        }
        EntityKind::BlockDisplay => {
            entity.insert(BlockDisplayMetadataBundle::default());
        }
        EntityKind::Bogged => {
            entity.insert(BoggedMetadataBundle::default());
        }
        EntityKind::Breeze => {
            entity.insert(BreezeMetadataBundle::default());
        }
        EntityKind::BreezeWindCharge => {
            entity.insert(BreezeWindChargeMetadataBundle::default());
        }
        EntityKind::Camel => {
            entity.insert(CamelMetadataBundle::default());
        }
        EntityKind::CamelHusk => {
            entity.insert(CamelHuskMetadataBundle::default());
        }
        EntityKind::Cat => {
            entity.insert(CatMetadataBundle::default());
        }
        EntityKind::CaveSpider => {
            entity.insert(CaveSpiderMetadataBundle::default());
        }
        EntityKind::CherryBoat => {
            entity.insert(CherryBoatMetadataBundle::default());
        }
        EntityKind::CherryChestBoat => {
            entity.insert(CherryChestBoatMetadataBundle::default());
        }
        EntityKind::ChestMinecart => {
            entity.insert(ChestMinecartMetadataBundle::default());
        }
        EntityKind::Chicken => {
            entity.insert(ChickenMetadataBundle::default());
        }
        EntityKind::Cod => {
            entity.insert(CodMetadataBundle::default());
        }
        EntityKind::CommandBlockMinecart => {
            entity.insert(CommandBlockMinecartMetadataBundle::default());
        }
        EntityKind::CopperGolem => {
            entity.insert(CopperGolemMetadataBundle::default());
        }
        EntityKind::Cow => {
            entity.insert(CowMetadataBundle::default());
        }
        EntityKind::Creaking => {
            entity.insert(CreakingMetadataBundle::default());
        }
        EntityKind::Creeper => {
            entity.insert(CreeperMetadataBundle::default());
        }
        EntityKind::DarkOakBoat => {
            entity.insert(DarkOakBoatMetadataBundle::default());
        }
        EntityKind::DarkOakChestBoat => {
            entity.insert(DarkOakChestBoatMetadataBundle::default());
        }
        EntityKind::Dolphin => {
            entity.insert(DolphinMetadataBundle::default());
        }
        EntityKind::Donkey => {
            entity.insert(DonkeyMetadataBundle::default());
        }
        EntityKind::DragonFireball => {
            entity.insert(DragonFireballMetadataBundle::default());
        }
        EntityKind::Drowned => {
            entity.insert(DrownedMetadataBundle::default());
        }
        EntityKind::Egg => {
            entity.insert(EggMetadataBundle::default());
        }
        EntityKind::ElderGuardian => {
            entity.insert(ElderGuardianMetadataBundle::default());
        }
        EntityKind::EndCrystal => {
            entity.insert(EndCrystalMetadataBundle::default());
        }
        EntityKind::EnderDragon => {
            entity.insert(EnderDragonMetadataBundle::default());
        }
        EntityKind::EnderPearl => {
            entity.insert(EnderPearlMetadataBundle::default());
        }
        EntityKind::Enderman => {
            entity.insert(EndermanMetadataBundle::default());
        }
        EntityKind::Endermite => {
            entity.insert(EndermiteMetadataBundle::default());
        }
        EntityKind::Evoker => {
            entity.insert(EvokerMetadataBundle::default());
        }
        EntityKind::EvokerFangs => {
            entity.insert(EvokerFangsMetadataBundle::default());
        }
        EntityKind::ExperienceBottle => {
            entity.insert(ExperienceBottleMetadataBundle::default());
        }
        EntityKind::ExperienceOrb => {
            entity.insert(ExperienceOrbMetadataBundle::default());
        }
        EntityKind::EyeOfEnder => {
            entity.insert(EyeOfEnderMetadataBundle::default());
        }
        EntityKind::FallingBlock => {
            entity.insert(FallingBlockMetadataBundle::default());
        }
        EntityKind::Fireball => {
            entity.insert(FireballMetadataBundle::default());
        }
        EntityKind::FireworkRocket => {
            entity.insert(FireworkRocketMetadataBundle::default());
        }
        EntityKind::FishingBobber => {
            entity.insert(FishingBobberMetadataBundle::default());
        }
        EntityKind::Fox => {
            entity.insert(FoxMetadataBundle::default());
        }
        EntityKind::Frog => {
            entity.insert(FrogMetadataBundle::default());
        }
        EntityKind::FurnaceMinecart => {
            entity.insert(FurnaceMinecartMetadataBundle::default());
        }
        EntityKind::Ghast => {
            entity.insert(GhastMetadataBundle::default());
        }
        EntityKind::Giant => {
            entity.insert(GiantMetadataBundle::default());
        }
        EntityKind::GlowItemFrame => {
            entity.insert(GlowItemFrameMetadataBundle::default());
        }
        EntityKind::GlowSquid => {
            entity.insert(GlowSquidMetadataBundle::default());
        }
        EntityKind::Goat => {
            entity.insert(GoatMetadataBundle::default());
        }
        EntityKind::Guardian => {
            entity.insert(GuardianMetadataBundle::default());
        }
        EntityKind::HappyGhast => {
            entity.insert(HappyGhastMetadataBundle::default());
        }
        EntityKind::Hoglin => {
            entity.insert(HoglinMetadataBundle::default());
        }
        EntityKind::HopperMinecart => {
            entity.insert(HopperMinecartMetadataBundle::default());
        }
        EntityKind::Horse => {
            entity.insert(HorseMetadataBundle::default());
        }
        EntityKind::Husk => {
            entity.insert(HuskMetadataBundle::default());
        }
        EntityKind::Illusioner => {
            entity.insert(IllusionerMetadataBundle::default());
        }
        EntityKind::Interaction => {
            entity.insert(InteractionMetadataBundle::default());
        }
        EntityKind::IronGolem => {
            entity.insert(IronGolemMetadataBundle::default());
        }
        EntityKind::Item => {
            entity.insert(ItemMetadataBundle::default());
        }
        EntityKind::ItemDisplay => {
            entity.insert(ItemDisplayMetadataBundle::default());
        }
        EntityKind::ItemFrame => {
            entity.insert(ItemFrameMetadataBundle::default());
        }
        EntityKind::JungleBoat => {
            entity.insert(JungleBoatMetadataBundle::default());
        }
        EntityKind::JungleChestBoat => {
            entity.insert(JungleChestBoatMetadataBundle::default());
        }
        EntityKind::LeashKnot => {
            entity.insert(LeashKnotMetadataBundle::default());
        }
        EntityKind::LightningBolt => {
            entity.insert(LightningBoltMetadataBundle::default());
        }
        EntityKind::LingeringPotion => {
            entity.insert(LingeringPotionMetadataBundle::default());
        }
        EntityKind::Llama => {
            entity.insert(LlamaMetadataBundle::default());
        }
        EntityKind::LlamaSpit => {
            entity.insert(LlamaSpitMetadataBundle::default());
        }
        EntityKind::MagmaCube => {
            entity.insert(MagmaCubeMetadataBundle::default());
        }
        EntityKind::MangroveBoat => {
            entity.insert(MangroveBoatMetadataBundle::default());
        }
        EntityKind::MangroveChestBoat => {
            entity.insert(MangroveChestBoatMetadataBundle::default());
        }
        EntityKind::Mannequin => {
            entity.insert(MannequinMetadataBundle::default());
        }
        EntityKind::Marker => {
            entity.insert(MarkerMetadataBundle::default());
        }
        EntityKind::Minecart => {
            entity.insert(MinecartMetadataBundle::default());
        }
        EntityKind::Mooshroom => {
            entity.insert(MooshroomMetadataBundle::default());
        }
        EntityKind::Mule => {
            entity.insert(MuleMetadataBundle::default());
        }
        EntityKind::Nautilus => {
            entity.insert(NautilusMetadataBundle::default());
        }
        EntityKind::OakBoat => {
            entity.insert(OakBoatMetadataBundle::default());
        }
        EntityKind::OakChestBoat => {
            entity.insert(OakChestBoatMetadataBundle::default());
        }
        EntityKind::Ocelot => {
            entity.insert(OcelotMetadataBundle::default());
        }
        EntityKind::OminousItemSpawner => {
            entity.insert(OminousItemSpawnerMetadataBundle::default());
        }
        EntityKind::Painting => {
            entity.insert(PaintingMetadataBundle::default());
        }
        EntityKind::PaleOakBoat => {
            entity.insert(PaleOakBoatMetadataBundle::default());
        }
        EntityKind::PaleOakChestBoat => {
            entity.insert(PaleOakChestBoatMetadataBundle::default());
        }
        EntityKind::Panda => {
            entity.insert(PandaMetadataBundle::default());
        }
        EntityKind::Parched => {
            entity.insert(ParchedMetadataBundle::default());
        }
        EntityKind::Parrot => {
            entity.insert(ParrotMetadataBundle::default());
        }
        EntityKind::Phantom => {
            entity.insert(PhantomMetadataBundle::default());
        }
        EntityKind::Pig => {
            entity.insert(PigMetadataBundle::default());
        }
        EntityKind::Piglin => {
            entity.insert(PiglinMetadataBundle::default());
        }
        EntityKind::PiglinBrute => {
            entity.insert(PiglinBruteMetadataBundle::default());
        }
        EntityKind::Pillager => {
            entity.insert(PillagerMetadataBundle::default());
        }
        EntityKind::Player => {
            entity.insert(PlayerMetadataBundle::default());
        }
        EntityKind::PolarBear => {
            entity.insert(PolarBearMetadataBundle::default());
        }
        EntityKind::Pufferfish => {
            entity.insert(PufferfishMetadataBundle::default());
        }
        EntityKind::Rabbit => {
            entity.insert(RabbitMetadataBundle::default());
        }
        EntityKind::Ravager => {
            entity.insert(RavagerMetadataBundle::default());
        }
        EntityKind::Salmon => {
            entity.insert(SalmonMetadataBundle::default());
        }
        EntityKind::Sheep => {
            entity.insert(SheepMetadataBundle::default());
        }
        EntityKind::Shulker => {
            entity.insert(ShulkerMetadataBundle::default());
        }
        EntityKind::ShulkerBullet => {
            entity.insert(ShulkerBulletMetadataBundle::default());
        }
        EntityKind::Silverfish => {
            entity.insert(SilverfishMetadataBundle::default());
        }
        EntityKind::Skeleton => {
            entity.insert(SkeletonMetadataBundle::default());
        }
        EntityKind::SkeletonHorse => {
            entity.insert(SkeletonHorseMetadataBundle::default());
        }
        EntityKind::Slime => {
            entity.insert(SlimeMetadataBundle::default());
        }
        EntityKind::SmallFireball => {
            entity.insert(SmallFireballMetadataBundle::default());
        }
        EntityKind::Sniffer => {
            entity.insert(SnifferMetadataBundle::default());
        }
        EntityKind::SnowGolem => {
            entity.insert(SnowGolemMetadataBundle::default());
        }
        EntityKind::Snowball => {
            entity.insert(SnowballMetadataBundle::default());
        }
        EntityKind::SpawnerMinecart => {
            entity.insert(SpawnerMinecartMetadataBundle::default());
        }
        EntityKind::SpectralArrow => {
            entity.insert(SpectralArrowMetadataBundle::default());
        }
        EntityKind::Spider => {
            entity.insert(SpiderMetadataBundle::default());
        }
        EntityKind::SplashPotion => {
            entity.insert(SplashPotionMetadataBundle::default());
        }
        EntityKind::SpruceBoat => {
            entity.insert(SpruceBoatMetadataBundle::default());
        }
        EntityKind::SpruceChestBoat => {
            entity.insert(SpruceChestBoatMetadataBundle::default());
        }
        EntityKind::Squid => {
            entity.insert(SquidMetadataBundle::default());
        }
        EntityKind::Stray => {
            entity.insert(StrayMetadataBundle::default());
        }
        EntityKind::Strider => {
            entity.insert(StriderMetadataBundle::default());
        }
        EntityKind::Tadpole => {
            entity.insert(TadpoleMetadataBundle::default());
        }
        EntityKind::TextDisplay => {
            entity.insert(TextDisplayMetadataBundle::default());
        }
        EntityKind::Tnt => {
            entity.insert(TntMetadataBundle::default());
        }
        EntityKind::TntMinecart => {
            entity.insert(TntMinecartMetadataBundle::default());
        }
        EntityKind::TraderLlama => {
            entity.insert(TraderLlamaMetadataBundle::default());
        }
        EntityKind::Trident => {
            entity.insert(TridentMetadataBundle::default());
        }
        EntityKind::TropicalFish => {
            entity.insert(TropicalFishMetadataBundle::default());
        }
        EntityKind::Turtle => {
            entity.insert(TurtleMetadataBundle::default());
        }
        EntityKind::Vex => {
            entity.insert(VexMetadataBundle::default());
        }
        EntityKind::Villager => {
            entity.insert(VillagerMetadataBundle::default());
        }
        EntityKind::Vindicator => {
            entity.insert(VindicatorMetadataBundle::default());
        }
        EntityKind::WanderingTrader => {
            entity.insert(WanderingTraderMetadataBundle::default());
        }
        EntityKind::Warden => {
            entity.insert(WardenMetadataBundle::default());
        }
        EntityKind::WindCharge => {
            entity.insert(WindChargeMetadataBundle::default());
        }
        EntityKind::Witch => {
            entity.insert(WitchMetadataBundle::default());
        }
        EntityKind::Wither => {
            entity.insert(WitherMetadataBundle::default());
        }
        EntityKind::WitherSkeleton => {
            entity.insert(WitherSkeletonMetadataBundle::default());
        }
        EntityKind::WitherSkull => {
            entity.insert(WitherSkullMetadataBundle::default());
        }
        EntityKind::Wolf => {
            entity.insert(WolfMetadataBundle::default());
        }
        EntityKind::Zoglin => {
            entity.insert(ZoglinMetadataBundle::default());
        }
        EntityKind::Zombie => {
            entity.insert(ZombieMetadataBundle::default());
        }
        EntityKind::ZombieHorse => {
            entity.insert(ZombieHorseMetadataBundle::default());
        }
        EntityKind::ZombieNautilus => {
            entity.insert(ZombieNautilusMetadataBundle::default());
        }
        EntityKind::ZombieVillager => {
            entity.insert(ZombieVillagerMetadataBundle::default());
        }
        EntityKind::ZombifiedPiglin => {
            entity.insert(ZombifiedPiglinMetadataBundle::default());
        }
    }
}
