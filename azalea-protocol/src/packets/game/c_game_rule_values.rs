use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::GameRule;
use indexmap::IndexMap;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundGameRuleValues {
    pub values: IndexMap<GameRule, String>,
}
