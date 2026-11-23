use std::collections::HashMap;

use azalea_codec::{Codec, types::CompactListCodec};
use azalea_core::{
    identifier::Identifier,
    position::{Vec3, Vec3i},
    sound::CustomSound,
};

use crate::{
    Holder, HolderSet, builtin::{BlockKind, EntityKind, GameEvent, MobEffect, ParticleKind, SoundEvent}, data::{DamageKind, DamageKindKey}, registry_holder::{
        block_predicate::BlockPredicate, block_state_provider::BlockStateProvider, float_provider::FloatProvider, particle::Particle, value::LevelBasedValue,
    },
};

#[derive(Clone, Debug, PartialEq, Codec)]
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

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct AllOf {
    pub effects: Vec<EntityEffect>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ApplyMobEffect {
    /// IDs of mob effects.
    pub to_apply: HolderSet<MobEffect>,
    pub min_duration: LevelBasedValue,
    pub max_duration: LevelBasedValue,
    pub min_amplifier: LevelBasedValue,
    pub max_amplifier: LevelBasedValue,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ChangeItemDamage {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct DamageEntity {
    pub min_damage: LevelBasedValue,
    pub max_damage: LevelBasedValue,
    #[codec(rename = "damage_type")]
    pub damage_kind: DamageKindKey,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct Explode {
    #[codec(optional)]
    pub attribute_to_user: bool,
    #[codec(rename = "damage_type")]
    #[codec(optional)]
    pub damage_kind: Option<DamageKind>,
    #[codec(optional)]
    pub knockback_multiplier: LevelBasedValue,
    #[codec(optional)]
    pub immune_blocks: HolderSet<BlockKind>,
    #[codec(optional)]
    pub offset: Vec3,
    pub radius: LevelBasedValue,
    #[codec(optional)]
    pub create_fire: bool,
    pub block_interaction: ExplosionInteraction,
    #[codec(optional)]
    pub small_particle: ParticleKindCodec,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct Ignite {
    pub duration: LevelBasedValue,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ApplyEntityImpulse {
    pub direction: Vec3,
    pub coordinate_scale: Vec3,
    pub magnitude: LevelBasedValue,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ApplyExhaustion {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct PlaySound {
    #[codec(with = CompactListCodec::<_>)]
    pub sound: Vec<Holder<SoundEvent, CustomSound>>,
    pub volume: FloatProvider,
    pub pitch: FloatProvider,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ReplaceBlock {
    #[codec(optional)]
    pub offset: Vec3i,
    #[codec(optional)]
    pub predicate: BlockPredicate,
    pub block_state: BlockStateProvider,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ReplaceDisk {
    pub radius: LevelBasedValue,
    pub height: LevelBasedValue,
    #[codec(optional)]
    pub offset: Vec3i,
    #[codec(optional)]
    pub predicate: BlockPredicate,
    pub block_state: BlockStateProvider,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct RunFunction {
    pub function: Identifier,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SetBlockProperties {
    pub properties: HashMap<String, String>,
    #[codec(optional)]
    pub offset: Option<Vec3i>,
    #[codec(optional)]
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SpawnParticles {
    pub particle: Particle,
    pub horizontal_position: SpawnParticlesPosition,
    pub vertical_position: SpawnParticlesPosition,
    pub horizontal_velocity: SpawnParticlesVelocity,
    pub vertical_velocity: SpawnParticlesVelocity,
    pub speed: Option<FloatProvider>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ParticleKindCodec {
    #[codec(rename = "type")]
    pub kind: ParticleKind,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SpawnParticlesPosition {
    #[codec(rename = "type")]
    pub kind: Identifier,
    #[codec(optional)]
    pub offset: Option<f32>,
    #[codec(optional)]
    pub scale: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SpawnParticlesVelocity {
    pub movement_scale: Option<f32>,
    #[codec(optional)]
    pub base: Option<FloatProvider>,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SummonEntity {
    pub entity: HolderSet<EntityKind>,
    #[codec(optional)]
    pub join_team: Option<bool>,
}
