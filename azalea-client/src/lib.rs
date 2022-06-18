//! Significantly abstract azalea-protocol so it's actually useable for bots.

mod connect;
pub mod ping;
mod player;

pub use connect::{Account, Client, Event};
pub use player::Player;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
