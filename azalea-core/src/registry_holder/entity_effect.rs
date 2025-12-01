use std::collections::HashMap;

use azalea_registry::{
    EnchantmentEntityEffectKind as EntityEffectKind, GameEvent, Holder, ParticleKind, SoundEvent,
};
use simdnbt::{
    Deserialize, DeserializeError,
    borrow::{NbtCompound, NbtTag},
};

use crate::{
    identifier::Identifier,
    position::{Vec3, Vec3i},
    registry_holder::{
        block_predicate::BlockPredicate, block_state_provider::BlockStateProvider,
        float_provider::FloatProvider, get_in_compound, value::LevelBasedValue,
    },
    sound::CustomSound,
};

#[derive(Debug, Clone)]
pub enum EntityEffect {
    AllOf(AllOf),
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

impl Deserialize for EntityEffect {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        match kind {
            EntityEffectKind::AllOf => Deserialize::from_compound(nbt).map(Self::AllOf),
            EntityEffectKind::ApplyMobEffect => {
                Deserialize::from_compound(nbt).map(Self::ApplyMobEffect)
            }
            EntityEffectKind::ChangeItemDamage => {
                Deserialize::from_compound(nbt).map(Self::ChangeItemDamage)
            }
            EntityEffectKind::DamageEntity => {
                Deserialize::from_compound(nbt).map(Self::DamageEntity)
            }
            EntityEffectKind::Explode => Deserialize::from_compound(nbt).map(Self::Explode),
            EntityEffectKind::Ignite => Deserialize::from_compound(nbt).map(Self::Ignite),
            EntityEffectKind::PlaySound => Deserialize::from_compound(nbt).map(Self::PlaySound),
            EntityEffectKind::ReplaceBlock => {
                Deserialize::from_compound(nbt).map(Self::ReplaceBlock)
            }
            EntityEffectKind::ReplaceDisk => Deserialize::from_compound(nbt).map(Self::ReplaceDisk),
            EntityEffectKind::RunFunction => Deserialize::from_compound(nbt).map(Self::RunFunction),
            EntityEffectKind::SetBlockProperties => {
                Deserialize::from_compound(nbt).map(Self::SetBlockProperties)
            }
            EntityEffectKind::SpawnParticles => {
                Deserialize::from_compound(nbt).map(Self::SpawnParticles)
            }
            EntityEffectKind::SummonEntity => {
                Deserialize::from_compound(nbt).map(Self::SummonEntity)
            }
        }
    }
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct AllOf {
    pub effects: Vec<EntityEffect>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
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
    pub ids: Vec<Identifier>,
}
impl simdnbt::FromNbtTag for HomogeneousList {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        if let Some(string) = tag.string() {
            return Some(Self {
                ids: vec![Identifier::new(string.to_str())],
            });
        }
        if let Some(list) = tag.list()
            && let Some(strings) = list.strings()
        {
            return Some(Self {
                ids: strings
                    .iter()
                    .map(|&s| Identifier::new(s.to_str()))
                    .collect(),
            });
        }
        None
    }
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ChangeItemDamage {
    pub amount: LevelBasedValue,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct DamageEntity {
    pub min_damage: LevelBasedValue,
    pub max_damage: LevelBasedValue,
    // TODO: convert to a DamageKind after azalea-registry refactor
    #[simdnbt(rename = "damage_type")]
    pub damage_kind: Identifier,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct Explode {
    pub attribute_to_user: Option<bool>,
    // TODO: convert to a DamageKind after azalea-registry refactor
    #[simdnbt(rename = "damage_type")]
    pub damage_kind: Option<Identifier>,
    pub knockback_multiplier: Option<LevelBasedValue>,
    pub immune_blocks: Option<HomogeneousList>,
    pub offset: Option<Vec3>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct Ignite {
    pub duration: LevelBasedValue,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ApplyEntityImpulse {
    pub direction: Vec3,
    pub coordinate_scale: Vec3,
    pub magnitude: LevelBasedValue,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ApplyExhaustion {
    pub amount: LevelBasedValue,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct PlaySound {
    pub sound: Holder<SoundEvent, CustomSound>,
    pub volume: FloatProvider,
    pub pitch: FloatProvider,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ReplaceBlock {
    pub offset: Option<Vec3i>,
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ReplaceDisk {
    pub radius: LevelBasedValue,
    pub height: LevelBasedValue,
    pub offset: Option<Vec3i>,
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct RunFunction {
    pub function: Identifier,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct SetBlockProperties {
    pub properties: HashMap<String, String>,
    pub offset: Option<Vec3i>,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct SpawnParticles {
    pub particle: ParticleKindCodec,
    pub horizontal_position: SpawnParticlesPosition,
    pub vertical_position: SpawnParticlesPosition,
    pub horizontal_velocity: SpawnParticlesVelocity,
    pub vertical_velocity: SpawnParticlesVelocity,
    pub speed: Option<FloatProvider>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct ParticleKindCodec {
    #[simdnbt(rename = "type")]
    pub kind: ParticleKind,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct SpawnParticlesPosition {
    #[simdnbt(rename = "type")]
    pub kind: Identifier,
    pub offset: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct SpawnParticlesVelocity {
    pub movement_scale: Option<f32>,
    pub base: Option<FloatProvider>,
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct SummonEntity {
    pub entity: HomogeneousList,
    pub join_team: Option<bool>,
}
