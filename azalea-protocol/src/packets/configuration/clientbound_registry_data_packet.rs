use azalea_buf::McBuf;
use azalea_core::registry_holder::RegistryHolder;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundRegistryDataPacket {
    pub registry_holder: RegistryHolder,
}
