#![doc = include_str!("../README.md")]

pub mod base_component;
mod click_event;
mod component;
pub mod hover_event;
#[cfg(feature = "numbers")]
pub mod numbers;
pub mod style;
pub mod text_component;
pub mod translatable_component;

pub use component::{DEFAULT_STYLE, FormattedText};
