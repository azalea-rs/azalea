use azalea_buf::AzBuf;
use azalea_codec::{Codec, types::StringLiteral};
use azalea_core::codec_utils::is_default;
use azalea_registry::{
    HolderSet,
    builtin::{ConsumeEffectKind, MobEffect, SoundEvent},
    identifier::Identifier,
};
use serde::Serialize;

use crate::components::MobEffectInstance;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case", tag = "type")]
#[codec(tag = "type")]
#[codec(default_snake_case_key)]
pub enum ConsumeEffect {
    ApplyEffects {
        #[serde(skip_serializing_if = "is_default")]
        #[codec(optional)]
        effects: Vec<MobEffectInstance>,
        #[serde(skip_serializing_if = "is_default")]
        #[codec(optional)]
        probability: f32,
    },
    RemoveEffects {
        #[serde(skip_serializing_if = "is_default")]
        #[codec(optional)]
        effects: HolderSet<MobEffect, Identifier>,
    },
    ClearAllEffects,
    TeleportRandomly {
        #[serde(skip_serializing_if = "is_default")]
        #[codec(optional)]
        diameter: f32,
    },
    PlaySound {
        sound: SoundEvent,
    },
}

impl From<ConsumeEffect> for ConsumeEffectKind {
    fn from(effect: ConsumeEffect) -> Self {
        match effect {
            ConsumeEffect::ApplyEffects { .. } => ConsumeEffectKind::ApplyEffects,
            ConsumeEffect::RemoveEffects { .. } => ConsumeEffectKind::RemoveEffects,
            ConsumeEffect::ClearAllEffects => ConsumeEffectKind::ClearAllEffects,
            ConsumeEffect::TeleportRandomly { .. } => ConsumeEffectKind::TeleportRandomly,
            ConsumeEffect::PlaySound { .. } => ConsumeEffectKind::PlaySound,
        }
    }
}
