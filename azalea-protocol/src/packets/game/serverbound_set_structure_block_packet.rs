use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetStructureBlockPacket {
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
pub update_type: UpdateType,
pub mode: StructureMode,
pub name: String,
pub offset: u8, // TODO: Does BlockPos::u, may not be implemented
pub offset: u8, // TODO: Does BlockPos::v, may not be implemented
pub offset: u8, // TODO: Does BlockPos::w, may not be implemented
pub size: u8, // TODO: Does Vec3i::getX, may not be implemented
pub size: u8, // TODO: Does Vec3i::getY, may not be implemented
pub size: u8, // TODO: Does Vec3i::getZ, may not be implemented
pub mirror: Mirror,
pub rotation: Rotation,
pub data: String,
pub integrity: f32,
#[var]
pub seed: u64,
// TODO: {'operation': 'store', 'type': 'int', 'value': '0', 'var': 'var2'}
// TODO: {'condition': 'n', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 1)', 'var': 'var2'}], 'operation': 'if'}
// TODO: {'condition': 'o', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 2)', 'var': 'var2'}], 'operation': 'if'}
// TODO: {'condition': 'p', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 4)', 'var': 'var2'}], 'operation': 'if'}
// TODO: unknown field {'field': 'var2', 'operation': 'write', 'type': 'byte'}
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum UpdateType {
    UpdateData=0,
    SaveArea=1,
    LoadArea=2,
    ScanArea=3,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum StructureMode {
    Save=0,
    Load=1,
    Corner=2,
    Data=3,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Mirror {
    None=0,
    LeftRight=1,
    FrontBack=2,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Rotation {
    None=0,
    Clockwise90=1,
    Clockwise180=2,
    Counterclockwise90=3,
}