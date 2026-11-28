#![doc = include_str!("../README.md")]
// used for `azalea_codec::types::ConstFromStr` impls
#![feature(const_trait_impl, const_cmp)]

// needed for the registry macro to be able to reference itself
extern crate self as azalea_registry_builtin;

use std::hash::Hash;

use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;

pub mod builtin;

pub trait BuiltinRegistry:
    AzBuf + PartialEq + PartialOrd + Ord + Clone + Hash + Into<Identifier> + TryFrom<Identifier> + Clone
{
    const NAME: &'static str;

    fn builtin_from_id(id: u32) -> Option<Self>;
    fn builtin_into_id(&self) -> u32;
}
