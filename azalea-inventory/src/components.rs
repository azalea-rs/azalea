use core::f64;
use std::{
    any::Any,
    collections::HashMap,
    io::{self, Cursor},
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::{
    codec_utils::*, filterable::Filterable, position::GlobalPos,
    resource_location::ResourceLocation, sound::CustomSound,
};
use azalea_registry::{
    self as registry, Attribute, Block, DamageKind, DataComponentKind, Enchantment, EntityKind,
    Holder, HolderSet, Item, MobEffect, Potion, SoundEvent, TrimMaterial, TrimPattern,
};
use serde::Serialize;
use simdnbt::owned::{Nbt, NbtCompound};
use tracing::trace;
use uuid::Uuid;

use crate::{ItemStack, item::consume_effect::ConsumeEffect};

pub trait DataComponent: Send + Sync + Any + Clone + Serialize {
    const KIND: DataComponentKind;
}

pub trait EncodableDataComponent: Send + Sync + Any {
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()>;
    fn crc_hash(&self) -> u32;
    // using the Clone trait makes it not be object-safe, so we have our own clone
    // function instead
    fn clone(&self) -> Box<dyn EncodableDataComponent>;
    // same thing here
    fn eq(&self, other: Box<dyn EncodableDataComponent>) -> bool;
}

impl<T> EncodableDataComponent for T
where
    T: DataComponent + Clone + AzaleaWrite + AzaleaRead + PartialEq,
{
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        self.azalea_write(buf)
    }
    fn crc_hash(&self) -> u32 {
        todo!()
    }
    fn clone(&self) -> Box<dyn EncodableDataComponent> {
        let cloned = self.clone();
        Box::new(cloned)
    }
    fn eq(&self, other: Box<dyn EncodableDataComponent>) -> bool {
        let other_any: Box<dyn Any> = other;
        match other_any.downcast_ref::<T>() {
            Some(other) => self == other,
            _ => false,
        }
    }
}

pub fn from_kind(
    kind: registry::DataComponentKind,
    buf: &mut Cursor<&[u8]>,
) -> Result<Box<dyn EncodableDataComponent>, BufReadError> {
    // if this is causing a compile-time error, look at DataComponents.java in the
    // decompiled vanilla code to see how to implement new components

    trace!("Reading data component {kind}");

    // note that this match statement is updated by genitemcomponents.py
    Ok(match kind {
        DataComponentKind::CustomData => Box::new(CustomData::azalea_read(buf)?),
        DataComponentKind::MaxStackSize => Box::new(MaxStackSize::azalea_read(buf)?),
        DataComponentKind::MaxDamage => Box::new(MaxDamage::azalea_read(buf)?),
        DataComponentKind::Damage => Box::new(Damage::azalea_read(buf)?),
        DataComponentKind::Unbreakable => Box::new(Unbreakable::azalea_read(buf)?),
        DataComponentKind::CustomName => Box::new(CustomName::azalea_read(buf)?),
        DataComponentKind::ItemName => Box::new(ItemName::azalea_read(buf)?),
        DataComponentKind::Lore => Box::new(Lore::azalea_read(buf)?),
        DataComponentKind::Rarity => Box::new(Rarity::azalea_read(buf)?),
        DataComponentKind::Enchantments => Box::new(Enchantments::azalea_read(buf)?),
        DataComponentKind::CanPlaceOn => Box::new(CanPlaceOn::azalea_read(buf)?),
        DataComponentKind::CanBreak => Box::new(CanBreak::azalea_read(buf)?),
        DataComponentKind::AttributeModifiers => Box::new(AttributeModifiers::azalea_read(buf)?),
        DataComponentKind::CustomModelData => Box::new(CustomModelData::azalea_read(buf)?),
        DataComponentKind::RepairCost => Box::new(RepairCost::azalea_read(buf)?),
        DataComponentKind::CreativeSlotLock => Box::new(CreativeSlotLock::azalea_read(buf)?),
        DataComponentKind::EnchantmentGlintOverride => {
            Box::new(EnchantmentGlintOverride::azalea_read(buf)?)
        }
        DataComponentKind::IntangibleProjectile => {
            Box::new(IntangibleProjectile::azalea_read(buf)?)
        }
        DataComponentKind::Food => Box::new(Food::azalea_read(buf)?),
        DataComponentKind::Tool => Box::new(Tool::azalea_read(buf)?),
        DataComponentKind::StoredEnchantments => Box::new(StoredEnchantments::azalea_read(buf)?),
        DataComponentKind::DyedColor => Box::new(DyedColor::azalea_read(buf)?),
        DataComponentKind::MapColor => Box::new(MapColor::azalea_read(buf)?),
        DataComponentKind::MapId => Box::new(MapId::azalea_read(buf)?),
        DataComponentKind::MapDecorations => Box::new(MapDecorations::azalea_read(buf)?),
        DataComponentKind::MapPostProcessing => Box::new(MapPostProcessing::azalea_read(buf)?),
        DataComponentKind::ChargedProjectiles => Box::new(ChargedProjectiles::azalea_read(buf)?),
        DataComponentKind::BundleContents => Box::new(BundleContents::azalea_read(buf)?),
        DataComponentKind::PotionContents => Box::new(PotionContents::azalea_read(buf)?),
        DataComponentKind::SuspiciousStewEffects => {
            Box::new(SuspiciousStewEffects::azalea_read(buf)?)
        }
        DataComponentKind::WritableBookContent => Box::new(WritableBookContent::azalea_read(buf)?),
        DataComponentKind::WrittenBookContent => Box::new(WrittenBookContent::azalea_read(buf)?),
        DataComponentKind::Trim => Box::new(Trim::azalea_read(buf)?),
        DataComponentKind::DebugStickState => Box::new(DebugStickState::azalea_read(buf)?),
        DataComponentKind::EntityData => Box::new(EntityData::azalea_read(buf)?),
        DataComponentKind::BucketEntityData => Box::new(BucketEntityData::azalea_read(buf)?),
        DataComponentKind::BlockEntityData => Box::new(BlockEntityData::azalea_read(buf)?),
        DataComponentKind::Instrument => Box::new(Instrument::azalea_read(buf)?),
        DataComponentKind::OminousBottleAmplifier => {
            Box::new(OminousBottleAmplifier::azalea_read(buf)?)
        }
        DataComponentKind::Recipes => Box::new(Recipes::azalea_read(buf)?),
        DataComponentKind::LodestoneTracker => Box::new(LodestoneTracker::azalea_read(buf)?),
        DataComponentKind::FireworkExplosion => Box::new(FireworkExplosion::azalea_read(buf)?),
        DataComponentKind::Fireworks => Box::new(Fireworks::azalea_read(buf)?),
        DataComponentKind::Profile => Box::new(Profile::azalea_read(buf)?),
        DataComponentKind::NoteBlockSound => Box::new(NoteBlockSound::azalea_read(buf)?),
        DataComponentKind::BannerPatterns => Box::new(BannerPatterns::azalea_read(buf)?),
        DataComponentKind::BaseColor => Box::new(BaseColor::azalea_read(buf)?),
        DataComponentKind::PotDecorations => Box::new(PotDecorations::azalea_read(buf)?),
        DataComponentKind::Container => Box::new(Container::azalea_read(buf)?),
        DataComponentKind::BlockState => Box::new(BlockState::azalea_read(buf)?),
        DataComponentKind::Bees => Box::new(Bees::azalea_read(buf)?),
        DataComponentKind::Lock => Box::new(Lock::azalea_read(buf)?),
        DataComponentKind::ContainerLoot => Box::new(ContainerLoot::azalea_read(buf)?),
        DataComponentKind::JukeboxPlayable => Box::new(JukeboxPlayable::azalea_read(buf)?),
        DataComponentKind::Consumable => Box::new(Consumable::azalea_read(buf)?),
        DataComponentKind::UseRemainder => Box::new(UseRemainder::azalea_read(buf)?),
        DataComponentKind::UseCooldown => Box::new(UseCooldown::azalea_read(buf)?),
        DataComponentKind::Enchantable => Box::new(Enchantable::azalea_read(buf)?),
        DataComponentKind::Repairable => Box::new(Repairable::azalea_read(buf)?),
        DataComponentKind::ItemModel => Box::new(ItemModel::azalea_read(buf)?),
        DataComponentKind::DamageResistant => Box::new(DamageResistant::azalea_read(buf)?),
        DataComponentKind::Equippable => Box::new(Equippable::azalea_read(buf)?),
        DataComponentKind::Glider => Box::new(Glider::azalea_read(buf)?),
        DataComponentKind::TooltipStyle => Box::new(TooltipStyle::azalea_read(buf)?),
        DataComponentKind::DeathProtection => Box::new(DeathProtection::azalea_read(buf)?),
        DataComponentKind::Weapon => Box::new(Weapon::azalea_read(buf)?),
        DataComponentKind::PotionDurationScale => Box::new(PotionDurationScale::azalea_read(buf)?),
        DataComponentKind::VillagerVariant => Box::new(VillagerVariant::azalea_read(buf)?),
        DataComponentKind::WolfVariant => Box::new(WolfVariant::azalea_read(buf)?),
        DataComponentKind::WolfCollar => Box::new(WolfCollar::azalea_read(buf)?),
        DataComponentKind::FoxVariant => Box::new(FoxVariant::azalea_read(buf)?),
        DataComponentKind::SalmonSize => Box::new(SalmonSize::azalea_read(buf)?),
        DataComponentKind::ParrotVariant => Box::new(ParrotVariant::azalea_read(buf)?),
        DataComponentKind::TropicalFishPattern => Box::new(TropicalFishPattern::azalea_read(buf)?),
        DataComponentKind::TropicalFishBaseColor => {
            Box::new(TropicalFishBaseColor::azalea_read(buf)?)
        }
        DataComponentKind::TropicalFishPatternColor => {
            Box::new(TropicalFishPatternColor::azalea_read(buf)?)
        }
        DataComponentKind::MooshroomVariant => Box::new(MooshroomVariant::azalea_read(buf)?),
        DataComponentKind::RabbitVariant => Box::new(RabbitVariant::azalea_read(buf)?),
        DataComponentKind::PigVariant => Box::new(PigVariant::azalea_read(buf)?),
        DataComponentKind::FrogVariant => Box::new(FrogVariant::azalea_read(buf)?),
        DataComponentKind::HorseVariant => Box::new(HorseVariant::azalea_read(buf)?),
        DataComponentKind::PaintingVariant => Box::new(PaintingVariant::azalea_read(buf)?),
        DataComponentKind::LlamaVariant => Box::new(LlamaVariant::azalea_read(buf)?),
        DataComponentKind::AxolotlVariant => Box::new(AxolotlVariant::azalea_read(buf)?),
        DataComponentKind::CatVariant => Box::new(CatVariant::azalea_read(buf)?),
        DataComponentKind::CatCollar => Box::new(CatCollar::azalea_read(buf)?),
        DataComponentKind::SheepColor => Box::new(SheepColor::azalea_read(buf)?),
        DataComponentKind::ShulkerColor => Box::new(ShulkerColor::azalea_read(buf)?),
        DataComponentKind::TooltipDisplay => Box::new(TooltipDisplay::azalea_read(buf)?),
        DataComponentKind::BlocksAttacks => Box::new(BlocksAttacks::azalea_read(buf)?),
        DataComponentKind::ProvidesTrimMaterial => {
            Box::new(ProvidesTrimMaterial::azalea_read(buf)?)
        }
        DataComponentKind::ProvidesBannerPatterns => {
            Box::new(ProvidesBannerPatterns::azalea_read(buf)?)
        }
        DataComponentKind::BreakSound => Box::new(BreakSound::azalea_read(buf)?),
        DataComponentKind::WolfSoundVariant => Box::new(WolfSoundVariant::azalea_read(buf)?),
        DataComponentKind::CowVariant => Box::new(CowVariant::azalea_read(buf)?),
        DataComponentKind::ChickenVariant => Box::new(ChickenVariant::azalea_read(buf)?),
    })
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CustomData {
    pub nbt: Nbt,
}
impl DataComponent for CustomData {
    const KIND: DataComponentKind = DataComponentKind::CustomData;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}
impl DataComponent for MaxStackSize {
    const KIND: DataComponentKind = DataComponentKind::MaxStackSize;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}
impl DataComponent for MaxDamage {
    const KIND: DataComponentKind = DataComponentKind::MaxDamage;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

impl DataComponent for Damage {
    const KIND: DataComponentKind = DataComponentKind::Damage;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Unbreakable;
impl DataComponent for Unbreakable {
    const KIND: DataComponentKind = DataComponentKind::Unbreakable;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CustomName {
    pub name: FormattedText,
}
impl DataComponent for CustomName {
    const KIND: DataComponentKind = DataComponentKind::CustomName;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ItemName {
    pub name: FormattedText,
}
impl DataComponent for ItemName {
    const KIND: DataComponentKind = DataComponentKind::ItemName;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}
impl DataComponent for Lore {
    const KIND: DataComponentKind = DataComponentKind::Lore;
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}
impl DataComponent for Rarity {
    const KIND: DataComponentKind = DataComponentKind::Rarity;
}

#[derive(Clone, PartialEq, AzBuf, Serialize)]
#[serde(transparent)]
pub struct Enchantments {
    #[var]
    pub levels: HashMap<Enchantment, u32>,
}
impl DataComponent for Enchantments {
    const KIND: DataComponentKind = DataComponentKind::Enchantments;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub enum BlockStateValueMatcher {
    Exact {
        value: String,
    },
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BlockStatePropertyMatcher {
    pub name: String,
    pub value_matcher: BlockStateValueMatcher,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BlockPredicate {
    #[serde(skip_serializing_if = "is_default")]
    pub blocks: Option<HolderSet<Block, ResourceLocation>>,
    #[serde(skip_serializing_if = "is_default")]
    pub properties: Option<Vec<BlockStatePropertyMatcher>>,
    #[serde(skip_serializing_if = "is_default")]
    pub nbt: Option<NbtCompound>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct AdventureModePredicate {
    #[serde(serialize_with = "flatten_array")]
    pub predicates: Vec<BlockPredicate>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CanPlaceOn {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanPlaceOn {
    const KIND: DataComponentKind = DataComponentKind::CanPlaceOn;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanBreak {
    const KIND: DataComponentKind = DataComponentKind::CanBreak;
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentSlotGroup {
    Any,
    Mainhand,
    Offhand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body,
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

// this is duplicated in azalea-entity, BUT the one there has a different
// protocol format (and we can't use it anyways because it would cause a
// circular dependency)
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct AttributeModifier {
    pub id: ResourceLocation,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct AttributeModifiersEntry {
    #[serde(rename = "type")]
    pub kind: Attribute,
    #[serde(flatten)]
    pub modifier: AttributeModifier,
    pub slot: EquipmentSlotGroup,
    #[serde(skip_serializing_if = "is_default")]
    pub display: AttributeModifierDisplay,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct AttributeModifiers {
    pub modifiers: Vec<AttributeModifiersEntry>,
}
impl DataComponent for AttributeModifiers {
    const KIND: DataComponentKind = DataComponentKind::AttributeModifiers;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeModifierDisplay {
    #[default]
    Default,
    Hidden,
    Override {
        text: FormattedText,
    },
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct CustomModelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub floats: Vec<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub colors: Vec<i32>,
}
impl DataComponent for CustomModelData {
    const KIND: DataComponentKind = DataComponentKind::CustomModelData;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}
impl DataComponent for RepairCost {
    const KIND: DataComponentKind = DataComponentKind::RepairCost;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct CreativeSlotLock;
impl DataComponent for CreativeSlotLock {
    const KIND: DataComponentKind = DataComponentKind::CreativeSlotLock;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}
impl DataComponent for EnchantmentGlintOverride {
    const KIND: DataComponentKind = DataComponentKind::EnchantmentGlintOverride;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct IntangibleProjectile;
impl DataComponent for IntangibleProjectile {
    const KIND: DataComponentKind = DataComponentKind::IntangibleProjectile;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct MobEffectDetails {
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub amplifier: i32,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub duration: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub ambient: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub show_particles: bool,
    pub show_icon: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub hidden_effect: Option<Box<MobEffectDetails>>,
}
impl MobEffectDetails {
    pub const fn new() -> Self {
        MobEffectDetails {
            amplifier: 0,
            duration: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
            hidden_effect: None,
        }
    }
}
impl Default for MobEffectDetails {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct MobEffectInstance {
    pub id: MobEffect,
    #[serde(flatten)]
    pub details: MobEffectDetails,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct PossibleEffect {
    pub effect: MobEffectInstance,
    pub probability: f32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Serialize)]
pub struct Food {
    #[var]
    pub nutrition: i32,
    pub saturation: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub can_always_eat: bool,
}
impl DataComponent for Food {
    const KIND: DataComponentKind = DataComponentKind::Food;
}
impl Food {
    pub const fn new() -> Self {
        Food {
            nutrition: 0,
            saturation: 0.,
            can_always_eat: false,
        }
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct ToolRule {
    pub blocks: HolderSet<Block, ResourceLocation>,
    #[serde(skip_serializing_if = "is_default")]
    pub speed: Option<f32>,
    #[serde(skip_serializing_if = "is_default")]
    pub correct_for_drops: Option<bool>,
}
impl ToolRule {
    pub const fn new() -> Self {
        ToolRule {
            blocks: HolderSet::Direct { contents: vec![] },
            speed: None,
            correct_for_drops: None,
        }
    }
}
impl Default for ToolRule {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Tool {
    #[serde(serialize_with = "flatten_array")]
    pub rules: Vec<ToolRule>,
    #[serde(skip_serializing_if = "is_default")]
    pub default_mining_speed: f32,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub damage_per_block: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub can_destroy_blocks_in_creative: bool,
}
impl DataComponent for Tool {
    const KIND: DataComponentKind = DataComponentKind::Tool;
}
impl Tool {
    pub const fn new() -> Self {
        Tool {
            rules: vec![],
            default_mining_speed: 1.,
            damage_per_block: 1,
            can_destroy_blocks_in_creative: true,
        }
    }
}
impl Default for Tool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct StoredEnchantments {
    #[var]
    pub enchantments: HashMap<Enchantment, i32>,
}
impl DataComponent for StoredEnchantments {
    const KIND: DataComponentKind = DataComponentKind::StoredEnchantments;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct DyedColor {
    pub rgb: i32,
}
impl DataComponent for DyedColor {
    const KIND: DataComponentKind = DataComponentKind::DyedColor;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapColor {
    pub color: i32,
}
impl DataComponent for MapColor {
    const KIND: DataComponentKind = DataComponentKind::MapColor;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapId {
    #[var]
    pub id: i32,
}
impl DataComponent for MapId {
    const KIND: DataComponentKind = DataComponentKind::MapId;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}
impl DataComponent for MapDecorations {
    const KIND: DataComponentKind = DataComponentKind::MapDecorations;
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}
impl DataComponent for MapPostProcessing {
    const KIND: DataComponentKind = DataComponentKind::MapPostProcessing;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemStack>,
}
impl DataComponent for ChargedProjectiles {
    const KIND: DataComponentKind = DataComponentKind::ChargedProjectiles;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BundleContents {
    pub items: Vec<ItemStack>,
}
impl DataComponent for BundleContents {
    const KIND: DataComponentKind = DataComponentKind::BundleContents;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct PotionContents {
    #[serde(skip_serializing_if = "is_default")]
    pub potion: Option<Potion>,
    #[serde(skip_serializing_if = "is_default")]
    pub custom_color: Option<i32>,
    #[serde(skip_serializing_if = "is_default")]
    pub custom_effects: Vec<MobEffectInstance>,
    #[serde(skip_serializing_if = "is_default")]
    pub custom_name: Option<String>,
}
impl DataComponent for PotionContents {
    const KIND: DataComponentKind = DataComponentKind::PotionContents;
}
impl PotionContents {
    pub const fn new() -> Self {
        PotionContents {
            potion: None,
            custom_color: None,
            custom_effects: vec![],
            custom_name: None,
        }
    }
}
impl Default for PotionContents {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct SuspiciousStewEffect {
    #[serde(rename = "id")]
    pub effect: MobEffect,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub duration: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct SuspiciousStewEffects {
    pub effects: Vec<SuspiciousStewEffect>,
}
impl DataComponent for SuspiciousStewEffects {
    const KIND: DataComponentKind = DataComponentKind::SuspiciousStewEffects;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct WritableBookContent {
    pub pages: Vec<String>,
}
impl DataComponent for WritableBookContent {
    const KIND: DataComponentKind = DataComponentKind::WritableBookContent;
}

#[derive(Clone, PartialEq, AzBuf, Serialize)]
pub struct WrittenBookContent {
    #[limit(32)]
    pub title: Filterable<String>,
    pub author: String,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub generation: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub pages: Vec<Filterable<FormattedText>>,
    #[serde(skip_serializing_if = "is_default")]
    pub resolved: bool,
}

impl DataComponent for WrittenBookContent {
    const KIND: DataComponentKind = DataComponentKind::WrittenBookContent;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
}
impl DataComponent for Trim {
    const KIND: DataComponentKind = DataComponentKind::Trim;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}
impl DataComponent for DebugStickState {
    const KIND: DataComponentKind = DataComponentKind::DebugStickState;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct EntityData {
    pub entity: NbtCompound,
}
impl DataComponent for EntityData {
    const KIND: DataComponentKind = DataComponentKind::EntityData;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BucketEntityData {
    const KIND: DataComponentKind = DataComponentKind::BucketEntityData;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BlockEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BlockEntityData {
    const KIND: DataComponentKind = DataComponentKind::BlockEntityData;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(untagged)]
pub enum Instrument {
    Registry(registry::Instrument),
    Holder(Holder<registry::Instrument, InstrumentData>),
}
impl DataComponent for Instrument {
    const KIND: DataComponentKind = DataComponentKind::Instrument;
}

#[derive(Clone, PartialEq, Debug, AzBuf, Serialize)]
pub struct InstrumentData {
    pub sound_event: Holder<SoundEvent, azalea_core::sound::CustomSound>,
    pub use_duration: f32,
    pub range: f32,
    pub description: FormattedText,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct OminousBottleAmplifier {
    #[var]
    pub amplifier: i32,
}
impl DataComponent for OminousBottleAmplifier {
    const KIND: DataComponentKind = DataComponentKind::OminousBottleAmplifier;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Recipes {
    pub recipes: Vec<ResourceLocation>,
}
impl DataComponent for Recipes {
    const KIND: DataComponentKind = DataComponentKind::Recipes;
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct LodestoneTracker {
    #[serde(skip_serializing_if = "is_default")]
    pub target: Option<GlobalPos>,
    #[serde(skip_serializing_if = "is_true")]
    pub tracked: bool,
}
impl DataComponent for LodestoneTracker {
    const KIND: DataComponentKind = DataComponentKind::LodestoneTracker;
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    pub colors: Vec<i32>,
    pub fade_colors: Vec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}
impl DataComponent for FireworkExplosion {
    const KIND: DataComponentKind = DataComponentKind::FireworkExplosion;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Fireworks {
    #[var]
    pub flight_duration: i32,
    #[limit(256)]
    pub explosions: Vec<FireworkExplosion>,
}
impl DataComponent for Fireworks {
    const KIND: DataComponentKind = DataComponentKind::Fireworks;
}
impl Fireworks {
    pub const fn new() -> Self {
        Fireworks {
            flight_duration: 0,
            explosions: vec![],
        }
    }
}
impl Default for Fireworks {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Profile {
    pub name: Option<String>,
    pub id: Option<Uuid>,
    pub properties: Vec<GameProfileProperty>,
}
impl DataComponent for Profile {
    const KIND: DataComponentKind = DataComponentKind::Profile;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct NoteBlockSound {
    pub sound: ResourceLocation,
}
impl DataComponent for NoteBlockSound {
    const KIND: DataComponentKind = DataComponentKind::NoteBlockSound;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BannerPattern {
    #[var]
    pub pattern: i32,
    #[var]
    pub color: i32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BannerPatterns {
    pub patterns: Vec<BannerPattern>,
}
impl DataComponent for BannerPatterns {
    const KIND: DataComponentKind = DataComponentKind::BannerPatterns;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BaseColor {
    pub color: DyeColor,
}
impl DataComponent for BaseColor {
    const KIND: DataComponentKind = DataComponentKind::BaseColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PotDecorations {
    pub items: Vec<Item>,
}
impl DataComponent for PotDecorations {
    const KIND: DataComponentKind = DataComponentKind::PotDecorations;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Container {
    pub items: Vec<ItemStack>,
}
impl DataComponent for Container {
    const KIND: DataComponentKind = DataComponentKind::Container;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BlockState {
    pub properties: HashMap<String, String>,
}
impl DataComponent for BlockState {
    const KIND: DataComponentKind = DataComponentKind::BlockState;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BeehiveOccupant {
    pub entity_data: NbtCompound,
    #[var]
    pub ticks_in_hive: i32,
    #[var]
    pub min_ticks_in_hive: i32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Bees {
    pub occupants: Vec<BeehiveOccupant>,
}
impl DataComponent for Bees {
    const KIND: DataComponentKind = DataComponentKind::Bees;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Lock {
    pub key: String,
}
impl DataComponent for Lock {
    const KIND: DataComponentKind = DataComponentKind::Lock;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ContainerLoot {
    pub loot: NbtCompound,
}
impl DataComponent for ContainerLoot {
    const KIND: DataComponentKind = DataComponentKind::ContainerLoot;
}

#[derive(Clone, PartialEq, AzBuf)]
pub enum JukeboxPlayable {
    Referenced(ResourceLocation),
    Direct(Holder<registry::JukeboxSong, JukeboxSongData>),
}
impl DataComponent for JukeboxPlayable {
    const KIND: DataComponentKind = DataComponentKind::JukeboxPlayable;
}
#[derive(Clone, PartialEq, AzBuf)]
pub struct JukeboxSongData {
    pub sound_event: Holder<SoundEvent, CustomSound>,
    pub description: FormattedText,
    pub length_in_seconds: f32,
    #[var]
    pub comparator_output: i32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Consumable {
    pub consume_seconds: f32,
    pub animation: ItemUseAnimation,
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
    pub has_consume_particles: bool,
    pub on_consume_effects: Vec<ConsumeEffect>,
}
impl DataComponent for Consumable {
    const KIND: DataComponentKind = DataComponentKind::Consumable;
}
impl Consumable {
    pub const fn new() -> Self {
        Self {
            consume_seconds: 1.6,
            animation: ItemUseAnimation::Eat,
            sound: azalea_registry::Holder::Reference(SoundEvent::EntityGenericEat),
            has_consume_particles: true,
            on_consume_effects: Vec::new(),
        }
    }
}
impl Default for Consumable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum ItemUseAnimation {
    None,
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct UseRemainder {
    pub convert_into: ItemStack,
}
impl DataComponent for UseRemainder {
    const KIND: DataComponentKind = DataComponentKind::UseRemainder;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct UseCooldown {
    pub seconds: f32,
    pub cooldown_group: Option<ResourceLocation>,
}
impl DataComponent for UseCooldown {
    const KIND: DataComponentKind = DataComponentKind::UseCooldown;
}
impl UseCooldown {
    pub const fn new() -> Self {
        Self {
            seconds: 0.,
            cooldown_group: None,
        }
    }
}
impl Default for UseCooldown {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Enchantable {
    #[var]
    pub value: u32,
}
impl DataComponent for Enchantable {
    const KIND: DataComponentKind = DataComponentKind::Enchantable;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Repairable {
    pub items: HolderSet<Item, ResourceLocation>,
}
impl DataComponent for Repairable {
    const KIND: DataComponentKind = DataComponentKind::Repairable;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ItemModel {
    pub resource_location: ResourceLocation,
}
impl DataComponent for ItemModel {
    const KIND: DataComponentKind = DataComponentKind::ItemModel;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DamageResistant {
    // in the vanilla code this is
    // ```
    // StreamCodec.composite(
    //     TagKey.streamCodec(Registries.DAMAGE_TYPE),
    //     DamageResistant::types,
    //     DamageResistant::new,
    // );
    // ```
    // i'm not entirely sure if this is meant to be a vec or something, i just made it a
    // resourcelocation for now
    pub types: ResourceLocation,
}
impl DataComponent for DamageResistant {
    const KIND: DataComponentKind = DataComponentKind::DamageResistant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Equippable {
    pub slot: EquipmentSlot,
    pub equip_sound: SoundEvent,
    pub asset_id: Option<ResourceLocation>,
    pub camera_overlay: Option<ResourceLocation>,
    pub allowed_entities: Option<HolderSet<EntityKind, ResourceLocation>>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
    pub equip_on_interact: bool,
    pub can_be_sheared: bool,
    pub shearing_sound: SoundEvent,
}
impl DataComponent for Equippable {
    const KIND: DataComponentKind = DataComponentKind::Equippable;
}
impl Equippable {
    pub const fn new() -> Self {
        Self {
            slot: EquipmentSlot::Body,
            equip_sound: SoundEvent::ItemArmorEquipGeneric,
            asset_id: None,
            camera_overlay: None,
            allowed_entities: None,
            dispensable: true,
            swappable: true,
            damage_on_hurt: true,
            equip_on_interact: false,
            can_be_sheared: false,
            shearing_sound: SoundEvent::ItemShearsSnip,
        }
    }
}
impl Default for Equippable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, AzBuf)]
pub enum EquipmentSlot {
    Mainhand,
    Offhand,
    Feet,
    Legs,
    Chest,
    Head,
    Body,
    Saddle,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Glider;
impl DataComponent for Glider {
    const KIND: DataComponentKind = DataComponentKind::Glider;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct TooltipStyle {
    pub resource_location: ResourceLocation,
}
impl DataComponent for TooltipStyle {
    const KIND: DataComponentKind = DataComponentKind::TooltipStyle;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DeathProtection {
    pub death_effects: Vec<ConsumeEffect>,
}
impl DataComponent for DeathProtection {
    const KIND: DataComponentKind = DataComponentKind::DeathProtection;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Weapon {
    #[var]
    pub item_damage_per_attack: i32,
    pub disable_blocking_for_seconds: f32,
}
impl DataComponent for Weapon {
    const KIND: DataComponentKind = DataComponentKind::Weapon;
}
impl Weapon {
    pub const fn new() -> Self {
        Self {
            item_damage_per_attack: 1,
            disable_blocking_for_seconds: 0.,
        }
    }
}
impl Default for Weapon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PotionDurationScale {
    pub value: f32,
}
impl DataComponent for PotionDurationScale {
    const KIND: DataComponentKind = DataComponentKind::PotionDurationScale;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct VillagerVariant {
    pub variant: registry::VillagerKind,
}
impl DataComponent for VillagerVariant {
    const KIND: DataComponentKind = DataComponentKind::VillagerVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct WolfVariant {
    pub variant: registry::WolfVariant,
}
impl DataComponent for WolfVariant {
    const KIND: DataComponentKind = DataComponentKind::WolfVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct WolfCollar {
    pub color: DyeColor,
}
impl DataComponent for WolfCollar {
    const KIND: DataComponentKind = DataComponentKind::WolfCollar;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct FoxVariant {
    pub variant: registry::FoxVariant,
}
impl DataComponent for FoxVariant {
    const KIND: DataComponentKind = DataComponentKind::FoxVariant;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum SalmonSize {
    Small,
    Medium,
    Large,
}
impl DataComponent for SalmonSize {
    const KIND: DataComponentKind = DataComponentKind::SalmonSize;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ParrotVariant {
    pub variant: registry::ParrotVariant,
}
impl DataComponent for ParrotVariant {
    const KIND: DataComponentKind = DataComponentKind::ParrotVariant;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum TropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
}
impl DataComponent for TropicalFishPattern {
    const KIND: DataComponentKind = DataComponentKind::TropicalFishPattern;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct TropicalFishBaseColor {
    pub color: DyeColor,
}
impl DataComponent for TropicalFishBaseColor {
    const KIND: DataComponentKind = DataComponentKind::TropicalFishBaseColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct TropicalFishPatternColor {
    pub color: DyeColor,
}
impl DataComponent for TropicalFishPatternColor {
    const KIND: DataComponentKind = DataComponentKind::TropicalFishPatternColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MooshroomVariant {
    pub variant: registry::MooshroomVariant,
}
impl DataComponent for MooshroomVariant {
    const KIND: DataComponentKind = DataComponentKind::MooshroomVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct RabbitVariant {
    pub variant: registry::RabbitVariant,
}
impl DataComponent for RabbitVariant {
    const KIND: DataComponentKind = DataComponentKind::RabbitVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PigVariant {
    pub variant: registry::PigVariant,
}
impl DataComponent for PigVariant {
    const KIND: DataComponentKind = DataComponentKind::PigVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct FrogVariant {
    pub variant: registry::FrogVariant,
}
impl DataComponent for FrogVariant {
    const KIND: DataComponentKind = DataComponentKind::FrogVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct HorseVariant {
    pub variant: registry::HorseVariant,
}
impl DataComponent for HorseVariant {
    const KIND: DataComponentKind = DataComponentKind::HorseVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PaintingVariant {
    pub variant: Holder<registry::PaintingVariant, PaintingVariantData>,
}
impl DataComponent for PaintingVariant {
    const KIND: DataComponentKind = DataComponentKind::PaintingVariant;
}
#[derive(Clone, PartialEq, AzBuf)]
pub struct PaintingVariantData {
    #[var]
    pub width: i32,
    #[var]
    pub height: i32,
    pub asset_id: ResourceLocation,
    pub title: Option<FormattedText>,
    pub author: Option<FormattedText>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct LlamaVariant {
    pub variant: registry::LlamaVariant,
}
impl DataComponent for LlamaVariant {
    const KIND: DataComponentKind = DataComponentKind::LlamaVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct AxolotlVariant {
    pub variant: registry::AxolotlVariant,
}
impl DataComponent for AxolotlVariant {
    const KIND: DataComponentKind = DataComponentKind::AxolotlVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CatVariant {
    pub variant: registry::CatVariant,
}
impl DataComponent for CatVariant {
    const KIND: DataComponentKind = DataComponentKind::CatVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CatCollar {
    pub color: DyeColor,
}
impl DataComponent for CatCollar {
    const KIND: DataComponentKind = DataComponentKind::CatCollar;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct SheepColor {
    pub color: DyeColor,
}
impl DataComponent for SheepColor {
    const KIND: DataComponentKind = DataComponentKind::SheepColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ShulkerColor {
    pub color: DyeColor,
}
impl DataComponent for ShulkerColor {
    const KIND: DataComponentKind = DataComponentKind::ShulkerColor;
}

#[derive(Clone, PartialEq, AzBuf, Default)]
pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    pub hidden_components: Vec<DataComponentKind>,
}
impl DataComponent for TooltipDisplay {
    const KIND: DataComponentKind = DataComponentKind::TooltipDisplay;
}
impl TooltipDisplay {
    pub const fn new() -> Self {
        Self {
            hide_tooltip: false,
            hidden_components: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BlocksAttacks {
    pub block_delay_seconds: f32,
    pub disable_cooldown_scale: f32,
    pub damage_reductions: Vec<DamageReduction>,
    pub item_damage: ItemDamageFunction,
    pub bypassed_by: Option<ResourceLocation>,
    pub block_sound: Option<azalea_registry::Holder<SoundEvent, CustomSound>>,
    pub disabled_sound: Option<azalea_registry::Holder<SoundEvent, CustomSound>>,
}
impl DataComponent for BlocksAttacks {
    const KIND: DataComponentKind = DataComponentKind::BlocksAttacks;
}
impl BlocksAttacks {
    pub fn new() -> Self {
        Self {
            block_delay_seconds: 0.,
            disable_cooldown_scale: 1.,
            damage_reductions: vec![DamageReduction {
                horizontal_blocking_angle: 90.,
                kind: None,
                base: 0.,
                factor: 1.,
            }],
            item_damage: ItemDamageFunction {
                threshold: 1.,
                base: 0.,
                factor: 1.,
            },
            bypassed_by: None,
            block_sound: None,
            disabled_sound: None,
        }
    }
}
impl Default for BlocksAttacks {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DamageReduction {
    pub horizontal_blocking_angle: f32,
    pub kind: Option<HolderSet<DamageKind, ResourceLocation>>,
    pub base: f32,
    pub factor: f32,
}
#[derive(Clone, PartialEq, AzBuf)]
pub struct ItemDamageFunction {
    pub threshold: f32,
    pub base: f32,
    pub factor: f32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub enum ProvidesTrimMaterial {
    Referenced(ResourceLocation),
    Direct(Holder<TrimMaterial, DirectTrimMaterial>),
}
impl DataComponent for ProvidesTrimMaterial {
    const KIND: DataComponentKind = DataComponentKind::ProvidesTrimMaterial;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DirectTrimMaterial {
    pub assets: MaterialAssetGroup,
    pub description: FormattedText,
}
#[derive(Clone, PartialEq, AzBuf)]
pub struct MaterialAssetGroup {
    pub base: AssetInfo,
    pub overrides: Vec<(ResourceLocation, AssetInfo)>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct AssetInfo {
    pub suffix: String,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ProvidesBannerPatterns {
    pub key: ResourceLocation,
}
impl DataComponent for ProvidesBannerPatterns {
    const KIND: DataComponentKind = DataComponentKind::ProvidesBannerPatterns;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BreakSound {
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
}
impl DataComponent for BreakSound {
    const KIND: DataComponentKind = DataComponentKind::BreakSound;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct WolfSoundVariant {
    pub variant: azalea_registry::WolfSoundVariant,
}
impl DataComponent for WolfSoundVariant {
    const KIND: DataComponentKind = DataComponentKind::WolfSoundVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CowVariant {
    pub variant: azalea_registry::CowVariant,
}
impl DataComponent for CowVariant {
    const KIND: DataComponentKind = DataComponentKind::CowVariant;
}

#[derive(Clone, PartialEq, AzBuf)]
pub enum ChickenVariant {
    Referenced(ResourceLocation),
    Direct(ChickenVariantData),
}
impl DataComponent for ChickenVariant {
    const KIND: DataComponentKind = DataComponentKind::ChickenVariant;
}
#[derive(Clone, PartialEq, AzBuf)]
pub struct ChickenVariantData {
    pub registry: azalea_registry::ChickenVariant,
}
