use std::{collections::HashMap, str::FromStr};

use azalea_registry::{
    Holder,
    builtin::{
        EnchantmentEntityEffectKind as EntityEffectKind, GameEvent, ParticleKind, SoundEvent,
    },
    data::DamageKindKey,
    identifier::Identifier,
};
use simdnbt::{
    Deserialize, DeserializeError,
    borrow::{NbtCompound, NbtTag},
};
use tracing::error;

use crate::{
    position::{Vec3, Vec3i},
    registry_holder::{
        block_predicate::BlockPredicate, block_state_provider::BlockStateProvider,
        float_provider::FloatProvider, get_in_compound, value::LevelBasedValue,
    },
    sound::CustomSound,
};

#[derive(Clone, Debug)]
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

impl Deserialize for EntityEffect {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        let res = match kind {
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
            EntityEffectKind::ApplyImpulse => {
                Deserialize::from_compound(nbt).map(Self::ApplyImpulse)
            }
            EntityEffectKind::ApplyExhaustion => {
                Deserialize::from_compound(nbt).map(Self::ApplyExhaustion)
            }
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
        };
        if res.is_err() {
            error!("Error deserializing EntityEffect {kind}: {nbt:?}");
        }
        res
    }
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct AllOf {
    pub effects: Vec<EntityEffect>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
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
#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ChangeItemDamage {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct DamageEntity {
    pub min_damage: LevelBasedValue,
    pub max_damage: LevelBasedValue,
    #[simdnbt(rename = "damage_type")]
    pub damage_kind: DamageKindKey,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct Explode {
    pub attribute_to_user: Option<bool>,
    #[simdnbt(rename = "damage_type")]
    pub damage_kind: Option<DamageKindKey>,
    pub knockback_multiplier: Option<LevelBasedValue>,
    pub immune_blocks: Option<HomogeneousList>,
    pub offset: Option<Vec3>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct Ignite {
    pub duration: LevelBasedValue,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ApplyEntityImpulse {
    pub direction: Vec3,
    pub coordinate_scale: Vec3,
    pub magnitude: LevelBasedValue,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ApplyExhaustion {
    pub amount: LevelBasedValue,
}

#[derive(Clone, Debug)]
pub struct PlaySound {
    // #[simdnbt(compact)]
    pub sound: Vec<Holder<SoundEvent, CustomSound>>,
    pub volume: FloatProvider,
    pub pitch: FloatProvider,
}

impl Deserialize for PlaySound {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let sound = if let Some(list) = nbt.list("sound") {
            // TODO: this will probably break in the future because it's only handling lists
            // of strings. you should refactor simdnbt to have an owned NbtTag enum that
            // contains the borrow types so this works for more than just
            // strings.
            list.strings()
                .ok_or(DeserializeError::MissingField)?
                .iter()
                .map(|s| {
                    SoundEvent::from_str(&s.to_str())
                        .map(Holder::Reference)
                        .ok()
                })
                .collect::<Option<_>>()
                .ok_or(DeserializeError::MissingField)?
        } else {
            vec![get_in_compound(&nbt, "sound")?]
        };

        let volume = get_in_compound(&nbt, "volume")?;
        let pitch = get_in_compound(&nbt, "pitch")?;

        Ok(Self {
            sound,
            volume,
            pitch,
        })
    }
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ReplaceBlock {
    pub offset: Option<Vec3i>,
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ReplaceDisk {
    pub radius: LevelBasedValue,
    pub height: LevelBasedValue,
    pub offset: Option<Vec3i>,
    pub predicate: Option<BlockPredicate>,
    pub block_state: BlockStateProvider,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct RunFunction {
    pub function: Identifier,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct SetBlockProperties {
    pub properties: HashMap<String, String>,
    pub offset: Option<Vec3i>,
    pub trigger_game_event: Option<GameEvent>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct SpawnParticles {
    pub particle: ParticleKindCodec,
    pub horizontal_position: SpawnParticlesPosition,
    pub vertical_position: SpawnParticlesPosition,
    pub horizontal_velocity: SpawnParticlesVelocity,
    pub vertical_velocity: SpawnParticlesVelocity,
    pub speed: Option<FloatProvider>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct ParticleKindCodec {
    #[simdnbt(rename = "type")]
    pub kind: ParticleKind,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct SpawnParticlesPosition {
    #[simdnbt(rename = "type")]
    pub kind: Identifier,
    pub offset: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct SpawnParticlesVelocity {
    pub movement_scale: Option<f32>,
    pub base: Option<FloatProvider>,
}

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct SummonEntity {
    pub entity: HomogeneousList,
    pub join_team: Option<bool>,
}
