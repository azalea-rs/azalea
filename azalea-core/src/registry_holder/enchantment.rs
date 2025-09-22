use azalea_chat::text_component::TextComponent;
use azalea_registry::EnchantmentEffectComponentKind;
use indexmap::IndexMap;

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    pub effects: IndexMap<EnchantmentEffectComponentKind, EnchantmentEffectComponent>,
}

#[derive(Debug, Clone)]
pub enum EnchantmentEffectComponent {
    Set {
        value: LevelBasedValue,
    },
    Add {
        value: LevelBasedValue,
    },
    Multiply {
        factor: LevelBasedValue,
    },
    RemoveBinomial {
        chance: LevelBasedValue,
    },
    AllOf {
        effects: Vec<EnchantmentEffectComponent>,
    },
}

#[derive(Debug, Clone)]
pub enum LevelBasedValue {
    Constant(f64),
    Linear {
        base: f64,
        per_level_above_first: f64,
    },
    LevelSquared {
        added: f64,
    },
    Clamped {
        value: Box<LevelBasedValue>,
        min: f64,
        max: f64,
    },
    Fraction {
        numerator: Box<LevelBasedValue>,
        denominator: Box<LevelBasedValue>,
    },
    Lookup {
        values: Vec<f64>,
        fallback: Box<LevelBasedValue>,
    },
}
