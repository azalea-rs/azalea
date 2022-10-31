use azalea_block::BlockState;
use azalea_buf::{BufReadError, McBufVarReadable, McBufVarWritable};
use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, GlobalPos, Particle, Slot};
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EntityMetadata(Vec<EntityDataItem>);

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub id: u8,
    pub value: EntityDataValue,
}

impl McBufReadable for EntityMetadata {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut metadata = Vec::new();
        loop {
            let id = u8::read_from(buf)?;
            if id == 0xff {
                break;
            }
            let value = EntityDataValue::read_from(buf)?;
            metadata.push(EntityDataItem { id, value });
        }
        Ok(EntityMetadata(metadata))
    }
}

impl McBufWritable for EntityMetadata {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in &self.0 {
            item.id.write_into(buf)?;
            item.value.write_into(buf)?;
        }
        0xffu8.write_into(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug, McBuf)]
pub enum EntityDataValue {
    Byte(u8),
    // varint
    Int(i32),
    Long(i64),
    Float(f32),
    String(String),
    Component(Component),
    OptionalComponent(Option<Component>),
    ItemStack(Slot),
    Boolean(bool),
    Rotations { x: f32, y: f32, z: f32 },
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    // 0 for absent (implies air); otherwise, a block state ID as per the global palette
    // this is a varint
    BlockState(BlockState),
    CompoundTag(azalea_nbt::Tag),
    Particle(Particle),
    VillagerData(VillagerData),
    // 0 for absent; 1 + actual value otherwise. Used for entity IDs.
    OptionalUnsignedInt(OptionalUnsignedInt),
    Pose(Pose),
    CatVariant(azalea_registry::CatVariant),
    FrogVariant(azalea_registry::FrogVariant),
    OptionalGlobalPos(Option<GlobalPos>),
    PaintingVariant(azalea_registry::PaintingVariant),
}

#[derive(Clone, Debug)]
pub struct OptionalUnsignedInt {
    pub value: Option<u32>,
}
impl McBufReadable for OptionalUnsignedInt {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let val = u32::var_read_from(buf)?;
        Ok(OptionalUnsignedInt {
            value: if val == 0 { None } else { Some(val - 1) },
        })
    }
}
impl McBufWritable for OptionalUnsignedInt {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.value {
            Some(val) => (val + 1).var_write_into(buf),
            None => 0u32.var_write_into(buf),
        }
    }
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum Pose {
    Standing = 0,
    FallFlying = 1,
    Sleeping = 2,
    Swimming = 3,
    SpinAttack = 4,
    Sneaking = 5,
    LongJumping = 6,
    Dying = 7,
}

#[derive(Debug, Clone, McBuf)]
pub struct VillagerData {
    #[var]
    type_: u32,
    #[var]
    profession: u32,
    #[var]
    level: u32,
}
