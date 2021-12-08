//! Things for working with Minecraft chat messages.

pub mod base_component;
pub mod component;
pub mod mutable_component;
pub mod text_component;
pub mod translatable_component;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
