use azalea_buf::{McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChatPacket {
    pub signature_or_id: SignatureOrId,
}

/// Messages can be deleted by either their signature or message id.
pub enum SignatureOrId {
    Signature(MessageSignature),
    Id(u32),
}

impl McBufReadable for SignatureOrId {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let id = u32::var_read_from(buf)?;
        if id == 0 {
            let full_signature = MessageSignature::read_from(buf)?;
            Ok(SignatureOrId::Signature(full_signature))
        } else {
            Ok(SignatureOrId::Id(id - 1))
        }
    }
}
impl McBufWritable for SignatureOrId {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            SignatureOrId::Signature(full_signature) => {
                0u32.var_write_into(buf);
                full_signature.write_into(buf);
            }
            SignatureOrId::Id(id) => {
                (id + 1).var_write_into(buf);
            }
        }
        Ok(())
    }
}
