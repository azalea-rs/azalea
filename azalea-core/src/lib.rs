//! Random miscellaneous things like UUIDs that don't deserve their own crate.

pub mod resource_location;
pub mod serializable_uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
