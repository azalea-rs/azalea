pub mod patch;
mod profile;

use core::f64;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, BufReadError};
use azalea_chat::FormattedText;
use azalea_codec::{Codec, types::CompactListCodec};
use azalea_core::{
    attribute_modifier_operation::AttributeModifierOperation, filterable::Filterable,
    identifier::Identifier, position::GlobalPos, sound::CustomSound,
};
use azalea_registry_macros::registry;
pub use profile::*;
use simdnbt::owned::NbtCompound;
use tracing::trace;

use crate::{
    Holder, HolderSet,
    builtin::{
        Attribute, BlockKind, DataComponentKind, EntityKind, ItemKind, MobEffect, Potion,
        SoundEvent, VillagerKind,
    },
    data::{self, BannerPatternKind, DamageKind, Enchantment, TrimMaterial, TrimPattern},
    item::{
        ItemStack, ItemStackTemplate,
        checksum::{Checksum, ChecksumOps},
        consume_effect::ConsumeEffect,
    },
};

#[macro_export]
macro_rules! define_data_components {
    ( $( $x:ident ),* $(,)? ) => {
        /// An enum of all data components.
        ///
        /// You probably don't want to use this directly. Consider using
        /// [`DataComponentPatch`] instead.
        ///
        /// [`DataComponentPatch`]: crate::item::components::patch::DataComponentPatch
        #[derive(Clone, Debug, PartialEq)]
        pub enum DataComponent {
            // this is boxed to prevent the enum from getting too big
            $( $x(Box<$x>), )*
        }
        impl DataComponent {
            pub fn codec_read_as<O: azalea_codec::Ops>(
                kind: DataComponentKind,
                ops: &O,
            ) -> azalea_codec::error::ReadResult<Self> {
                Ok(match kind {
                    $(
                        DataComponentKind::$x => {
                            $x::codec_read(ops)
                                .map(|i| Self::$x(Box::new(i)))?
                        },
                    )*
                })
            }
            pub fn codec_write_value<O: azalea_codec::Ops>(
                &self,
            ) -> azalea_codec::error::WriteResult<O> {
                // we do this instead of implementing Codec since the codec_read
                // implementation on this type wouldn't make much sense. maybe
                // in the future we could implement MapCodec without Codec or
                // something, so it only works when flattened? idk
                Ok(match self {
                    $(
                        Self::$x(v) => {
                            v.codec_write()?
                        },
                    )*
                })
            }
            pub fn kind(&self) -> DataComponentKind {
                match self {
                    $(
                        Self::$x(_) => DataComponentKind::$x,
                    )*
                }
            }
            pub fn azalea_read_as(
                kind: DataComponentKind,
                ctx: azalea_buf::Ctx,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                trace!("Reading data component {kind}");

                Ok(match kind {
                    $(
                        DataComponentKind::$x => {
                            let v = $x::azalea_read(ctx, buf)?;
                            Self::$x(Box::new(v))
                        },
                    )*
                })
            }
            pub fn azalea_write_value(
                &self,
                ctx: azalea_buf::Ctx,
                buf: &mut impl std::io::Write,
            ) -> io::Result<()> {
                match self {
                    $( Self::$x(v) => v.azalea_write(ctx, buf)?, )*
                };

                Ok(())
            }
        }

        $(
            impl From<$x> for DataComponent {
                fn from(c: $x) -> Self {
                    Self::$x(Box::new(c))
                }
            }
            impl DataComponentTrait for $x {
                const KIND: DataComponentKind = DataComponentKind::$x;
                // this isn't a TryFrom impl because if we wanted
                // DataComponentTrait to depend on it then it'd require an
                // annoying where clause.
                fn try_from_data_component<'a>(c: &DataComponent) -> Option<&Self> {
                    if let DataComponent::$x(v) = c {
                        Some(&*v)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

pub trait DataComponentTrait:
    Send + Sync + Clone + Into<DataComponent> + Codec<ChecksumOps> + Sized + 'static
{
    const KIND: DataComponentKind;
    fn checksum(&self) -> Checksum {
        self.clone().into().checksum()
    }
    fn try_from_data_component(c: &DataComponent) -> Option<&Self>;
}

impl AzBuf for DataComponent {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let kind = DataComponentKind::azalea_read(ctx, buf)?;
        Self::azalea_read_as(kind, ctx, buf)
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        self.kind().azalea_write(ctx, buf)?;
        self.azalea_write_value(ctx, buf)
    }
}

// if this is causing a compile-time error, look at DataComponents.java in the
// decompiled vanilla code to see how to implement new components

// note that this statement is updated by gendatacomponents.py
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct CustomData(pub NbtCompound);

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MaxStackSize {
    #[var]
    pub count: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MaxDamage {
    #[var]
    pub amount: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Damage {
    #[var]
    pub amount: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Unbreakable;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CustomName {
    pub name: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ItemName {
    pub name: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Lore {
    pub lines: Vec<FormattedText>,
    // vanilla also has styled_lines here but it doesn't appear to be used for the protocol
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

#[derive(AzBuf, Clone, Default, PartialEq, Debug, Codec)]
#[codec(transparent)]
pub struct Enchantments {
    /// Enchantment levels here are 1-indexed, level 0 does not exist.
    #[var]
    pub levels: HashMap<Enchantment, i32>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub enum BlockStateValueMatcher {
    Exact {
        value: String,
    },
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BlockStatePropertyMatcher {
    pub name: String,
    pub value_matcher: BlockStateValueMatcher,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BlockPredicate {
    #[codec(optional)]
    pub blocks: Option<HolderSet<BlockKind>>,
    #[codec(optional)]
    pub properties: Option<Vec<BlockStatePropertyMatcher>>,
    #[codec(optional)]
    pub nbt: Option<NbtCompound>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct AdventureModePredicate {
    #[codec(with = CompactListCodec::<_>)]
    pub predicates: Vec<BlockPredicate>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CanPlaceOn {
    pub predicate: AdventureModePredicate,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CanBreak {
    pub predicate: AdventureModePredicate,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
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
#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct AttributeModifier {
    pub id: Identifier,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct AttributeModifiersEntry {
    #[codec(rename = "type")]
    pub kind: Attribute,
    #[codec(flatten)]
    pub modifier: AttributeModifier,
    pub slot: EquipmentSlotGroup,
    #[codec(optional)]
    pub display: AttributeModifierDisplay,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
#[codec(transparent)]
pub struct AttributeModifiers {
    pub modifiers: Vec<AttributeModifiersEntry>,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
#[codec(default_snake_case_key)]
#[codec(tag = "type")]
pub enum AttributeModifierDisplay {
    #[default]
    Default,
    Hidden,
    Override {
        text: FormattedText,
    },
}
azalea_codec::impl_defaultable!(AttributeModifierDisplay);

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct CustomModelData {
    #[codec(optional)]
    pub floats: Vec<f32>,
    #[codec(optional)]
    pub flags: Vec<bool>,
    #[codec(optional)]
    pub strings: Vec<String>,
    #[codec(optional)]
    pub colors: Vec<i32>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct RepairCost {
    #[var]
    pub cost: u32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct CreativeSlotLock;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct EnchantmentGlintOverride {
    pub show_glint: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct IntangibleProjectile;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct MobEffectDetails {
    #[var]
    #[codec(optional)]
    pub amplifier: i32,
    #[var]
    #[codec(optional)]
    pub duration: i32,
    #[codec(optional)]
    pub ambient: bool,
    #[codec(optional)]
    pub show_particles: bool,
    pub show_icon: bool,
    #[codec(optional)]
    pub hidden_effect: Option<Box<MobEffectDetails>>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct MobEffectInstance {
    pub id: MobEffect,
    #[codec(flatten)]
    pub details: MobEffectDetails,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct PossibleEffect {
    pub effect: MobEffectInstance,
    pub probability: f32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct Food {
    #[var]
    pub nutrition: i32,
    pub saturation: f32,
    #[codec(optional)]
    pub can_always_eat: bool,
}

#[derive(AzBuf, Default, Clone, Debug, PartialEq, Codec)]
pub struct ToolRule {
    pub blocks: HolderSet<BlockKind>,
    #[codec(optional)]
    pub speed: Option<f32>,
    #[codec(optional)]
    pub correct_for_drops: Option<bool>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Tool {
    pub rules: Vec<ToolRule>,
    #[codec(optional)]
    pub default_mining_speed: f32,
    #[var]
    #[codec(optional)]
    pub damage_per_block: i32,
    #[codec(optional)]
    pub can_destroy_blocks_in_creative: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct StoredEnchantments {
    #[var]
    pub enchantments: HashMap<Enchantment, i32>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct DyedColor {
    pub rgb: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MapId {
    #[var]
    pub id: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MapDecorations {
    pub decorations: NbtCompound,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ChargedProjectiles {
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BundleContents {
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct PotionContents {
    #[codec(optional)]
    pub potion: Option<Potion>,
    #[codec(optional)]
    pub custom_color: Option<i32>,
    #[codec(optional)]
    pub custom_effects: Vec<MobEffectInstance>,
    #[codec(optional)]
    pub custom_name: Option<String>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct SuspiciousStewEffect {
    #[codec(rename = "id")]
    pub effect: MobEffect,
    #[var]
    #[codec(optional)]
    pub duration: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct SuspiciousStewEffects {
    pub effects: Vec<SuspiciousStewEffect>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct WritableBookContent {
    #[codec(optional)]
    pub pages: Vec<Filterable<String>>,
}

#[derive(AzBuf, Clone, PartialEq, Debug, Codec)]
pub struct WrittenBookContent {
    #[limit(32)]
    pub title: Filterable<String>,
    pub author: String,
    #[var]
    #[codec(optional)]
    pub generation: i32,
    #[codec(optional)]
    pub pages: Vec<Filterable<FormattedText>>,
    #[codec(optional)]
    pub resolved: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Trim {
    pub material: TrimMaterial,
    pub pattern: TrimPattern,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct DebugStickState {
    pub properties: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct EntityData {
    #[codec(rename = "id")]
    pub kind: EntityKind,
    #[codec(flatten)]
    pub data: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BucketEntityData {
    pub entity: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BlockEntityData {
    #[codec(rename = "id")]
    pub kind: EntityKind,
    #[codec(flatten)]
    pub data: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Instrument {
    pub value: Holder<crate::data::Instrument, InstrumentData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct InstrumentData {
    pub sound_event: Holder<SoundEvent, azalea_core::sound::CustomSound>,
    pub use_duration: f32,
    pub range: f32,
    pub description: FormattedText,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct OminousBottleAmplifier {
    #[var]
    pub amplifier: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Recipes {
    pub recipes: Vec<Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct LodestoneTracker {
    #[codec(optional)]
    pub target: Option<GlobalPos>,
    #[codec(optional, default = true)]
    pub tracked: bool,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    #[codec(optional)]
    pub colors: Vec<i32>,
    #[codec(optional)]
    pub fade_colors: Vec<i32>,
    #[codec(optional)]
    pub has_trail: bool,
    #[codec(optional)]
    pub has_twinkle: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Fireworks {
    #[var]
    #[codec(optional)]
    pub flight_duration: i32,
    #[limit(256)]
    // TODO
    // #[codec(limit = 256)]
    #[codec(optional)]
    pub explosions: Vec<FireworkExplosion>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct NoteBlockSound {
    pub sound: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BannerPatternLayer {
    #[var]
    pub pattern: i32,
    #[var]
    pub color: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BannerPatterns {
    pub patterns: Vec<BannerPatternLayer>,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Default, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BaseColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct PotDecorations {
    #[codec(optional)]
    pub back: Option<ItemStackTemplate>,
    #[codec(optional)]
    pub left: Option<ItemStackTemplate>,
    #[codec(optional)]
    pub right: Option<ItemStackTemplate>,
    #[codec(optional)]
    pub front: Option<ItemStackTemplate>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Container {
    #[limit(256)]
    // #[codec(limit(256))]
    pub items: Vec<ItemStack>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BlockState {
    pub properties: HashMap<String, String>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BeehiveOccupant {
    #[codec(optional)]
    pub entity_data: NbtCompound,
    #[var]
    pub ticks_in_hive: i32,
    #[var]
    pub min_ticks_in_hive: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct Bees {
    pub occupants: Vec<BeehiveOccupant>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Lock {
    pub key: String,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct ContainerLoot {
    pub loot_table: NbtCompound,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct JukeboxPlayable {
    pub value: Holder<crate::data::JukeboxSong, JukeboxSongData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct JukeboxSongData {
    pub sound_event: Holder<SoundEvent, CustomSound>,
    pub description: FormattedText,
    pub length_in_seconds: f32,
    #[var]
    pub comparator_output: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Consumable {
    #[codec(optional, default = 1.6)]
    pub consume_seconds: f32,
    #[codec(optional, default = ItemUseAnimation::Eat)]
    pub animation: ItemUseAnimation,
    #[codec(optional, default = crate::Holder::Reference(SoundEvent::EntityGenericEat))]
    pub sound: crate::Holder<SoundEvent, CustomSound>,
    #[codec(optional, default = true)]
    pub has_consume_particles: bool,
    #[codec(optional)]
    pub on_consume_effects: Vec<ConsumeEffect>,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct UseRemainder {
    pub convert_into: ItemStackTemplate,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct UseCooldown {
    pub seconds: f32,
    #[codec(optional)]
    pub cooldown_group: Option<Identifier>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Enchantable {
    #[var]
    pub value: u32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Repairable {
    pub items: HolderSet<ItemKind>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ItemModel {
    pub resource_location: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct DamageResistant {
    pub types: HolderSet<DamageKind>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Equippable {
    pub slot: EquipmentSlot,
    #[codec(optional, default = SoundEvent::ItemArmorEquipGeneric)]
    pub equip_sound: SoundEvent,
    #[codec(optional)]
    pub asset_id: Option<Identifier>,
    #[codec(optional)]
    pub camera_overlay: Option<Identifier>,
    #[codec(optional)]
    pub allowed_entities: Option<HolderSet<EntityKind>>,
    #[codec(optional, default = true)]
    pub dispensable: bool,
    #[codec(optional, default = true)]
    pub swappable: bool,
    #[codec(optional, default = true)]
    pub damage_on_hurt: bool,
    #[codec(optional)]
    pub equip_on_interact: bool,
    #[codec(optional)]
    pub can_be_sheared: bool,
    #[codec(optional, default = SoundEvent::ItemShearsSnip)]
    pub shearing_sound: SoundEvent,
}

/// An enum that represents inventory slots that can hold items.
#[derive(AzBuf, Clone, Copy, Debug, Eq, Hash, PartialEq, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Glider;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct TooltipStyle {
    pub resource_location: Identifier,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct DeathProtection {
    pub death_effects: Vec<ConsumeEffect>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct Weapon {
    #[var]
    #[codec(optional, default = 1)]
    pub item_damage_per_attack: i32,
    #[codec(optional)]
    pub disable_blocking_for_seconds: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct PotionDurationScale {
    pub value: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct VillagerVariant {
    pub variant: VillagerKind,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct WolfVariant {
    pub variant: data::WolfVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct WolfCollar {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct FoxVariant {
    pub variant: FoxVariantKind,
}

registry! {
#[derive(Default)]
enum FoxVariantKind => "fox/variant" {
    #[default]
    Red => "red",
    Snow => "snow",
}
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum SalmonSize {
    Small,
    Medium,
    Large,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ParrotVariant {
    pub variant: ParrotVariantKind,
}
registry! {
enum ParrotVariantKind => "parrot/variant" {
    RedBlue => "red_blue",
    Blue => "blue",
    Green => "green",
    YellowBlue => "yellow_blue",
    Gray => "gray",
}
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct TropicalFishBaseColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct TropicalFishPatternColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MooshroomVariant {
    pub variant: MooshroomVariantKind,
}
registry! {
#[derive(Default)]
enum MooshroomVariantKind => "mooshroom/variant" {
    #[default]
    Red => "red",
    Brown => "brown",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct RabbitVariant {
    pub variant: RabbitVariantKind,
}
registry! {
#[derive(Default)]
enum RabbitVariantKind => "rabbit/variant" {
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct PigVariant {
    pub variant: data::PigVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct FrogVariant {
    pub variant: data::FrogVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct HorseVariant {
    pub variant: HorseVariantKind,
}
registry! {
#[derive(Default)]
enum HorseVariantKind => "horse/variant" {
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct PaintingVariant {
    pub variant: Holder<data::PaintingVariant, PaintingVariantData>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct PaintingVariantData {
    #[var]
    pub width: i32,
    #[var]
    pub height: i32,
    pub asset_id: Identifier,
    #[codec(optional)]
    pub title: Option<FormattedText>,
    #[codec(optional)]
    pub author: Option<FormattedText>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct LlamaVariant {
    pub variant: LlamaVariantKind,
}
registry! {
#[derive(Default)]
enum LlamaVariantKind => "llama/variant" {
    #[default]
    Creamy => "creamy",
    White => "white",
    Brown => "brown",
    Gray => "gray",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct AxolotlVariant {
    pub variant: AxolotlVariantKind,
}
registry! {
#[derive(Default)]
enum AxolotlVariantKind => "axolotl/variant" {
    #[default]
    Lucy => "lucy",
    Wild => "wild",
    Gold => "gold",
    Cyan => "cyan",
    Blue => "blue",
}
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CatVariant {
    pub variant: data::CatVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CatCollar {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct SheepColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ShulkerColor {
    pub color: DyeColor,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct TooltipDisplay {
    #[codec(optional)]
    pub hide_tooltip: bool,
    #[codec(optional)]
    pub hidden_components: Vec<DataComponentKind>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct BlocksAttacks {
    #[codec(optional)]
    pub block_delay_seconds: f32,
    #[codec(optional, default = 1.)]
    pub disable_cooldown_scale: f32,
    #[codec(optional)]
    pub damage_reductions: Vec<DamageReduction>,
    #[codec(optional)]
    pub item_damage: ItemDamageFunction,
    #[codec(optional)]
    pub bypassed_by: Option<HolderSet<DamageKind>>,
    #[codec(optional)]
    pub block_sound: Option<crate::Holder<SoundEvent, CustomSound>>,
    #[codec(optional)]
    pub disabled_sound: Option<crate::Holder<SoundEvent, CustomSound>>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct DamageReduction {
    #[codec(optional, default = 90.)]
    pub horizontal_blocking_angle: f32,
    #[codec(optional)]
    pub kind: Option<HolderSet<DamageKind>>,
    pub base: f32,
    pub factor: f32,
}
#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
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

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ProvidesTrimMaterial {
    pub value: Holder<TrimMaterial, DirectTrimMaterial>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct DirectTrimMaterial {
    #[codec(flatten)]
    pub assets: MaterialAssetGroup,
    pub description: FormattedText,
}
#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct MaterialAssetGroup {
    pub assert_name: AssetInfo,
    #[codec(optional)]
    pub override_armor_assets: Vec<(Identifier, AssetInfo)>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct AssetInfo {
    pub suffix: String,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ProvidesBannerPatterns {
    pub key: HolderSet<BannerPatternKind>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct BreakSound {
    pub sound: crate::Holder<SoundEvent, CustomSound>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct WolfSoundVariant {
    pub variant: crate::data::WolfSoundVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct CowVariant {
    pub variant: crate::data::CowVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ChickenVariant {
    pub data: crate::data::ChickenVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ZombieNautilusVariant {
    pub value: crate::data::ZombieNautilusVariant,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct UseEffects {
    #[codec(optional)]
    pub can_sprint: bool,
    #[codec(optional)]
    pub interact_vibrations: bool,
    #[codec(optional)]
    pub speed_multiplier: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct MinimumAttackCharge {
    pub value: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct DamageType {
    pub value: crate::data::DamageKind,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct PiercingWeapon {
    #[codec(optional, default = true)]
    pub deals_knockback: bool,
    #[codec(optional)]
    pub dismounts: bool,
    #[codec(optional)]
    pub sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
    #[codec(optional)]
    pub hit_sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct KineticWeapon {
    #[var]
    #[codec(optional, default = 10)]
    pub contact_cooldown_ticks: i32,
    #[var]
    #[codec(optional)]
    pub delay_ticks: i32,
    #[codec(optional)]
    pub dismount_conditions: Option<KineticWeaponCondition>,
    #[codec(optional)]
    pub knockback_conditions: Option<KineticWeaponCondition>,
    #[codec(optional)]
    pub damage_conditions: Option<KineticWeaponCondition>,
    #[codec(optional)]
    pub forward_movement: f32,
    #[codec(optional, default = 1.0)]
    pub damage_multiplier: f32,
    #[codec(optional)]
    pub sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
    #[codec(optional)]
    pub hit_sound: Option<Holder<SoundEvent, azalea_core::sound::CustomSound>>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct KineticWeaponCondition {
    #[var]
    pub max_duration_ticks: i32,
    #[codec(optional)]
    pub min_speed: f32,
    #[codec(optional)]
    pub min_relative_speed: f32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct AdditionalTradeCost {
    #[var]
    pub cost: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct Dye {
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct PigSoundVariant {
    pub value: crate::data::PigSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct CowSoundVariant {
    pub value: crate::data::CowSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct ChickenSoundVariant {
    pub value: crate::data::ChickenSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct CatSoundVariant {
    pub value: crate::data::CatSoundVariant,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct SulfurCubeContent {
    pub absorbed_block_item_stack: ItemStack,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct BlockTransformer {
    pub value: crate::data::BlockTransformer,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct ProvidesPotteryPattern {
    pub value: crate::data::DecoratedPotPattern,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct AttackAnimation {
    pub value: SwingAnimation,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct SwingAnimation {
    #[codec(rename = "type")]
    #[codec(optional)]
    pub kind: SwingAnimationKind,
    #[var]
    #[codec(optional)]
    pub duration: i32,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Default, Codec)]
#[codec(default_snake_case_key)]
pub enum SwingAnimationKind {
    #[default]
    None,
    Whack,
    Stab,
}
azalea_codec::impl_defaultable!(SwingAnimationKind);

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct InteractAnimation {
    pub value: SwingAnimation,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct VillagerFood {
    #[var]
    pub nutrition: i32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct Compostable {
    pub layers: ResolvableInt,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub enum ResolvableInt {
    Reference(Identifier),
    Constant(i32),
}
#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub enum ResolvableFloat {
    Reference(Identifier),
    Constant(f32),
}
impl Default for ResolvableInt {
    fn default() -> Self {
        Self::Constant(0)
    }
}
impl Default for ResolvableFloat {
    fn default() -> Self {
        Self::Constant(0.)
    }
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Codec)]
pub struct CookingFuel {
    pub burn_time: ResolvableInt,
    pub speed_multiplier: ResolvableFloat,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct BrewingFuel {
    pub uses: ResolvableInt,
    pub speed_multiplier: ResolvableFloat,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct MobVisibility {
    pub targeting_entity_types: HolderSet<EntityKind>,
    pub visibility: f32,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct SignTextFront {
    pub value: SignText,
}
#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
#[codec(transparent)]
pub struct SignTextBack {
    pub value: SignText,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Default, Codec)]
pub struct SignText {
    pub messages: [FormattedText; 4],
    #[codec(lenient)]
    pub filtered_messages: Option<[FormattedText; 4]>,
    // TODO
    // #[codec(optional_always_present)]
    #[codec(optional)]
    pub color: Option<DyeColor>,
    // #[codec(optional_always_present)]
    #[codec(optional)]
    pub has_glowing_text: bool,
}

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct Waxed;

#[derive(Clone, PartialEq, AzBuf, Debug, Codec)]
pub struct CushionColor(pub DyeColor);
