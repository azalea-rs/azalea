use crate::{read::BufReadError, McBufReadable, McBufWritable};
use std::io::{Cursor, Write};
use uuid::Uuid;

pub trait SerializableUuid {
    fn to_int_array(&self) -> [u32; 4];
    fn from_int_array(array: [u32; 4]) -> Self;
}

fn least_most_to_int_array(most: u64, least: u64) -> [u32; 4] {
    [
        (most >> 32) as u32,
        most as u32,
        (least >> 32) as u32,
        least as u32,
    ]
}

impl SerializableUuid for Uuid {
    fn to_int_array(&self) -> [u32; 4] {
        let most_significant_bits = (self.as_u128() >> 64) as u64;
        let least_significant_bits = (self.as_u128() & 0xffffffffffffffff) as u64;

        least_most_to_int_array(most_significant_bits, least_significant_bits)
    }

    fn from_int_array(array: [u32; 4]) -> Self {
        let most = ((array[0] as u64) << 32) | ((array[1] as u64) & 0xFFFFFFFF);
        let least = ((array[2] as u64) << 32) | ((array[3] as u64) & 0xFFFFFFFF);

        Uuid::from_u128(((most as u128) << 64) | least as u128)
    }
}

impl McBufReadable for Uuid {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Uuid::from_int_array([
            u32::read_from(buf)?,
            u32::read_from(buf)?,
            u32::read_from(buf)?,
            u32::read_from(buf)?,
        ]))
    }
}

impl McBufWritable for Uuid {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let [a, b, c, d] = self.to_int_array();
        a.write_into(buf)?;
        b.write_into(buf)?;
        c.write_into(buf)?;
        d.write_into(buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_int_array() {
        let u = Uuid::parse_str("6536bfed-8695-48fd-83a1-ecd24cf2a0fd").unwrap();
        assert_eq!(
            u.to_int_array(),
            [0x6536bfed, 0x869548fd, 0x83a1ecd2, 0x4cf2a0fd]
        );
    }

    #[test]
    fn from_int_array() {
        let u = Uuid::from_int_array([0x6536bfed, 0x869548fd, 0x83a1ecd2, 0x4cf2a0fd]);
        assert_eq!(u.to_string(), "6536bfed-8695-48fd-83a1-ecd24cf2a0fd");
    }

    #[test]
    fn read_write() {
        let u = Uuid::parse_str("6536bfed-8695-48fd-83a1-ecd24cf2a0fd").unwrap();
        let mut buf = Vec::new();
        u.write_into(&mut buf).unwrap();
        println!("{buf:?}");
        assert_eq!(buf.len(), 16);
        let u2 = Uuid::read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(u, u2);
    }
}
