use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::position::{BlockPos, Vec3i};
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::builtin::TestInstanceKind;

use super::s_set_structure_block::Rotation;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundTestInstanceBlockAction {
    pub pos: BlockPos,
    pub action: Action,
    pub data: TestInstanceBlockEntityData,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum Action {
    #[default]
    Init,
    Query,
    Set,
    Reset,
    Save,
    Export,
    Run,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq)]
pub struct TestInstanceBlockEntityData {
    pub test: Option<TestInstanceKind>,
    pub size: Vec3i,
    pub rotation: Rotation,
    pub ignore_entities: bool,
    pub status: TestInstanceBlockEntityStatus,
    pub error_message: Option<FormattedText>,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum TestInstanceBlockEntityStatus {
    #[default]
    Cleared,
    Running,
    Finished,
}
