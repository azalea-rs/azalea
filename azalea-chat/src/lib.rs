#![feature(cursor_remaining)]
#![doc = include_str!("../README.md")]

pub mod base_component;
mod component;
#[cfg(feature = "numbers")]
pub mod numbers;
pub mod style;
pub mod text_component;
pub mod translatable_component;

pub use component::FormattedText;
