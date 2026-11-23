use std::{any::Any, fmt::Debug, mem::ManuallyDrop};

use azalea_codec::Codec;
use azalea_registry::builtin::{EnchantmentEffectComponentKind, SoundEvent};
use simdnbt::{
    DeserializeError,
    borrow::{NbtCompound, NbtList},
};

use crate::registry_holder::{
    entity_effect::EntityEffect,
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

#[derive(Clone, Debug, Codec)]
pub struct ConditionalEffect<T: Debug + Clone> {
    pub effect: T,
    // pub requirements
}
#[derive(Clone, Debug, Codec)]
pub struct TargetedConditionalEffect<T: Debug + Clone> {
    pub effect: T,
    // pub enchanted
    // pub affected
    // pub requirements
}

// makes for cleaner-looking types
type ConditionalValueEffect = ConditionalEffect<ValueEffect>;
type ConditionalEntityEffect = ConditionalEffect<EntityEffect>;

macro_rules! declare_newtype_components {
    ( $( $struct_name:ident: $inner_type:ty ),* $(,)? ) => {
        $(
            #[derive(Clone, Debug, Codec)]
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

#[derive(Clone, Debug, Codec)]
pub struct DamageImmunity {}

#[derive(Clone, Debug, Codec)]
pub struct CrossbowChargingSounds(pub Vec<CrossbowChargingSound>);

#[derive(Clone, Debug, Codec)]
pub struct CrossbowChargingSound {
    #[codec(optional)]
    pub start: Option<SoundEvent>,
    #[codec(optional)]
    pub mid: Option<SoundEvent>,
    #[codec(optional)]
    pub end: Option<SoundEvent>,
}

#[derive(Clone, Debug, Codec)]
pub struct TridentSound(pub Vec<SoundEvent>);

#[derive(Clone, Debug, Codec)]
#[codec(allow_unknown_keys)]
pub struct LocationBasedEffect {
    // TODO
}

#[derive(Clone, Debug, Codec)]
#[codec(allow_unknown_keys)]
pub struct PreventEquipmentDrop {}

#[derive(Clone, Debug, Codec)]
#[codec(allow_unknown_keys)]
pub struct PreventArmorChange {}
