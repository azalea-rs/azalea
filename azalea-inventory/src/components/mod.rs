mod profile;

use core::f64;
use std::{
    any::Any,
    collections::HashMap,
    fmt::{self, Debug, Display},
    io::{self, Cursor},
    mem::ManuallyDrop,
};

use azalea_buf::{AzBuf, BufReadError};
use azalea_chat::FormattedText;
use azalea_codec::Codec;
use azalea_core::{
    attribute_modifier_operation::AttributeModifierOperation,
    checksum::{Checksum, get_checksum},
    codec_utils::*,
    filterable::Filterable,
    position::GlobalPos,
    registry_holder::RegistryHolder,
    sound::CustomSound,
};
use azalea_registry::{
    Holder, HolderSet,
    builtin::{
        Attribute, BlockKind, DataComponentKind, EntityKind, ItemKind, MobEffect, Potion,
        SoundEvent, VillagerKind,
    },
    data::{self, BannerPatternKind, DamageKind, Enchantment, TrimMaterial, TrimPattern},
    identifier::Identifier,
};
pub use profile::*;
use serde::{Serialize, ser::SerializeMap};
use simdnbt::owned::NbtCompound;
use tracing::trace;

use crate::{ItemStack, item::consume_effect::ConsumeEffect};

pub trait DataComponentTrait:
    Send + Sync + Any + Clone + Serialize + Into<DataComponentUnion>
{
    const KIND: DataComponentKind;
}

pub trait EncodableDataComponent: Send + Sync + Any + Debug {
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()>;
    fn crc_hash(&self, registries: &RegistryHolder) -> Checksum;
    // using the Clone trait makes it not be object-safe, so we have our own
    // clone function instead
    fn clone(&self) -> Box<dyn EncodableDataComponent>;
    // same thing here
    fn eq(&self, other: &dyn EncodableDataComponent) -> bool;
}

impl<T> EncodableDataComponent for T
where
    T: DataComponentTrait + Clone + AzBuf + PartialEq + Debug,
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
            pub unsafe fn codec_write_as<O: azalea_codec::Ops>(
                &self,
                kind: DataComponentKind,
            ) -> azalea_codec::error::WriteResult<O> {
                match kind {
                    $( DataComponentKind::$x => { unsafe { self.$x.codec_write() } }, )*
                }
            }
            pub fn codec_read_as<O: azalea_codec::Ops>(
                ops: &O,
                kind: DataComponentKind,
            ) -> azalea_codec::error::ReadResult<Self> {
                match kind {
                    $( DataComponentKind::$x => { Ok(Self { $x: <_>::codec_read(ops)? }) }, )*
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
                kind: DataComponentKind,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                trace!("Reading data component {kind}");

                Ok(match kind {
                    $( DataComponentKind::$x => {
                        let v = $x::azalea_read(buf)?;
                        Self { $x: ManuallyDrop::new(v) }
                    }, )*
                })
            }
            /// # Safety
            ///
            /// `kind` must be the correct value for this union.
            pub unsafe fn azalea_write_as(
                &self,
                kind: DataComponentKind,
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
                kind: DataComponentKind,
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
                kind: DataComponentKind,
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
    UseEffects,
    MinimumAttackCharge,
    DamageType,
    PiercingWeapon,
    KineticWeapon,
    ZombieNautilusVariant,
    AttackRange,
    AdditionalTradeCost,
    Dye,
    PigSoundVariant,
    CowSoundVariant,
    ChickenSoundVariant,
    CatSoundVariant,
    SulfurCubeContent,
    BlockTransformer,
    ProvidesPotteryPattern,
    AttackAnimation,
    InteractAnimation,
    VillagerFood,
    Compostable,
    CookingFuel,
    BrewingFuel,
    MobVisibility,
    SignTextFront,
    SignTextBack,
    Waxed,
    CushionColor,
);

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CustomData {
    pub nbt: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Unbreakable;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CustomName {
    pub name: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ItemName {
    pub name: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

#[derive(AzBuf, Clone, Default, PartialEq, Serialize, Debug, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Enchantments {
    /// Enchantment levels here are 1-indexed, level 0 does not exist.
    #[var]
    pub levels: HashMap<Enchantment, i32>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub enum BlockStateValueMatcher {
    Exact {
        value: String,
    },
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BlockStatePropertyMatcher {
    pub name: String,
    pub value_matcher: BlockStateValueMatcher,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BlockPredicate {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub blocks: Option<HolderSet<BlockKind, Identifier>>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub properties: Option<Vec<BlockStatePropertyMatcher>>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub nbt: Option<NbtCompound>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct AdventureModePredicate {
    #[serde(serialize_with = "flatten_array")]
    pub predicates: Vec<BlockPredicate>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CanPlaceOn {
    pub predicate: AdventureModePredicate,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
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

// this is duplicated in azalea-entity, BUT the one there has a different
// protocol format (and we can't use it anyways because it would cause a
// circular dependency)
#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct AttributeModifier {
    pub id: Identifier,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct AttributeModifiersEntry {
    #[serde(rename = "type")]
    pub kind: Attribute,
    #[serde(flatten)]
    #[codec(flatten)]
    pub modifier: AttributeModifier,
    pub slot: EquipmentSlotGroup,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub display: AttributeModifierDisplay,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct AttributeModifiers {
    pub modifiers: Vec<AttributeModifiersEntry>,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum AttributeModifierDisplay {
    #[default]
    Default,
    Hidden,
    Override {
        text: FormattedText,
    },
}
azalea_codec::impl_defaultable!(AttributeModifierDisplay);

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct CreativeSlotLock;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct IntangibleProjectile;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct MobEffectDetails {
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub amplifier: i32,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub duration: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub ambient: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub show_particles: bool,
    pub show_icon: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct MobEffectInstance {
    pub id: MobEffect,
    #[serde(flatten)]
    #[codec(flatten)]
    pub details: MobEffectDetails,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct PossibleEffect {
    pub effect: MobEffectInstance,
    pub probability: f32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
pub struct Food {
    #[var]
    pub nutrition: i32,
    pub saturation: f32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct ToolRule {
    pub blocks: HolderSet<BlockKind, Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub speed: Option<f32>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Tool {
    #[serde(serialize_with = "flatten_array")]
    pub rules: Vec<ToolRule>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub default_mining_speed: f32,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub damage_per_block: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct StoredEnchantments {
    #[var]
    pub enchantments: HashMap<Enchantment, i32>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct DyedColor {
    pub rgb: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MapId {
    #[var]
    pub id: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[codec(default_snake_case_key)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BundleContents {
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct PotionContents {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub potion: Option<Potion>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub custom_color: Option<i32>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub custom_effects: Vec<MobEffectInstance>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct SuspiciousStewEffect {
    #[serde(rename = "id")]
    pub effect: MobEffect,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub duration: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct SuspiciousStewEffects {
    pub effects: Vec<SuspiciousStewEffect>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct WritableBookContent {
    pub pages: Vec<Filterable<String>>,
}

#[derive(AzBuf, Clone, PartialEq, Debug, Serialize, Codec)]
pub struct WrittenBookContent {
    #[limit(32)]
    pub title: Filterable<String>,
    pub author: String,
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub generation: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub pages: Vec<Filterable<FormattedText>>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub resolved: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct EntityData {
    #[serde(rename = "id")]
    pub kind: EntityKind,
    #[serde(flatten)]
    #[codec(flatten)]
    pub data: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BlockEntityData {
    #[serde(rename = "id")]
    pub kind: EntityKind,
    #[serde(flatten)]
    #[codec(flatten)]
    pub data: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Instrument {
    pub value: Holder<azalea_registry::data::Instrument, InstrumentData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct InstrumentData {
    pub sound_event: Holder<SoundEvent, azalea_core::sound::CustomSound>,
    pub use_duration: f32,
    pub range: f32,
    pub description: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct OminousBottleAmplifier {
    #[var]
    pub amplifier: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Recipes {
    pub recipes: Vec<Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct LodestoneTracker {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub target: Option<GlobalPos>,
    #[serde(skip_serializing_if = "is_true")]
    #[codec(optional)]
    pub tracked: bool,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub colors: Vec<i32>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub fade_colors: Vec<i32>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub has_trail: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub has_twinkle: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Fireworks {
    #[var]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct NoteBlockSound {
    pub sound: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BannerPatternLayer {
    #[var]
    pub pattern: i32,
    #[var]
    pub color: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BannerPatterns {
    pub patterns: Vec<BannerPatternLayer>,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Default, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum DyeColor {
    #[default]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BaseColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct PotDecorations {
    pub back: Option<ItemStack>,
    pub left: Option<ItemStack>,
    pub right: Option<ItemStack>,
    pub front: Option<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Container {
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BlockState {
    pub properties: HashMap<String, String>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BeehiveOccupant {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub entity_data: NbtCompound,
    #[var]
    pub ticks_in_hive: i32,
    #[var]
    pub min_ticks_in_hive: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Bees {
    pub occupants: Vec<BeehiveOccupant>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Lock {
    pub key: String,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct ContainerLoot {
    pub loot_table: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct JukeboxPlayable {
    pub value: Holder<azalea_registry::data::JukeboxSong, JukeboxSongData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct JukeboxSongData {
    pub sound_event: Holder<SoundEvent, CustomSound>,
    pub description: FormattedText,
    pub length_in_seconds: f32,
    #[var]
    pub comparator_output: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Consumable {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub consume_seconds: f32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub animation: ItemUseAnimation,
    #[serde(skip_serializing_if = "is_default_eat_sound")]
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub has_consume_particles: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum ItemUseAnimation {
    #[default]
    None,
    Eat,
    Drink,
    BlockKind,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}
azalea_codec::impl_defaultable!(ItemUseAnimation);

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct UseRemainder {
    pub convert_into: ItemStack,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct UseCooldown {
    pub seconds: f32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Enchantable {
    #[var]
    pub value: u32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Repairable {
    pub items: HolderSet<ItemKind, Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ItemModel {
    pub resource_location: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct DamageResistant {
    pub types: HolderSet<DamageKind, Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Equippable {
    pub slot: EquipmentSlot,
    #[serde(skip_serializing_if = "is_default_equip_sound")]
    pub equip_sound: SoundEvent,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub asset_id: Option<Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub camera_overlay: Option<Identifier>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub allowed_entities: Option<HolderSet<EntityKind, Identifier>>,
    #[serde(skip_serializing_if = "is_true")]
    pub dispensable: bool,
    #[serde(skip_serializing_if = "is_true")]
    pub swappable: bool,
    #[serde(skip_serializing_if = "is_true")]
    pub damage_on_hurt: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub equip_on_interact: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

/// An enum that represents inventory slots that can hold items.
#[derive(AzBuf, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum EquipmentSlot {
    Mainhand,
    Offhand,
    Feet,
    Legs,
    Chest,
    Head,
    /// This is for animal armor, use [`Self::Chest`] for the chestplate slot.
    Body,
    Saddle,
}
impl EquipmentSlot {
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        let value = match byte {
            0 => Self::Mainhand,
            1 => Self::Offhand,
            2 => Self::Feet,
            3 => Self::Legs,
            4 => Self::Chest,
            5 => Self::Head,
            6 => Self::Body,
            7 => Self::Saddle,
            _ => return None,
        };
        Some(value)
    }
    pub fn values() -> [Self; 8] {
        [
            Self::Mainhand,
            Self::Offhand,
            Self::Feet,
            Self::Legs,
            Self::Chest,
            Self::Head,
            Self::Body,
            Self::Saddle,
        ]
    }
    /// Get the display name for the equipment slot, like "mainhand".
    pub fn name(self) -> &'static str {
        match self {
            Self::Mainhand => "mainhand",
            Self::Offhand => "offhand",
            Self::Feet => "feet",
            Self::Legs => "legs",
            Self::Chest => "chest",
            Self::Head => "head",
            Self::Body => "body",
            Self::Saddle => "saddle",
        }
    }
}
impl Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Glider;

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct TooltipStyle {
    pub resource_location: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct DeathProtection {
    pub death_effects: Vec<ConsumeEffect>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct Weapon {
    #[var]
    #[serde(skip_serializing_if = "is_default_item_damage_per_attack")]
    pub item_damage_per_attack: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct PotionDurationScale {
    pub value: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct VillagerVariant {
    pub variant: VillagerKind,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct WolfVariant {
    pub variant: data::WolfVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct WolfCollar {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct FoxVariant {
    pub variant: FoxVariantKind,
}

azalea_registry::macros::registry! {
#[derive(Default)]
enum FoxVariantKind {
    #[default]
    Red => "red",
    Snow => "snow",
}
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum SalmonSize {
    Small,
    Medium,
    Large,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ParrotVariant {
    pub variant: ParrotVariantKind,
}
azalea_registry::macros::registry! {
enum ParrotVariantKind {
    RedBlue => "red_blue",
    Blue => "blue",
    Green => "green",
    YellowBlue => "yellow_blue",
    Gray => "gray",
}
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct TropicalFishBaseColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct TropicalFishPatternColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MooshroomVariant {
    pub variant: MooshroomVariantKind,
}
azalea_registry::macros::registry! {
#[derive(Default)]
enum MooshroomVariantKind {
    #[default]
    Red => "red",
    Brown => "brown",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct RabbitVariant {
    pub variant: RabbitVariantKind,
}
azalea_registry::macros::registry! {
#[derive(Default)]
enum RabbitVariantKind {
    #[default]
    Brown => "brown",
    White => "white",
    Black => "black",
    WhiteSplotched => "white_splotched",
    Gold => "gold",
    Salt => "salt",
    Evil => "evil",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct PigVariant {
    pub variant: data::PigVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct FrogVariant {
    pub variant: data::FrogVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct HorseVariant {
    pub variant: HorseVariantKind,
}
azalea_registry::macros::registry! {
#[derive(Default)]
enum HorseVariantKind {
    #[default]
    White => "white",
    Creamy => "creamy",
    Chestnut => "chestnut",
    Brown => "brown",
    Black => "black",
    Gray => "gray",
    DarkBrown => "dark_brown",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct PaintingVariant {
    pub variant: Holder<data::PaintingVariant, PaintingVariantData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct PaintingVariantData {
    #[var]
    pub width: i32,
    #[var]
    pub height: i32,
    pub asset_id: Identifier,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub title: Option<FormattedText>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub author: Option<FormattedText>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct LlamaVariant {
    pub variant: LlamaVariantKind,
}
azalea_registry::macros::registry! {
#[derive(Default)]
enum LlamaVariantKind {
    #[default]
    Creamy => "creamy",
    White => "white",
    Brown => "brown",
    Gray => "gray",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct AxolotlVariant {
    pub variant: AxolotlVariantKind,
}
azalea_registry::macros::registry! {
#[derive(Default)]
enum AxolotlVariantKind {
    #[default]
    Lucy => "lucy",
    Wild => "wild",
    Gold => "gold",
    Cyan => "cyan",
    Blue => "blue",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CatVariant {
    pub variant: data::CatVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CatCollar {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct SheepColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ShulkerColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct TooltipDisplay {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub hide_tooltip: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct BlocksAttacks {
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub block_delay_seconds: f32,
    #[serde(skip_serializing_if = "is_default_disable_cooldown_scale")]
    pub disable_cooldown_scale: f32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub damage_reductions: Vec<DamageReduction>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub item_damage: ItemDamageFunction,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub bypassed_by: Option<HolderSet<DamageKind, Identifier>>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub block_sound: Option<azalea_registry::Holder<SoundEvent, CustomSound>>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct DamageReduction {
    #[serde(skip_serializing_if = "is_default_horizontal_blocking_angle")]
    pub horizontal_blocking_angle: f32,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub kind: Option<HolderSet<DamageKind, Identifier>>,
    pub base: f32,
    pub factor: f32,
}
fn is_default_horizontal_blocking_angle(value: &f32) -> bool {
    *value == 90.
}
#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
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
azalea_codec::impl_defaultable!(ItemDamageFunction);

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ProvidesTrimMaterial {
    pub value: Holder<azalea_registry::data::TrimMaterial, DirectTrimMaterial>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct DirectTrimMaterial {
    #[serde(flatten)]
    #[codec(flatten)]
    pub assets: MaterialAssetGroup,
    pub description: FormattedText,
}
#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct MaterialAssetGroup {
    pub assert_name: AssetInfo,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub override_armor_assets: Vec<(Identifier, AssetInfo)>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct AssetInfo {
    pub suffix: String,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ProvidesBannerPatterns {
    pub key: HolderSet<BannerPatternKind, Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct BreakSound {
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct WolfSoundVariant {
    pub variant: azalea_registry::data::WolfSoundVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CowVariant {
    pub variant: azalea_registry::data::CowVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ChickenVariant {
    pub data: azalea_registry::data::ChickenVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ZombieNautilusVariant {
    pub value: azalea_registry::data::ZombieNautilusVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct UseEffects {
    pub can_sprint: bool,
    pub interact_vibrations: bool,
    pub speed_multiplier: f32,
}
impl UseEffects {
    pub const fn new() -> Self {
        Self {
            can_sprint: false,
            interact_vibrations: true,
            speed_multiplier: 0.2,
        }
    }
}
impl Default for UseEffects {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct MinimumAttackCharge {
    pub value: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct DamageType {
    pub value: azalea_registry::data::DamageKind,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct PiercingWeapon {
    pub deals_knockback: bool,
    pub dismounts: bool,
    pub sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
    pub hit_sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
}
impl PiercingWeapon {
    pub const fn new() -> Self {
        Self {
            deals_knockback: true,
            dismounts: false,
            sound: None,
            hit_sound: None,
        }
    }
}
impl Default for PiercingWeapon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct KineticWeapon {
    #[var]
    pub contact_cooldown_ticks: i32,
    #[var]
    pub delay_ticks: i32,
    pub dismount_conditions: Option<KineticWeaponCondition>,
    pub knockback_conditions: Option<KineticWeaponCondition>,
    pub damage_conditions: Option<KineticWeaponCondition>,
    pub forward_movement: f32,
    pub damage_multiplier: f32,
    pub sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
    pub hit_sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
}
impl KineticWeapon {
    pub const fn new() -> Self {
        Self {
            contact_cooldown_ticks: 10,
            delay_ticks: 0,
            dismount_conditions: None,
            knockback_conditions: None,
            damage_conditions: None,
            forward_movement: 0.,
            damage_multiplier: 1.,
            sound: None,
            hit_sound: None,
        }
    }
}
impl Default for KineticWeapon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct KineticWeaponCondition {
    #[var]
    pub max_duration_ticks: i32,
    pub min_speed: f32,
    pub min_relative_speed: f32,
}
impl KineticWeaponCondition {
    pub const fn new() -> Self {
        Self {
            max_duration_ticks: 0,
            min_speed: 0.,
            min_relative_speed: 0.,
        }
    }
}
impl Default for KineticWeaponCondition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}
impl AttackRange {
    pub const fn new() -> Self {
        Self {
            min_reach: 0.,
            max_reach: 3.,
            min_creative_reach: 0.,
            max_creative_reach: 5.,
            hitbox_margin: 0.3,
            mob_factor: 1.,
        }
    }
}
impl Default for AttackRange {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct AdditionalTradeCost {
    #[var]
    pub cost: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct Dye {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct PigSoundVariant {
    pub value: azalea_registry::data::PigSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CowSoundVariant {
    pub value: azalea_registry::data::CowSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ChickenSoundVariant {
    pub value: azalea_registry::data::ChickenSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct CatSoundVariant {
    pub value: azalea_registry::data::CatSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct SulfurCubeContent {
    pub absorbed_block_item_stack: ItemStack,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct BlockTransformer {
    pub value: azalea_registry::data::BlockTransformer,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct ProvidesPotteryPattern {
    pub value: azalea_registry::data::DecoratedPotPattern,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct AttackAnimation {
    pub value: SwingAnimation,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct SwingAnimation {
    #[serde(rename = "type")]
    pub kind: SwingAnimationKind,
    #[var]
    pub duration: i32,
}
impl SwingAnimation {
    pub const fn new() -> Self {
        Self {
            kind: SwingAnimationKind::Whack,
            duration: 6,
        }
    }
}
impl Default for SwingAnimation {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum SwingAnimationKind {
    None,
    Whack,
    Stab,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct InteractAnimation {
    pub value: SwingAnimation,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct VillagerFood {
    #[var]
    pub nutrition: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct Compostable {
    pub layers: ResolvableInt,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub enum ResolvableInt {
    Reference { context_int_provider: Identifier },
    Constant { value: i32 },
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub enum ResolvableFloat {
    Reference { context_float_provider: Identifier },
    Constant { value: f32 },
}
impl Default for ResolvableInt {
    fn default() -> Self {
        Self::Constant { value: 0 }
    }
}
impl Default for ResolvableFloat {
    fn default() -> Self {
        Self::Constant { value: 0. }
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Serialize, Codec)]
pub struct CookingFuel {
    pub burn_time: ResolvableInt,
    pub speed_multiplier: ResolvableFloat,
}
impl CookingFuel {
    pub const fn new() -> Self {
        Self {
            burn_time: ResolvableInt::Constant { value: 0 },
            speed_multiplier: ResolvableFloat::Constant { value: 0. },
        }
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct BrewingFuel {
    pub uses: ResolvableInt,
    pub speed_multiplier: ResolvableFloat,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct MobVisibility {
    pub targeting_entity_types: HolderSet<EntityKind, Identifier>,
    pub visibility: f32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct SignTextFront {
    pub value: SignText,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct SignTextBack {
    pub value: SignText,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Serialize, Codec)]
pub struct SignText {
    pub messages: [FormattedText; 4],
    pub filtered_messages_for_serialization: Option<[FormattedText; 4]>,
    pub color: DyeColor,
    pub has_glowing_text: bool,
}
impl SignText {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct Waxed;

#[derive(Clone, PartialEq, AzBuf, Debug, Serialize, Codec)]
pub struct CushionColor {
    pub value: DyeColor,
}
