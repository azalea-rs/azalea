use core::f64;
use std::{any::Any, collections::HashMap, io::Cursor};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_chat::FormattedText;
use azalea_core::{position::GlobalPos, resource_location::ResourceLocation};
use azalea_registry::{
    Attribute, Block, DataComponentKind, Enchantment, HolderSet, Item, MobEffect, Potion,
    TrimMaterial, TrimPattern,
};
use simdnbt::owned::{Nbt, NbtCompound};
use uuid::Uuid;

use crate::ItemSlot;

pub trait DataComponent: Send + Sync + Any {}

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
    T: DataComponent + Clone + McBufWritable + McBufReadable + PartialEq,
{
    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.write_into(buf)
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
    Ok(match kind {
        DataComponentKind::CustomData => Box::new(CustomData::read_from(buf)?),
        DataComponentKind::MaxStackSize => Box::new(MaxStackSize::read_from(buf)?),
        DataComponentKind::MaxDamage => Box::new(MaxDamage::read_from(buf)?),
        DataComponentKind::Damage => Box::new(Damage::read_from(buf)?),
        DataComponentKind::Unbreakable => Box::new(Unbreakable::read_from(buf)?),
        DataComponentKind::CustomName => Box::new(CustomName::read_from(buf)?),
        DataComponentKind::ItemName => Box::new(ItemName::read_from(buf)?),
        DataComponentKind::Lore => Box::new(Lore::read_from(buf)?),
        DataComponentKind::Rarity => Box::new(Rarity::read_from(buf)?),
        DataComponentKind::Enchantments => Box::new(Enchantments::read_from(buf)?),
        DataComponentKind::CanPlaceOn => Box::new(CanPlaceOn::read_from(buf)?),
        DataComponentKind::CanBreak => Box::new(CanBreak::read_from(buf)?),
        DataComponentKind::AttributeModifiers => Box::new(AttributeModifiers::read_from(buf)?),
        DataComponentKind::CustomModelData => Box::new(CustomModelData::read_from(buf)?),
        DataComponentKind::HideAdditionalTooltip => {
            Box::new(HideAdditionalTooltip::read_from(buf)?)
        }
        DataComponentKind::HideTooltip => Box::new(HideTooltip::read_from(buf)?),
        DataComponentKind::RepairCost => Box::new(RepairCost::read_from(buf)?),
        DataComponentKind::CreativeSlotLock => Box::new(CreativeSlotLock::read_from(buf)?),
        DataComponentKind::EnchantmentGlintOverride => {
            Box::new(EnchantmentGlintOverride::read_from(buf)?)
        }
        DataComponentKind::IntangibleProjectile => Box::new(IntangibleProjectile::read_from(buf)?),
        DataComponentKind::Food => Box::new(Food::read_from(buf)?),
        DataComponentKind::FireResistant => Box::new(FireResistant::read_from(buf)?),
        DataComponentKind::Tool => Box::new(Tool::read_from(buf)?),
        DataComponentKind::StoredEnchantments => Box::new(StoredEnchantments::read_from(buf)?),
        DataComponentKind::DyedColor => Box::new(DyedColor::read_from(buf)?),
        DataComponentKind::MapColor => Box::new(MapColor::read_from(buf)?),
        DataComponentKind::MapId => Box::new(MapId::read_from(buf)?),
        DataComponentKind::MapDecorations => Box::new(MapDecorations::read_from(buf)?),
        DataComponentKind::MapPostProcessing => Box::new(MapPostProcessing::read_from(buf)?),
        DataComponentKind::ChargedProjectiles => Box::new(ChargedProjectiles::read_from(buf)?),
        DataComponentKind::BundleContents => Box::new(BundleContents::read_from(buf)?),
        DataComponentKind::PotionContents => Box::new(PotionContents::read_from(buf)?),
        DataComponentKind::SuspiciousStewEffects => {
            Box::new(SuspiciousStewEffects::read_from(buf)?)
        }
        DataComponentKind::WritableBookContent => Box::new(WritableBookContent::read_from(buf)?),
        DataComponentKind::WrittenBookContent => Box::new(WrittenBookContent::read_from(buf)?),
        DataComponentKind::Trim => Box::new(Trim::read_from(buf)?),
        DataComponentKind::DebugStickState => Box::new(DebugStickState::read_from(buf)?),
        DataComponentKind::EntityData => Box::new(EntityData::read_from(buf)?),
        DataComponentKind::BucketEntityData => Box::new(BucketEntityData::read_from(buf)?),
        DataComponentKind::BlockEntityData => Box::new(BlockEntityData::read_from(buf)?),
        DataComponentKind::Instrument => Box::new(Instrument::read_from(buf)?),
        DataComponentKind::OminousBottleAmplifier => {
            Box::new(OminousBottleAmplifier::read_from(buf)?)
        }
        DataComponentKind::Recipes => Box::new(Recipes::read_from(buf)?),
        DataComponentKind::LodestoneTracker => Box::new(LodestoneTracker::read_from(buf)?),
        DataComponentKind::FireworkExplosion => Box::new(FireworkExplosion::read_from(buf)?),
        DataComponentKind::Fireworks => Box::new(Fireworks::read_from(buf)?),
        DataComponentKind::Profile => Box::new(Profile::read_from(buf)?),
        DataComponentKind::NoteBlockSound => Box::new(NoteBlockSound::read_from(buf)?),
        DataComponentKind::BannerPatterns => Box::new(BannerPatterns::read_from(buf)?),
        DataComponentKind::BaseColor => Box::new(BaseColor::read_from(buf)?),
        DataComponentKind::PotDecorations => Box::new(PotDecorations::read_from(buf)?),
        DataComponentKind::Container => Box::new(Container::read_from(buf)?),
        DataComponentKind::BlockState => Box::new(BlockState::read_from(buf)?),
        DataComponentKind::Bees => Box::new(Bees::read_from(buf)?),
        DataComponentKind::Lock => Box::new(Lock::read_from(buf)?),
        DataComponentKind::ContainerLoot => Box::new(ContainerLoot::read_from(buf)?),
        DataComponentKind::JukeboxPlayable => todo!(),
    })
}

#[derive(Clone, PartialEq, McBuf)]
pub struct CustomData {
    pub nbt: Nbt,
}
impl DataComponent for CustomData {}

#[derive(Clone, PartialEq, McBuf)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}
impl DataComponent for MaxStackSize {}

#[derive(Clone, PartialEq, McBuf)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}
impl DataComponent for MaxDamage {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

impl DataComponent for Damage {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Unbreakable {
    pub show_in_tooltip: bool,
}
impl DataComponent for Unbreakable {}
impl Default for Unbreakable {
    fn default() -> Self {
        Self {
            show_in_tooltip: true,
        }
    }
}

#[derive(Clone, PartialEq, McBuf)]
pub struct CustomName {
    pub name: FormattedText,
}
impl DataComponent for CustomName {}

#[derive(Clone, PartialEq, McBuf)]
pub struct ItemName {
    pub name: FormattedText,
}
impl DataComponent for ItemName {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}
impl DataComponent for Lore {}

#[derive(Clone, PartialEq, Copy, McBuf)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}
impl DataComponent for Rarity {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Enchantments {
    #[var]
    pub levels: HashMap<Enchantment, u32>,
    pub show_in_tooltip: bool,
}
impl DataComponent for Enchantments {}

#[derive(Clone, PartialEq, McBuf)]
pub enum BlockStateValueMatcher {
    Exact {
        value: String,
    },
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

#[derive(Clone, PartialEq, McBuf)]
pub struct BlockStatePropertyMatcher {
    pub name: String,
    pub value_matcher: BlockStateValueMatcher,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct BlockPredicate {
    pub blocks: Option<HolderSet<Block, ResourceLocation>>,
    pub properties: Option<Vec<BlockStatePropertyMatcher>>,
    pub nbt: Option<NbtCompound>,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct AdventureModePredicate {
    pub predicates: Vec<BlockPredicate>,
    pub show_in_tooltip: bool,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct CanPlaceOn {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanPlaceOn {}

#[derive(Clone, PartialEq, McBuf)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
}
impl DataComponent for CanBreak {}

#[derive(Clone, Copy, PartialEq, McBuf)]
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

#[derive(Clone, Copy, PartialEq, McBuf)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
}

// this is duplicated in azalea-entity, BUT the one there has a different
// protocol format (and we can't use it anyways because it would cause a
// circular dependency)
#[derive(Clone, PartialEq, McBuf)]
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub name: String,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct AttributeModifiersEntry {
    pub attribute: Attribute,
    pub modifier: AttributeModifier,
    pub slot: EquipmentSlotGroup,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct AttributeModifiers {
    pub modifiers: Vec<AttributeModifiersEntry>,
    pub show_in_tooltip: bool,
}
impl DataComponent for AttributeModifiers {}

#[derive(Clone, PartialEq, McBuf)]
pub struct CustomModelData {
    #[var]
    pub value: i32,
}
impl DataComponent for CustomModelData {}

#[derive(Clone, PartialEq, McBuf)]
pub struct HideAdditionalTooltip;
impl DataComponent for HideAdditionalTooltip {}

#[derive(Clone, PartialEq, McBuf)]
pub struct HideTooltip;
impl DataComponent for HideTooltip {}

#[derive(Clone, PartialEq, McBuf)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}
impl DataComponent for RepairCost {}

#[derive(Clone, PartialEq, McBuf)]
pub struct CreativeSlotLock;
impl DataComponent for CreativeSlotLock {}

#[derive(Clone, PartialEq, McBuf)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}
impl DataComponent for EnchantmentGlintOverride {}

#[derive(Clone, PartialEq, McBuf)]
pub struct IntangibleProjectile;
impl DataComponent for IntangibleProjectile {}

#[derive(Clone, PartialEq, McBuf)]
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

#[derive(Clone, PartialEq, McBuf)]
pub struct MobEffectInstance {
    pub effect: MobEffect,
    pub details: MobEffectDetails,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct PossibleEffect {
    pub effect: MobEffectInstance,
    pub probability: f32,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct Food {
    #[var]
    pub nutrition: i32,
    pub saturation: f32,
    pub can_always_eat: bool,
    pub eat_seconds: f32,
    pub effects: Vec<PossibleEffect>,
}
impl DataComponent for Food {}

#[derive(Clone, PartialEq, McBuf)]
pub struct FireResistant;
impl DataComponent for FireResistant {}

#[derive(Clone, PartialEq, McBuf)]
pub struct ToolRule {
    pub blocks: HolderSet<Block, ResourceLocation>,
    pub speed: Option<f32>,
    pub correct_for_drops: Option<bool>,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct Tool {
    pub rules: Vec<ToolRule>,
    pub default_mining_speed: f32,
    #[var]
    pub damage_per_block: i32,
}
impl DataComponent for Tool {}

#[derive(Clone, PartialEq, McBuf)]
pub struct StoredEnchantments {
    #[var]
    pub enchantments: HashMap<Enchantment, i32>,
    pub show_in_tooltip: bool,
}
impl DataComponent for StoredEnchantments {}

#[derive(Clone, PartialEq, McBuf)]
pub struct DyedColor {
    pub rgb: i32,
    pub show_in_tooltip: bool,
}
impl DataComponent for DyedColor {}

#[derive(Clone, PartialEq, McBuf)]
pub struct MapColor {
    pub color: i32,
}
impl DataComponent for MapColor {}

#[derive(Clone, PartialEq, McBuf)]
pub struct MapId {
    #[var]
    pub id: i32,
}
impl DataComponent for MapId {}

#[derive(Clone, PartialEq, McBuf)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}
impl DataComponent for MapDecorations {}

#[derive(Clone, Copy, PartialEq, McBuf)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}
impl DataComponent for MapPostProcessing {}

#[derive(Clone, PartialEq, McBuf)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemSlot>,
}
impl DataComponent for ChargedProjectiles {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BundleContents {
    pub items: Vec<ItemSlot>,
}
impl DataComponent for BundleContents {}

#[derive(Clone, PartialEq, McBuf)]
pub struct PotionContents {
    pub potion: Option<Potion>,
    pub custom_color: Option<i32>,
    pub custom_effects: Vec<MobEffectInstance>,
}
impl DataComponent for PotionContents {}

#[derive(Clone, PartialEq, McBuf)]
pub struct SuspiciousStewEffect {
    pub effect: MobEffect,
    #[var]
    pub duration: i32,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct SuspiciousStewEffects {
    pub effects: Vec<SuspiciousStewEffect>,
}
impl DataComponent for SuspiciousStewEffects {}

#[derive(Clone, PartialEq, McBuf)]
pub struct WritableBookContent {
    pub pages: Vec<String>,
}
impl DataComponent for WritableBookContent {}

#[derive(Clone, PartialEq, McBuf)]
pub struct WrittenBookContent {
    pub title: String,
    pub author: String,
    #[var]
    pub generation: i32,
    pub pages: Vec<FormattedText>,
    pub resolved: bool,
}
impl DataComponent for WrittenBookContent {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
    pub show_in_tooltip: bool,
}
impl DataComponent for Trim {}

#[derive(Clone, PartialEq, McBuf)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}
impl DataComponent for DebugStickState {}

#[derive(Clone, PartialEq, McBuf)]
pub struct EntityData {
    pub entity: NbtCompound,
}
impl DataComponent for EntityData {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BucketEntityData {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BlockEntityData {
    pub entity: NbtCompound,
}
impl DataComponent for BlockEntityData {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Instrument {
    pub instrument: azalea_registry::Instrument,
}
impl DataComponent for Instrument {}

#[derive(Clone, PartialEq, McBuf)]
pub struct OminousBottleAmplifier {
    #[var]
    pub amplifier: i32,
}
impl DataComponent for OminousBottleAmplifier {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Recipes {
    pub recipes: Vec<ResourceLocation>,
}
impl DataComponent for Recipes {}

#[derive(Clone, PartialEq, McBuf)]
pub struct LodestoneTracker {
    pub target: Option<GlobalPos>,
    pub tracked: bool,
}
impl DataComponent for LodestoneTracker {}

#[derive(Clone, Copy, PartialEq, McBuf)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    pub colors: Vec<i32>,
    pub fade_colors: Vec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}
impl DataComponent for FireworkExplosion {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Fireworks {
    #[var]
    pub flight_duration: i32,
    pub explosions: Vec<FireworkExplosion>,
}
impl DataComponent for Fireworks {}

#[derive(Clone, PartialEq, McBuf)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct Profile {
    pub name: String,
    pub id: Option<Uuid>,
    pub properties: Vec<GameProfileProperty>,
}
impl DataComponent for Profile {}

#[derive(Clone, PartialEq, McBuf)]
pub struct NoteBlockSound {
    pub sound: ResourceLocation,
}
impl DataComponent for NoteBlockSound {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BannerPattern {
    #[var]
    pub pattern: i32,
    #[var]
    pub color: i32,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct BannerPatterns {
    pub patterns: Vec<BannerPattern>,
}
impl DataComponent for BannerPatterns {}

#[derive(Clone, Copy, PartialEq, McBuf)]
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

#[derive(Clone, PartialEq, McBuf)]
pub struct BaseColor {
    pub color: DyeColor,
}
impl DataComponent for BaseColor {}

#[derive(Clone, PartialEq, McBuf)]
pub struct PotDecorations {
    pub items: Vec<Item>,
}
impl DataComponent for PotDecorations {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Container {
    pub items: Vec<ItemSlot>,
}
impl DataComponent for Container {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BlockState {
    pub properties: HashMap<String, String>,
}
impl DataComponent for BlockState {}

#[derive(Clone, PartialEq, McBuf)]
pub struct BeehiveOccupant {
    pub entity_data: NbtCompound,
    #[var]
    pub ticks_in_hive: i32,
    #[var]
    pub min_ticks_in_hive: i32,
}

#[derive(Clone, PartialEq, McBuf)]
pub struct Bees {
    pub occupants: Vec<BeehiveOccupant>,
}
impl DataComponent for Bees {}

#[derive(Clone, PartialEq, McBuf)]
pub struct Lock {
    pub key: String,
}
impl DataComponent for Lock {}

#[derive(Clone, PartialEq, McBuf)]
pub struct ContainerLoot {
    pub loot: NbtCompound,
}
impl DataComponent for ContainerLoot {}

#[derive(Clone, PartialEq, McBuf)]
pub struct JukeboxPlayable {
    pub song: azalea_registry::JukeboxSong,
    pub show_in_tooltip: bool,
}
impl DataComponent for JukeboxPlayable {}
