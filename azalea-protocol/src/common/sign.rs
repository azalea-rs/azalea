use azalea_buf::AzBuf;

/// The editable side of a sign.
///
/// Minecraft 26.3 serializes these IDs directly with its `SignTextSlot` codec.
#[derive(AzBuf, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignTextSlot {
    Back = 0,
    Front = 1,
}
