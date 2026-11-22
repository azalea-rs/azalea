mod attrs;
mod derive_enum;
mod derive_struct;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Field, Ident, parse_macro_input};

use crate::attrs::{parse_field_attrs, parse_struct_attrs};

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
            derive_enum::derive_codec_for_enum(&vis, &attrs, &ident, variants)
        }
        _ => {
            panic!("#[derive(Codec)] can only be used on structs")
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
        quote! { #with::codec_read(ops).map(|c| c.0) }
    } else {
        quote! { <_>::codec_read(ops) }
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
        fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
            #read_expr
        }
        fn codec_write(&self, ctx: O::Context) -> azalea_codec::error::WriteResult<O> {
            ::azalea_codec::Codec::codec_write(&#write_expr, ctx)
        }
    }
    }
    .into()
}

fn derive_unit_struct_impl(ident: &Ident) -> TokenStream {
    quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
            <()>::codec_read(ops)?;
            Ok(Self)
        }
        fn codec_write(&self, ctx: O::Context) -> azalea_codec::error::WriteResult<O> {
            ::azalea_codec::Codec::codec_write(&(), ctx)
        }
    }
    }
    .into()
}
