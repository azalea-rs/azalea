use std::{any::Any, fmt::Debug, mem::ManuallyDrop, str::FromStr};

use azalea_registry::builtin::{EnchantmentEffectComponentKind, SoundEvent};
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
/// from [`EnchantmentData::get`](super::enchantment::EnchantmentData::get).
pub trait EffectComponentTrait: Any {
    const KIND: EnchantmentEffectComponentKind;
}

// this exists because EffectComponentTrait isn't dyn-compatible
pub trait ResolvedEffectComponent: Any {}
impl<T: EffectComponentTrait> ResolvedEffectComponent for T {}

/// An alternative to `simdnbt::borrow::NbtTag` used internally when
/// deserializing effects.
///
/// When deserializing effect components from the registry, we're given NBT tags
/// in either a list of compounds or a list of lists. This means that we can't
/// just use `from_nbt_tag`, because `borrow::NbtTag` can't be constructed on
/// its own. To work around this, we have this `EffectNbtTag` struct that we
/// *can* construct that we use when deserializing.
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
pub struct ConditionalEffect<T: simdnbt::Deserialize + Debug + Clone> {
    pub effect: T,
    // pub requirements
}
#[derive(Clone, Debug)]
pub struct TargetedConditionalEffect<T: simdnbt::Deserialize + Debug + Clone> {
    pub effect: T,
    // pub enchanted
    // pub affected
    // pub requirements
}

// makes for cleaner-looking types
type ConditionalValueEffect = ConditionalEffect<ValueEffect>;
type ConditionalEntityEffect = ConditionalEffect<EntityEffect>;

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
        let effect = get_in_compound(&nbt, "effect")?;
        Ok(Self { effect })
    }
}

macro_rules! declare_newtype_components {
    ( $( $struct_name:ident: $inner_type:ty ),* $(,)? ) => {
        $(
            #[derive(Clone, Debug, simdnbt::Deserialize)]
            pub struct $struct_name(pub $inner_type);
            impl_from_effect_nbt_tag!($struct_name);
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
    EquipmentDrops: ConditionalValueEffect,
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
        let Ok(nbt) = nbt.list("CrossbowChargingSounds") else {
            return Ok(Self(vec![simdnbt::Deserialize::from_compound(
                nbt.compound("CrossbowChargingSounds")?,
            )?]));
        };

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
    pub start: Option<SoundEvent>,
    pub mid: Option<SoundEvent>,
    pub end: Option<SoundEvent>,
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
