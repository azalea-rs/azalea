use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar};
use azalea_core::position::Vec3;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundInteract {
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

impl AzaleaWrite for ActionType {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            ActionType::Interact { hand } => {
                0u32.azalea_write_var(buf)?;
                hand.azalea_write(buf)?;
            }
            ActionType::Attack => {
                1u32.azalea_write_var(buf)?;
            }
            ActionType::InteractAt { location, hand } => {
                2u32.azalea_write_var(buf)?;
                (location.x as f32).azalea_write(buf)?;
                (location.y as f32).azalea_write(buf)?;
                (location.z as f32).azalea_write(buf)?;
                hand.azalea_write(buf)?;
            }
        }
        Ok(())
    }
}

impl AzaleaRead for ActionType {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action_type = u32::azalea_read_var(buf)?;
        match action_type {
            0 => {
                let hand = InteractionHand::azalea_read(buf)?;
                Ok(ActionType::Interact { hand })
            }
            1 => Ok(ActionType::Attack),
            2 => {
                let x = f32::azalea_read(buf)?;
                let y = f32::azalea_read(buf)?;
                let z = f32::azalea_read(buf)?;
                let hand = InteractionHand::azalea_read(buf)?;
                Ok(ActionType::InteractAt {
                    location: Vec3 {
                        x: f64::from(x),
                        y: f64::from(y),
                        z: f64::from(z),
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

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum InteractionHand {
    MainHand = 0,
    OffHand = 1,
}
