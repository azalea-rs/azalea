use std::any::Any;

use crate::{BufReadError, BufReadErrorRepr};

/// Some context that's maintained when reading and writing packets.
///
/// For packets that involve registries, you are required to pass a
/// `RegistryHolder`, otherwise reading and writing them will fail. For other
/// packets, it's OK to pass `Ctx::default()`.
#[derive(Clone, Copy)]
pub struct Ctx<'a> {
    depth: u32,
    pub resolver: &'a dyn ResolverTrait,
}
impl<'a> Ctx<'a> {
    pub fn new(resolver: &'a dyn ResolverTrait) -> Self {
        Self { depth: 0, resolver }
    }
    /// Increment the depth counter, or error if it exceeds the limit.
    ///
    /// This exists to prevent exploits that make us crash by exhausting our
    /// stack space. The `AzBuf` macro adds this whenever [`Ctx`] is used,
    /// though if you know that recursion isn't possible with a type then it's
    /// not strictly necessary.
    pub fn inc_depth(mut self) -> Result<Self, BufReadError> {
        if self.depth > 255 {
            return Err(BufReadErrorRepr::DepthLimitExceeded.into());
        }
        self.depth += 1;
        Ok(self)
    }
}
impl<'a> Default for Ctx<'a> {
    fn default() -> Self {
        Self {
            depth: 0,
            resolver: &(),
        }
    }
}

/// A marker trait for types that can be passed to [`Ctx`].
pub trait ResolverTrait: Any + Send + Sync {}
impl ResolverTrait for () {}
