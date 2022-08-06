use azalea_buf::McBufVarReadable;
use azalea_buf::{McBuf, McBufReadable, McBufVarWritable, McBufWritable};
use azalea_core::EntityPos;
use packet_macros::ServerboundGamePacket;
use std::io::{Read, Write};

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
        location: EntityPos,
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
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
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
                    location: EntityPos {
                        x: x as f64,
                        y: y as f64,
                        z: z as f64,
                    },
                    hand,
                })
            }
            _ => Err(format!("Invalid action type: {}", action_type)),
        }
    }
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum InteractionHand {
    MainHand = 0,
    OffHand = 1,
}
