use azalea_buf::{McBufReadable, McBufVarWritable, McBufWritable};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, ClientboundGamePacket, McBufReadable)]
pub struct ClientboundInitializeBorderPacket {
    pub new_center_x: f64,
    pub new_center_z: f64,
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub lerp_time: u64,
    #[var]
    pub new_absolute_max_size: u32,
    #[var]
    pub warning_blocks: u32,
    #[var]
    pub warning_time: u32,
}

impl McBufWritable for ClientboundInitializeBorderPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.new_center_x.write_into(buf)?;
        self.new_center_z.write_into(buf)?;
        self.old_size.write_into(buf)?;
        self.new_size.write_into(buf)?;
        self.lerp_time.var_write_into(buf)?;
        // Odd
        0u8.write_into(buf)?;
        self.new_absolute_max_size.var_write_into(buf)?;
        self.warning_blocks.var_write_into(buf)?;
        self.warning_time.var_write_into(buf)?;
        Ok(())
    }
}
