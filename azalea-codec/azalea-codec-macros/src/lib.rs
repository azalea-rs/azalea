mod attrs;
mod derive_enum;
mod derive_struct;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, parse_macro_input};

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
                derive_transparent_impl(&ident, &fields.unnamed[0])
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

fn derive_transparent_impl(ident: &Ident, field: &Field) -> TokenStream {
    if let Some(field_ident) = &field.ident {
        quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
            fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
                Ok(Self { #field_ident: <_>::codec_read(ops)? })
            }

            fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
                ::azalea_codec::Codec::codec_write(&self.#field_ident)
            }
        }
        }
        .into()
    } else {
        quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
            fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
                <_>::codec_read(ops).map(Self)
            }

            fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
                ::azalea_codec::Codec::codec_write(&self.0)
            }
        }
        }
        .into()
    }
}

fn derive_unit_struct_impl(ident: &Ident) -> TokenStream {
    quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
            <()>::codec_read(ops)?;
            Ok(Self)
        }
        fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
            ::azalea_codec::Codec::codec_write(&())
        }
    }
    }
    .into()
}
