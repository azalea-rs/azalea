use std::io::{Cursor, Write};

use azalea_buf::{McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDamageEventPacket {
    #[var]
    pub entity_id: u32,
    #[var]
    pub source_type_id: u32,
    pub source_cause_id: OptionalEntityId,
    pub source_direct_id: OptionalEntityId,
    pub source_position: Option<Vec3>,
}

#[derive(Clone, Debug)]
pub struct OptionalEntityId(pub Option<u32>);
impl McBufReadable for OptionalEntityId {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        match u32::var_read_from(buf)? {
            0 => Ok(OptionalEntityId(None)),
            id => Ok(OptionalEntityId(Some(id - 1))),
        }
    }
}
impl McBufWritable for OptionalEntityId {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.0 {
            Some(id) => (id + 1).var_write_into(buf),
            None => 0u32.var_write_into(buf),
        }
    }
}
