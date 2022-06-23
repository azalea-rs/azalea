mod decode;
mod encode;
mod error;
mod tag;

pub use error::Error;
pub use tag::Tag;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, io::Cursor};

    #[test]
    fn mcbuf_nbt() {
        let mut buf = Vec::new();
        buf.write_nbt(&Tag::Compound(HashMap::from_iter(vec![(
            "hello world".to_string(),
            Tag::Compound(HashMap::from_iter(vec![(
                "name".to_string(),
                Tag::String("Bananrama".to_string()),
            )])),
        )])))
        .unwrap();

        let mut buf = Cursor::new(buf);

        let result = buf.read_nbt().unwrap();
        assert_eq!(
            result,
            Tag::Compound(HashMap::from_iter(vec![(
                "hello world".to_string(),
                Tag::Compound(HashMap::from_iter(vec![(
                    "name".to_string(),
                    Tag::String("Bananrama".to_string()),
                )])),
            )]))
        );
    }
}
