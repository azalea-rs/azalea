use azalea_buf::AzBuf;
use azalea_core::{codec_utils::is_default, identifier::Identifier};
use azalea_registry::{HolderSet, MobEffect, SoundEvent};
use serde::Serialize;

use crate::components::MobEffectInstance;

#[derive(Clone, PartialEq, Debug, AzBuf, Serialize)]
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

impl From<ConsumeEffect> for azalea_registry::ConsumeEffectKind {
    fn from(effect: ConsumeEffect) -> Self {
        match effect {
            ConsumeEffect::ApplyEffects { .. } => azalea_registry::ConsumeEffectKind::ApplyEffects,
            ConsumeEffect::RemoveEffects { .. } => {
                azalea_registry::ConsumeEffectKind::RemoveEffects
            }
            ConsumeEffect::ClearAllEffects => azalea_registry::ConsumeEffectKind::ClearAllEffects,
            ConsumeEffect::TeleportRandomly { .. } => {
                azalea_registry::ConsumeEffectKind::TeleportRandomly
            }
            ConsumeEffect::PlaySound { .. } => azalea_registry::ConsumeEffectKind::PlaySound,
        }
    }
}
