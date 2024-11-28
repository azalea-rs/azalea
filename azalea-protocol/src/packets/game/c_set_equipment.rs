use std::io::Cursor;

use azalea_buf::{AzBuf, BufReadError};
use azalea_buf::{AzaleaRead, AzaleaWrite};
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetEquipment {
    #[var]
    pub entity_id: u32,
    pub slots: EquipmentSlots,
}

#[derive(Clone, Debug)]
pub struct EquipmentSlots {
    pub slots: Vec<(EquipmentSlot, ItemStack)>,
}

impl AzaleaRead for EquipmentSlots {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut slots = vec![];

        loop {
            let equipment_byte = u8::azalea_read(buf)?;
            let equipment_slot =
                EquipmentSlot::from_byte(equipment_byte & 127).ok_or_else(|| {
                    BufReadError::UnexpectedEnumVariant {
                        id: equipment_byte.into(),
                    }
                })?;
            let item = ItemStack::azalea_read(buf)?;
            slots.push((equipment_slot, item));
            if equipment_byte & 128 == 0 {
                break;
            };
        }

        Ok(EquipmentSlots { slots })
    }
}
impl AzaleaWrite for EquipmentSlots {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        for i in 0..self.slots.len() {
            let (equipment_slot, item) = &self.slots[i];
            let mut equipment_byte = *equipment_slot as u8;
            if i != self.slots.len() - 1 {
                equipment_byte |= 128;
            }
            equipment_byte.azalea_write(buf)?;
            item.azalea_write(buf)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Copy, AzBuf)]
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
