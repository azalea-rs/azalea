use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, BufReadError};
use azalea_chat::FormattedText;
use azalea_codec::Codec;
use azalea_protocol_macros::ClientboundStatusPacket;

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct SamplePlayer {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[codec(lenient)]
    pub sample: Vec<SamplePlayer>,
}

// the entire packet is just json, which is why it has Codec
#[derive(ClientboundStatusPacket, Clone, Debug, PartialEq, Codec)]
pub struct ClientboundStatusResponse {
    #[codec(lenient)]
    pub description: FormattedText,
    #[codec(lenient)]
    pub favicon: Option<String>,
    #[codec(lenient)]
    pub players: Option<Players>,
    #[codec(lenient)]
    pub version: Option<Version>,
    #[codec(lenient)]
    #[codec(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
    #[codec(lenient)]
    pub contact: Option<String>,
}

impl AzBuf for ClientboundStatusResponse {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let string = String::azalea_read(ctx, buf)?;
        let mut string = string.into_bytes();
        let json = simd_json::from_slice::<simd_json::OwnedValue>(&mut string)?;

        Ok(Self::codec_read(&json, Default::default())?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let json: simd_json::OwnedValue = Self::codec_write(self, Default::default()).unwrap();
        let string = simd_json::to_string(&json)?;
        string.azalea_write(buf)
    }
}
