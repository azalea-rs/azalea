use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};

use crate::Registry;

/// A registry which has its values decided by the server in the
/// `ClientboundRegistryData` packet.
///
/// These can be resolved into their actual values with
/// `ResolvableDataRegistry` from azalea-core.
pub trait DataRegistry: AzaleaRead + AzaleaWrite {
    const NAME: &'static str;

    fn protocol_id(&self) -> u32;
    fn new_raw(id: u32) -> Self;
}
impl<T: DataRegistry> Registry for T {
    fn from_u32(value: u32) -> Option<Self> {
        Some(Self::new_raw(value))
    }

    fn to_u32(&self) -> u32 {
        self.protocol_id()
    }
}

macro_rules! data_registry {
    ($(#[$doc:meta])* $name:ident, $registry_name:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, AzBuf, PartialEq, Eq, Hash)]
        pub struct $name {
            #[var]
            id: u32,
        }
        impl DataRegistry for $name {
            const NAME: &'static str = $registry_name;
            fn protocol_id(&self) -> u32 {
                self.id
            }
            fn new_raw(id: u32) -> Self {
                Self { id }
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // see ChecksumSerializer::serialize_newtype_variant
                serializer.serialize_newtype_variant(concat!("minecraft:", $registry_name), self.id, "", &())
            }
        }
    };
}

// TODO: these should be represented as an enum with like a "Custom(u32)"
// variant, this is necessary to have a correct `impl DefaultableComponent for
// DamageType`

data_registry! {Enchantment, "enchantment"}
data_registry! {DimensionType, "dimension_type"}
data_registry! {DamageKind, "damage_kind"}
data_registry! {Dialog, "dialog"}

// entity variants
data_registry! {WolfSoundVariant, "wolf_sound_variant"}
data_registry! {CowVariant, "cow_variant"}
data_registry! {ChickenVariant, "chicken_variant"}
data_registry! {FrogVariant, "frog_variant"}
data_registry! {CatVariant, "cat_variant"}
data_registry! {PigVariant, "pig_variant"}
data_registry! {PaintingVariant, "painting_variant"}
data_registry! {WolfVariant, "wolf_variant"}
data_registry! {ZombieNautilusVariant, "zombie_nautilus_variant"}

data_registry! {
    /// An opaque biome identifier.
    ///
    /// You'll probably want to resolve this into its name before using it, by
    /// using `Client::with_resolved_registry` or a similar function.
    Biome,
    "worldgen/biome"
}

// these extra traits are required for Biome to be allowed to be palletable
impl Default for Biome {
    fn default() -> Self {
        Self::new_raw(0)
    }
}
impl From<u32> for Biome {
    fn from(id: u32) -> Self {
        Self::new_raw(id)
    }
}
impl From<Biome> for u32 {
    fn from(biome: Biome) -> Self {
        biome.protocol_id()
    }
}
