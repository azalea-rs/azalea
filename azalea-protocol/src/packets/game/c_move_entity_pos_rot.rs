use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzBufVar, BufReadError};
use azalea_core::{delta::PositionDelta8, entity_id::MinecraftEntityId};
use azalea_entity::{LookDirection, vec_delta_codec::VecDeltaCodec};
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::game::c_entity_position_sync::{PositionPath, PositionStep};

/// This packet is sent by the server when an entity moves less then 8 blocks.
#[derive(ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveEntityPosRot {
    pub entity_id: MinecraftEntityId,
    pub delta: VecDelta,
    pub look_direction: CompactLookDirection,
    pub properties: Properties,
}
impl AzBuf for ClientboundMoveEntityPosRot {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        read_move_entity_packet(buf, true)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        write_move_entity_packet(buf, self, true)
    }
}

pub fn read_move_entity_packet(
    buf: &mut Cursor<&[u8]>,
    include_look_direction: bool,
) -> Result<ClientboundMoveEntityPosRot, BufReadError> {
    let entity_id = MinecraftEntityId::azalea_read_var(buf)?;
    let properties = Properties::azalea_read(buf)?;

    let delta = VecDelta::read_with_steps(buf, properties.step_count())?;

    let look_direction = if include_look_direction {
        CompactLookDirection::azalea_read(buf)?
    } else {
        Default::default()
    };

    Ok(ClientboundMoveEntityPosRot {
        entity_id,
        properties,
        delta,
        look_direction,
    })
}
pub fn write_move_entity_packet(
    buf: &mut impl Write,
    p: &ClientboundMoveEntityPosRot,
    include_look_direction: bool,
) -> io::Result<()> {
    p.entity_id.azalea_write_var(buf)?;
    p.properties.azalea_write(buf)?;
    p.delta.write_and_get_steps(buf)?;

    if include_look_direction {
        p.look_direction.azalea_write(buf)?;
    };

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub enum VecDelta {
    Linear(PositionDelta8),
    Stepped(Vec<DeltaStep>),
}
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct DeltaStep {
    #[var]
    pub ticks: i32,
    pub pos: PositionDelta8,
}
impl VecDelta {
    pub fn read_with_steps(
        buf: &mut Cursor<&[u8]>,
        step_count: i32,
    ) -> Result<Self, azalea_buf::BufReadError> {
        if step_count <= 0 {
            return Ok(Self::Linear(PositionDelta8::azalea_read(buf)?));
        }

        let mut steps = Vec::new();
        for _ in 0..step_count {
            steps.push(DeltaStep::azalea_read(buf)?);
        }
        Ok(Self::Stepped(steps))
    }
    pub fn write_and_get_steps(&self, buf: &mut impl Write) -> io::Result<i32> {
        match self {
            VecDelta::Linear(v) => {
                v.azalea_write(buf)?;
                Ok(0)
            }
            VecDelta::Stepped(steps) => {
                for s in steps {
                    s.azalea_write(buf)?;
                }
                Ok(steps.len() as i32)
            }
        }
    }

    pub fn decode(&self, codec: &VecDeltaCodec) -> PositionPath {
        match self {
            VecDelta::Linear(p) => PositionPath::Linear(codec.decode(p)),
            VecDelta::Stepped(steps) => {
                if steps.is_empty() {
                    return PositionPath::Linear(codec.base());
                }
                let mut position_steps = Vec::with_capacity(steps.len());
                let mut codec = codec.clone();

                for step in steps {
                    let pos = codec.decode(&step.pos);
                    position_steps.push(PositionStep {
                        pos,
                        tick_offset: step.ticks,
                    });
                    codec.set_base(pos);
                }
                PositionPath::Stepped(position_steps)
            }
        }
    }
}

#[derive(AzBuf, Copy, Clone, Debug, PartialEq, Default)]
pub struct Properties {
    #[var]
    data: u32,
}
impl Properties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_ground(self) -> bool {
        self.data & 0b1 != 0
    }
    pub fn with_on_ground(mut self, value: bool) -> Self {
        if value {
            self.data |= 0b1;
        } else {
            self.data = !(!self.data | 0b1);
        }
        self
    }

    pub fn step_count(self) -> i32 {
        (self.data >> 1) as i32
    }
    pub fn with_step_count(mut self, value: u32) -> Self {
        self.data = (value << 1) | (self.on_ground() as u32);
        self
    }
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Default)]
pub struct CompactLookDirection {
    pub y_rot: i8,
    pub x_rot: i8,
}

impl From<CompactLookDirection> for LookDirection {
    fn from(l: CompactLookDirection) -> Self {
        LookDirection::new(
            (l.y_rot as i32 * 360) as f32 / 256.,
            (l.x_rot as i32 * 360) as f32 / 256.,
        )
    }
}
