use azalea_buf::McBuf;
use uuid::Uuid;

#[derive(Debug, Clone, McBuf)]
pub struct SaltSignaturePair {
    pub salt: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Default, McBuf, PartialEq)]
pub struct MessageSignature {
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct SignedMessageHeader {
    pub previous_signature: Option<MessageSignature>,
    pub sender: Uuid,
}

/// Generates a random u64 to use as a salt
pub fn make_salt() -> u64 {
    rand::random()
}
