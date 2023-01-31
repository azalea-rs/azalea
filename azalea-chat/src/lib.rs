#![doc = include_str!("../README.md")]

pub mod base_component;
mod component;
pub mod style;
pub mod text_component;
pub mod translatable_component;

pub use component::FormattedText;
