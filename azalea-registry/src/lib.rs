#![doc = include_str!("../README.md")]
// used for `azalea_codec::types::ConstFromStr` impls
#![feature(const_trait_impl)]
#![feature(const_cmp)]

// The contents of the macros below are generated in
// codegen/lib/code/registry.py, though the rest of the file isn't
// auto-generated (so you can add doc comments to the registry enums if you
// want).

extern crate self as azalea_registry;

pub mod builtin;
pub mod data;
pub mod data_registry;
pub mod registry_holder;
pub mod tags;

use std::{
    fmt::{self, Debug},
    hash::Hash,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzBufVar, BufReadError, BufReadErrorRepr};
use azalea_codec::Codec;
use azalea_core::identifier::Identifier;
pub use azalea_registry_macros as macros;

#[doc(hidden)]
pub mod identifier {
    #[deprecated]
    pub type Identifier = azalea_core::identifier::Identifier;
}

// TODO: remove this next update
macro_rules! define_deprecated_builtin {
    ($($r:ident) *) => {
        $(
            #[doc(hidden)]
            #[deprecated = concat!("moved to `azalea_registry::builtin::", stringify!($r), "`")]
            pub type $r = builtin::$r;
        )*
    };
}
define_deprecated_builtin!(Activity Attribute BlockEntityKind BlockPredicateKind ChunkStatus CommandArgumentKind CustomStat EntityKind FloatProviderKind Fluid GameEvent HeightProviderKind IntProviderKind LootConditionKind LootFunctionKind LootNbtProviderKind LootPoolEntryKind LootScoreProviderKind MemoryModuleKind MobEffect ParticleKind PointOfInterestKind PosRuleTest PositionSourceKind Potion RecipeSerializer RecipeKind RuleTest SensorKind SoundEvent StatKind VillagerProfession VillagerKind WorldgenBiomeSource WorldgenBlockStateProviderKind WorldgenChunkGenerator WorldgenDensityFunctionKind WorldgenFeatureSizeKind WorldgenFoliagePlacerKind WorldgenPlacementModifierKind WorldgenRootPlacerKind WorldgenStructurePiece WorldgenStructurePlacement WorldgenStructurePoolElement WorldgenStructureProcessor WorldgenStructureKind WorldgenTreeDecoratorKind WorldgenTrunkPlacerKind RuleBlockEntityModifier CreativeModeTab MenuKind WorldgenPoolAliasBinding TriggerKind NumberFormatKind DataComponentKind EntitySubPredicateKind MapDecorationKind EnchantmentEffectComponentKind EnchantmentEntityEffectKind EnchantmentLevelBasedValueKind EnchantmentLocationBasedEffectKind EnchantmentProviderKind EnchantmentValueEffectKind ConsumeEffectKind RecipeBookCategory RecipeDisplay SlotDisplay TicketKind TestEnvironmentDefinitionKind TestFunction TestInstanceKind DataComponentPredicateKind SpawnConditionKind DialogBodyKind DialogKind InputControlKind DialogActionKind DebugSubscription IncomingRpcMethods OutgoingRpcMethods AttributeKind EnvironmentAttribute GameRule PermissionCheckKind PermissionKind SlotSourceKind);
macro_rules! define_deprecated_data {
    ($($r:ident) *) => {
        $(
            #[doc(hidden)]
            #[deprecated = concat!("moved to `azalea_registry::data::", stringify!($r), "`")]
            pub type $r = data::$r;
        )*
    };
}
define_deprecated_data!(Enchantment DamageKind Dialog WolfSoundVariant CowVariant ChickenVariant FrogVariant CatVariant PigVariant PaintingVariant WolfVariant ZombieNautilusVariant Biome);

#[doc(hidden)]
#[deprecated = "renamed to `azalea_registry::builtin::ItemKind`"]
pub type Item = builtin::ItemKind;
#[doc(hidden)]
#[deprecated = "renamed to `azalea_registry::builtin::BlockKind`"]
pub type Block = builtin::BlockKind;
#[doc(hidden)]
#[deprecated = "renamed to `azalea_registry::data::DimensionKind`"]
pub type DimensionType = data::DimensionKind;

pub trait Registry: AzBuf + PartialEq + PartialOrd + Ord + Copy + Hash
where
    Self: Sized,
{
    fn from_u32(value: u32) -> Option<Self>;
    fn to_u32(&self) -> u32;
}

/// A registry that might not be present.
///
/// This is transmitted as a single varint in the protocol.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OptionalRegistry<T: Registry>(pub Option<T>);

impl<T: Registry> AzBuf for OptionalRegistry<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(OptionalRegistry(match u32::azalea_read_var(buf)? {
            0 => None,
            value => Some(
                T::from_u32(value - 1)
                    .ok_or(BufReadErrorRepr::UnexpectedEnumVariant { id: value as i32 })?,
            ),
        }))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match &self.0 {
            None => 0u32.azalea_write_var(buf),
            Some(value) => (value.to_u32() + 1).azalea_write_var(buf),
        }
    }
}

/// A registry that will either take an ID or a resource location.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CustomRegistry<D: Registry, C: AzBuf> {
    Direct(D),
    Custom(C),
}

impl<D: Registry, C: AzBuf> AzBuf for CustomRegistry<D, C> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let direct_registry = OptionalRegistry::<D>::azalea_read(buf)?;
        if let Some(direct_registry) = direct_registry.0 {
            return Ok(CustomRegistry::Direct(direct_registry));
        }
        Ok(CustomRegistry::Custom(C::azalea_read(buf)?))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            CustomRegistry::Direct(direct_registry) => {
                // write the id + 1
                (direct_registry.to_u32() + 1).azalea_write_var(buf)
            }
            CustomRegistry::Custom(custom_registry) => {
                // write 0, then the custom registry
                0u32.azalea_write_var(buf)?;
                custom_registry.azalea_write(buf)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum HolderSet<D: Registry> {
    Direct {
        contents: Vec<D>,
    },
    Named {
        key: Identifier,
        contents: Vec<Identifier>,
    },
}
impl<D: Registry> AzBuf for HolderSet<D> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let size = i32::azalea_read_var(buf)?.wrapping_sub(1);
        if size == -1 {
            let key = Identifier::azalea_read(buf)?;
            Ok(Self::Named {
                key,
                contents: Vec::new(),
            })
        } else {
            let mut contents = Vec::new();
            for _ in 0..size {
                contents.push(D::azalea_read(buf)?);
            }
            Ok(Self::Direct { contents })
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Direct { contents } => {
                (contents.len() as i32 + 1).azalea_write_var(buf)?;
                for item in contents {
                    item.azalea_write(buf)?;
                }
            }
            Self::Named { key, contents: _ } => {
                0i32.azalea_write_var(buf)?;
                key.azalea_write(buf)?;
            }
        }
        Ok(())
    }
}
impl<D: Registry + Debug> Debug for HolderSet<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct { contents } => f.debug_list().entries(contents).finish(),
            Self::Named { key, contents } => f
                .debug_struct("Named")
                .field("key", key)
                .field("contents", contents)
                .finish(),
        }
    }
}
impl<D: Registry> From<Vec<D>> for HolderSet<D> {
    fn from(contents: Vec<D>) -> Self {
        Self::Direct { contents }
    }
}
impl<D: Registry + Codec<O>, O: azalea_codec::Ops> Codec<O> for HolderSet<D> {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        if let Ok(key) = Identifier::codec_read(ops, ctx) {
            return Ok(Self::Named {
                key,
                contents: vec![],
            });
        }
        if let Ok(d) = D::codec_read(ops, ctx) {
            return Ok(Self::Direct { contents: vec![d] });
        }
        if let Ok(contents) = Vec::<D>::codec_read(ops, ctx) {
            return Ok(Self::Direct { contents });
        }

        Err(azalea_codec::error::ReadError::custom("invalid holderset"))
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        match self {
            Self::Direct { contents } => {
                if contents.len() == 1 {
                    contents[0].codec_write(ctx)
                } else {
                    contents.codec_write(ctx)
                }
            }
            Self::Named { key, contents: _ } => key.codec_write(ctx),
        }
    }
}

impl<D: Registry> Default for HolderSet<D> {
    fn default() -> Self {
        Self::Direct {
            contents: Vec::new(),
        }
    }
}

impl<O: azalea_codec::Ops, D: Registry + Codec<O>> azalea_codec::Defaultable<O> for HolderSet<D> {
    fn is_default(&self) -> bool {
        match self {
            Self::Direct { contents } => contents.is_empty(),
            _ => false,
        }
    }
}

impl<D: Registry> HolderSet<D>
where
    Identifier: From<D>,
{
    pub fn contains(&self, value: D) -> bool {
        match self {
            HolderSet::Direct { contents } => contents.contains(&value),
            HolderSet::Named { key: _, contents } => contents.contains(&Identifier::from(value)),
        }
    }
}

/// A reference to either a registry or a custom value (usually something with
/// an `Identifier`).
pub enum Holder<R: Registry, Direct: AzBuf> {
    Reference(R),
    Direct(Direct),
}
impl<R: Registry, Direct: AzBuf> AzBuf for Holder<R, Direct> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::azalea_read_var(buf)?;
        if id == 0 {
            Ok(Self::Direct(Direct::azalea_read(buf)?))
        } else {
            let id = id - 1;
            let Some(value) = R::from_u32(id) else {
                return Err(BufReadErrorRepr::UnexpectedEnumVariant { id: id as i32 }.into());
            };
            Ok(Self::Reference(value))
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Reference(value) => (value.to_u32() + 1).azalea_write_var(buf),
            Self::Direct(value) => {
                0u32.azalea_write_var(buf)?;
                value.azalea_write(buf)
            }
        }
    }
}
impl<R: Registry + Debug, Direct: AzBuf + Debug> Debug for Holder<R, Direct> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(value) => f.debug_tuple("Reference").field(value).finish(),
            Self::Direct(value) => f.debug_tuple("Direct").field(value).finish(),
        }
    }
}
impl<R: Registry + Clone, Direct: AzBuf + Clone> Clone for Holder<R, Direct> {
    fn clone(&self) -> Self {
        match self {
            Self::Reference(value) => Self::Reference(*value),
            Self::Direct(value) => Self::Direct(value.clone()),
        }
    }
}
impl<R: Registry + PartialEq, Direct: AzBuf + PartialEq> PartialEq for Holder<R, Direct> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Reference(a), Self::Reference(b)) => a == b,
            (Self::Direct(a), Self::Direct(b)) => a == b,
            _ => false,
        }
    }
}
impl<R: Registry + Default, Direct: AzBuf> Default for Holder<R, Direct> {
    fn default() -> Self {
        Self::Reference(R::default())
    }
}

impl<O: azalea_codec::Ops, R: Registry + Codec<O>, Direct: Codec<O> + AzBuf> Codec<O>
    for Holder<R, Direct>
{
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        if let Ok(reference) = R::codec_read(ops, ctx) {
            return Ok(Self::Reference(reference));
        };
        Direct::codec_read(ops, ctx).map(Self::Direct)
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        match self {
            Self::Reference(value) => value.codec_write(ctx),
            Self::Direct(value) => value.codec_write(ctx),
        }
    }
}

/// A registry which has its values decided by the server in the
/// `ClientboundRegistryData` packet.
///
/// These can be resolved into their actual values with
/// `ResolvableDataRegistry` from azalea-core.
pub trait DataRegistry: AzBuf + PartialEq + PartialOrd + Ord + Copy + Hash {
    const NAME: &'static str;
    type Key: DataRegistryKey;

    fn protocol_id(&self) -> u32;
    fn new_raw(id: u32) -> Self;
}
pub trait DataRegistryKey {
    type Borrow<'a>: DataRegistryKeyRef<'a>;

    fn from_ident(ident: Identifier) -> Self;
    fn into_ident(self) -> Identifier;
}
pub trait DataRegistryKeyRef<'a> {
    type Owned: DataRegistryKey;

    fn to_owned(self) -> Self::Owned;
    fn from_ident(ident: &'a Identifier) -> Self;
    fn into_ident(self) -> Identifier;
}
impl<T: DataRegistry> Registry for T {
    fn from_u32(value: u32) -> Option<Self> {
        Some(Self::new_raw(value))
    }
    fn to_u32(&self) -> u32 {
        self.protocol_id()
    }
}
