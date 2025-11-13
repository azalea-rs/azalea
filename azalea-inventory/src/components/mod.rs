mod profile;

use core::f64;
use std::{
    any::Any,
    collections::HashMap,
    io::{self, Cursor},
    mem::ManuallyDrop,
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::{
    checksum::{Checksum, get_checksum},
    codec_utils::*,
    filterable::Filterable,
    identifier::Identifier,
    position::GlobalPos,
    registry_holder::RegistryHolder,
    sound::CustomSound,
};
use azalea_registry::{
    self as registry, Attribute, Block, DamageKind, DataComponentKind, Enchantment, EntityKind,
    Holder, HolderSet, Item, MobEffect, Potion, SoundEvent, TrimMaterial, TrimPattern,
};
pub use profile::*;
use serde::{Serialize, ser::SerializeMap};
use simdnbt::owned::{Nbt, NbtCompound};
use tracing::trace;

use crate::{ItemStack, item::consume_effect::ConsumeEffect};

pub trait DataComponentTrait:
    Send + Sync + Any + Clone + Serialize + Into<DataComponentUnion>
{
    const KIND: DataComponentKind;
}

pub trait EncodableDataComponent: Send + Sync + Any {
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()>;
    fn crc_hash(&self, registries: &RegistryHolder) -> Checksum;
    // using the Clone trait makes it not be object-safe, so we have our own clone
    // function instead
    fn clone(&self) -> Box<dyn EncodableDataComponent>;
    // same thing here
    fn eq(&self, other: &dyn EncodableDataComponent) -> bool;
}

impl<T> EncodableDataComponent for T
where
    T: DataComponentTrait + Clone + AzaleaWrite + AzaleaRead + PartialEq,
{
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        self.azalea_write(buf)
    }
    fn crc_hash(&self, registries: &RegistryHolder) -> Checksum {
        get_checksum(self, registries).expect("serializing data components should always succeed")
    }
    fn clone(&self) -> Box<dyn EncodableDataComponent> {
        let cloned = self.clone();
        Box::new(cloned)
    }
    fn eq(&self, other: &dyn EncodableDataComponent) -> bool {
        let other_any: &dyn Any = other;
        match other_any.downcast_ref::<T>() {
            Some(other) => self == other,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! define_data_components {
    ( $( $x:ident ),* $(,)? ) => {
        /// A union of all data components.
        ///
        /// You probably don't want to use this directly. Consider [`DataComponentPatch`] instead.
        ///
        /// This type does not know its own value, and as such every function for it requires the
        /// `DataComponentKind` to be passed in. Passing the wrong `DataComponentKind` will result
        /// in undefined behavior. Also, all of the values are `ManuallyDrop`.
        ///
        /// [`DataComponentPatch`]: crate::DataComponentPatch
        #[allow(non_snake_case)]
        pub union DataComponentUnion {
            $( $x: ManuallyDrop<$x>, )*
        }
        impl DataComponentUnion {
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn serialize_entry_as<S: SerializeMap>(
                &self,
                serializer: &mut S,
                kind: DataComponentKind,
            ) -> Result<(), S::Error> {
                match kind {
                    $( DataComponentKind::$x => { unsafe { serializer.serialize_entry(&kind, &*self.$x) } }, )*
                }
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn drop_as(&mut self, kind: DataComponentKind) {
                match kind {
                    $( DataComponentKind::$x => { unsafe { ManuallyDrop::drop(&mut self.$x) } }, )*
                }
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn as_kind(&self, kind: DataComponentKind) -> &dyn EncodableDataComponent {
                match kind {
                    $( DataComponentKind::$x => { unsafe { &**(&self.$x as &ManuallyDrop<dyn EncodableDataComponent>) } }, )*
                }
            }
            pub fn azalea_read_as(
                kind: registry::DataComponentKind,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                trace!("Reading data component {kind}");

                Ok(match kind {
                    $( DataComponentKind::$x => {
                        Self { $x: ManuallyDrop::new($x::azalea_read(buf)?) }
                    }, )*
                })
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn azalea_write_as(
                &self,
                kind: registry::DataComponentKind,
                buf: &mut impl std::io::Write,
            ) -> io::Result<()> {
                let mut value = Vec::new();
                match kind {
                    $( DataComponentKind::$x => unsafe { self.$x.encode(&mut value)? }, )*
                };
                buf.write_all(&value)?;

                Ok(())
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn clone_as(
                &self,
                kind: registry::DataComponentKind,
            ) -> Self {
                match kind {
                    $( DataComponentKind::$x => {
                        Self { $x: unsafe { self.$x.clone() } }
                    }, )*
                }
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn eq_as(
                &self,
                other: &Self,
                kind: registry::DataComponentKind,
            ) -> bool {
                match kind {
                    $( DataComponentKind::$x => unsafe { self.$x.eq(&other.$x) }, )*
                }
            }
        }
        $(
            impl From<$x> for DataComponentUnion {
                fn from(value: $x) -> Self {
                    DataComponentUnion { $x: ManuallyDrop::new(value) }
                }
            }
        )*

        $(
            impl DataComponentTrait for $x {
                const KIND: DataComponentKind = DataComponentKind::$x;
            }
        )*
    };
}

// if this is causing a compile-time error, look at DataComponents.java in the
// decompiled vanilla code to see how to implement new components

// note that this statement is updated by genitemcomponents.py
define_data_components!(
    CustomData,
    MaxStackSize,
    MaxDamage,
    Damage,
    Unbreakable,
    CustomName,
    ItemName,
    ItemModel,
    Lore,
    Rarity,
    Enchantments,
    CanPlaceOn,
    CanBreak,
    AttributeModifiers,
    CustomModelData,
    TooltipDisplay,
    RepairCost,
    CreativeSlotLock,
    EnchantmentGlintOverride,
    IntangibleProjectile,
    Food,
    Consumable,
    UseRemainder,
    UseCooldown,
    DamageResistant,
    Tool,
    Weapon,
    Enchantable,
    Equippable,
    Repairable,
    Glider,
    TooltipStyle,
    DeathProtection,
    BlocksAttacks,
    StoredEnchantments,
    DyedColor,
    MapColor,
    MapId,
    MapDecorations,
    MapPostProcessing,
    ChargedProjectiles,
    BundleContents,
    PotionContents,
    PotionDurationScale,
    SuspiciousStewEffects,
    WritableBookContent,
    WrittenBookContent,
    Trim,
    DebugStickState,
    EntityData,
    BucketEntityData,
    BlockEntityData,
    Instrument,
    ProvidesTrimMaterial,
    OminousBottleAmplifier,
    JukeboxPlayable,
    ProvidesBannerPatterns,
    Recipes,
    LodestoneTracker,
    FireworkExplosion,
    Fireworks,
    Profile,
    NoteBlockSound,
    BannerPatterns,
    BaseColor,
    PotDecorations,
    Container,
    BlockState,
    Bees,
    Lock,
    ContainerLoot,
    BreakSound,
    VillagerVariant,
    WolfVariant,
    WolfSoundVariant,
    WolfCollar,
    FoxVariant,
    SalmonSize,
    ParrotVariant,
    TropicalFishPattern,
    TropicalFishBaseColor,
    TropicalFishPatternColor,
    MooshroomVariant,
    RabbitVariant,
    PigVariant,
    CowVariant,
    ChickenVariant,
    FrogVariant,
    HorseVariant,
    PaintingVariant,
    LlamaVariant,
    AxolotlVariant,
    CatVariant,
    CatCollar,
    SheepColor,
    ShulkerColor,
);

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CustomData {
    pub nbt: Nbt,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Unbreakable;

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CustomName {
    pub name: FormattedText,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ItemName {
    pub name: FormattedText,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

#[derive(Clone, PartialEq, AzBuf, Serialize)]
#[serde(transparent)]
pub struct Enchantments {
    #[var]
    pub levels: HashMap<Enchantment, u32>,
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
    pub blocks: Option<HolderSet<Block, Identifier>>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
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
    pub id: Identifier,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct CreativeSlotLock;

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct IntangibleProjectile;

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
    pub blocks: HolderSet<Block, Identifier>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct DyedColor {
    pub rgb: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapColor {
    pub color: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapId {
    #[var]
    pub id: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemStack>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BundleContents {
    pub items: Vec<ItemStack>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct WritableBookContent {
    pub pages: Vec<Filterable<String>>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct EntityData {
    #[serde(rename = "id")]
    pub kind: EntityKind,
    #[serde(flatten)]
    pub data: NbtCompound,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BlockEntityData {
    #[serde(rename = "id")]
    pub kind: EntityKind,
    #[serde(flatten)]
    pub data: NbtCompound,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(untagged)]
pub enum Instrument {
    Registry(registry::Instrument),
    Holder(Holder<registry::Instrument, InstrumentData>),
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Recipes {
    pub recipes: Vec<Identifier>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct LodestoneTracker {
    #[serde(skip_serializing_if = "is_default")]
    pub target: Option<GlobalPos>,
    #[serde(skip_serializing_if = "is_true")]
    pub tracked: bool,
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
    #[serde(skip_serializing_if = "is_default")]
    pub colors: Vec<i32>,
    #[serde(skip_serializing_if = "is_default")]
    pub fade_colors: Vec<i32>,
    #[serde(skip_serializing_if = "is_default")]
    pub has_trail: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub has_twinkle: bool,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Fireworks {
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    pub flight_duration: i32,
    #[limit(256)]
    pub explosions: Vec<FireworkExplosion>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct NoteBlockSound {
    pub sound: Identifier,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BannerPattern {
    #[var]
    pub pattern: i32,
    #[var]
    pub color: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BannerPatterns {
    pub patterns: Vec<BannerPattern>,
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BaseColor {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct PotDecorations {
    pub items: Vec<Item>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Container {
    pub items: Vec<ItemStack>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BlockState {
    pub properties: HashMap<String, String>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BeehiveOccupant {
    #[serde(skip_serializing_if = "is_default")]
    pub entity_data: NbtCompound,
    #[var]
    pub ticks_in_hive: i32,
    #[var]
    pub min_ticks_in_hive: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct Bees {
    pub occupants: Vec<BeehiveOccupant>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Lock {
    pub key: String,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct ContainerLoot {
    pub loot_table: NbtCompound,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(untagged)]
pub enum JukeboxPlayable {
    Referenced(Identifier),
    Direct(Holder<registry::JukeboxSong, JukeboxSongData>),
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct JukeboxSongData {
    pub sound_event: Holder<SoundEvent, CustomSound>,
    pub description: FormattedText,
    pub length_in_seconds: f32,
    #[var]
    pub comparator_output: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Consumable {
    #[serde(skip_serializing_if = "is_default")]
    pub consume_seconds: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub animation: ItemUseAnimation,
    #[serde(skip_serializing_if = "is_default_eat_sound")]
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
    #[serde(skip_serializing_if = "is_default")]
    pub has_consume_particles: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub on_consume_effects: Vec<ConsumeEffect>,
}
fn is_default_eat_sound(sound: &azalea_registry::Holder<SoundEvent, CustomSound>) -> bool {
    matches!(
        sound,
        azalea_registry::Holder::Reference(SoundEvent::EntityGenericEat)
    )
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

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemUseAnimation {
    #[default]
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct UseRemainder {
    pub convert_into: ItemStack,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct UseCooldown {
    pub seconds: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub cooldown_group: Option<Identifier>,
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Enchantable {
    #[var]
    pub value: u32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Repairable {
    pub items: HolderSet<Item, Identifier>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ItemModel {
    pub resource_location: Identifier,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct DamageResistant {
    /// In vanilla this only allows tag keys, i.e. it must start with '#'
    pub types: Identifier,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Equippable {
    pub slot: EquipmentSlot,
    #[serde(skip_serializing_if = "is_default_equip_sound")]
    pub equip_sound: SoundEvent,
    #[serde(skip_serializing_if = "is_default")]
    pub asset_id: Option<Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    pub camera_overlay: Option<Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    pub allowed_entities: Option<HolderSet<EntityKind, Identifier>>,
    #[serde(skip_serializing_if = "is_true")]
    pub dispensable: bool,
    #[serde(skip_serializing_if = "is_true")]
    pub swappable: bool,
    #[serde(skip_serializing_if = "is_true")]
    pub damage_on_hurt: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub equip_on_interact: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub can_be_sheared: bool,
    #[serde(skip_serializing_if = "is_default_shearing_sound")]
    pub shearing_sound: SoundEvent,
}
fn is_default_equip_sound(sound: &SoundEvent) -> bool {
    matches!(sound, SoundEvent::ItemArmorEquipGeneric)
}
fn is_default_shearing_sound(sound: &SoundEvent) -> bool {
    matches!(sound, SoundEvent::ItemShearsSnip)
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

#[derive(Clone, Copy, Debug, PartialEq, AzBuf, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Glider;

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct TooltipStyle {
    pub resource_location: Identifier,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct DeathProtection {
    pub death_effects: Vec<ConsumeEffect>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct Weapon {
    #[var]
    #[serde(skip_serializing_if = "is_default_item_damage_per_attack")]
    pub item_damage_per_attack: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub disable_blocking_for_seconds: f32,
}
fn is_default_item_damage_per_attack(value: &i32) -> bool {
    *value == 1
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct PotionDurationScale {
    pub value: f32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct VillagerVariant {
    pub variant: registry::VillagerKind,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct WolfVariant {
    pub variant: registry::WolfVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct WolfCollar {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct FoxVariant {
    pub variant: registry::FoxVariant,
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SalmonSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ParrotVariant {
    pub variant: registry::ParrotVariant,
}

#[derive(Clone, Copy, PartialEq, AzBuf, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct TropicalFishBaseColor {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct TropicalFishPatternColor {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct MooshroomVariant {
    pub variant: registry::MooshroomVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct RabbitVariant {
    pub variant: registry::RabbitVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct PigVariant {
    pub variant: registry::PigVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct FrogVariant {
    pub variant: registry::FrogVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct HorseVariant {
    pub variant: registry::HorseVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct PaintingVariant {
    pub variant: Holder<registry::PaintingVariant, PaintingVariantData>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct PaintingVariantData {
    #[var]
    pub width: i32,
    #[var]
    pub height: i32,
    pub asset_id: Identifier,
    #[serde(skip_serializing_if = "is_default")]
    pub title: Option<FormattedText>,
    #[serde(skip_serializing_if = "is_default")]
    pub author: Option<FormattedText>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct LlamaVariant {
    pub variant: registry::LlamaVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct AxolotlVariant {
    pub variant: registry::AxolotlVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CatVariant {
    pub variant: registry::CatVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CatCollar {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct SheepColor {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ShulkerColor {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct TooltipDisplay {
    #[serde(skip_serializing_if = "is_default")]
    pub hide_tooltip: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub hidden_components: Vec<DataComponentKind>,
}

impl TooltipDisplay {
    pub const fn new() -> Self {
        Self {
            hide_tooltip: false,
            hidden_components: Vec::new(),
        }
    }
}
impl Default for TooltipDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct BlocksAttacks {
    #[serde(skip_serializing_if = "is_default")]
    pub block_delay_seconds: f32,
    #[serde(skip_serializing_if = "is_default_disable_cooldown_scale")]
    pub disable_cooldown_scale: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub damage_reductions: Vec<DamageReduction>,
    #[serde(skip_serializing_if = "is_default")]
    pub item_damage: ItemDamageFunction,
    #[serde(skip_serializing_if = "is_default")]
    pub bypassed_by: Option<Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    pub block_sound: Option<azalea_registry::Holder<SoundEvent, CustomSound>>,
    #[serde(skip_serializing_if = "is_default")]
    pub disabled_sound: Option<azalea_registry::Holder<SoundEvent, CustomSound>>,
}
fn is_default_disable_cooldown_scale(value: &f32) -> bool {
    *value == 1.
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
            item_damage: ItemDamageFunction::default(),
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

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct DamageReduction {
    #[serde(skip_serializing_if = "is_default_horizontal_blocking_angle")]
    pub horizontal_blocking_angle: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub kind: Option<HolderSet<DamageKind, Identifier>>,
    pub base: f32,
    pub factor: f32,
}
fn is_default_horizontal_blocking_angle(value: &f32) -> bool {
    *value == 90.
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct ItemDamageFunction {
    pub threshold: f32,
    pub base: f32,
    pub factor: f32,
}
impl Default for ItemDamageFunction {
    fn default() -> Self {
        ItemDamageFunction {
            threshold: 1.,
            base: 0.,
            factor: 1.,
        }
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(untagged)]
pub enum ProvidesTrimMaterial {
    Referenced(Identifier),
    Direct(Holder<TrimMaterial, DirectTrimMaterial>),
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct DirectTrimMaterial {
    pub assets: MaterialAssetGroup,
    pub description: FormattedText,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct MaterialAssetGroup {
    pub base: AssetInfo,
    #[serde(skip_serializing_if = "is_default")]
    pub overrides: Vec<(Identifier, AssetInfo)>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct AssetInfo {
    pub suffix: String,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct ProvidesBannerPatterns {
    pub key: Identifier,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct BreakSound {
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct WolfSoundVariant {
    pub variant: azalea_registry::WolfSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(transparent)]
pub struct CowVariant {
    pub variant: azalea_registry::CowVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
#[serde(untagged)]
pub enum ChickenVariant {
    Referenced(Identifier),
    Direct(ChickenVariantData),
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]
pub struct ChickenVariantData {
    pub registry: azalea_registry::ChickenVariant,
}
