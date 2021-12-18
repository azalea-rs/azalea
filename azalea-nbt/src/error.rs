#[derive(Debug)]
pub enum Error {
    InvalidTagType(u8),
    InvalidTag,
}
