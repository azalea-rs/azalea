use std::{any::type_name, fmt::Debug, mem::ManuallyDrop, str::FromStr};

use azalea_registry::{EnchantmentEffectComponentKind, SoundEvent};
use simdnbt::{
    DeserializeError,
    borrow::{NbtCompound, NbtList, NbtTag},
};

use crate::registry_holder::{
    entity_effect::EntityEffect,
    get_in_compound,
    value::{AttributeEffect, ValueEffect},
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

            pub fn from_effect_nbt_tag_as(
                kind: EnchantmentEffectComponentKind,
                tag: EffectNbtTag,
            ) -> Result<Self, DeserializeError> {
                println!("from_nbt_tag_as {kind:?} {tag:?}");
                Ok(match kind {
                    $( EnchantmentEffectComponentKind::$x => {
                        Self { $x: ManuallyDrop::new(<$t>::from_effect_nbt_tag(tag)?) }
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
    CrossbowChargingSounds: CrossbowChargingSounds,
    TridentSound: TridentSound,
    PreventEquipmentDrop: PreventEquipmentDrop,
    PreventArmorChange: PreventArmorChange,
    TridentSpinAttackStrength: ValueEffect,
);

/// An alternative to `simdnbt::borrow::NbtTag` used internally when
/// deserializing effects.
///
/// When deserializing effect components from the registry, we're given NBT tags
/// in either a list of compounds or a list of lists. This means that we can't
/// just use `from_nbt_tag`, because `borrow::NbtTag` can't be constructed on
/// its own. To work around this, we have this `EffectNbtTag` struct that we
/// *can* construct that we use when deserializing.
#[derive(Debug, Clone)]
pub enum EffectNbtTag<'a, 'tape> {
    Compound(NbtCompound<'a, 'tape>),
    List(NbtList<'a, 'tape>),
}

impl<'a, 'tape> EffectNbtTag<'a, 'tape> {
    pub fn compound(self, error_name: &str) -> Result<NbtCompound<'a, 'tape>, DeserializeError> {
        if let Self::Compound(nbt) = self {
            Ok(nbt)
        } else {
            Err(DeserializeError::MismatchedFieldType(error_name.to_owned()))
        }
    }
    pub fn list(self, error_name: &str) -> Result<NbtList<'a, 'tape>, DeserializeError> {
        if let Self::List(nbt) = self {
            Ok(nbt)
        } else {
            Err(DeserializeError::MismatchedFieldType(error_name.to_owned()))
        }
    }
}
macro_rules! impl_from_effect_nbt_tag {
    (<$g:tt : $generic_type:tt $(::$generic_type2:tt)* $(+ $generic_type3:tt)+> $ty:ident <$generic_name:ident>) => {
        impl<$g: $generic_type$(::$generic_type2)* $(+ $generic_type3)+> $ty<$generic_name> {
            fn from_effect_nbt_tag(nbt: crate::registry_holder::components::EffectNbtTag) -> Result<Self, DeserializeError> {
                let nbt = nbt.compound(stringify!($ty))?;
                simdnbt::Deserialize::from_compound(nbt)
            }
        }
    };
    ($ty:ident) => {
        impl $ty {
            pub fn from_effect_nbt_tag(nbt: crate::registry_holder::components::EffectNbtTag) -> Result<Self, DeserializeError> {
                let nbt = nbt.compound(stringify!($ty))?;
                simdnbt::Deserialize::from_compound(nbt)
            }
        }
    };
}
pub(crate) use impl_from_effect_nbt_tag;

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
        let effect = get_in_compound(&nbt, "effect")?;
        Ok(Self { effect })
    }
}
impl_from_effect_nbt_tag!(<T: simdnbt::Deserialize + Debug + Clone> ConditionalEffect<T>);

impl<T: simdnbt::Deserialize + Debug + Clone> simdnbt::Deserialize
    for TargetedConditionalEffect<T>
{
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        println!(
            "parsing TargetedConditionalEffect<{}> in {nbt:?}",
            type_name::<T>()
        );
        let effect = get_in_compound(&nbt, "effect")?;
        println!("parsed TargetedConditionalEffect");
        Ok(Self { effect })
    }
}
impl_from_effect_nbt_tag!(<T: simdnbt::Deserialize + Debug + Clone> TargetedConditionalEffect<T>);

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct DamageImmunity {}
impl_from_effect_nbt_tag!(DamageImmunity);

#[derive(Clone, Debug)]
pub struct CrossbowChargingSounds(pub Vec<CrossbowChargingSound>);
impl simdnbt::FromNbtTag for CrossbowChargingSounds {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        simdnbt::FromNbtTag::from_nbt_tag(tag).map(Self)
    }
}
impl CrossbowChargingSounds {
    pub fn from_effect_nbt_tag(nbt: EffectNbtTag) -> Result<Self, DeserializeError> {
        let nbt = nbt.list("CrossbowChargingSounds")?;

        Ok(Self(
            nbt.compounds()
                .ok_or_else(|| {
                    DeserializeError::MismatchedFieldType("CrossbowChargingSounds".to_owned())
                })?
                .into_iter()
                .map(|c| simdnbt::Deserialize::from_compound(c))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct CrossbowChargingSound {
    pub start: SoundEvent,
    pub mid: SoundEvent,
    pub end: SoundEvent,
}

#[derive(Clone, Debug)]
pub struct TridentSound(pub Vec<SoundEvent>);
impl simdnbt::FromNbtTag for TridentSound {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        let sounds = tag.list()?.strings()?;

        sounds
            .iter()
            .map(|s| SoundEvent::from_str(&s.to_str()).ok())
            .collect::<Option<Vec<_>>>()
            .map(Self)
    }
}
impl TridentSound {
    pub fn from_effect_nbt_tag(nbt: EffectNbtTag) -> Result<Self, DeserializeError> {
        let sounds = nbt
            .list("TridentSound")?
            .strings()
            .ok_or_else(|| DeserializeError::MismatchedFieldType("TridentSound".to_owned()))?;

        sounds
            .iter()
            .map(|s| SoundEvent::from_str(&s.to_str()).ok())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| DeserializeError::MismatchedFieldType("TridentSound".to_owned()))
            .map(Self)
    }
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct LocationBasedEffect {
    // TODO
}
impl_from_effect_nbt_tag!(LocationBasedEffect);

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct PreventEquipmentDrop {}
impl_from_effect_nbt_tag!(PreventEquipmentDrop);

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct PreventArmorChange {}
impl_from_effect_nbt_tag!(PreventArmorChange);
