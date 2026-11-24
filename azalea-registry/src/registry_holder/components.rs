use std::{any::Any, fmt::Debug, mem::ManuallyDrop};

use azalea_codec::Codec;
use azalea_registry_builtin::builtin::LootConditionKind;

use crate::{
    HolderSet,
    builtin::{EnchantmentEffectComponentKind, SoundEvent},
    registry_holder::{
        entity_effect::EntityEffect,
        value::{AttributeEffect, ValueEffect},
    },
};

#[macro_export]
macro_rules! define_effect_components {
    ( $( $x:ident: $t:ty ),* $(,)? ) => {
        #[allow(non_snake_case)]
        pub union EffectComponentUnion {
            $( $x: ManuallyDrop<$t>, )*
        }

        impl EffectComponentUnion {
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn drop_as(&mut self, kind: EnchantmentEffectComponentKind) {
                match kind {
                    $( EnchantmentEffectComponentKind::$x => { unsafe { ManuallyDrop::drop(&mut self.$x) } }, )*
                }
            }

            pub fn codec_read_as<O: azalea_codec::Ops>(
                kind: EnchantmentEffectComponentKind,
                ops: &O,
                ctx: azalea_codec::Ctx<O>,
            ) -> azalea_codec::error::ReadResult<Vec<Self>> {
                Ok(match kind {
                    $( EnchantmentEffectComponentKind::$x => {
                        azalea_codec::types::CompactListCodec::<Vec<$t>>::codec_read(ops, ctx)?.0
                            .into_iter()
                            .map(ManuallyDrop::new)
                            .map(|c| Self { $x: c })
                            .collect()
                    }, )*
                })
            }
            pub fn codec_write_as<O: azalea_codec::Ops>(
                items: Vec<Self>,
                kind: EnchantmentEffectComponentKind,
                ctx: azalea_codec::Ctx<O>,
            ) -> azalea_codec::error::WriteResult<O> {
                Ok(match kind {
                    $( EnchantmentEffectComponentKind::$x => {
                        azalea_codec::types::CompactListCodec(
                            items.into_iter().map(|i| unsafe { i.$x }).collect::<Vec<ManuallyDrop<_>>>()
                        ).codec_write(ctx)?
                    }, )*
                })
            }

            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn clone_as(
                &self,
                kind: EnchantmentEffectComponentKind,
            ) -> Self {
                match kind {
                    $( EnchantmentEffectComponentKind::$x => {
                        Self { $x: unsafe { self.$x.clone() } }
                    }, )*
                }
            }

            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn as_kind(&self, kind: EnchantmentEffectComponentKind) -> &dyn ResolvedEffectComponent {
                match kind {
                    $( EnchantmentEffectComponentKind::$x => { unsafe { &**(&self.$x as &ManuallyDrop<dyn ResolvedEffectComponent>) } }, )*
                }
            }
        }

        $(
            impl EffectComponentTrait for $t {
                const KIND: EnchantmentEffectComponentKind = EnchantmentEffectComponentKind::$x;
            }
        )*
    };
}

define_effect_components!(
    DamageProtection: DamageProtection,
    DamageImmunity: ConditionalEffect<DamageImmunity>,
    Damage: Damage,
    SmashDamagePerFallenBlock: SmashDamagePerFallenBlock,
    Knockback: Knockback,
    ArmorEffectiveness: ArmorEffectiveness,
    PostAttack: PostAttack,
    PostPiercingAttack: PostPiercingAttack,
    HitBlock: ConditionalEntityEffect,
    ItemDamage: ConditionalValueEffect,
    Attributes: AttributeEffect,
    EquipmentDrops: EquipmentDrops,
    LocationChanged: ConditionalEffect<LocationBasedEffect>,
    Tick: Tick,
    AmmoUse: AmmoUse,
    ProjectilePiercing: ProjectilePiercing,
    ProjectileSpawned: ProjectileSpawned,
    ProjectileSpread: ProjectileSpread,
    ProjectileCount: ProjectileCount,
    TridentReturnAcceleration: TridentReturnAcceleration,
    FishingTimeReduction: FishingTimeReduction,
    FishingLuckBonus: FishingLuckBonus,
    BlockExperience: BlockExperience,
    MobExperience: MobExperience,
    RepairWithXp: RepairWithXp,
    CrossbowChargeTime: CrossbowChargeTime,
    CrossbowChargingSounds: CrossbowChargingSounds,
    TridentSound: TridentSound,
    PreventEquipmentDrop: PreventEquipmentDrop,
    PreventArmorChange: PreventArmorChange,
    TridentSpinAttackStrength: TridentSpinAttackStrength,
);

/// A trait that's implemented on all effect components so we can access them
/// from [`Effects::get`](super::enchantment::Effects::get).
pub trait EffectComponentTrait: Any {
    const KIND: EnchantmentEffectComponentKind;
}

// this exists because EffectComponentTrait isn't dyn-compatible
pub trait ResolvedEffectComponent: Any {
    fn component_eq(&self, other: &dyn ResolvedEffectComponent) -> bool;
}
impl<T: EffectComponentTrait + PartialEq> ResolvedEffectComponent for T {
    fn component_eq(&self, other: &dyn ResolvedEffectComponent) -> bool {
        let other = other as &dyn Any;
        let Some(other) = other.downcast_ref::<T>() else {
            return false;
        };
        self.eq(other)
    }
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ConditionalEffect<T: Debug + Clone + PartialEq> {
    pub effect: T,
    #[codec(optional)]
    pub requirements: Option<LootItemCondition>,
}
#[derive(Clone, Debug, PartialEq, Codec)]
pub struct TargetedConditionalEffect<T: Debug + Clone + PartialEq> {
    pub enchanted: EnchantmentTarget,
    pub affected: EnchantmentTarget,
    pub effect: T,
    #[codec(optional)]
    pub requirements: Option<LootItemCondition>,
}
#[derive(Clone, Debug, PartialEq, Codec)]
pub struct EquipmentDropsEffect<T: Debug + Clone + PartialEq> {
    pub enchanted: NonDamageEnchantmentTarget,
    pub effect: T,
    #[codec(optional)]
    pub requirements: Option<LootItemCondition>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum EnchantmentTarget {
    Attacker,
    DamagingEntity,
    Victim,
}
#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum NonDamageEnchantmentTarget {
    Attacker,
    Victim,
}

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type", tag_as = LootConditionKind)]
#[codec(default_snake_case_key)]
pub enum LootItemCondition {
    Inverted {
        term: Box<LootItemCondition>,
    },
    AnyOf {
        terms: HolderSet<LootItemCondition>,
    },
    AllOf {
        terms: HolderSet<LootItemCondition>,
    },
    // TODO: implement the rest
    #[codec(allow_unknown_keys)]
    RandomChance,
    #[codec(allow_unknown_keys)]
    RandomChanceWithEnchantedBonus,
    #[codec(allow_unknown_keys)]
    EntityProperties,
    #[codec(allow_unknown_keys)]
    KilledByPlayer,
    #[codec(allow_unknown_keys)]
    EntityScores,
    #[codec(allow_unknown_keys)]
    MatchBlock,
    #[codec(allow_unknown_keys)]
    MatchTool,
    #[codec(allow_unknown_keys)]
    TableBonus,
    #[codec(allow_unknown_keys)]
    SurvivesExplosion,
    #[codec(allow_unknown_keys)]
    DamageSourceProperties,
    #[codec(allow_unknown_keys)]
    LocationCheck,
    #[codec(allow_unknown_keys)]
    WeatherCheck,
    #[codec(allow_unknown_keys)]
    TimeCheck,
    #[codec(allow_unknown_keys)]
    IntValueCheck,
    #[codec(allow_unknown_keys)]
    FloatValueCheck,
    #[codec(allow_unknown_keys)]
    EnchantmentActiveCheck,
    #[codec(allow_unknown_keys)]
    EnvironmentAttributeCheck,
}
// azalea_codec::impl_defaultable!(LootItemCondition);

// makes for cleaner-looking types
type ConditionalValueEffect = ConditionalEffect<ValueEffect>;
type ConditionalEntityEffect = ConditionalEffect<EntityEffect>;

macro_rules! declare_newtype_components {
    ( $( $struct_name:ident: $inner_type:ty ),* $(,)? ) => {
        $(
            #[derive(Clone, Debug, PartialEq, Codec)]
            pub struct $struct_name(pub $inner_type);
        )*
    };
}

declare_newtype_components! {
    DamageProtection: ConditionalValueEffect,
    Damage: ConditionalValueEffect,
    SmashDamagePerFallenBlock: ConditionalValueEffect,
    Knockback: ConditionalValueEffect,
    ArmorEffectiveness: ConditionalValueEffect,
    PostAttack: TargetedConditionalEffect<EntityEffect>,
    PostPiercingAttack: ConditionalEffect<EntityEffect>,
    HitBlock: ConditionalEntityEffect,
    ItemDamage: ConditionalValueEffect,
    EquipmentDrops: EquipmentDropsEffect<ValueEffect>,
    Tick: ConditionalEntityEffect,
    AmmoUse: ConditionalValueEffect,
    ProjectilePiercing: ConditionalValueEffect,
    ProjectileSpawned: ConditionalEntityEffect,
    ProjectileSpread: ConditionalValueEffect,
    ProjectileCount: ConditionalValueEffect,
    TridentReturnAcceleration: ConditionalValueEffect,
    FishingTimeReduction: ConditionalValueEffect,
    FishingLuckBonus: ConditionalValueEffect,
    BlockExperience: ConditionalValueEffect,
    MobExperience: ConditionalValueEffect,
    RepairWithXp: ConditionalValueEffect,
    CrossbowChargeTime: ValueEffect,
    TridentSpinAttackStrength: ValueEffect,

}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct DamageImmunity {}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct CrossbowChargingSounds(pub Vec<CrossbowChargingSound>);

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct CrossbowChargingSound {
    #[codec(optional)]
    pub start: Option<SoundEvent>,
    #[codec(optional)]
    pub mid: Option<SoundEvent>,
    #[codec(optional)]
    pub end: Option<SoundEvent>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct TridentSound(pub Vec<SoundEvent>);

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(allow_unknown_keys)]
pub struct LocationBasedEffect {
    // TODO
}

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(allow_unknown_keys)]
pub struct PreventEquipmentDrop {}

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(allow_unknown_keys)]
pub struct PreventArmorChange {}
