//! Things for working with Minecraft chat messages, inspired by the Minecraft source code and prismarine-chat.

pub mod base_component;
pub mod component;
pub mod mutable_component;
pub mod style;
pub mod text_component;
pub mod translatable_component;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
