use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Debug, Clone, McBuf)]
pub struct SaltSignaturePair {
    pub salt: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MessageSignature {
    pub bytes: [u8; 256],
}
impl McBufReadable for MessageSignature {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut bytes = [0; 256];
        println!(
            "remaining bytes: {:?}",
            &buf.get_ref()[buf.position() as usize..]
        );
        std::io::Read::read_exact(buf, &mut bytes)?;
        Ok(MessageSignature { bytes })
    }
}
impl McBufWritable for MessageSignature {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_all(&self.bytes)?;
        Ok(())
    }
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct SignedMessageHeader {
    pub previous_signature: Option<MessageSignature>,
    pub sender: Uuid,
}

/// Generates a random u64 to use as a salt
pub fn make_salt() -> u64 {
    rand::random()
}
