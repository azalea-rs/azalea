use azalea_buf::AzBuf;
use azalea_chat::FormattedText;

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ServerLinkEntry {
    pub kind: ServerLinkKind,
    pub link: String,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum ServerLinkKind {
    Component(FormattedText),
    Known(KnownLinkKind),
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
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
