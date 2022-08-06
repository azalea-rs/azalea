use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundInteractPacket {
    #[var]
    pub entity_id: u32,
    pub action: ActionType, // TODO: Does ServerboundInteractPacket$Action::getType, may not be implemented
    // TODO: {'args': 'packetbuffer', 'field': 'b', 'method': 'a(Lqx;)V', 'name': 'a', 'operation': 'interfacecall', 'target': 'ye$a', 'type': 'interface'}
    pub using_secondary_action: bool,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ActionType {
    Interact = 0,
    Attack = 1,
    InteractAt = 2,
}
