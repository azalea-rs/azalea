use crate::packets::BufReadError;
use azalea_buf::{McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_core::Vec3;
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundInteractPacket {
    #[var]
    pub entity_id: u32,
    pub action: ActionType,
    /// Whether the player is sneaking
    pub using_secondary_action: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ActionType {
    Interact {
        hand: InteractionHand,
    },
    Attack,
    InteractAt {
        location: Vec3,
        hand: InteractionHand,
    },
}

impl McBufWritable for ActionType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            ActionType::Interact { hand } => {
                0u32.var_write_into(buf)?;
                hand.write_into(buf)?;
            }
            ActionType::Attack => {
                1u32.var_write_into(buf)?;
            }
            ActionType::InteractAt { location, hand } => {
                2u32.var_write_into(buf)?;
                (location.x as f32).write_into(buf)?;
                (location.y as f32).write_into(buf)?;
                (location.z as f32).write_into(buf)?;
                hand.write_into(buf)?;
            }
        }
        Ok(())
    }
}

impl McBufReadable for ActionType {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action_type = u32::var_read_from(buf)?;
        match action_type {
            0 => {
                let hand = InteractionHand::read_from(buf)?;
                Ok(ActionType::Interact { hand })
            }
            1 => Ok(ActionType::Attack),
            2 => {
                let x = f32::read_from(buf)?;
                let y = f32::read_from(buf)?;
                let z = f32::read_from(buf)?;
                let hand = InteractionHand::read_from(buf)?;
                Ok(ActionType::InteractAt {
                    location: Vec3 {
                        x: x as f64,
                        y: y as f64,
                        z: z as f64,
                    },
                    hand,
                })
            }
            _ => Err(BufReadError::UnexpectedEnumVariant {
                id: action_type as i32,
            }),
        }
    }
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum InteractionHand {
    MainHand = 0,
    OffHand = 1,
}
