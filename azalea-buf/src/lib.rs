//! Utilities for reading and writing for the Minecraft protocol

#![feature(min_specialization)]

mod definitions;
mod read;
mod serializable_uuid;
mod write;

pub use buf_macros::*;
pub use definitions::*;
pub use read::{read_varint_async, BufReadError, McBufReadable, McBufVarReadable, Readable};
pub use serializable_uuid::*;
pub use write::{McBufVarWritable, McBufWritable, Writable};

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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

    #[test]
    fn test_read_varint() {
        let mut buf = Cursor::new(vec![0]);
        assert_eq!(buf.read_varint().unwrap(), 0);
        assert_eq!(buf.get_varint_size(0), 1);

        let mut buf = Cursor::new(vec![1]);
        assert_eq!(buf.read_varint().unwrap(), 1);
        assert_eq!(buf.get_varint_size(1), 1);

        let mut buf = Cursor::new(vec![2]);
        assert_eq!(buf.read_varint().unwrap(), 2);
        assert_eq!(buf.get_varint_size(2), 1);

        let mut buf = Cursor::new(vec![127]);
        assert_eq!(buf.read_varint().unwrap(), 127);
        assert_eq!(buf.get_varint_size(127), 1);

        let mut buf = Cursor::new(vec![128, 1]);
        assert_eq!(buf.read_varint().unwrap(), 128);
        assert_eq!(buf.get_varint_size(128), 2);

        let mut buf = Cursor::new(vec![255, 1]);
        assert_eq!(buf.read_varint().unwrap(), 255);
        assert_eq!(buf.get_varint_size(255), 2);

        let mut buf = Cursor::new(vec![221, 199, 1]);
        assert_eq!(buf.read_varint().unwrap(), 25565);
        assert_eq!(buf.get_varint_size(25565), 3);

        let mut buf = Cursor::new(vec![255, 255, 127]);
        assert_eq!(buf.read_varint().unwrap(), 2097151);
        assert_eq!(buf.get_varint_size(2097151), 3);

        let mut buf = Cursor::new(vec![255, 255, 255, 255, 7]);
        assert_eq!(buf.read_varint().unwrap(), 2147483647);
        assert_eq!(buf.get_varint_size(2147483647), 5);

        let mut buf = Cursor::new(vec![255, 255, 255, 255, 15]);
        assert_eq!(buf.read_varint().unwrap(), -1);
        assert_eq!(buf.get_varint_size(-1), 5);

        let mut buf = Cursor::new(vec![128, 128, 128, 128, 8]);
        assert_eq!(buf.read_varint().unwrap(), -2147483648);
        assert_eq!(buf.get_varint_size(-2147483648), 5);
    }

    #[test]
    fn test_read_varint_longer() {
        let mut buf = Cursor::new(vec![138, 56, 0, 135, 56, 123]);
        assert_eq!(buf.read_varint().unwrap(), 7178);
    }

    #[test]
    fn test_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec!["a", "bc", "def"], |buf, s| buf.write_utf(s))
            .unwrap();

        // there's no read_list because idk how to do it in rust
        let mut buf = Cursor::new(buf);

        let mut result = Vec::new();
        let length = buf.read_varint().unwrap();
        for _ in 0..length {
            result.push(buf.read_utf().unwrap());
        }

        assert_eq!(result, vec!["a", "bc", "def"]);
    }

    #[test]
    fn test_int_id_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec![1, 2, 3], |buf, i| buf.write_varint(*i))
            .unwrap();

        let mut buf = Cursor::new(buf);

        let result = buf.read_int_id_list().unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_map() {
        let mut buf = Vec::new();
        buf.write_map(
            vec![("a", 1), ("bc", 23), ("def", 456)],
            Vec::write_utf,
            Vec::write_varint,
        )
        .unwrap();

        let mut buf = Cursor::new(buf);

        let mut result = Vec::new();
        let length = buf.read_varint().unwrap();
        for _ in 0..length {
            result.push((buf.read_utf().unwrap(), buf.read_varint().unwrap()));
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

    #[test]
    fn test_long() {
        let mut buf = Vec::new();
        buf.write_long(123456).unwrap();

        let mut buf = Cursor::new(buf);

        assert_eq!(buf.read_long().unwrap(), 123456);
    }
}
