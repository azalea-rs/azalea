use azalea_buf::AzBuf;
use azalea_core::codec_utils::is_default;
use azalea_registry::{
    HolderSet,
    builtin::{ConsumeEffectKind, MobEffect, SoundEvent},
    identifier::Identifier,
};
use serde::Serialize;

use crate::components::MobEffectInstance;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConsumeEffect {
    ApplyEffects {
        #[serde(skip_serializing_if = "is_default")]
        effects: Vec<MobEffectInstance>,
        #[serde(skip_serializing_if = "is_default")]
        probability: f32,
    },
    RemoveEffects {
        #[serde(skip_serializing_if = "is_default")]
        effects: HolderSet<MobEffect, Identifier>,
    },
    ClearAllEffects,
    TeleportRandomly {
        #[serde(skip_serializing_if = "is_default")]
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
