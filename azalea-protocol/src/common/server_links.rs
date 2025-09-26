use azalea_buf::AzBuf;
use azalea_chat::FormattedText;

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct ServerLinkEntry {
    pub kind: ServerLinkKind,
    pub link: String,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub enum ServerLinkKind {
    Component(FormattedText),
    Known(KnownLinkKind),
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
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
