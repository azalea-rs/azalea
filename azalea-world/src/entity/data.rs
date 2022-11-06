use azalea_block::BlockState;
use azalea_buf::{BufReadError, McBufVarReadable};
use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, GlobalPos, Particle, Slot};
use enum_as_inner::EnumAsInner;
use log::warn;
use nohash_hasher::IntSet;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EntityMetadataItems(pub Vec<EntityDataItem>);

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl McBufReadable for EntityMetadataItems {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut metadata = Vec::new();
        loop {
            let index = u8::read_from(buf)?;
            if index == 0xff {
                break;
            }
            let value = EntityDataValue::read_from(buf)?;
            metadata.push(EntityDataItem { index, value });
        }
        Ok(EntityMetadataItems(metadata))
    }
}

impl McBufWritable for EntityMetadataItems {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in &self.0 {
            item.index.write_into(buf)?;
            item.value.write_into(buf)?;
        }
        0xffu8.write_into(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum EntityDataValue {
    Byte(u8),
    Int(i32),
    Float(f32),
    String(String),
    Component(Component),
    OptionalComponent(Option<Component>),
    ItemStack(Slot),
    Boolean(bool),
    Rotations(Rotations),
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    // 0 for absent (implies air); otherwise, a block state ID as per the global palette
    // this is a varint
    OptionalBlockState(Option<BlockState>),
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

#[derive(Clone, Debug, McBuf, Default)]
pub struct Rotations {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl McBufReadable for EntityDataValue {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
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
            8 => EntityDataValue::Rotations(Rotations::read_from(buf)?),
            9 => EntityDataValue::BlockPos(BlockPos::read_from(buf)?),
            10 => EntityDataValue::OptionalBlockPos(Option::<BlockPos>::read_from(buf)?),
            11 => EntityDataValue::Direction(Direction::read_from(buf)?),
            12 => EntityDataValue::OptionalUuid(Option::<Uuid>::read_from(buf)?),
            13 => EntityDataValue::OptionalBlockState({
                let val = u32::var_read_from(buf)?;
                if val == 0 {
                    None
                } else {
                    Some(BlockState::try_from(val - 1).unwrap_or_else(|_| {
                        warn!("Invalid block state ID {} in entity metadata", val - 1);
                        BlockState::Air
                    }))
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
                    Some(val - 1)
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

#[derive(Clone, Debug, Copy, McBuf, Default)]
pub enum Pose {
    #[default]
    Standing = 0,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
}

#[derive(Debug, Clone, McBuf, Default)]
pub struct VillagerData {
    #[var]
    type_: u32,
    #[var]
    profession: u32,
    #[var]
    level: u32,
}

impl TryFrom<EntityMetadataItems> for Vec<EntityDataValue> {
    type Error = String;

    fn try_from(data: EntityMetadataItems) -> Result<Self, Self::Error> {
        let mut data = data.0;

        data.sort_by(|a, b| a.index.cmp(&b.index));

        let mut prev_indexes = IntSet::default();
        let len = data.len();
        // check to make sure it's valid, in vanilla this is guaranteed to pass
        // but it's possible there's mods that mess with it so we want to make
        // sure it's good
        for item in &data {
            if prev_indexes.contains(&item.index) {
                return Err(format!("Index {} is duplicated", item.index));
            }
            if item.index as usize > len {
                return Err(format!("Index {} is too big", item.index));
            }
            prev_indexes.insert(item.index);
        }

        let data = data.into_iter().map(|d| d.value).collect();

        Ok(data)
    }
}
