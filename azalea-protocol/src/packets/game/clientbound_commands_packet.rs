use azalea_buf::BufReadError;
use azalea_buf::McBuf;
use azalea_buf::McBufVarReadable;
use azalea_buf::{McBufReadable, McBufVarWritable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use log::warn;
use std::io::Cursor;
use std::io::Write;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCommandsPacket {
    pub entries: Vec<BrigadierNodeStub>,
    #[var]
    pub root_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrigadierNodeStub {
    pub is_executable: bool,
    pub children: Vec<u32>,
    pub redirect_node: Option<u32>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Eq)]
pub struct BrigadierNumber<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}
impl<T> BrigadierNumber<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> BrigadierNumber<T> {
        BrigadierNumber { min, max }
    }
}
impl<T: PartialEq> PartialEq for BrigadierNumber<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.min, &self.max, &other.min, &other.max) {
            (Some(f_min), None, Some(s_min), None) => f_min == s_min,
            (None, Some(f_max), None, Some(s_max)) => f_max == s_max,
            (Some(f_min), Some(f_max), Some(s_min), Some(s_max)) => {
                f_min == s_min && f_max == s_max
            }
            (None, None, None, None) => true,
            _ => false,
        }
    }
}
impl<T: McBufReadable> McBufReadable for BrigadierNumber<T> {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = u8::read_from(buf)?;
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
        let mut flags: u8 = 0;
        if self.min.is_some() {
            flags |= 0x01;
        }
        if self.max.is_some() {
            flags |= 0x02;
        }
        flags.write_into(buf)?;
        if let Some(min) = &self.min {
            min.write_into(buf)?;
        }
        if let Some(max) = &self.max {
            max.write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, McBuf, PartialEq, Eq)]
pub enum BrigadierString {
    /// Reads a single word
    SingleWord = 0,
    // If it starts with a ", keeps reading until another " (allowing escaping with \). Otherwise
    // behaves the same as SINGLE_WORD
    QuotablePhrase = 1,
    // Reads the rest of the content after the cursor. Quotes will not be removed.
    GreedyPhrase = 2,
}

#[derive(Debug, Clone, PartialEq)]
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
    Time,
    ResourceOrTag { registry_key: ResourceLocation },
    Resource { registry_key: ResourceLocation },
    TemplateMirror,
    TemplateRotation,
}

impl McBufReadable for BrigadierParser {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let parser_type = u32::var_read_from(buf)?;

        match parser_type {
            0 => Ok(BrigadierParser::Bool),
            1 => Ok(BrigadierParser::Float(BrigadierNumber::read_from(buf)?)),
            2 => Ok(BrigadierParser::Double(BrigadierNumber::read_from(buf)?)),
            3 => Ok(BrigadierParser::Integer(BrigadierNumber::read_from(buf)?)),
            4 => Ok(BrigadierParser::Long(BrigadierNumber::read_from(buf)?)),
            5 => Ok(BrigadierParser::String(BrigadierString::read_from(buf)?)),
            6 => {
                let flags = u8::read_from(buf)?;
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
            19 => Ok(BrigadierParser::Nbt),
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
                let flags = u8::read_from(buf)?;
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
            _ => Err(BufReadError::UnexpectedEnumVariant {
                id: parser_type as i32,
            }),
        }
    }
}

impl McBufWritable for BrigadierParser {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match &self {
            BrigadierParser::Bool => {
                u32::var_write_into(&0, buf)?;
            }
            BrigadierParser::Float(f) => {
                u32::var_write_into(&1, buf)?;
                f.write_into(buf)?;
            }
            BrigadierParser::Double(d) => {
                u32::var_write_into(&2, buf)?;
                d.write_into(buf)?;
            }
            BrigadierParser::Integer(i) => {
                u32::var_write_into(&3, buf)?;
                i.write_into(buf)?;
            }
            BrigadierParser::Long(l) => {
                u32::var_write_into(&4, buf)?;
                l.write_into(buf)?;
            }
            BrigadierParser::String(s) => {
                u32::var_write_into(&5, buf)?;
                s.write_into(buf)?;
            }
            BrigadierParser::Entity {
                single,
                players_only,
            } => {
                u32::var_write_into(&6, buf)?;
                let mut bitmask: u8 = 0x00;
                if *single {
                    bitmask |= 0x01;
                }
                if *players_only {
                    bitmask |= 0x02;
                }
                bitmask.write_into(buf)?;
            }
            BrigadierParser::GameProfile => {
                u32::var_write_into(&7, buf)?;
            }
            BrigadierParser::BlockPos => {
                u32::var_write_into(&8, buf)?;
            }
            BrigadierParser::ColumnPos => {
                u32::var_write_into(&9, buf)?;
            }
            BrigadierParser::Vec3 => {
                u32::var_write_into(&10, buf)?;
            }
            BrigadierParser::Vec2 => {
                u32::var_write_into(&11, buf)?;
            }
            BrigadierParser::BlockState => {
                u32::var_write_into(&12, buf)?;
            }
            BrigadierParser::BlockPredicate => {
                u32::var_write_into(&13, buf)?;
            }
            BrigadierParser::ItemStack => {
                u32::var_write_into(&14, buf)?;
            }
            BrigadierParser::ItemPredicate => {
                u32::var_write_into(&15, buf)?;
            }
            BrigadierParser::Color => {
                u32::var_write_into(&16, buf)?;
            }
            BrigadierParser::Component => {
                u32::var_write_into(&17, buf)?;
            }
            BrigadierParser::Message => {
                u32::var_write_into(&18, buf)?;
            }
            BrigadierParser::Nbt => {
                u32::var_write_into(&19, buf)?;
            }
            BrigadierParser::NbtTag => {
                u32::var_write_into(&20, buf)?;
            }
            BrigadierParser::NbtPath => {
                u32::var_write_into(&21, buf)?;
            }
            BrigadierParser::Objective => {
                u32::var_write_into(&22, buf)?;
            }
            BrigadierParser::ObjectiveCriteira => {
                u32::var_write_into(&23, buf)?;
            }
            BrigadierParser::Operation => {
                u32::var_write_into(&24, buf)?;
            }
            BrigadierParser::Particle => {
                u32::var_write_into(&25, buf)?;
            }
            BrigadierParser::Angle => {
                u32::var_write_into(&26, buf)?;
            }
            BrigadierParser::Rotation => {
                u32::var_write_into(&27, buf)?;
            }
            BrigadierParser::ScoreboardSlot => {
                u32::var_write_into(&28, buf)?;
            }
            BrigadierParser::ScoreHolder { allows_multiple } => {
                u32::var_write_into(&29, buf)?;
                if *allows_multiple {
                    buf.write_all(&[0x01])?;
                } else {
                    buf.write_all(&[0x00])?;
                }
            }
            BrigadierParser::Swizzle => {
                u32::var_write_into(&30, buf)?;
            }
            BrigadierParser::Team => {
                u32::var_write_into(&31, buf)?;
            }
            BrigadierParser::ItemSlot => {
                u32::var_write_into(&32, buf)?;
            }
            BrigadierParser::ResourceLocation => {
                u32::var_write_into(&33, buf)?;
            }
            BrigadierParser::MobEffect => {
                u32::var_write_into(&34, buf)?;
            }
            BrigadierParser::Function => {
                u32::var_write_into(&35, buf)?;
            }
            BrigadierParser::EntityAnchor => {
                u32::var_write_into(&36, buf)?;
            }
            BrigadierParser::IntRange => {
                u32::var_write_into(&37, buf)?;
            }
            BrigadierParser::FloatRange => {
                u32::var_write_into(&38, buf)?;
            }
            BrigadierParser::ItemEnchantment => {
                u32::var_write_into(&39, buf)?;
            }
            BrigadierParser::EntitySummon => {
                u32::var_write_into(&40, buf)?;
            }
            BrigadierParser::Dimension => {
                u32::var_write_into(&41, buf)?;
            }
            BrigadierParser::Time => {
                u32::var_write_into(&42, buf)?;
            }
            BrigadierParser::ResourceOrTag { registry_key } => {
                u32::var_write_into(&43, buf)?;
                registry_key.write_into(buf)?;
            }
            BrigadierParser::Resource { registry_key } => {
                u32::var_write_into(&44, buf)?;
                registry_key.write_into(buf)?;
            }
            BrigadierParser::TemplateMirror => {
                u32::var_write_into(&45, buf)?;
            }
            BrigadierParser::TemplateRotation => {
                u32::var_write_into(&46, buf)?;
            }
            BrigadierParser::Uuid => {
                u32::var_write_into(&47, buf)?;
            }
        }
        Ok(())
    }
}

// TODO: BrigadierNodeStub should have more stuff
impl McBufReadable for BrigadierNodeStub {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = u8::read_from(buf)?;
        if flags > 31 {
            warn!(
                "Warning: The flags from a Brigadier node are over 31 ({flags}; {flags:#b}). This is probably a bug.",
            );
        }

        let node_type = flags & 0x03;
        let is_executable = flags & 0x04 != 0;
        let has_redirect = flags & 0x08 != 0;
        let has_suggestions_type = flags & 0x10 != 0;

        let children = Vec::<u32>::var_read_from(buf)?;
        let redirect_node = if has_redirect {
            Some(u32::var_read_from(buf)?)
        } else {
            None
        };

        // argument node
        if node_type == 2 {
            let name = String::read_from(buf)?;
            let parser = BrigadierParser::read_from(buf)?;
            let suggestions_type = if has_suggestions_type {
                Some(ResourceLocation::read_from(buf)?)
            } else {
                None
            };
            return Ok(BrigadierNodeStub {
                is_executable,
                children,
                redirect_node,
                node_type: NodeType::Argument {
                    name,
                    parser,
                    suggestions_type,
                },
            });
        }
        // literal node
        if node_type == 1 {
            let name = String::read_from(buf)?;
            return Ok(BrigadierNodeStub {
                is_executable,
                children,
                redirect_node,
                node_type: NodeType::Literal { name },
            });
        }
        Ok(BrigadierNodeStub {
            is_executable,
            children,
            redirect_node,
            node_type: NodeType::Root,
        })
    }
}

impl McBufWritable for BrigadierNodeStub {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match &self.node_type {
            NodeType::Root => {
                let mut flags = 0x00;
                if self.is_executable {
                    flags |= 0x04;
                }
                if self.redirect_node.is_some() {
                    flags |= 0x08;
                }
                flags.var_write_into(buf)?;

                self.children.var_write_into(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.var_write_into(buf)?;
                }
            }
            NodeType::Literal { name } => {
                let mut flags = 0x01;
                if self.is_executable {
                    flags |= 0x04;
                }
                if self.redirect_node.is_some() {
                    flags |= 0x08;
                }
                flags.var_write_into(buf)?;

                self.children.var_write_into(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.var_write_into(buf)?;
                }

                name.write_into(buf)?;
            }
            NodeType::Argument {
                name,
                parser,
                suggestions_type,
            } => {
                let mut flags = 0x02;
                if self.is_executable {
                    flags |= 0x04;
                }
                if self.redirect_node.is_some() {
                    flags |= 0x08;
                }
                if suggestions_type.is_some() {
                    flags |= 0x10;
                }
                flags.var_write_into(buf)?;

                self.children.var_write_into(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.var_write_into(buf)?;
                }

                name.write_into(buf)?;
                parser.write_into(buf)?;

                if let Some(suggestion) = suggestions_type {
                    suggestion.write_into(buf)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Root,
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser: BrigadierParser,
        suggestions_type: Option<ResourceLocation>,
    },
}

impl BrigadierNodeStub {
    pub fn name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Root => None,
            NodeType::Literal { name } => Some(name),
            NodeType::Argument { name, .. } => Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brigadier_node_stub_root() {
        let data = BrigadierNodeStub {
            is_executable: false,
            children: vec![1, 2],
            redirect_node: None,
            node_type: NodeType::Root,
        };
        let mut buf = Vec::new();
        data.write_into(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::read_from(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }

    #[test]
    fn test_brigadier_node_stub_literal() {
        let data = BrigadierNodeStub {
            is_executable: true,
            children: vec![],
            redirect_node: None,
            node_type: NodeType::Literal {
                name: "String".to_string(),
            },
        };
        let mut buf = Vec::new();
        data.write_into(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::read_from(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }

    #[test]
    fn test_brigadier_node_stub_argument() {
        let data = BrigadierNodeStub {
            is_executable: false,
            children: vec![6, 9],
            redirect_node: Some(5),
            node_type: NodeType::Argument {
                name: "position".to_string(),
                parser: BrigadierParser::Vec3,
                suggestions_type: Some(ResourceLocation::new("minecraft:test_suggestion").unwrap()),
            },
        };
        let mut buf = Vec::new();
        data.write_into(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::read_from(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }
}
