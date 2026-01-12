use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
    io::{self, Cursor},
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use derive_more::{Deref, DerefMut};

// note: this is here instead of in azalea-entity because azalea-world depends
// on it. and this isn't in azalea-world so azalea-protocol doesn't need to
// depend on azalea-world.

/// An entity ID used by Minecraft.
///
/// These IDs are picked by the server. Some server software (like Bungeecord)
/// may pick entity IDs per-player, so you should avoid relying on them for
/// identifying IDs (especially if you're using a shared world -- i.e. a swarm).
///
/// You might find [`Entity`] more useful, since that's an ID decided by us that
/// is likely to be correct across shared worlds. You could also use the
/// `EntityUuid` from `azalea_entity`, that one is unlikely to change even
/// across server restarts.
///
/// This serializes as a i32. Usually it's a VarInt in the protocol, but not
/// always. If you do need it to serialize as a VarInt, make sure to use use the
/// `#[var]` attribute.
///
/// [`Entity`]: bevy_ecs::entity::Entity
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Eq, PartialEq)]
pub struct MinecraftEntityId(pub i32);

impl Hash for MinecraftEntityId {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.0);
    }
}
impl nohash_hasher::IsEnabled for MinecraftEntityId {}

// we can't have the default be #[var] because mojang doesn't use varints for
// entities sometimes :(
impl AzaleaRead for MinecraftEntityId {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i32::azalea_read(buf).map(MinecraftEntityId)
    }
}
impl AzaleaWrite for MinecraftEntityId {
    fn azalea_write(&self, buf: &mut impl io::Write) -> io::Result<()> {
        i32::azalea_write(&self.0, buf)
    }
}
impl AzaleaReadVar for MinecraftEntityId {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i32::azalea_read_var(buf).map(MinecraftEntityId)
    }
}
impl AzaleaWriteVar for MinecraftEntityId {
    fn azalea_write_var(&self, buf: &mut impl io::Write) -> io::Result<()> {
        i32::azalea_write_var(&self.0, buf)
    }
}
impl Display for MinecraftEntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "eid({})", self.0)
    }
}
impl From<i32> for MinecraftEntityId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}
impl From<u32> for MinecraftEntityId {
    fn from(id: u32) -> Self {
        Self(id as i32)
    }
}
