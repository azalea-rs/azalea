#![doc = include_str!("../README.md")]

mod decode;
mod encode;
mod error;
mod tag;

pub use error::Error;
pub use tag::*;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::tag::NbtCompound;

    use super::*;
    use azalea_buf::{McBufReadable, McBufWritable};

    #[test]
    fn mcbuf_nbt() {
        let mut buf = Vec::new();
        let tag = Nbt::Compound(NbtCompound::from_iter(vec![(
            "hello world".into(),
            Nbt::Compound(NbtCompound::from_iter(vec![(
                "name".into(),
                Nbt::String("Bananrama".into()),
            )])),
        )]));
        tag.write_into(&mut buf).unwrap();

        let mut buf = Cursor::new(&buf[..]);

        let result = Nbt::read_from(&mut buf).unwrap();
        assert_eq!(
            result,
            Nbt::Compound(NbtCompound::from_iter(vec![(
                "hello world".into(),
                Nbt::Compound(NbtCompound::from_iter(vec![(
                    "name".into(),
                    Nbt::String("Bananrama".into()),
                )])),
            )]))
        );
    }
}
