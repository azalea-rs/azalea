use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::common::ClientInformation;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket, PartialEq, Eq)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::{McBufReadable, McBufWritable};

    use super::*;

    #[test]
    fn test_client_information_packet() {
        {
            let data = ClientInformation::default();
            let mut buf = Vec::new();
            data.write_into(&mut buf).unwrap();
            let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);

            let read_data = ClientInformation::read_from(&mut data_cursor).unwrap();
            assert_eq!(read_data, data);
        }

        {
            let data = ClientInformation {
                language: "en_gb".to_string(),
                view_distance: 24,
                chat_visibility: ChatVisibility::Hidden,
                chat_colors: false,
                model_customization: ModelCustomization {
                    cape: false,
                    jacket: false,
                    left_sleeve: true,
                    right_sleeve: false,
                    left_pants: true,
                    right_pants: false,
                    hat: true,
                },
                main_hand: HumanoidArm::Left,
                text_filtering_enabled: true,
                allows_listing: true,
                particle_status: ParticleStatus::Decreased,
            };
            let mut buf = Vec::new();
            data.write_into(&mut buf).unwrap();
            let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);

            let read_data = ClientInformation::read_from(&mut data_cursor).unwrap();
            assert_eq!(read_data, data);
        }
    }
}
