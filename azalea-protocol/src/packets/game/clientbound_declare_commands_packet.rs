use super::GamePacket;
use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use async_trait::async_trait;
use azalea_core::resource_location::ResourceLocation;
use std::hash::Hash;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundDeclareCommandsPacket {
    pub entries: Vec<BrigadierNodeStub>,
    pub root_index: i32,
}

impl ClientboundDeclareCommandsPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundDeclareCommandsPacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        panic!("ClientboundDeclareCommandsPacket::write not implemented")
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let node_count = buf.read_varint().await?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        for _ in 0..node_count {
            let node = BrigadierNodeStub::read_into(buf).await?;
            nodes.push(node);
        }
        let root_index = buf.read_varint().await?;
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
#[async_trait]
impl<T: McBufReadable + Send> McBufReadable for BrigadierNumber<T> {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let flags = buf.read_byte().await?;
        let min = if flags & 0x01 != 0 {
            Some(T::read_into(buf).await?)
        } else {
            None
        };
        let max = if flags & 0x02 != 0 {
            Some(T::read_into(buf).await?)
        } else {
            None
        };
        Ok(BrigadierNumber { min, max })
    }
}
impl<T: McBufWritable> McBufWritable for BrigadierNumber<T> {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

#[async_trait]
impl McBufReadable for BrigadierString {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let id = buf.read_byte().await?;
        Ok(match id {
            0 => BrigadierString::SingleWord,
            1 => BrigadierString::QuotablePhrase,
            2 => BrigadierString::GreedyPhrase,
            _ => panic!("Unknown BrigadierString id: {}", id),
        })
    }
}
impl McBufWritable for BrigadierString {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_byte(*self as u8)?;
        Ok(())
    }
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

#[async_trait]
impl McBufReadable for BrigadierParser {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let parser = buf.read_resource_location().await?;

        if parser == ResourceLocation::new("brigadier:bool")? {
            Ok(BrigadierParser::Bool)
        } else if parser == ResourceLocation::new("brigadier:double")? {
            Ok(BrigadierParser::Double(
                BrigadierNumber::read_into(buf).await?,
            ))
        } else if parser == ResourceLocation::new("brigadier:float")? {
            Ok(BrigadierParser::Float(
                BrigadierNumber::read_into(buf).await?,
            ))
        } else if parser == ResourceLocation::new("brigadier:integer")? {
            Ok(BrigadierParser::Integer(
                BrigadierNumber::read_into(buf).await?,
            ))
        } else if parser == ResourceLocation::new("brigadier:long")? {
            Ok(BrigadierParser::Long(
                BrigadierNumber::read_into(buf).await?,
            ))
        } else if parser == ResourceLocation::new("brigadier:string")? {
            Ok(BrigadierParser::String(
                BrigadierString::read_into(buf).await?,
            ))
        } else if parser == ResourceLocation::new("minecraft:entity")? {
            let flags = buf.read_byte().await?;
            Ok(BrigadierParser::Entity {
                single: flags & 0x01 != 0,
                players_only: flags & 0x02 != 0,
            })
        } else if parser == ResourceLocation::new("minecraft:game_profile")? {
            Ok(BrigadierParser::GameProfile)
        } else if parser == ResourceLocation::new("minecraft:block_pos")? {
            Ok(BrigadierParser::BlockPos)
        } else if parser == ResourceLocation::new("minecraft:column_pos")? {
            Ok(BrigadierParser::ColumnPos)
        } else if parser == ResourceLocation::new("minecraft:vec3")? {
            Ok(BrigadierParser::Vec3)
        } else if parser == ResourceLocation::new("minecraft:vec2")? {
            Ok(BrigadierParser::Vec2)
        } else if parser == ResourceLocation::new("minecraft:block_state")? {
            Ok(BrigadierParser::BlockState)
        } else if parser == ResourceLocation::new("minecraft:block_predicate")? {
            Ok(BrigadierParser::BlockPredicate)
        } else if parser == ResourceLocation::new("minecraft:item_stack")? {
            Ok(BrigadierParser::ItemStack)
        } else if parser == ResourceLocation::new("minecraft:item_predicate")? {
            Ok(BrigadierParser::ItemPredicate)
        } else if parser == ResourceLocation::new("minecraft:color")? {
            Ok(BrigadierParser::Color)
        } else if parser == ResourceLocation::new("minecraft:component")? {
            Ok(BrigadierParser::Component)
        } else if parser == ResourceLocation::new("minecraft:message")? {
            Ok(BrigadierParser::Message)
        } else if parser == ResourceLocation::new("minecraft:nbt")? {
            Ok(BrigadierParser::Nbt)
        } else if parser == ResourceLocation::new("minecraft:nbt_path")? {
            Ok(BrigadierParser::NbtPath)
        } else if parser == ResourceLocation::new("minecraft:objective")? {
            Ok(BrigadierParser::Objective)
        } else if parser == ResourceLocation::new("minecraft:objective_criteria")? {
            Ok(BrigadierParser::ObjectiveCriteira)
        } else if parser == ResourceLocation::new("minecraft:operation")? {
            Ok(BrigadierParser::Operation)
        } else if parser == ResourceLocation::new("minecraft:particle")? {
            Ok(BrigadierParser::Particle)
        } else if parser == ResourceLocation::new("minecraft:rotation")? {
            Ok(BrigadierParser::Rotation)
        } else if parser == ResourceLocation::new("minecraft:angle")? {
            Ok(BrigadierParser::Angle)
        } else if parser == ResourceLocation::new("minecraft:scoreboard_slot")? {
            Ok(BrigadierParser::ScoreboardSlot)
        } else if parser == ResourceLocation::new("minecraft:score_holder")? {
            let flags = buf.read_byte().await?;
            Ok(BrigadierParser::ScoreHolder {
                allows_multiple: flags & 0x01 != 0,
            })
        } else if parser == ResourceLocation::new("minecraft:swizzle")? {
            Ok(BrigadierParser::Swizzle)
        } else if parser == ResourceLocation::new("minecraft:team")? {
            Ok(BrigadierParser::Team)
        } else if parser == ResourceLocation::new("minecraft:item_slot")? {
            Ok(BrigadierParser::ItemSlot)
        } else if parser == ResourceLocation::new("minecraft:resource_location")? {
            Ok(BrigadierParser::ResourceLocation)
        } else if parser == ResourceLocation::new("minecraft:mob_effect")? {
            Ok(BrigadierParser::MobEffect)
        } else if parser == ResourceLocation::new("minecraft:function")? {
            Ok(BrigadierParser::Function)
        } else if parser == ResourceLocation::new("minecraft:entity_anchor")? {
            Ok(BrigadierParser::EntityAnchor)
        } else if parser == ResourceLocation::new("minecraft:range")? {
            Ok(BrigadierParser::Range {
                decimals_allowed: buf.read_boolean().await?,
            })
        } else if parser == ResourceLocation::new("minecraft:int_range")? {
            Ok(BrigadierParser::IntRange)
        } else if parser == ResourceLocation::new("minecraft:float_range")? {
            Ok(BrigadierParser::FloatRange)
        } else if parser == ResourceLocation::new("minecraft:item_enchantment")? {
            Ok(BrigadierParser::ItemEnchantment)
        } else if parser == ResourceLocation::new("minecraft:entity_summon")? {
            Ok(BrigadierParser::EntitySummon)
        } else if parser == ResourceLocation::new("minecraft:dimension")? {
            Ok(BrigadierParser::Dimension)
        } else if parser == ResourceLocation::new("minecraft:uuid")? {
            Ok(BrigadierParser::Uuid)
        } else if parser == ResourceLocation::new("minecraft:nbt_tag")? {
            Ok(BrigadierParser::NbtTag)
        } else if parser == ResourceLocation::new("minecraft:nbt_compound_tag")? {
            Ok(BrigadierParser::NbtCompoundTag)
        } else if parser == ResourceLocation::new("minecraft:time")? {
            Ok(BrigadierParser::Time)
        } else if parser == ResourceLocation::new("minecraft:resource_or_tag")? {
            Ok(BrigadierParser::ResourceOrTag {
                registry_key: buf.read_resource_location().await?,
            })
        } else if parser == ResourceLocation::new("minecraft:resource")? {
            Ok(BrigadierParser::Resource {
                registry_key: buf.read_resource_location().await?,
            })
        } else {
            panic!("Unknown Brigadier parser: {}", parser)
        }
    }
}

// azalea_brigadier::tree::CommandNode
#[async_trait]
impl McBufReadable for BrigadierNodeStub {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let flags = u8::read_into(buf).await?;
        if flags > 31 {
            println!(
                "Warning: The flags from a Brigadier node are over 31. This is probably a bug."
            );
        }

        let node_type = flags & 0x03;
        let is_executable = flags & 0x04 != 0;
        let has_redirect = flags & 0x08 != 0;
        let has_suggestions_type = flags & 0x10 != 0;

        let children = buf.read_int_id_list().await?;
        let redirect_node = if has_redirect {
            buf.read_varint().await?
        } else {
            0
        };

        // argument node
        if node_type == 2 {
            let name = buf.read_utf().await?;

            let parser = BrigadierParser::read_into(buf).await?;

            let suggestions_type = if has_suggestions_type {
                Some(buf.read_resource_location().await?)
            } else {
                None
            };
            return Ok(BrigadierNodeStub {});
        }
        // literal node
        if node_type == 1 {
            let name = buf.read_utf().await?;
            return Ok(BrigadierNodeStub {});
        }
        Ok(BrigadierNodeStub {})
        // return Err("Unknown node type".to_string());
    }
}
