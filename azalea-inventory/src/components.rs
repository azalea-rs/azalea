use core::f64;
use std::{any::Any, collections::HashMap, io::Cursor};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::{position::GlobalPos, resource_location::ResourceLocation};
use azalea_registry::{
    Attribute, Block, ConsumeEffectKind, DataComponentKind, Enchantment, EntityKind, HolderSet,
    Item, MobEffect, Potion, SoundEvent, TrimMaterial, TrimPattern,
};
use simdnbt::owned::{Nbt, NbtCompound};
use uuid::Uuid;

use crate::ItemStack;

pub trait DataComponent: Send + Sync + Any {
    const KIND: DataComponentKind;
}

pub trait EncodableDataComponent: Send + Sync + Any {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error>;
    // using the Clone trait makes it not be object-safe, so we have our own clone
    // function instead
    fn clone(&self) -> Box<dyn EncodableDataComponent>;
    // same deal here
    fn eq(&self, other: Box<dyn EncodableDataComponent>) -> bool;
}

impl<T> EncodableDataComponent for T
where
    T: DataComponent + Clone + AzaleaWrite + AzaleaRead + PartialEq,
{
    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.azalea_write(buf)
    }
    fn clone(&self) -> Box<dyn EncodableDataComponent> {
        let cloned = self.clone();
        Box::new(cloned)
    }
    fn eq(&self, other: Box<dyn EncodableDataComponent>) -> bool {
        let other_any: Box<dyn Any> = other;
        if let Some(other) = other_any.downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

pub fn from_kind(
    kind: azalea_registry::DataComponentKind,
    buf: &mut Cursor<&[u8]>,
) -> Result<Box<dyn EncodableDataComponent>, BufReadError> {
    // if this is causing a compile-time error, look at DataComponents.java in the
    // decompiled vanilla code to see how to implement new components

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
        DataComponentKind::HideAdditionalTooltip => {
            Box::new(HideAdditionalTooltip::azalea_read(buf)?)
        }
        DataComponentKind::HideTooltip => Box::new(HideTooltip::azalea_read(buf)?),
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
    })
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CustomData {
    pub nbt: Nbt,
}
impl DataComponent for CustomData {
    const KIND: DataComponentKind = DataComponentKind::CustomData;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}
impl DataComponent for MaxStackSize {
    const KIND: DataComponentKind = DataComponentKind::MaxStackSize;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}
impl DataComponent for MaxDamage {
    const KIND: DataComponentKind = DataComponentKind::MaxDamage;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

impl DataComponent for Damage {
    const KIND: DataComponentKind = DataComponentKind::Damage;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Unbreakable {
    pub show_in_tooltip: bool,
}
impl DataComponent for Unbreakable {
    const KIND: DataComponentKind = DataComponentKind::Unbreakable;
}
impl Default for Unbreakable {
    fn default() -> Self {
        Self {
            show_in_tooltip: true,
        }
    }
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CustomName {
    pub name: FormattedText,
}
impl DataComponent for CustomName {
    const KIND: DataComponentKind = DataComponentKind::CustomName;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ItemName {
    pub name: FormattedText,
}
impl DataComponent for ItemName {
    const KIND: DataComponentKind = DataComponentKind::ItemName;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}
impl DataComponent for Lore {
    const KIND: DataComponentKind = DataComponentKind::Lore;
}

#[derive(Clone, PartialEq, Copy, AzBuf)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}
impl DataComponent for Rarity {
    const KIND: DataComponentKind = DataComponentKind::Rarity;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Enchantments {
    #[var]
    pub levels: HashMap<Enchantment, u32>,
    pub show_in_tooltip: bool,
}
impl DataComponent for Enchantments {
    const KIND: DataComponentKind = DataComponentKind::Enchantments;
}

#[derive(Clone, PartialEq, AzBuf)]
pub enum BlockStateValueMatcher {
    Exact {
        value: String,
    },
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BlockStatePropertyMatcher {
    pub name: String,
    pub value_matcher: BlockStateValueMatcher,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BlockPredicate {
    pub blocks: Option<HolderSet<Block, ResourceLocation>>,
    pub properties: Option<Vec<BlockStatePropertyMatcher>>,
    pub nbt: Option<NbtCompound>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct AdventureModePredicate {
    pub predicates: Vec<BlockPredicate>,
    pub show_in_tooltip: bool,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CanPlaceOn {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanPlaceOn {
    const KIND: DataComponentKind = DataComponentKind::CanPlaceOn;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanBreak {
    const KIND: DataComponentKind = DataComponentKind::CanBreak;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
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

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
}

// this is duplicated in azalea-entity, BUT the one there has a different
// protocol format (and we can't use it anyways because it would cause a
// circular dependency)
#[derive(Clone, PartialEq, AzBuf)]
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub name: String,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct AttributeModifiersEntry {
    pub attribute: Attribute,
    pub modifier: AttributeModifier,
    pub slot: EquipmentSlotGroup,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct AttributeModifiers {
    pub modifiers: Vec<AttributeModifiersEntry>,
    pub show_in_tooltip: bool,
}
impl DataComponent for AttributeModifiers {
    const KIND: DataComponentKind = DataComponentKind::AttributeModifiers;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CustomModelData {
    #[var]
    pub value: i32,
}
impl DataComponent for CustomModelData {
    const KIND: DataComponentKind = DataComponentKind::CustomModelData;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct HideAdditionalTooltip;
impl DataComponent for HideAdditionalTooltip {
    const KIND: DataComponentKind = DataComponentKind::HideAdditionalTooltip;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct HideTooltip;
impl DataComponent for HideTooltip {
    const KIND: DataComponentKind = DataComponentKind::HideTooltip;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}
impl DataComponent for RepairCost {
    const KIND: DataComponentKind = DataComponentKind::RepairCost;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct CreativeSlotLock;
impl DataComponent for CreativeSlotLock {
    const KIND: DataComponentKind = DataComponentKind::CreativeSlotLock;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}
impl DataComponent for EnchantmentGlintOverride {
    const KIND: DataComponentKind = DataComponentKind::EnchantmentGlintOverride;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct IntangibleProjectile;
impl DataComponent for IntangibleProjectile {
    const KIND: DataComponentKind = DataComponentKind::IntangibleProjectile;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MobEffectDetails {
    #[var]
    pub amplifier: i32,
    #[var]
    pub duration: i32,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub hidden_effect: Option<Box<MobEffectDetails>>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MobEffectInstance {
    pub effect: MobEffect,
    pub details: MobEffectDetails,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PossibleEffect {
    pub effect: MobEffectInstance,
    pub probability: f32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Food {
    #[var]
    pub nutrition: i32,
    pub saturation: f32,
    pub can_always_eat: bool,
    pub eat_seconds: f32,
    pub effects: Vec<PossibleEffect>,
}
impl DataComponent for Food {
    const KIND: DataComponentKind = DataComponentKind::Food;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ToolRule {
    pub blocks: HolderSet<Block, ResourceLocation>,
    pub speed: Option<f32>,
    pub correct_for_drops: Option<bool>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Tool {
    pub rules: Vec<ToolRule>,
    pub default_mining_speed: f32,
    #[var]
    pub damage_per_block: i32,
}
impl DataComponent for Tool {
    const KIND: DataComponentKind = DataComponentKind::Tool;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct StoredEnchantments {
    #[var]
    pub enchantments: HashMap<Enchantment, i32>,
    pub show_in_tooltip: bool,
}
impl DataComponent for StoredEnchantments {
    const KIND: DataComponentKind = DataComponentKind::StoredEnchantments;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DyedColor {
    pub rgb: i32,
    pub show_in_tooltip: bool,
}
impl DataComponent for DyedColor {
    const KIND: DataComponentKind = DataComponentKind::DyedColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MapColor {
    pub color: i32,
}
impl DataComponent for MapColor {
    const KIND: DataComponentKind = DataComponentKind::MapColor;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MapId {
    #[var]
    pub id: i32,
}
impl DataComponent for MapId {
    const KIND: DataComponentKind = DataComponentKind::MapId;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}
impl DataComponent for MapDecorations {
    const KIND: DataComponentKind = DataComponentKind::MapDecorations;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}
impl DataComponent for MapPostProcessing {
    const KIND: DataComponentKind = DataComponentKind::MapPostProcessing;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemStack>,
}
impl DataComponent for ChargedProjectiles {
    const KIND: DataComponentKind = DataComponentKind::ChargedProjectiles;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BundleContents {
    pub items: Vec<ItemStack>,
}
impl DataComponent for BundleContents {
    const KIND: DataComponentKind = DataComponentKind::BundleContents;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct PotionContents {
    pub potion: Option<Potion>,
    pub custom_color: Option<i32>,
    pub custom_effects: Vec<MobEffectInstance>,
}
impl DataComponent for PotionContents {
    const KIND: DataComponentKind = DataComponentKind::PotionContents;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct SuspiciousStewEffect {
    pub effect: MobEffect,
    #[var]
    pub duration: i32,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct SuspiciousStewEffects {
    pub effects: Vec<SuspiciousStewEffect>,
}
impl DataComponent for SuspiciousStewEffects {
    const KIND: DataComponentKind = DataComponentKind::SuspiciousStewEffects;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct WritableBookContent {
    pub pages: Vec<String>,
}
impl DataComponent for WritableBookContent {
    const KIND: DataComponentKind = DataComponentKind::WritableBookContent;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct WrittenBookContent {
    pub title: String,
    pub author: String,
    #[var]
    pub generation: i32,
    pub pages: Vec<FormattedText>,
    pub resolved: bool,
}
impl DataComponent for WrittenBookContent {
    const KIND: DataComponentKind = DataComponentKind::WrittenBookContent;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
    pub show_in_tooltip: bool,
}
impl DataComponent for Trim {
    const KIND: DataComponentKind = DataComponentKind::Trim;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}
impl DataComponent for DebugStickState {
    const KIND: DataComponentKind = DataComponentKind::DebugStickState;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct EntityData {
    pub entity: NbtCompound,
}
impl DataComponent for EntityData {
    const KIND: DataComponentKind = DataComponentKind::EntityData;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BucketEntityData {
    const KIND: DataComponentKind = DataComponentKind::BucketEntityData;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct BlockEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BlockEntityData {
    const KIND: DataComponentKind = DataComponentKind::BlockEntityData;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Instrument {
    pub instrument: azalea_registry::Instrument,
}
impl DataComponent for Instrument {
    const KIND: DataComponentKind = DataComponentKind::Instrument;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct OminousBottleAmplifier {
    #[var]
    pub amplifier: i32,
}
impl DataComponent for OminousBottleAmplifier {
    const KIND: DataComponentKind = DataComponentKind::OminousBottleAmplifier;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Recipes {
    pub recipes: Vec<ResourceLocation>,
}
impl DataComponent for Recipes {
    const KIND: DataComponentKind = DataComponentKind::Recipes;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct LodestoneTracker {
    pub target: Option<GlobalPos>,
    pub tracked: bool,
}
impl DataComponent for LodestoneTracker {
    const KIND: DataComponentKind = DataComponentKind::LodestoneTracker;
}

#[derive(Clone, Copy, PartialEq, AzBuf)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[derive(Clone, PartialEq, AzBuf)]
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
    pub explosions: Vec<FireworkExplosion>,
}
impl DataComponent for Fireworks {
    const KIND: DataComponentKind = DataComponentKind::Fireworks;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Profile {
    pub name: String,
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
pub struct JukeboxPlayable {
    pub song: azalea_registry::JukeboxSong,
    pub show_in_tooltip: bool,
}
impl DataComponent for JukeboxPlayable {
    const KIND: DataComponentKind = DataComponentKind::JukeboxPlayable;
}

#[derive(Clone, PartialEq, AzBuf)]
pub struct Consumable {
    pub consume_seconds: f32,
    pub animation: ItemUseAnimation,
    pub sound: SoundEvent,
    pub has_consume_particles: bool,
    pub on_consuime_effects: Vec<ConsumeEffectKind>,
}
impl DataComponent for Consumable {
    const KIND: DataComponentKind = DataComponentKind::Consumable;
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
    //   TagKey.streamCodec(Registries.DAMAGE_TYPE), DamageResistant::types, DamageResistant::new
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
    pub model: Option<ResourceLocation>,
    pub allowed_entities: HolderSet<EntityKind, ResourceLocation>,
}
impl DataComponent for Equippable {
    const KIND: DataComponentKind = DataComponentKind::Equippable;
}

#[derive(Clone, Copy, Debug, PartialEq, AzBuf)]
pub enum EquipmentSlot {
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
    pub death_effects: Vec<ConsumeEffectKind>,
}
impl DataComponent for DeathProtection {
    const KIND: DataComponentKind = DataComponentKind::DeathProtection;
}
