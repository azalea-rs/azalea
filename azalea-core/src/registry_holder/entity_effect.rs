use azalea_registry::{
    DamageKind, EnchantmentEntityEffectKind as EntityEffectKind, Holder, MobEffect,
};
use simdnbt::{DeserializeError, borrow::NbtTag};

use crate::{
    position::Vec3, registry_holder::value::LevelBasedValue, resource_location::ResourceLocation,
    sound::CustomSound,
};

#[derive(Debug, Clone)]
pub enum EntityEffect {
    AllOf(Vec<EntityEffect>),
    ApplyMobEffect(ApplyMobEffect),
    ChangeItemDamage(ChangeItemDamage),
    DamageEntity(DamageEntity),
    Explode(Explode),
    Ignite(Ignite),
    PlaySound(PlaySound),
    ReplaceBlock(ReplaceBlock),
    ReplaceDisk(ReplaceDisk),
    RunFunction(RunFunction),
    SetBlockProperties(SetBlockProperties),
    SpawnParticles(SpawnParticles),
    SummonEntity(SummonEntity),
}

impl From<EntityEffectKind> for EntityEffect {
    fn from(kind: EntityEffectKind) -> Self {
        // this is mostly just here to make it so we get a compilation error whenever
        // new items are added to EntityEffectKind
        match kind {
            EntityEffectKind::AllOf => Self::AllOf(Default::default()),
            EntityEffectKind::ApplyMobEffect => Self::ApplyMobEffect(Default::default()),
            EntityEffectKind::ChangeItemDamage => Self::ChangeItemDamage(Default::default()),
            EntityEffectKind::DamageEntity => Self::DamageEntity(Default::default()),
            EntityEffectKind::Explode => Self::Explode(Default::default()),
            EntityEffectKind::Ignite => Self::Ignite(Default::default()),
            EntityEffectKind::PlaySound => Self::PlaySound(Default::default()),
            EntityEffectKind::ReplaceBlock => Self::ReplaceBlock(Default::default()),
            EntityEffectKind::ReplaceDisk => Self::ReplaceDisk(Default::default()),
            EntityEffectKind::RunFunction => Self::RunFunction(Default::default()),
            EntityEffectKind::SetBlockProperties => Self::SetBlockProperties(Default::default()),
            EntityEffectKind::SpawnParticles => Self::SpawnParticles(Default::default()),
            EntityEffectKind::SummonEntity => Self::SummonEntity(Default::default()),
        }
    }
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct ApplyMobEffect {
    /// IDs of mob effects.
    pub to_apply: HomogeneousList,
    pub min_duration: LevelBasedValue,
    pub max_duration: LevelBasedValue,
    pub min_amplifier: LevelBasedValue,
    pub max_amplifier: LevelBasedValue,
}

// TODO: in vanilla this is just a HolderSetCodec using a RegistryFixedCodec,
// azalea registries should probably be refactored first tho
#[derive(Debug, Clone, Default)]
pub struct HomogeneousList {
    pub ids: Vec<ResourceLocation>,
}
impl simdnbt::FromNbtTag for HomogeneousList {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        if let Some(string) = tag.string() {
            return Some(Self {
                ids: vec![ResourceLocation::new(string.to_str())],
            });
        }
        if let Some(list) = tag.list() {
            if let Some(strings) = list.strings() {
                return Some(Self {
                    ids: strings
                        .iter()
                        .map(|&s| ResourceLocation::new(s.to_str()))
                        .collect(),
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct ChangeItemDamage {
    pub amount: LevelBasedValue,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct DamageEntity {
    pub min_damage: LevelBasedValue,
    pub max_damage: LevelBasedValue,
    // TODO: convert to a DamageKind after azalea-registry refactor
    pub damage_kind: ResourceLocation,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct Explode {
    pub attribute_to_user: bool,
    // TODO: convert to a DamageKind after azalea-registry refactor
    pub damage_kind: ResourceLocation,
    pub knockback_multiplier: LevelBasedValue,
    pub immune_blocks: HomogeneousList,
    pub offset: Vec3,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct Ignite {
    pub duration: LevelBasedValue,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct ApplyEntityImpulse {
    pub direction: Vec3,
    pub coordinate_scale: Vec3,
    pub magnitude: LevelBasedValue,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct ApplyExhaustion {
    pub amount: LevelBasedValue,
}

#[derive(Debug, Clone, Default, simdnbt::Deserialize)]
pub struct PlaySound {
    pub sound: Holder<SoundEvent, CustomSound>,
}
