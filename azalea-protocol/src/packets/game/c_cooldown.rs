use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::identifier::Identifier;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundCooldown {
    pub cooldown_group: Identifier,
    #[var]
    pub duration: u32,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_read_cooldown() {
        let mut bytes = vec![21];
        bytes.extend_from_slice(b"minecraft:ender_pearl");
        bytes.push(20);

        let mut buf = Cursor::new(bytes.as_slice());
        let packet = ClientboundCooldown::azalea_read(&mut buf).unwrap();
        assert_eq!(packet.cooldown_group, Identifier::new("minecraft:ender_pearl"));
        assert_eq!(packet.duration, 20);
        assert_eq!(buf.position() as usize, bytes.len());
    }
}
