use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, Data, DeriveInput, FieldsNamed, Ident};

fn create_impl_mcbufreadable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let FieldsNamed { named, .. } = match fields {
                syn::Fields::Named(f) => f,
                _ => panic!("#[derive(McBuf)] can only be used on structs with named fields"),
            };

            let read_fields = named
                .iter()
                .map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    // do a different buf.write_* for each field depending on the type
                    // if it's a string, use buf.write_string
                    match field_type {
                        syn::Type::Path(_) | syn::Type::Array(_) => {
                            if f.attrs.iter().any(|a| a.path.is_ident("var")) {
                                quote! {
                                    let #field_name = azalea_buf::McBufVarReadable::var_read_from(buf)?;
                                }
                            } else {
                                quote! {
                                    let #field_name = azalea_buf::McBufReadable::read_from(buf)?;
                                }
                            }
                        }
                        _ => panic!(
                            "Error reading field {}: {}",
                            field_name.clone().unwrap(),
                            field_type.to_token_stream()
                        ),
                    }
                })
                .collect::<Vec<_>>();
            let read_field_names = named.iter().map(|f| &f.ident).collect::<Vec<_>>();

            quote! {
            impl azalea_buf::McBufReadable for #ident {
                fn read_from(buf: &mut impl std::io::Read) -> Result<Self, azalea_buf::BufReadError> {
                    #(#read_fields)*
                    Ok(#ident {
                        #(#read_field_names: #read_field_names),*
                    })
                }
            }
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut match_contents = quote!();
            let mut variant_discrim: u32 = 0;
            for variant in variants {
                let variant_name = &variant.ident;
                match &variant.discriminant.as_ref() {
                    Some(d) => {
                        variant_discrim = match &d.1 {
                            syn::Expr::Lit(e) => match &e.lit {
                                syn::Lit::Int(i) => i.base10_parse().unwrap(),
                                _ => panic!("Error parsing enum discriminant as int"),
                            },
                            syn::Expr::Unary(_) => {
                                panic!("Negative enum discriminants are not supported")
                            }
                            _ => {
                                panic!("Error parsing enum discriminant as literal (is {:?})", d.1)
                            }
                        }
                    }
                    None => {
                        variant_discrim += 1;
                    }
                }
                match_contents.extend(quote! {
                    #variant_discrim => Ok(Self::#variant_name),
                });
            }

            quote! {
            impl azalea_buf::McBufReadable for #ident {
                fn read_from(buf: &mut impl std::io::Read) -> Result<Self, azalea_buf::BufReadError>
                {
                    let id = azalea_buf::McBufVarReadable::var_read_from(buf)?;
                    match id {
                        #match_contents
                        _ => Err(azalea_buf::BufReadError::UnexpectedEnumVariant { id: id as i32 }),
                    }
                }
            }
            }
        }
        _ => panic!("#[derive(McBuf)] can only be used on structs"),
    }
}

fn create_impl_mcbufwritable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let FieldsNamed { named, .. } = match fields {
                syn::Fields::Named(f) => f,
                _ => panic!("#[derive(McBuf)] can only be used on structs with named fields"),
            };

            let write_fields = named
            .iter()
            .map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                // do a different buf.write_* for each field depending on the type
                // if it's a string, use buf.write_string
                match field_type {
                    syn::Type::Path(_) | syn::Type::Array(_) => {
                        if f.attrs.iter().any(|attr| attr.path.is_ident("var")) {
                            quote! {
                                azalea_buf::McBufVarWritable::var_write_into(&self.#field_name, buf)?;
                            }
                        } else {
                            quote! {
                                azalea_buf::McBufWritable::write_into(&self.#field_name, buf)?;
                            }
                        }
                    }
                    _ => panic!(
                        "Error writing field {}: {}",
                        field_name.clone().unwrap(),
                        field_type.to_token_stream()
                    ),
                }
            })
            .collect::<Vec<_>>();

            quote! {
                impl azalea_buf::McBufWritable for #ident {
                    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                        #(#write_fields)*
                        Ok(())
                    }
                }
            }
        }
        syn::Data::Enum(syn::DataEnum { .. }) => {
            quote! {
                impl azalea_buf::McBufWritable for #ident {
                    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                        azalea_buf::Writable::write_varint(buf, *self as i32)
                    }
                }
            }
        }
        _ => panic!("#[derive(McBuf)] can only be used on structs"),
    }
}

#[proc_macro_derive(McBufReadable, attributes(var))]
pub fn derive_mcbufreadable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    create_impl_mcbufreadable(&ident, &data).into()
}

#[proc_macro_derive(McBufWritable, attributes(var))]
pub fn derive_mcbufwritable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    create_impl_mcbufwritable(&ident, &data).into()
}

#[proc_macro_derive(McBuf, attributes(var))]
pub fn derive_mcbuf(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let writable = create_impl_mcbufwritable(&ident, &data);
    let readable = create_impl_mcbufreadable(&ident, &data);
    quote! {
        #writable
        #readable
    }
    .into()
}
