use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWriteVar, AzaleaWrite};
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundDamageEvent {
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
impl AzaleaRead for OptionalEntityId {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        match u32::azalea_read_var(buf)? {
            0 => Ok(OptionalEntityId(None)),
            id => Ok(OptionalEntityId(Some(id - 1))),
        }
    }
}
impl AzaleaWrite for OptionalEntityId {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.0 {
            Some(id) => (id + 1).azalea_write_var(buf),
            None => 0u32.azalea_write_var(buf),
        }
    }
}
