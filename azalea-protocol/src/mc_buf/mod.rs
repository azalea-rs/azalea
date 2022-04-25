//! Utilities for reading and writing for the Minecraft protocol

mod read;
mod write;

pub use read::{McBufReadable, McBufVarintReadable, Readable};
pub use write::{McBufVarintWritable, McBufWritable, Writable};
use std::ops::Deref;

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteArray(Vec<u8>);

impl Deref for ByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use azalea_core::resource_location::ResourceLocation;
    use std::{collections::HashMap, io::Cursor};
    use tokio::io::BufReader;

    #[test]
    fn test_write_varint() {
        let mut buf = Vec::new();
        buf.write_varint(0).unwrap();
        assert_eq!(buf, vec![0]);

        let mut buf = Vec::new();
        buf.write_varint(1).unwrap();
        assert_eq!(buf, vec![1]);

        let mut buf = Vec::new();
        buf.write_varint(2).unwrap();
        assert_eq!(buf, vec![2]);

        let mut buf = Vec::new();
        buf.write_varint(127).unwrap();
        assert_eq!(buf, vec![127]);

        let mut buf = Vec::new();
        buf.write_varint(128).unwrap();
        assert_eq!(buf, vec![128, 1]);

        let mut buf = Vec::new();
        buf.write_varint(255).unwrap();
        assert_eq!(buf, vec![255, 1]);

        let mut buf = Vec::new();
        buf.write_varint(25565).unwrap();
        assert_eq!(buf, vec![221, 199, 1]);

        let mut buf = Vec::new();
        buf.write_varint(2097151).unwrap();
        assert_eq!(buf, vec![255, 255, 127]);

        let mut buf = Vec::new();
        buf.write_varint(2147483647).unwrap();
        assert_eq!(buf, vec![255, 255, 255, 255, 7]);

        let mut buf = Vec::new();
        buf.write_varint(-1).unwrap();
        assert_eq!(buf, vec![255, 255, 255, 255, 15]);

        let mut buf = Vec::new();
        buf.write_varint(-2147483648).unwrap();
        assert_eq!(buf, vec![128, 128, 128, 128, 8]);
    }

    #[tokio::test]
    async fn test_read_varint() {
        let mut buf = BufReader::new(Cursor::new(vec![0]));
        assert_eq!(buf.read_varint().await.unwrap(), 0);
        assert_eq!(buf.get_varint_size(0), 1);

        let mut buf = BufReader::new(Cursor::new(vec![1]));
        assert_eq!(buf.read_varint().await.unwrap(), 1);
        assert_eq!(buf.get_varint_size(1), 1);

        let mut buf = BufReader::new(Cursor::new(vec![2]));
        assert_eq!(buf.read_varint().await.unwrap(), 2);
        assert_eq!(buf.get_varint_size(2), 1);

        let mut buf = BufReader::new(Cursor::new(vec![127]));
        assert_eq!(buf.read_varint().await.unwrap(), 127);
        assert_eq!(buf.get_varint_size(127), 1);

        let mut buf = BufReader::new(Cursor::new(vec![128, 1]));
        assert_eq!(buf.read_varint().await.unwrap(), 128);
        assert_eq!(buf.get_varint_size(128), 2);

        let mut buf = BufReader::new(Cursor::new(vec![255, 1]));
        assert_eq!(buf.read_varint().await.unwrap(), 255);
        assert_eq!(buf.get_varint_size(255), 2);

        let mut buf = BufReader::new(Cursor::new(vec![221, 199, 1]));
        assert_eq!(buf.read_varint().await.unwrap(), 25565);
        assert_eq!(buf.get_varint_size(25565), 3);

        let mut buf = BufReader::new(Cursor::new(vec![255, 255, 127]));
        assert_eq!(buf.read_varint().await.unwrap(), 2097151);
        assert_eq!(buf.get_varint_size(2097151), 3);

        let mut buf = BufReader::new(Cursor::new(vec![255, 255, 255, 255, 7]));
        assert_eq!(buf.read_varint().await.unwrap(), 2147483647);
        assert_eq!(buf.get_varint_size(2147483647), 5);

        let mut buf = BufReader::new(Cursor::new(vec![255, 255, 255, 255, 15]));
        assert_eq!(buf.read_varint().await.unwrap(), -1);
        assert_eq!(buf.get_varint_size(-1), 5);

        let mut buf = BufReader::new(Cursor::new(vec![128, 128, 128, 128, 8]));
        assert_eq!(buf.read_varint().await.unwrap(), -2147483648);
        assert_eq!(buf.get_varint_size(-2147483648), 5);
    }

    #[tokio::test]
    async fn test_read_varint_longer() {
        let mut buf = BufReader::new(Cursor::new(vec![138, 56, 0, 135, 56, 123]));
        assert_eq!(buf.read_varint().await.unwrap(), 7178);
    }

    #[tokio::test]
    async fn test_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec!["a", "bc", "def"], |buf, s| buf.write_utf(s))
            .unwrap();

        // there's no read_list because idk how to do it in rust
        let mut buf = BufReader::new(Cursor::new(buf));

        let mut result = Vec::new();
        let length = buf.read_varint().await.unwrap();
        for _ in 0..length {
            result.push(buf.read_utf().await.unwrap());
        }

        assert_eq!(result, vec!["a", "bc", "def"]);
    }

    #[tokio::test]
    async fn test_int_id_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec![1, 2, 3], |buf, i| buf.write_varint(*i))
            .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let result = buf.read_int_id_list().await.unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_map() {
        let mut buf = Vec::new();
        buf.write_map(
            vec![("a", 1), ("bc", 23), ("def", 456)],
            Vec::write_utf,
            Vec::write_varint,
        )
        .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let mut result = Vec::new();
        let length = buf.read_varint().await.unwrap();
        for _ in 0..length {
            result.push((
                buf.read_utf().await.unwrap(),
                buf.read_varint().await.unwrap(),
            ));
        }

        assert_eq!(
            result,
            vec![
                ("a".to_string(), 1),
                ("bc".to_string(), 23),
                ("def".to_string(), 456)
            ]
        );
    }

    #[tokio::test]
    async fn test_nbt() {
        let mut buf = Vec::new();
        buf.write_nbt(&azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
            "hello world".to_string(),
            azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                "name".to_string(),
                azalea_nbt::Tag::String("Bananrama".to_string()),
            )])),
        )])))
        .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let result = buf.read_nbt().await.unwrap();
        assert_eq!(
            result,
            azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                "hello world".to_string(),
                azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                    "name".to_string(),
                    azalea_nbt::Tag::String("Bananrama".to_string()),
                )])),
            )]))
        );
    }

    #[tokio::test]
    async fn test_long() {
        let mut buf = Vec::new();
        buf.write_long(123456).unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        assert_eq!(buf.read_long().await.unwrap(), 123456);
    }

    #[tokio::test]
    async fn test_resource_location() {
        let mut buf = Vec::new();
        buf.write_resource_location(&ResourceLocation::new("minecraft:dirt").unwrap())
            .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        assert_eq!(
            buf.read_resource_location().await.unwrap(),
            ResourceLocation::new("minecraft:dirt").unwrap()
        );
    }
}
