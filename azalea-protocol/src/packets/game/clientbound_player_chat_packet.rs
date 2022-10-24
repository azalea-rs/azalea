use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::component::Component;
use azalea_core::BitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: PackedSignedMessageBody,
    pub unsigned_content: Option<Component>,
    pub filter_mask: FilterMask,
    pub chat_type: Option<Component>,
}

#[derive(Clone, Debug, McBuf)]
pub struct PackedSignedMessageBody {
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

#[derive(Clone, Debug, McBuf)]
pub struct PackedLastSeenMessages {
    pub entries: PackedMessageSignature,
}

/// Messages can be deleted by either their signature or message id.
#[derive(Clone, Debug)]
pub enum PackedMessageSignature {
    Signature(MessageSignature),
    Id(u32),
}

#[derive(Clone, Debug, McBuf)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

impl McBufReadable for PackedMessageSignature {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        if id == 0 {
            let full_signature = MessageSignature::read_from(buf)?;
            Ok(PackedMessageSignature::Signature(full_signature))
        } else {
            Ok(PackedMessageSignature::Id(id - 1))
        }
    }
}
impl McBufWritable for PackedMessageSignature {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            PackedMessageSignature::Signature(full_signature) => {
                0u32.var_write_into(buf);
                full_signature.write_into(buf);
            }
            PackedMessageSignature::Id(id) => {
                (id + 1).var_write_into(buf);
            }
        }
        Ok(())
    }
}
