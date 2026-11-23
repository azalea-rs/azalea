use azalea_buf::AzBuf;
use azalea_codec::Codec;
use azalea_registry::{
    HolderSet,
    builtin::{ConsumeEffectKind, MobEffect, SoundEvent},
};

use crate::components::MobEffectInstance;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type")]
#[codec(default_snake_case_key)]
pub enum ConsumeEffect {
    ApplyEffects {
        #[codec(optional)]
        effects: Vec<MobEffectInstance>,
        #[codec(optional)]
        probability: f32,
    },
    RemoveEffects {
        #[codec(optional)]
        effects: HolderSet<MobEffect>,
    },
    ClearAllEffects,
    TeleportRandomly {
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
