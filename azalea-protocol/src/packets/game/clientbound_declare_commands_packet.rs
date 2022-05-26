use super::GamePacket;
use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::resource_location::ResourceLocation;
use packet_macros::McBuf;
use std::{
    hash::Hash,
    io::{Read, Write},
};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundDeclareCommandsPacket {
    pub entries: Vec<BrigadierNodeStub>,
    pub root_index: i32,
}

impl ClientboundDeclareCommandsPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundDeclareCommandsPacket(self)
    }

    pub fn write(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        panic!("ClientboundDeclareCommandsPacket::write not implemented")
    }

    pub fn read<T: Read>(buf: &mut T) -> Result<GamePacket, String> {
        let node_count = buf.read_varint()?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        for _ in 0..node_count {
            let node = BrigadierNodeStub::read_into(buf)?;
            nodes.push(node);
        }
        let root_index = buf.read_varint()?;
        Ok(GamePacket::ClientboundDeclareCommandsPacket(
            ClientboundDeclareCommandsPacket {
                entries: nodes,
                root_index,
            },
        ))
    }
}

#[derive(Hash, Debug, Clone)]
pub struct BrigadierNodeStub {}

#[derive(Debug, Clone)]
pub struct BrigadierNumber<T> {
    min: Option<T>,
    max: Option<T>,
}
impl<T: McBufReadable> McBufReadable for BrigadierNumber<T> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let flags = buf.read_byte()?;
        let min = if flags & 0x01 != 0 {
            Some(T::read_into(buf)?)
        } else {
            None
        };
        let max = if flags & 0x02 != 0 {
            Some(T::read_into(buf)?)
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

#[derive(Debug, Clone, Copy)]
pub enum BrigadierString {
    /// Reads a single word
    SingleWord = 0,
    // If it starts with a ", keeps reading until another " (allowing escaping with \). Otherwise behaves the same as SINGLE_WORD
    QuotablePhrase = 1,
    // Reads the rest of the content after the cursor. Quotes will not be removed.
    GreedyPhrase = 2,
}

impl McBufReadable for BrigadierString {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let id = buf.read_byte()?;
        Ok(match id {
            0 => BrigadierString::SingleWord,
            1 => BrigadierString::QuotablePhrase,
            2 => BrigadierString::GreedyPhrase,
            _ => panic!("Unknown BrigadierString id: {}", id),
        })
    }
}
impl McBufWritable for BrigadierString {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(*self as u8)?;
        Ok(())
    }
}

#[derive(Debug, Clone, McBuf)]
pub enum BrigadierParserType {
    Bool = 0,
    Double,
    Float,
    Integer,
    Long,
    String,
    Entity,
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
    ScoreHolder,
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    MobEffect,
    Function,
    EntityAnchor,
    Range,
    IntRange,
    FloatRange,
    ItemEnchantment,
    EntitySummon,
    Dimension,
    Uuid,
    NbtTag,
    NbtCompoundTag,
    Time,
    ResourceOrTag,
    Resource,
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
    Range { decimals_allowed: bool },
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
}

impl McBufReadable for BrigadierParser {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let parser_type = BrigadierParserType::read_into(buf)?;

        match parser_type {
            BrigadierParserType::Bool => Ok(BrigadierParser::Bool),
            BrigadierParserType::Double => {
                Ok(BrigadierParser::Double(BrigadierNumber::read_into(buf)?))
            }
            BrigadierParserType::Float => {
                Ok(BrigadierParser::Float(BrigadierNumber::read_into(buf)?))
            }
            BrigadierParserType::Integer => {
                Ok(BrigadierParser::Integer(BrigadierNumber::read_into(buf)?))
            }
            BrigadierParserType::Long => {
                Ok(BrigadierParser::Long(BrigadierNumber::read_into(buf)?))
            }
            BrigadierParserType::String => {
                Ok(BrigadierParser::String(BrigadierString::read_into(buf)?))
            }
            BrigadierParserType::Entity {
                single,
                players_only,
            } => Ok(BrigadierParser::Entity {
                single,
                players_only,
            }),
            BrigadierParserType::GameProfile => Ok(BrigadierParser::GameProfile),
            BrigadierParserType::BlockPos => Ok(BrigadierParser::BlockPos),
            BrigadierParserType::ColumnPos => Ok(BrigadierParser::ColumnPos),
            BrigadierParserType::Vec3 => Ok(BrigadierParser::Vec3),
            BrigadierParserType::Vec2 => Ok(BrigadierParser::Vec2),
            BrigadierParserType::BlockState => Ok(BrigadierParser::BlockState),
            BrigadierParserType::BlockPredicate => Ok(BrigadierParser::BlockPredicate),
            BrigadierParserType::ItemStack => Ok(BrigadierParser::ItemStack),
            BrigadierParserType::ItemPredicate => Ok(BrigadierParser::ItemPredicate),
            BrigadierParserType::Color => Ok(BrigadierParser::Color),
            BrigadierParserType::Component => Ok(BrigadierParser::Component),
            BrigadierParserType::Message => Ok(BrigadierParser::Message),
            BrigadierParserType::Nbt => Ok(BrigadierParser::Nbt),
            BrigadierParserType::NbtPath => Ok(BrigadierParser::NbtPath),
            BrigadierParserType::Objective => Ok(BrigadierParser::Objective),
            BrigadierParserType::ObjectiveCriteira => Ok(BrigadierParser::ObjectiveCriteira),
            BrigadierParserType::Operation => Ok(BrigadierParser::Operation),
            BrigadierParserType::Particle => Ok(BrigadierParser::Particle),
            BrigadierParserType::Rotation => Ok(BrigadierParser::Rotation),
            BrigadierParserType::Angle => Ok(BrigadierParser::Angle),
            BrigadierParserType::ScoreboardSlot => Ok(BrigadierParser::ScoreboardSlot),
            BrigadierParserType::ScoreHolder => {
                let flags = buf.read_byte()?;
                Ok(BrigadierParser::ScoreHolder {
                    allows_multiple: flags & 0x01 != 0,
                })
            }
            BrigadierParserType::Swizzle => Ok(BrigadierParser::Swizzle),
            BrigadierParserType::Team => Ok(BrigadierParser::Team),
            BrigadierParserType::ItemSlot => Ok(BrigadierParser::ItemSlot),
            BrigadierParserType::ResourceLocation => Ok(BrigadierParser::ResourceLocation),
            BrigadierParserType::MobEffect => Ok(BrigadierParser::MobEffect),
            BrigadierParserType::Function => Ok(BrigadierParser::Function),
            BrigadierParserType::EntityAnchor => Ok(BrigadierParser::EntityAnchor),
            BrigadierParserType::Range => Ok(BrigadierParser::Range {
                decimals_allowed: buf.read_boolean()?,
            }),
            BrigadierParserType::IntRange => Ok(BrigadierParser::IntRange),
            BrigadierParserType::FloatRange => Ok(BrigadierParser::FloatRange),
            BrigadierParserType::ItemEnchantment => Ok(BrigadierParser::ItemEnchantment),
            BrigadierParserType::EntitySummon => Ok(BrigadierParser::EntitySummon),
            BrigadierParserType::Dimension => Ok(BrigadierParser::Dimension),
            BrigadierParserType::Uuid => Ok(BrigadierParser::Uuid),
            BrigadierParserType::NbtTag => Ok(BrigadierParser::NbtTag),
            BrigadierParserType::NbtCompoundTag => Ok(BrigadierParser::NbtCompoundTag),
            BrigadierParserType::Time => Ok(BrigadierParser::Time),
            BrigadierParserType::ResourceOrTag => Ok(BrigadierParser::ResourceOrTag {
                registry_key: buf.read_resource_location()?,
            }),
            BrigadierParserType::Resource => Ok(BrigadierParser::Resource {
                registry_key: buf.read_resource_location()?,
            }),
        }
    }
}

// TODO: BrigadierNodeStub should have more stuff
impl McBufReadable for BrigadierNodeStub {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let flags = u8::read_into(buf)?;
        if flags > 31 {
            println!(
                "Warning: The flags from a Brigadier node are over 31. This is probably a bug."
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
            let _parser = BrigadierParser::read_into(buf)?;
            let _suggestions_type = if has_suggestions_type {
                Some(buf.read_resource_location()?)
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
