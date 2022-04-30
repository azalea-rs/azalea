//! Significantly abstract azalea-protocol so it's actually useable for bots.

mod connect;
pub mod ping;

pub use connect::{Account, Client, Event};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
