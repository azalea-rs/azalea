use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::server_links::ServerLinkEntry;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundServerLinks {
    pub links: Vec<ServerLinkEntry>,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzaleaRead;

    use super::*;

    #[test]
    fn test_read_server_links() {
        tracing_subscriber::fmt::try_init().ok();
        let contents = [
            1, 0, 10, 8, 0, 5, 99, 111, 108, 111, 114, 0, 7, 35, 48, 48, 70, 66, 57, 65, 8, 0, 4,
            116, 101, 120, 116, 0, 15, 65, 98, 111, 117, 116, 32, 86, 101, 108, 111, 99, 105, 116,
            97, 98, 0, 40, 104, 116, 116, 112, 115, 58, 47, 47, 119, 105, 108, 108, 105, 97, 109,
            50, 55, 56, 46, 110, 101, 116, 47, 112, 114, 111, 106, 101, 99, 116, 47, 118, 101, 108,
            111, 99, 105, 116, 97, 98,
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundServerLinks::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
