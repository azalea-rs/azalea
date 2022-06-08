use azalea_crypto::SaltSignaturePair;
use packet_macros::{LoginPacket, McBuf};
use std::io::{Read, Write};

use crate::mc_buf::{McBufReadable, McBufWritable};

#[derive(Clone, Debug, McBuf, LoginPacket)]
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
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let is_nonce = bool::read_into(buf)?;
        if is_nonce {
            Ok(NonceOrSaltSignature::Nonce(Vec::<u8>::read_into(buf)?))
        } else {
            Ok(NonceOrSaltSignature::SaltSignature(
                SaltSignaturePair::read_into(buf)?,
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
