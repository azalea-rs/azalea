pub type EntityMetadata = Vec<EntityDataItem>;

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl McBufReadable for Vec<EntityDataItem> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let mut metadata = Vec::new();
        loop {
            let index = u8::read_into(buf)?;
            if index == 0xff {
                break;
            }
            let value = EntityDataValue::read_into(buf)?;
            metadata.push(EntityDataItem { index, value });
        }
        Ok(metadata)
    }
}

impl McBufWritable for Vec<EntityDataItem> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in self {
            buf.write_byte(item.index)?;
            item.value.write_into(buf)?;
        }
        buf.write_byte(0xff)?;
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
}

impl McBufReadable for EntityDataValue {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let type_ = buf.read_varint()?;
        Ok(match type_ {
            0 => EntityDataValue::Byte(buf.read_byte()?),
            1 => EntityDataValue::Int(buf.read_varint()?),
            2 => EntityDataValue::Float(buf.read_float()?),
            3 => EntityDataValue::String(buf.read_utf()?),
            4 => EntityDataValue::Component(Component::read_into(buf)?),
            5 => EntityDataValue::OptionalComponent(Option::<Component>::read_into(buf)?),
            6 => EntityDataValue::ItemStack(Slot::read_into(buf)?),
            7 => EntityDataValue::Boolean(buf.read_boolean()?),
            8 => EntityDataValue::Rotations {
                x: buf.read_float()?,
                y: buf.read_float()?,
                z: buf.read_float()?,
            },
            9 => EntityDataValue::BlockPos(BlockPos::read_into(buf)?),
            10 => EntityDataValue::OptionalBlockPos(Option::<BlockPos>::read_into(buf)?),
            11 => EntityDataValue::Direction(Direction::read_into(buf)?),
            12 => EntityDataValue::OptionalUuid(Option::<Uuid>::read_into(buf)?),
            13 => EntityDataValue::OptionalBlockState({
                let val = i32::read_into(buf)?;
                if val == 0 {
                    None
                } else {
                    Some(val)
                }
            }),
            14 => EntityDataValue::CompoundTag(azalea_nbt::Tag::read_into(buf)?),
            15 => EntityDataValue::Particle(Particle::read_into(buf)?),
            16 => EntityDataValue::VillagerData(VillagerData::read_into(buf)?),
            17 => EntityDataValue::OptionalUnsignedInt({
                let val = buf.read_varint()?;
                if val == 0 {
                    None
                } else {
                    Some((val - 1) as u32)
                }
            }),
            18 => EntityDataValue::Pose(Pose::read_into(buf)?),
            _ => return Err(format!("Unknown entity data type: {}", type_)),
        })
    }
}

impl McBufWritable for EntityDataValue {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
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
