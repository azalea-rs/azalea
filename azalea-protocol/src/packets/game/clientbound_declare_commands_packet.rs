use super::GamePacket;
use azalea_buf::McBuf;
use azalea_buf::McBufVarReadable;
use azalea_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::ResourceLocation;
use packet_macros::GamePacket;
use std::{
    hash::Hash,
    io::{Read, Write},
};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundDeclareCommandsPacket {
    pub entries: Vec<BrigadierNodeStub>,
    #[var]
    pub root_index: i32,
}

#[derive(Hash, Debug, Clone)]
pub struct BrigadierNodeStub {}

#[derive(Debug, Clone)]
pub struct BrigadierNumber<T> {
    min: Option<T>,
    max: Option<T>,
}
impl<T: McBufReadable> McBufReadable for BrigadierNumber<T> {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let flags = buf.read_byte()?;
        let min = if flags & 0x01 != 0 {
            Some(T::read_from(buf)?)
        } else {
            None
        };
        let max = if flags & 0x02 != 0 {
            Some(T::read_from(buf)?)
        } else {
            None
        };
        Ok(BrigadierNumber { min, max })
    }
}
impl<T: McBufWritable> McBufWritable for BrigadierNumber<T> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut flags = 0;
        if self.min.is_some() {
            flags |= 0x01;
        }
        if self.max.is_some() {
            flags |= 0x02;
        }
        buf.write_byte(flags)?;
        if let Some(min) = &self.min {
            min.write_into(buf)?;
        }
        if let Some(max) = &self.max {
            max.write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, McBuf)]
pub enum BrigadierString {
    /// Reads a single word
    SingleWord = 0,
    // If it starts with a ", keeps reading until another " (allowing escaping with \). Otherwise behaves the same as SINGLE_WORD
    QuotablePhrase = 1,
    // Reads the rest of the content after the cursor. Quotes will not be removed.
    GreedyPhrase = 2,
}

#[derive(Debug, Clone)]
pub enum BrigadierParser {
    Bool,
    Double(BrigadierNumber<f64>),
    Float(BrigadierNumber<f32>),
    Integer(BrigadierNumber<i32>),
    Long(BrigadierNumber<i64>),
    String(BrigadierString),
    Entity { single: bool, players_only: bool },
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Message,
    Nbt,
    NbtPath,
    Objective,
    ObjectiveCriteira,
    Operation,
    Particle,
    Rotation,
    Angle,
    ScoreboardSlot,
    ScoreHolder { allows_multiple: bool },
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    MobEffect,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    ItemEnchantment,
    EntitySummon,
    Dimension,
    Uuid,
    NbtTag,
    NbtCompoundTag,
    Time,
    ResourceOrTag { registry_key: ResourceLocation },
    Resource { registry_key: ResourceLocation },
    TemplateMirror,
    TemplateRotation,
}

impl McBufReadable for BrigadierParser {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let parser_type = u32::var_read_from(buf)?;

        match parser_type {
            0 => Ok(BrigadierParser::Bool),
            1 => Ok(BrigadierParser::Float(BrigadierNumber::read_from(buf)?)),
            2 => Ok(BrigadierParser::Double(BrigadierNumber::read_from(buf)?)),
            3 => Ok(BrigadierParser::Integer(BrigadierNumber::read_from(buf)?)),
            4 => Ok(BrigadierParser::Long(BrigadierNumber::read_from(buf)?)),
            5 => Ok(BrigadierParser::String(BrigadierString::read_from(buf)?)),
            6 => {
                let flags = buf.read_byte()?;
                Ok(BrigadierParser::Entity {
                    single: flags & 0x01 != 0,
                    players_only: flags & 0x02 != 0,
                })
            }
            7 => Ok(BrigadierParser::GameProfile),
            8 => Ok(BrigadierParser::BlockPos),
            9 => Ok(BrigadierParser::ColumnPos),
            10 => Ok(BrigadierParser::Vec3),
            11 => Ok(BrigadierParser::Vec2),
            12 => Ok(BrigadierParser::BlockState),
            13 => Ok(BrigadierParser::BlockPredicate),
            14 => Ok(BrigadierParser::ItemStack),
            15 => Ok(BrigadierParser::ItemPredicate),
            16 => Ok(BrigadierParser::Color),
            17 => Ok(BrigadierParser::Component),
            18 => Ok(BrigadierParser::Message),
            19 => Ok(BrigadierParser::NbtCompoundTag),
            20 => Ok(BrigadierParser::NbtTag),
            21 => Ok(BrigadierParser::NbtPath),
            22 => Ok(BrigadierParser::Objective),
            23 => Ok(BrigadierParser::ObjectiveCriteira),
            24 => Ok(BrigadierParser::Operation),
            25 => Ok(BrigadierParser::Particle),
            26 => Ok(BrigadierParser::Angle),
            27 => Ok(BrigadierParser::Rotation),
            28 => Ok(BrigadierParser::ScoreboardSlot),
            29 => {
                let flags = buf.read_byte()?;
                Ok(BrigadierParser::ScoreHolder {
                    allows_multiple: flags & 0x01 != 0,
                })
            }
            30 => Ok(BrigadierParser::Swizzle),
            31 => Ok(BrigadierParser::Team),
            32 => Ok(BrigadierParser::ItemSlot),
            33 => Ok(BrigadierParser::ResourceLocation),
            34 => Ok(BrigadierParser::MobEffect),
            35 => Ok(BrigadierParser::Function),
            36 => Ok(BrigadierParser::EntityAnchor),
            37 => Ok(BrigadierParser::IntRange),
            38 => Ok(BrigadierParser::FloatRange),
            39 => Ok(BrigadierParser::ItemEnchantment),
            40 => Ok(BrigadierParser::EntitySummon),
            41 => Ok(BrigadierParser::Dimension),
            42 => Ok(BrigadierParser::Time),
            43 => Ok(BrigadierParser::ResourceOrTag {
                registry_key: ResourceLocation::read_from(buf)?,
            }),
            44 => Ok(BrigadierParser::Resource {
                registry_key: ResourceLocation::read_from(buf)?,
            }),
            45 => Ok(BrigadierParser::TemplateMirror),
            46 => Ok(BrigadierParser::TemplateRotation),
            47 => Ok(BrigadierParser::Uuid),
            _ => Err(format!("Unknown BrigadierParser type: {}", parser_type)),
        }
    }
}

// TODO: BrigadierNodeStub should have more stuff
impl McBufReadable for BrigadierNodeStub {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let flags = u8::read_from(buf)?;
        if flags > 31 {
            println!(
                "Warning: The flags from a Brigadier node are over 31 ({flags}; {flags:#b}). This is probably a bug.",
            );
        }

        let node_type = flags & 0x03;
        let _is_executable = flags & 0x04 != 0;
        let has_redirect = flags & 0x08 != 0;
        let has_suggestions_type = flags & 0x10 != 0;

        let _children = buf.read_int_id_list()?;
        let _redirect_node = if has_redirect { buf.read_varint()? } else { 0 };

        // argument node
        if node_type == 2 {
            let _name = buf.read_utf()?;
            let _parser = BrigadierParser::read_from(buf)?;
            let _suggestions_type = if has_suggestions_type {
                Some(ResourceLocation::read_from(buf)?)
            } else {
                None
            };
            return Ok(BrigadierNodeStub {});
        }
        // literal node
        if node_type == 1 {
            let _name = buf.read_utf()?;
            return Ok(BrigadierNodeStub {});
        }
        Ok(BrigadierNodeStub {})
        // return Err("Unknown node type".to_string());
    }
}

impl McBufWritable for BrigadierNodeStub {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}
