use std::{fmt::Debug, mem::ManuallyDrop};

use azalea_registry::{
    EnchantmentEffectComponentKind, EnchantmentEntityEffectKind as EntityEffectKind, SoundEvent,
};
use serde::ser::SerializeMap;
use simdnbt::{DeserializeError, borrow::NbtCompound};

use crate::registry_holder::{entity_effect::EntityEffect, value::ValueEffect};

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

            pub fn from_compound_as(
                kind: EnchantmentEffectComponentKind,
                compound: NbtCompound
            ) -> Result<Self, DeserializeError> {
                Ok(match kind {
                    $( EnchantmentEffectComponentKind::$x => {
                        Self { $x: ManuallyDrop::new(<$t>::from_compound(buf)?) }
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
            pub unsafe fn eq_as(
                &self,
                other: &Self,
                kind: EnchantmentEffectComponentKind,
            ) -> bool {
                match kind {
                    $( EnchantmentEffectComponentKind::$x => unsafe { self.$x.eq(&other.$x) }, )*
                }
            }
        }
    };
}

define_effect_components!(
    DamageProtection: ConditionalEffect<ValueEffect>,
    DamageImmunity: ConditionalEffect<DamageImmunity>,
    Damage: ConditionalEffect<ValueEffect>,
    SmashDamagePerFallenBlock: ConditionalEffect<ValueEffect>,
    Knockback: ConditionalEffect<ValueEffect>,
    ArmorEffectiveness: ConditionalEffect<ValueEffect>,
    PostAttack: TargetedConditionalEffect<EntityEffect>,
    HitBlock: ConditionalEffect<EntityEffect>,
    ItemDamage: ConditionalEffect<ValueEffect>,
    Attributes: AttributeEffect,
    EquipmentDrops: ConditionalEffect<ValueEffect>,
    LocationChanged: ConditionalEffect<LocationBasedEffect>,
    Tick: ConditionalEffect<EntityEffect>,
    AmmoUse: ConditionalEffect<ValueEffect>,
    ProjectilePiercing: ConditionalEffect<ValueEffect>,
    ProjectileSpawned: ConditionalEffect<EntityEffect>,
    ProjectileSpread: ConditionalEffect<ValueEffect>,
    ProjectileCount: ConditionalEffect<ValueEffect>,
    TridentReturnAcceleration: ConditionalEffect<ValueEffect>,
    FishingTimeReduction: ConditionalEffect<ValueEffect>,
    FishingLuckBonus: ConditionalEffect<ValueEffect>,
    BlockExperience: ConditionalEffect<ValueEffect>,
    MobExperience: ConditionalEffect<ValueEffect>,
    RepairWithXp: ConditionalEffect<ValueEffect>,
    CrossbowChargeTime: ValueEffect,
    CrossbowChargingSounds: Vec<CrossbowChargingSounds>,
    TridentSound: Vec<SoundEvent>,
    PreventEquipmentDrop: (),
    PreventArmorChange: (),
    TridentSpinAttackStrength: ValueEffect,
);

#[derive(Debug, Clone)]
pub struct ConditionalEffect<T: simdnbt::Deserialize + Debug + Clone> {
    pub effect: T,
    // pub requirements
}
#[derive(Debug, Clone)]
pub struct TargetedConditionalEffect<T: simdnbt::Deserialize + Debug + Clone> {
    pub effect: T,
    // pub enchanted
    // pub affected
    // pub requirements
}

impl<T: simdnbt::Deserialize + Debug + Clone> simdnbt::Deserialize for ConditionalEffect<T> {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let effect = T::from_compound(nbt.compound("effect"))?;

        Self { effect }
    }
}

impl<T: simdnbt::Deserialize + Debug + Clone> simdnbt::Deserialize
    for TargetedConditionalEffect<T>
{
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let effect = T::from_compound(nbt.compound("effect"))?;

        Self { effect }
    }
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct DamageImmunity {}

pub struct CrossbowChargingSounds {
    pub start: SoundEvent,
    pub mid: SoundEvent,
    pub end: SoundEvent,
}
