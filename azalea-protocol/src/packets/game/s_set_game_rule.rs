use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::builtin::GameRule;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetGameRule {
    pub entries: GameRuleEntry,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct GameRuleEntry {
    pub key: GameRule,
    pub value: String,
}
