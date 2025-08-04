use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_registry::{HolderSet, MobEffect, SoundEvent};

use crate::components::MobEffectInstance;

#[derive(Clone, PartialEq, AzBuf)]
pub enum ConsumeEffect {
    ApplyEffects {
        effects: Vec<MobEffectInstance>,
        probability: f32,
    },
    RemoveEffects {
        effects: HolderSet<MobEffect, ResourceLocation>,
    },
    ClearAllEffects,
    TeleportRandomly {
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
