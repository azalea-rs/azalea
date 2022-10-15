//! Utilities for reading and writing for the Minecraft protocol

#![feature(min_specialization)]
// these two are necessary for thiserror backtraces
#![feature(error_generic_member_access)]
#![feature(provide_any)]

mod definitions;
mod read;
mod serializable_uuid;
mod write;

pub use azalea_buf_macros::*;
pub use definitions::*;
pub use read::{BufReadError, McBufReadable, McBufVarReadable};
pub use serializable_uuid::*;
pub use write::{McBufVarWritable, McBufWritable};

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, io::Cursor};

    #[test]
    fn test_write_varint() {
        let mut buf = Vec::new();
        0.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![0]);

        let mut buf = Vec::new();
        1.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![1]);

        let mut buf = Vec::new();
        2.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![2]);

        let mut buf = Vec::new();
        127.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![127]);

        let mut buf = Vec::new();
        128.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![128, 1]);

        let mut buf = Vec::new();
        255.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![255, 1]);

        let mut buf = Vec::new();
        25565.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![221, 199, 1]);

        let mut buf = Vec::new();
        2097151.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![255, 255, 127]);

        let mut buf = Vec::new();
        2147483647.var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![255, 255, 255, 255, 7]);

        let mut buf = Vec::new();
        (-1).var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![255, 255, 255, 255, 15]);

        let mut buf = Vec::new();
        (-2147483648).var_write_into(&mut buf).unwrap();
        assert_eq!(buf, vec![128, 128, 128, 128, 8]);
    }

    #[test]
    fn test_read_varint() {
        // let buf = &mut &vec![0][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 0);
        let buf = vec![0];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 0);

        // let buf = &mut &vec![1][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 1);
        let buf = vec![1];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 1);

        // let buf = &mut &vec![2][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 2);
        let buf = vec![2];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 2);

        // let buf = &mut &vec![127][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 127);
        let buf = vec![127];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 127);

        // let buf = &mut &vec![128, 1][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 128);
        let buf = vec![128, 1];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 128);

        // let buf = &mut &vec![255, 1][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 255);
        let buf = vec![255, 1];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 255);

        // let buf = &mut &vec![221, 199, 1][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 25565);
        let buf = vec![221, 199, 1];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 25565);

        // let buf = &mut &vec![255, 255, 127][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 2097151);
        let buf = vec![255, 255, 127];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 2097151);

        // let buf = &mut &vec![255, 255, 255, 255, 7][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), 2147483647);
        let buf = vec![255, 255, 255, 255, 7];
        assert_eq!(
            i32::var_read_from(&mut Cursor::new(&buf)).unwrap(),
            2147483647
        );

        // let buf = &mut &vec![255, 255, 255, 255, 15][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), -1);
        let buf = vec![255, 255, 255, 255, 15];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), -1);

        // let buf = &mut &vec![128, 128, 128, 128, 8][..];
        // assert_eq!(i32::var_read_from(buf).unwrap(), -2147483648);
        let buf = vec![128, 128, 128, 128, 8];
        assert_eq!(
            i32::var_read_from(&mut Cursor::new(&buf)).unwrap(),
            -2147483648
        );
    }

    #[test]
    fn test_read_varint_longer() {
        let buf = vec![138, 56, 0, 135, 56, 123];
        assert_eq!(i32::var_read_from(&mut Cursor::new(&buf)).unwrap(), 7178);
    }

    #[test]
    fn test_list() {
        let original_vec = vec!["a".to_string(), "bc".to_string(), "def".to_string()];

        let mut buf = Vec::new();
        original_vec.write_into(&mut buf).unwrap();

        dbg!(&buf);

        let result = Vec::<String>::read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(result, original_vec);
    }

    #[test]
    fn test_int_id_list() {
        let mut buf = Vec::new();
        vec![1, 2, 3].var_write_into(&mut buf).unwrap();

        let result = Vec::<i32>::var_read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_map() {
        let original_map = HashMap::from([
            ("a".to_string(), 1),
            ("bc".to_string(), 23),
            ("def".to_string(), 456),
        ]);
        let mut buf = Vec::new();
        original_map.var_write_into(&mut buf).unwrap();

        let result = HashMap::<String, i32>::var_read_from(&mut Cursor::new(&buf)).unwrap();

        assert_eq!(result, original_map);
    }

    #[test]
    fn test_long() {
        let mut buf: Vec<u8> = Vec::new();
        123456u64.write_into(&mut buf).unwrap();

        assert_eq!(u64::read_from(&mut Cursor::new(&buf)).unwrap(), 123456);
    }
}
