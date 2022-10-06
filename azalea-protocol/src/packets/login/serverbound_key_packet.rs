use azalea_buf::{BufReadError, McBuf};
use azalea_crypto::SaltSignaturePair;
use azalea_protocol_macros::ServerboundLoginPacket;
use std::io::{Cursor, Write};

use azalea_buf::{McBufReadable, McBufWritable};

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundKeyPacket {
    pub key_bytes: Vec<u8>,
    pub nonce_or_salt_signature: NonceOrSaltSignature,
}

#[derive(Clone, Debug)]
pub enum NonceOrSaltSignature {
    Nonce(Vec<u8>),
    SaltSignature(SaltSignaturePair),
}

impl McBufReadable for NonceOrSaltSignature {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let is_nonce = bool::read_from(buf)?;
        if is_nonce {
            Ok(NonceOrSaltSignature::Nonce(Vec::<u8>::read_from(buf)?))
        } else {
            Ok(NonceOrSaltSignature::SaltSignature(
                SaltSignaturePair::read_from(buf)?,
            ))
        }
    }
}

impl McBufWritable for NonceOrSaltSignature {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            NonceOrSaltSignature::Nonce(nonce) => {
                bool::write_into(&true, buf)?;
                nonce.write_into(buf)?;
            }
            NonceOrSaltSignature::SaltSignature(salt_signature) => {
                bool::write_into(&false, buf)?;
                salt_signature.write_into(buf)?;
            }
        }
        Ok(())
    }
}
