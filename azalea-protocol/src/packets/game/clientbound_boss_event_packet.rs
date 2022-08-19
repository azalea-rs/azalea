use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBossEventPacket {
    pub id: Uuid,
    pub operation: OperationType, // TODO: Does ClientboundBossEventPacket$Operation::getType, may not be implemented
    // TODO: {'args': 'packetbuffer', 'field': 'e', 'method': 'a(Lqx;)V', 'name': 'a', 'operation': 'interfacecall', 'target': 'tq$c', 'type': 'interface'}
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum OperationType {
    Add {
        pub name: Component,
        pub progress: f32,
        pub color: BossBarColor,
        pub overlay: BossBarOverlay,
        TODO: these are bitflags
        pub darken_screen: bool,
        pub play_music: bool,
        pub create_world_fog: bool,
    },
    Remove = 1,
    UpdateProgress = 2,
    UpdateName = 3,
    UpdateStyle = 4,
    UpdateProperties = 5,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BossBarOverlay {
    Progress = 0,
    Notched6 = 1,
    Notched10 = 2,
    Notched12 = 3,
    Notched20 = 4,
}