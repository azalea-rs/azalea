use azalea_buf::{BufReadError, McBufVarReadable};
use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_chat::component::Component;
use azalea_core::{BlockPos, Direction, GlobalPos, Particle, Slot};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EntityMetadata(Vec<EntityDataItem>);

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl McBufReadable for EntityMetadata {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let mut metadata = Vec::new();
        loop {
            let index = u8::read_from(buf)?;
            if index == 0xff {
                break;
            }
            let value = EntityDataValue::read_from(buf)?;
            metadata.push(EntityDataItem { index, value });
        }
        Ok(EntityMetadata(metadata))
    }
}

impl McBufWritable for EntityMetadata {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in &self.0 {
            item.index.write_into(buf)?;
            item.value.write_into(buf)?;
        }
        0xffu8.write_into(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum EntityDataValue {
    Byte(u8),
    // varint
    Int(i32),
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
    OptionalBlockState(Option<i32>),
    CompoundTag(azalea_nbt::Tag),
    Particle(Particle),
    VillagerData(VillagerData),
    // 0 for absent; 1 + actual value otherwise. Used for entity IDs.
    OptionalUnsignedInt(Option<u32>),
    Pose(Pose),
    CatVariant(azalea_registry::CatVariant),
    FrogVariant(azalea_registry::FrogVariant),
    GlobalPos(GlobalPos),
    PaintingVariant(azalea_registry::PaintingVariant),
}

impl McBufReadable for EntityDataValue {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let data_type = u32::var_read_from(buf)?;
        Ok(match data_type {
            0 => EntityDataValue::Byte(u8::read_from(buf)?),
            1 => EntityDataValue::Int(i32::var_read_from(buf)?),
            2 => EntityDataValue::Float(f32::read_from(buf)?),
            3 => EntityDataValue::String(String::read_from(buf)?),
            4 => EntityDataValue::Component(Component::read_from(buf)?),
            5 => EntityDataValue::OptionalComponent(Option::<Component>::read_from(buf)?),
            6 => EntityDataValue::ItemStack(Slot::read_from(buf)?),
            7 => EntityDataValue::Boolean(bool::read_from(buf)?),
            8 => EntityDataValue::Rotations {
                x: f32::read_from(buf)?,
                y: f32::read_from(buf)?,
                z: f32::read_from(buf)?,
            },
            9 => EntityDataValue::BlockPos(BlockPos::read_from(buf)?),
            10 => EntityDataValue::OptionalBlockPos(Option::<BlockPos>::read_from(buf)?),
            11 => EntityDataValue::Direction(Direction::read_from(buf)?),
            12 => EntityDataValue::OptionalUuid(Option::<Uuid>::read_from(buf)?),
            13 => EntityDataValue::OptionalBlockState({
                let val = i32::var_read_from(buf)?;
                if val == 0 {
                    None
                } else {
                    Some(val)
                }
            }),
            14 => EntityDataValue::CompoundTag(azalea_nbt::Tag::read_from(buf)?),
            15 => EntityDataValue::Particle(Particle::read_from(buf)?),
            16 => EntityDataValue::VillagerData(VillagerData::read_from(buf)?),
            17 => EntityDataValue::OptionalUnsignedInt({
                let val = u32::var_read_from(buf)?;
                if val == 0 {
                    None
                } else {
                    Some((val - 1) as u32)
                }
            }),
            18 => EntityDataValue::Pose(Pose::read_from(buf)?),
            19 => EntityDataValue::CatVariant(azalea_registry::CatVariant::read_from(buf)?),
            20 => EntityDataValue::FrogVariant(azalea_registry::FrogVariant::read_from(buf)?),
            21 => EntityDataValue::GlobalPos(GlobalPos::read_from(buf)?),
            22 => {
                EntityDataValue::PaintingVariant(azalea_registry::PaintingVariant::read_from(buf)?)
            }
            _ => {
                return Err(BufReadError::UnexpectedEnumVariant {
                    id: data_type as i32,
                })
            }
        })
    }
}

impl McBufWritable for EntityDataValue {
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!();
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
