use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, BufReadError};
use azalea_core::entity_id::MinecraftEntityId;
use azalea_inventory::{ItemStack, components::EquipmentSlot};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetEquipment {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub slots: EquipmentSlots,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentSlots {
    pub slots: Vec<(EquipmentSlot, ItemStack)>,
}

impl AzBuf for EquipmentSlots {
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
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_read_lifesteal_net_set_equipment() {
        let contents = [1, 128, 0, 129, 0, 130, 0, 131, 0, 132, 0, 133, 0, 7, 0];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundSetEquipment::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
