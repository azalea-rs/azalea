#![doc = include_str!("../README.md")]

mod attrs;
mod derive_enum;
mod derive_struct;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Field, Ident, parse_macro_input};

use crate::attrs::{parse_field_attrs, parse_struct_attrs};

/// Automatically implement the `Codec` trait for arbitrary structs or enums.
///
/// ## Structs
///
/// Structs are typically represented as maps.
/// ```
/// # use azalea_codec::Codec;
/// # use serde_json::json;
/// #[derive(Codec, Debug, PartialEq)]
/// struct Thing {
///     meow: String,
///     woof: i32,
/// }
/// assert_eq!(
///     Thing::codec_read(&json!({"meow": "!!!", "woof": 100}), Default::default()).unwrap(),
///     Thing { meow: "!!!".into(), woof: 100 }
/// );
/// ```
///
/// You can also use structs with a single item to pass-through to another type,
/// like so:
/// ```
/// # use azalea_codec::Codec;
/// #[derive(Codec)]
/// struct Thing(String);
/// #[derive(Codec)]
/// #[codec(transparent)] // this attribute is required since the field is named
/// struct SameThing {
///     thing: Thing,
/// }
/// ```
///
/// TODO: it's 4am i'm tired i can finish writing this later
///
/// ## Enums
///
/// When `Codec` is derived on an enum, reading will attempt each variant
/// sequentially until one matches.
/// ```
/// # use azalea_codec::Codec;
/// #[derive(Codec)]
/// enum Thing {
///     A(i32),
///     B(String),
/// }
/// ```
///
/// You can easily represent enums that are serialized as strings with
/// #[codec(key = "...")].
/// ```
/// # use azalea_codec::Codec;
/// #[derive(Codec)]
/// enum Thing {
///     #[codec(key = "first")]
///     A,
///     #[codec(key = "second")]
///     B,
/// }
/// ```
///
/// On occasion, you may want keys in variants that are maps.
/// ```
/// # use azalea_codec::Codec;
/// # use serde_json::json;
/// #[derive(Codec, Debug, PartialEq)]
/// #[codec(tag = "type")]
/// enum Thing {
///     #[codec(key = "first")]
///     A { value: i32 },
///     #[codec(key = "second")]
///     B { value: String },
/// }
/// assert_eq!(
///     Thing::codec_read(&json!({"type": "first", "value": 1}), Default::default()).unwrap(),
///     Thing::A { value:1 }
/// );
/// ```
///
/// You may not want to manually write keys for every variant if their key
/// should be obvious from the variant name. For this, there is
/// `#[codec(default_snake_case_key)]`
///
/// ```
/// # use azalea_codec::Codec;
/// #[derive(Codec, Debug, PartialEq)]
/// #[codec(tag = "type")]
/// #[codec(default_snake_case_key)]
/// enum Thing {
///     A { value: i32 },
///     B { value: String },
/// }
/// ```
///
/// If the key should be read as a different static `Codec`, this is supported
/// too. Your keys are validated against that `const_from_str` at build-time, by
/// the way.
/// ```
/// #![feature(const_trait_impl, const_cmp)]
/// # use azalea_codec::Codec;
/// # use serde_json::json;
/// # use std::fmt::{self, Display};
/// use azalea_codec::types::ConstFromStr;
/// #[derive(Codec, Debug, PartialEq)]
/// #[codec(tag = "type", tag_as = Key)]
/// #[codec(default_snake_case_key)]
/// enum Thing {
///     A { value: i32 },
///     B { value: String },
/// }
/// #[derive(Codec, PartialEq)]
/// #[codec(default_snake_case_key)]
/// enum Key {
///     First,
///     Second,
/// }
/// const impl ConstFromStr for Key {
///     fn const_from_str(s: &str) -> Option<Self> {
///         if s == "a" { Some(Self::First) }
///         else if s == "b" { Some(Self::Second) }
///         else { None }
///     }
/// }
/// assert_eq!(
///     Thing::codec_read(&json!({"type": "first", "value": 1}), Default::default()).unwrap(),
///     Thing::A { value: 1 }
/// );
/// assert_eq!(
///     Thing::codec_read(&json!({"type": "second", "value": ""}), Default::default()).unwrap(),
///     Thing::B { value: "".to_string() }
/// );
/// ```
///
/// Sadly, implementing `ConstFromStr` requires enabling a Nightly-only feature:
/// [`const_trait_impl`](https://doc.rust-lang.org/beta/unstable-book/language-features/const-trait-impl.html).
///
/// Named variants are treated exactly the same as structs, and support all of
/// their `#[codec(...)]` attributes.
/// ```
/// # use azalea_codec::Codec;
/// #[derive(Codec)]
/// #[codec(tag = "id", default_snake_case_key)]
/// enum Thing {
///     #[codec(allow_unknown_keys)]
///     A { value: i32 },
///     B {
///         #[codec(lenient)]
///         value: Option<String>,
///     },
/// }
/// ```
#[proc_macro_derive(Codec, attributes(codec))]
pub fn derive_codec(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        vis,
        attrs,
        generics,
    } = parse_macro_input!(input);

    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(fields) => derive_struct::derive_codec_for_named_fields_struct(
                &vis, &attrs, &ident, &generics, fields,
            ),
            syn::Fields::Unnamed(fields) => {
                assert_eq!(
                    fields.unnamed.len(),
                    1,
                    "#[derive(Codec)] cannot be used on structs with more than one unnamed field."
                );
                derive_transparent_impl(&attrs, &ident, &fields.unnamed[0])
            }
            syn::Fields::Unit => derive_unit_struct_impl(&ident),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            derive_enum::derive_codec_for_enum(&vis, &attrs, &ident, &generics, variants)
        }
        _ => {
            panic!("#[derive(Codec)] can only be used on structs and enums")
        }
    }
}

fn derive_transparent_impl(attrs: &[Attribute], ident: &Ident, field: &Field) -> TokenStream {
    let _struct_attrs = match parse_struct_attrs(attrs) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error().into(),
    };
    let field_attrs = match parse_field_attrs(&field.attrs) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error().into(),
    };

    let base_read_expr = if let Some(with) = &field_attrs.with {
        quote! { #with::codec_read(ops, ctx).map(|c| c.0) }
    } else {
        quote! { <_>::codec_read(ops, ctx) }
    };

    let read_expr;
    let write_expr;
    if let Some(field_ident) = &field.ident {
        read_expr = quote! {
            #base_read_expr.map(|#field_ident| Self { #field_ident })
        };
        write_expr = quote! { self.#field_ident };
    } else {
        read_expr = quote! { #base_read_expr.map(Self) };
        write_expr = quote! { self.0 };
    };

    let write_expr = if let Some(with) = &field_attrs.with {
        quote! { #with(#write_expr.clone()) }
    } else {
        write_expr
    };

    quote! {
    impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        fn codec_read(ops: &O, ctx: ::azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
            #read_expr
        }
        fn codec_write(&self, ctx: ::azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
            ::azalea_codec::Codec::codec_write(&#write_expr, ctx)
        }
    }
    }
    .into()
}

fn derive_unit_struct_impl(ident: &Ident) -> TokenStream {
    quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        fn codec_read(ops: &O, ctx: ::azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
            <()>::codec_read(ops, ctx)?;
            Ok(Self)
        }
        fn codec_write(&self, ctx: ::azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
            ::azalea_codec::Codec::codec_write(&(), ctx)
        }
    }
    }
    .into()
}
