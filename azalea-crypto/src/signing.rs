use azalea_buf::{McBufReadable, McBufWritable};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct SaltSignaturePair {
    pub salt: u64,
    pub signature: Vec<u8>,
}

impl McBufReadable for SaltSignaturePair {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let salt = u64::read_into(buf)?;
        let signature = Vec::<u8>::read_into(buf)?;
        Ok(SaltSignaturePair { salt, signature })
    }
}

impl McBufWritable for SaltSignaturePair {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.salt.write_into(buf)?;
        self.signature.write_into(buf)?;
        Ok(())
    }
}
