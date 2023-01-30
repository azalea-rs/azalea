use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_protocol_macros::ClientboundGamePacket;
use std::{
    io::{Cursor, Write},
    ops::Not,
};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundSetScorePacket {
    pub owner: String,
    pub method: Method,
    pub objective_name: Option<String>,
}

impl McBufReadable for ClientboundSetScorePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let owner = String::read_from(buf)?;
        let method_id = u32::var_read_from(buf)?;
        let objective_name = String::read_from(buf)?;
        let objective_name = objective_name.is_empty().not().then_some(objective_name);
        // if it's change, read the score
        let method = match method_id {
            0 => Method::Change {
                score: u32::var_read_from(buf)?,
            },
            1 => Method::Remove,
            id => return Err(BufReadError::UnexpectedEnumVariant { id: id as i32 }),
        };
        Ok(ClientboundSetScorePacket {
            owner,
            method,
            objective_name,
        })
    }
}

impl McBufWritable for ClientboundSetScorePacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.owner.write_into(buf)?;
        match self.method {
            Method::Change { .. } => 0u32,
            Method::Remove => 1u32,
        }
        .var_write_into(buf)?;
        // convert None to an empty string
        self.objective_name
            .as_ref()
            .unwrap_or(&String::new())
            .write_into(buf)?;
        if let Method::Change { score } = self.method {
            score.var_write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Method {
    Change { score: u32 },
    Remove,
}
