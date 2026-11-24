#![doc = include_str!("../README.md")]
// used for `azalea_codec::types::ConstFromStr` impls
#![feature(const_trait_impl, const_cmp)]

// needed for the registry macro to be able to reference itself
extern crate self as azalea_registry_builtin;

use std::hash::Hash;

use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;

pub mod builtin;
pub mod tags;

pub trait Registry: AzBuf + PartialEq + PartialOrd + Ord + Copy + Hash
where
    Self: Sized,
{
    fn from_u32(value: u32) -> Option<Self>;
    fn to_u32(&self) -> u32;
}

/// A registry which has its values decided by the server in the
/// `ClientboundRegistryData` packet.
///
/// These can be resolved into their actual values with
/// `ResolvableDataRegistry` from azalea-core.
pub trait DataRegistry: AzBuf + PartialEq + PartialOrd + Ord + Copy + Hash {
    const NAME: &'static str;
    type Key: DataRegistryKey;

    fn protocol_id(&self) -> u32;
    fn new_raw(id: u32) -> Self;
}
pub trait DataRegistryKey {
    type Borrow<'a>: DataRegistryKeyRef<'a>;

    fn from_ident(ident: Identifier) -> Self;
    fn into_ident(self) -> Identifier;
}
pub trait DataRegistryKeyRef<'a> {
    type Owned: DataRegistryKey;

    fn to_owned(self) -> Self::Owned;
    fn from_ident(ident: &'a Identifier) -> Self;
    fn into_ident(self) -> Identifier;
}
impl<T: DataRegistry> Registry for T {
    fn from_u32(value: u32) -> Option<Self> {
        Some(Self::new_raw(value))
    }
    fn to_u32(&self) -> u32 {
        self.protocol_id()
    }
}
