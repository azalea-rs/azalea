use std::collections::HashMap;

use azalea_codec::{Codec, types::CompactListCodec};
use azalea_core::{
    identifier::Identifier,
    position::{Vec3, Vec3i},
    sound::CustomSound,
};
use azalea_registry::{
    Holder,
    builtin::{GameEvent, ParticleKind, SoundEvent},
    data::DamageKindKey,
};

use crate::registry_holder::{
    block_predicate::BlockPredicate, block_state_provider::BlockStateProvider,
    float_provider::FloatProvider, value::LevelBasedValue,
};

#[derive(Clone, Debug, Codec)]
#[codec(tag = "type")]
#[codec(default_snake_case_key)]
pub enum EntityEffect {
    AllOf(AllOf),
    ApplyMobEffect(ApplyMobEffect),
    ChangeItemDamage(ChangeItemDamage),
    DamageEntity(DamageEntity),
    Explode(Explode),
    Ignite(Ignite),
    ApplyImpulse(ApplyEntityImpulse),
    ApplyExhaustion(ApplyExhaustion),
    PlaySound(PlaySound),
    ReplaceBlock(ReplaceBlock),
    ReplaceDisk(ReplaceDisk),
    RunFunction(RunFunction),
    SetBlockProperties(SetBlockProperties),
    SpawnParticles(SpawnParticles),
    SummonEntity(SummonEntity),
}

#[derive(Clone, Debug, Codec)]
pub struct AllOf {
    pub effects: Vec<EntityEffect>,
}

#[derive(Clone, Debug, Codec)]
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
#[derive(Clone, Debug, Default, Codec)]
pub struct HomogeneousList {
    #[codec(with = CompactListCodec::<_>)]
    pub ids: Vec<Identifier>,
}

#[derive(Clone, Debug, Codec)]
pub struct ChangeItemDamage {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug, Codec)]
pub struct DamageEntity {
    pub min_damage: LevelBasedValue,
    pub max_damage: LevelBasedValue,
    #[codec(rename = "damage_type")]
    pub damage_kind: DamageKindKey,
}

#[derive(Clone, Debug, Codec)]
pub struct Explode {
    #[codec(optional)]
    pub attribute_to_user: Option<bool>,
    #[codec(rename = "damage_type")]
    #[codec(optional)]
    pub damage_kind: Option<DamageKindKey>,
    #[codec(optional)]
    pub knockback_multiplier: Option<LevelBasedValue>,
    #[codec(optional)]
    pub immune_blocks: Option<HomogeneousList>,
    #[codec(optional)]
    pub offset: Option<Vec3>,
}

#[derive(Clone, Debug, Codec)]
pub struct Ignite {
    pub duration: LevelBasedValue,
}

#[derive(Clone, Debug, Codec)]
pub struct ApplyEntityImpulse {
    pub direction: Vec3,
    pub coordinate_scale: Vec3,
    pub magnitude: LevelBasedValue,
}

#[derive(Clone, Debug, Codec)]
pub struct ApplyExhaustion {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug, Codec)]
pub struct PlaySound {
    #[codec(with = CompactListCodec::<_>)]
    pub sound: Vec<Holder<SoundEvent, CustomSound>>,
    pub volume: FloatProvider,
    pub pitch: FloatProvider,
}

#[derive(Clone, Debug, Codec)]
pub struct ReplaceBlock {
    #[codec(optional)]
    pub offset: Option<Vec3i>,
    #[codec(optional)]
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, Codec)]
pub struct ReplaceDisk {
    pub radius: LevelBasedValue,
    pub height: LevelBasedValue,
    #[codec(optional)]
    pub offset: Option<Vec3i>,
    #[codec(optional)]
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, Codec)]
pub struct RunFunction {
    pub function: Identifier,
}

#[derive(Clone, Debug, Codec)]
pub struct SetBlockProperties {
    pub properties: HashMap<String, String>,
    #[codec(optional)]
    pub offset: Option<Vec3i>,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, Codec)]
pub struct SpawnParticles {
    pub particle: ParticleKindCodec,
    pub horizontal_position: SpawnParticlesPosition,
    pub vertical_position: SpawnParticlesPosition,
    pub horizontal_velocity: SpawnParticlesVelocity,
    pub vertical_velocity: SpawnParticlesVelocity,
    pub speed: Option<FloatProvider>,
}

#[derive(Clone, Debug, Codec)]
pub struct ParticleKindCodec {
    #[codec(rename = "type")]
    pub kind: ParticleKind,
}

#[derive(Clone, Debug, Codec)]
pub struct SpawnParticlesPosition {
    #[codec(rename = "type")]
    pub kind: Identifier,
    #[codec(optional)]
    pub offset: Option<f32>,
    #[codec(optional)]
    pub scale: Option<f32>,
}

#[derive(Clone, Debug, Codec)]
pub struct SpawnParticlesVelocity {
    pub movement_scale: Option<f32>,
    #[codec(optional)]
    pub base: Option<FloatProvider>,
}

#[derive(Clone, Debug, Codec)]
pub struct SummonEntity {
    pub entity: HomogeneousList,
    #[codec(optional)]
    pub join_team: Option<bool>,
}
