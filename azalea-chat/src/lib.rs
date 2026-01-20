#![doc = include_str!("../README.md")]

pub mod base_component;
pub mod click_event;
mod component;
pub mod hover_event;
#[cfg(feature = "numbers")]
pub mod numbers;
pub mod style;
pub mod text_component;
pub mod translatable_component;

pub use component::{DEFAULT_STYLE, FormattedText};
#[cfg(feature = "simdnbt")]
use simdnbt::{DeserializeError, FromNbtTag};

// TODO: remove this after simdnbt refactor
#[cfg(feature = "simdnbt")]
pub(crate) fn get_in_compound<T: FromNbtTag>(
    compound: &simdnbt::borrow::NbtCompound,
    key: &str,
) -> Result<T, DeserializeError> {
    let value = compound.get(key).ok_or(DeserializeError::MissingField)?;
    T::from_nbt_tag(value).ok_or(DeserializeError::MissingField)
}
