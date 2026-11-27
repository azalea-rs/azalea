use std::env;

use quote::{ToTokens, quote};
use syn::{Attribute, Data, Field, FieldsNamed, Ident, Type, punctuated::Punctuated, token::Comma};

pub fn create_fn_azalea_read(data: &Data, attrs: &[Attribute]) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let (read_fields, read_field_names) = read_named_fields(named);

                quote! {
                    fn azalea_read(ctx: ::azalea_buf::Ctx, buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        #(#read_fields)*
                        Ok(Self {
                            #(#read_field_names),*
                        })
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        Ok(Self)
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let read_fields = read_unnamed_fields(&fields.unnamed);

                quote! {
                    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        Ok(Self(
                            #(#read_fields),*
                        ))
                    }
                }
            }
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut match_contents = quote!();
            let mut variant_discrim: u32 = 0;
            let mut first = true;
            let mut first_reader = None;

            let mut could_be_a_transmute = true;

            let mut is_repr_u32 = false;
            for attr in attrs {
                if attr.path().is_ident("repr")
                    && let Ok(list) = attr.meta.require_list()
                    && let Ok(ty) = list.parse_args::<Type>()
                    && ty.to_token_stream().to_string() == "u32"
                {
                    is_repr_u32 = true;
                    break;
                }
            }

            for variant in variants {
                let variant_name = &variant.ident;
                match &variant.discriminant.as_ref() {
                    Some(d) => {
                        let parsed_variant_discrim = match &d.1 {
                            syn::Expr::Lit(e) => match &e.lit {
                                syn::Lit::Int(i) => i.base10_parse().unwrap(),
                                _ => panic!("Error parsing enum discriminant as int (is {e:?})"),
                            },
                            syn::Expr::Unary(_) => {
                                panic!("Negative enum discriminants are not supported")
                            }
                            _ => {
                                panic!("Error parsing enum discriminant as literal (is {:?})", d.1)
                            }
                        };
                        let previous_plus_one = if first { 0 } else { variant_discrim + 1 };
                        if parsed_variant_discrim != previous_plus_one {
                            could_be_a_transmute = false;
                        }
                        variant_discrim = parsed_variant_discrim;
                    }
                    None => {
                        if !first {
                            variant_discrim += 1;
                        }
                    }
                }

                #[allow(clippy::needless_late_init)] // kept for readability
                let reader;
                match &variant.fields {
                    syn::Fields::Named(f) => {
                        could_be_a_transmute = false;
                        let (read_fields, read_field_names) = read_named_fields(&f.named);

                        reader = quote! {
                            #(#read_fields)*
                            Ok(Self::#variant_name {
                                #(#read_field_names),*
                            })
                        };
                    }
                    syn::Fields::Unnamed(fields) => {
                        could_be_a_transmute = false;
                        let mut reader_code = quote! {};
                        for f in &fields.unnamed {
                            let is_variable_length =
                                f.attrs.iter().any(|a| a.path().is_ident("var"));
                            let limit =
                                f.attrs
                                    .iter()
                                    .find(|a| a.path().is_ident("limit"))
                                    .map(|a| {
                                        a.parse_args::<syn::LitInt>()
                                            .unwrap()
                                            .base10_parse::<u32>()
                                            .unwrap()
                                    });

                            if is_variable_length && limit.is_some() {
                                panic!("Fields cannot have both var and limit attributes");
                            }

                            if is_variable_length {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzBufVar::azalea_read_var(ctx.inc_depth()?, buf)?),
                                });
                            } else if let Some(limit) = limit {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzBufLimited::azalea_read_limited(ctx.inc_depth()?, buf, #limit)?),
                                });
                            } else {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzBuf::azalea_read(ctx.inc_depth()?, buf)?),
                                });
                            }
                        }
                        reader = quote! { Ok(#reader_code) };
                    }
                    syn::Fields::Unit => reader = quote! { Ok(Self::#variant_name) },
                };
                if first {
                    first_reader = Some(reader.clone());
                    first = false;
                };

                match_contents.extend(quote! {
                    #variant_discrim => {
                        #reader
                    },
                });
            }

            let first_reader = first_reader.expect("There should be at least one variant");

            let match_expr = if could_be_a_transmute && is_repr_u32 {
                // match statements are often slow to compile and execute, so if
                // the user gave us a #[repr(u32)] we'll just transmute :)
                quote! {
                    if id > #variant_discrim {
                        return #first_reader;
                    }
                    Ok(unsafe { std::mem::transmute::<u32, Self>(id) })
                }
            } else {
                quote! {
                    match id {
                        #match_contents
                        // you'd THINK this throws an error, but mojang decided to make it default for some reason
                        _ => {#first_reader}
                    }
                }
            };

            quote! {
                fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                    let id = azalea_buf::AzBufVar::azalea_read_var(ctx.inc_depth()?, buf)?;
                    #match_expr
                }
            }
        }
        _ => panic!("#[derive(AzBuf)] can only be used on structs"),
    }
}

fn read_named_fields(
    named: &Punctuated<Field, Comma>,
) -> (Vec<proc_macro2::TokenStream>, Vec<&Option<Ident>>) {
    let read_fields = named
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();

            let reader_call = get_reader_call(f);
            let ty = &f.ty;

            // this is only done in debug mode for performance
            if cfg!(debug_assertions) {
                let span_message = field_name.to_string();

                // updated by the build.rs. it's done this way so the user
                // doesn't have to force a recompile themselves if they change
                // the env variable.
                let log_all_packet_fields =
                    env!("__INTERNAL_AZALEA_LOG_ALL_PACKET_FIELDS") == "true";

                let trace_field_value_code = if log_all_packet_fields {
                    quote! {
                        let __internal_debug = azalea_buf::internal::debug_format_any!(#field_name);
                        azalea_buf::internal::tracing::trace!("{__internal_debug}");
                    }
                } else {
                    quote! {}
                };

                quote! {
                    let span = azalea_buf::internal::tracing::span!(
                        azalea_buf::internal::tracing::Level::INFO,
                        #span_message
                    ).entered();
                    let #field_name: #ty = #reader_call;
                    #trace_field_value_code
                    span.exit();
                }
            } else {
                quote! { let #field_name = #reader_call; }
            }
        })
        .collect::<Vec<_>>();
    let read_field_names = named.iter().map(|f| &f.ident).collect::<Vec<_>>();

    (read_fields, read_field_names)
}

fn read_unnamed_fields(unnamed: &Punctuated<Field, Comma>) -> Vec<proc_macro2::TokenStream> {
    unnamed
        .iter()
        .map(|f| {
            let reader_call = get_reader_call(f);
            quote! { #reader_call }
        })
        .collect::<Vec<_>>()
}

fn get_reader_call(f: &Field) -> proc_macro2::TokenStream {
    let is_variable_length = f
        .attrs
        .iter()
        .any(|a: &syn::Attribute| a.path().is_ident("var"));
    let limit = f
        .attrs
        .iter()
        .find(|a| a.path().is_ident("limit"))
        .map(|a| {
            a.parse_args::<syn::LitInt>()
                .unwrap()
                .base10_parse::<u32>()
                .unwrap()
        });

    if is_variable_length && limit.is_some() {
        panic!("Fields cannot have both var and limit attributes");
    }

    let field_type = &f.ty;

    // do a different buf.write_* for each field depending on the type
    // if it's a string, use buf.write_string
    match field_type {
        syn::Type::Path(_) | syn::Type::Array(_) => {
            if is_variable_length {
                quote! {
                    azalea_buf::AzBufVar::azalea_read_var(ctx.inc_depth()?, buf)?
                }
            } else if let Some(limit) = limit {
                quote! {
                    azalea_buf::AzBufLimited::azalea_read_limited(ctx.inc_depth()?, buf, #limit)?
                }
            } else {
                quote! {
                    azalea_buf::AzBuf::azalea_read(ctx.inc_depth()?, buf)?
                }
            }
        }
        _ => panic!(
            "Error reading field {:?}: {}",
            f.ident.clone(),
            field_type.to_token_stream()
        ),
    }
}
