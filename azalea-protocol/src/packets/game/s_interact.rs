use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzBufVar};
use azalea_core::{
    entity_id::MinecraftEntityId,
    position::{Vec3, Vec3f32},
};
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundInteract {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub action: ActionType,
    /// Whether the player is sneaking
    pub using_secondary_action: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl AzBuf for ActionType {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action_type = u32::azalea_read_var(buf)?;
        match action_type {
            0 => {
                let hand = InteractionHand::azalea_read(buf)?;
                Ok(ActionType::Interact { hand })
            }
            1 => Ok(ActionType::Attack),
            2 => {
                let pos = Vec3f32::azalea_read(buf)?;
                let hand = InteractionHand::azalea_read(buf)?;
                Ok(ActionType::InteractAt {
                    location: Vec3::from(pos),
                    hand,
                })
            }
            _ => Err(BufReadError::UnexpectedEnumVariant {
                id: action_type as i32,
            }),
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
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

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum InteractionHand {
    #[default]
    MainHand = 0,
    OffHand = 1,
}
