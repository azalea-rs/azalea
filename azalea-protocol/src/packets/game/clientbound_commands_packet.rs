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

#[derive(Debug, Clone)]
pub struct BrigadierNodeStub {
    pub is_executable: bool,
    pub children: Vec<u32>,
    pub redirect_node: Option<u32>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone)]
pub struct BrigadierNumber<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}
impl<T> BrigadierNumber<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> BrigadierNumber<T> {
        BrigadierNumber { min, max }
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

#[derive(Debug, Clone, Copy, McBuf)]
pub enum BrigadierString {
    /// Reads a single word
    SingleWord = 0,
    // If it starts with a ", keeps reading until another " (allowing escaping with \). Otherwise
    // behaves the same as SINGLE_WORD
    QuotablePhrase = 1,
    // Reads the rest of the content after the cursor. Quotes will not be removed.
    GreedyPhrase = 2,
}

#[derive(Debug, Clone, McBuf)]
pub enum BrigadierParser {
    Bool,
    Float(BrigadierNumber<f32>),
    Double(BrigadierNumber<f64>),
    Integer(BrigadierNumber<i32>),
    Long(BrigadierNumber<i64>),
    String(BrigadierString),
    Entity(EntityParser),
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
    FormattedText,
    Message,
    NbtCompoundTag,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder { allows_multiple: bool },
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    GameMode,
    Time,
    ResourceOrTag { registry_key: ResourceLocation },
    ResourceOrTagKey { registry_key: ResourceLocation },
    Resource { registry_key: ResourceLocation },
    ResourceKey { registry_key: ResourceLocation },
    TemplateMirror,
    TemplateRotation,
    Uuid,
}

#[derive(Debug, Clone)]
pub struct EntityParser {
    pub single: bool,
    pub players_only: bool,
}
impl McBufReadable for EntityParser {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = u8::read_from(buf)?;
        Ok(EntityParser {
            single: flags & 0x01 != 0,
            players_only: flags & 0x02 != 0,
        })
    }
}
impl McBufWritable for EntityParser {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut flags: u8 = 0;
        if self.single {
            flags |= 0x01;
        }
        if self.players_only {
            flags |= 0x02;
        }
        flags.write_into(buf)?;
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
            let node = BrigadierNodeStub {
                is_executable,
                children,
                redirect_node,
                node_type: NodeType::Argument {
                    name,
                    parser,
                    suggestions_type,
                },
            };
            return Ok(node);
        }
        // literal node
        else if node_type == 1 {
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

#[derive(Debug, Clone)]
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
