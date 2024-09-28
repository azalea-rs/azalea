use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundServerLinksPacket {
    pub links: Vec<ServerLinkEntry>,
}

#[derive(Clone, Debug, McBuf)]
pub struct ServerLinkEntry {
    pub kind: ServerLinkKind,
    pub link: String,
}

#[derive(Clone, Debug, McBuf)]
pub enum ServerLinkKind {
    Known(KnownLinkKind),
    Component(FormattedText),
}

#[derive(Clone, Copy, Debug, McBuf)]
pub enum KnownLinkKind {
    BugReport,
    CommunityGuidelines,
    Support,
    Status,
    Feedback,
    Community,
    Website,
    Forums,
    News,
    Announcements,
}
