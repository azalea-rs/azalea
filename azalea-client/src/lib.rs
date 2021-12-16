//! Significantly abstract azalea-protocol so it's actually useable for bots.

pub mod connect;
pub mod ping;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
