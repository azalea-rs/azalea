//! Things for working with Minecraft chat messages.
//! This was inspired by Minecraft and prismarine-chat.

#[macro_use]
extern crate lazy_static;

pub mod base_component;
mod component;
pub mod style;
pub mod text_component;
pub mod translatable_component;

pub use component::Component;
