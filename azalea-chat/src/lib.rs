#![doc = include_str!("../README.md")]

pub mod click_event;
pub mod component;
pub mod hover_event;
pub mod style;

pub use component::{DEFAULT_STYLE, FormattedText};

pub mod translatable_component {
    #![deprecated]
    pub type TranslatableComponent = crate::component::translatable::TranslatableComponent;
    pub type PrimitiveOrComponent = crate::component::translatable::PrimitiveOrComponent;
}
pub mod text_component {
    #![deprecated]
    pub type TextComponent = crate::component::text::TextComponent;
}
