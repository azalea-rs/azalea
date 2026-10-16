use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundMoveVehicle {
    pub pos: Vec3,
    pub look_direction: LookDirection,
    pub on_ground: bool,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_move_vehicle_roundtrip() {
        let packet = ServerboundMoveVehicle {
            pos: Vec3::new(1.5, 64., -3.25),
            look_direction: LookDirection::new(90., 10.),
            on_ground: true,
        };
        let mut bytes = Vec::new();
        packet.azalea_write(&mut bytes).unwrap();

        // 3 doubles + 2 floats + bool
        assert_eq!(bytes.len(), 8 * 3 + 4 * 2 + 1);
        assert_eq!(*bytes.last().unwrap(), 1);

        let read = ServerboundMoveVehicle::azalea_read(&mut Cursor::new(bytes.as_slice())).unwrap();
        assert_eq!(read, packet);
    }
}
