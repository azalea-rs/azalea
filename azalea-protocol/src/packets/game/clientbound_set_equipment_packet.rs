use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::Slot;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::Cursor;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEquipmentPacket {
    #[var]
    pub entity: i32,
    pub slots: EquipmentSlots,
}

#[derive(Clone, Debug)]
pub struct EquipmentSlots {
    pub slots: Vec<(EquipmentSlot, Slot)>,
}

impl McBufReadable for EquipmentSlots {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut slots = vec![];

        loop {
            let equipment_byte = u8::read_from(buf)?;
            let equipment_slot =
                EquipmentSlot::from_byte(equipment_byte & 127).ok_or_else(|| {
                    BufReadError::UnexpectedEnumVariant {
                        id: equipment_byte.into(),
                    }
                })?;
            let item = Slot::read_from(buf)?;
            slots.push((equipment_slot, item));
            if equipment_byte & 128 == 0 {
                break;
            };
        }

        Ok(EquipmentSlots { slots })
    }
}
impl McBufWritable for EquipmentSlots {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        for i in 0..self.slots.len() {
            let (equipment_slot, item) = &self.slots[i];
            let mut equipment_byte = *equipment_slot as u8;
            if i != self.slots.len() - 1 {
                equipment_byte |= 128;
            }
            equipment_byte.write_into(buf)?;
            item.write_into(buf)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum EquipmentSlot {
    MainHand = 0,
    OffHand = 1,
    Feet = 2,
    Legs = 3,
    Chest = 4,
    Head = 5,
}

impl EquipmentSlot {
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(EquipmentSlot::MainHand),
            1 => Some(EquipmentSlot::OffHand),
            2 => Some(EquipmentSlot::Feet),
            3 => Some(EquipmentSlot::Legs),
            4 => Some(EquipmentSlot::Chest),
            5 => Some(EquipmentSlot::Head),
            _ => None,
        }
    }
}
