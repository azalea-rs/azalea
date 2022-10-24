use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_chat::component::Component;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: todo!(),
    pub unsigned_content: Option<Component>,
    // TODO: {'field': 'f.f', 'operation': 'write', 'type': 'enum'}
    // TODO: {'condition': 'f.f == sl$a.c', 'instructions': [{'field': 'f.e.toLongArray().length', 'operation': 'write', 'type': 'varint'}, {'field': 'f.e.toLongArray()', 'operation': 'write', 'type': 'long[]'}], 'operation': 'if'}
    // TODO: {'field': 'g.a', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'g.b', 'operation': 'write', 'type': 'chatcomponent'}
    pub chat_type: Option<Component>,
}

#[derive(McBuf)]
pub struct PackedSignedMessageBody {
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

pub struct PackedLastSeenMessages {
    pub entries: PackedMessageSignature,
}

/// Messages can be deleted by either their signature or message id.
#[derive(Clone, Debug)]
pub enum PackedMessageSignature {
    Signature(MessageSignature),
    Id(u32),
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
