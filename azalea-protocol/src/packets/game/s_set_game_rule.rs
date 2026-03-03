use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::identifier::Identifier;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetGameRule {
    pub entries: Vec<GameRuleEntry>,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct GameRuleEntry {
    /// An identifier for a [`GameRule`](azalea_registry::builtin::GameRule).
    pub key: Identifier,
    pub value: String,
}
