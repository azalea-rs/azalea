use azalea_buf::AzBuf;
use azalea_chat::FormattedText;

#[derive(Clone, Debug, AzBuf)]
pub struct ServerLinkEntry {
    pub kind: ServerLinkKind,
    pub link: String,
}

#[derive(Clone, Debug, AzBuf)]
pub enum ServerLinkKind {
    Known(KnownLinkKind),
    Component(FormattedText),
}

#[derive(Clone, Copy, Debug, AzBuf)]
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
