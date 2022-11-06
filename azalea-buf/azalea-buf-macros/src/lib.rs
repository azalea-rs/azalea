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
                fn read_from(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
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
            let mut first = true;
            let mut first_reader = None;
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
                        if !first {
                            variant_discrim += 1;
                        }
                    }
                }
                let reader = match variant.fields {
                    syn::Fields::Named(_) => {
                        panic!("writing named fields in enums is not supported")
                    }
                    syn::Fields::Unnamed(_) => quote! {
                        Ok(Self::#variant_name(azalea_buf::McBufReadable::read_from(buf)?))
                    },
                    syn::Fields::Unit => quote! {
                        Ok(Self::#variant_name)
                    },
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

            quote! {
            impl azalea_buf::McBufReadable for #ident {
                fn read_from(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
                    let id = azalea_buf::McBufVarReadable::var_read_from(buf)?;
                    Self::read_from_id(buf, id)
                }
            }

            impl #ident {
                pub fn read_from_id(buf: &mut std::io::Cursor<&[u8]>, id: u32) -> Result<Self, azalea_buf::BufReadError> {
                    match id {
                        #match_contents
                        // you'd THINK this throws an error, but mojang decided to make it default for some reason
                        _ => #first_reader
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
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            // remember whether it's a data variant so we can do an optimization later
            let mut is_data_enum = false;
            let mut match_arms = quote!();
            let mut match_arms_without_id = quote!();
            let mut variant_discrim: u32 = 0;
            let mut first = true;
            for variant in variants {
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
                        if first {
                            first = false;
                        } else {
                            variant_discrim += 1;
                        }
                    }
                }

                match &variant.fields {
                    syn::Fields::Named(_) => {
                        panic!("Enum variants with named fields are not supported yet");
                    }
                    syn::Fields::Unit => {
                        let variant_name = &variant.ident;
                        match_arms.extend(quote! {
                            Self::#variant_name => {
                                azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
                            }
                        });
                        match_arms_without_id.extend(quote! {
                            Self::#variant_name => {}
                        });
                    }
                    syn::Fields::Unnamed(_) => {
                        is_data_enum = true;
                        let variant_name = &variant.ident;
                        match_arms.extend(quote! {
                            Self::#variant_name(data) => {
                                azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
                                azalea_buf::McBufWritable::write_into(data, buf)?;
                            }
                        });
                        match_arms_without_id.extend(quote! {
                            Self::#variant_name(data) => {
                                azalea_buf::McBufWritable::write_into(data, buf)?;
                            }
                        });
                    }
                }
            }
            if is_data_enum {
                quote! {
                    impl azalea_buf::McBufWritable for #ident {
                        fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            match self {
                                #match_arms
                            }
                            Ok(())
                        }
                    }
                    impl #ident {
                        pub fn write_without_id(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            match self {
                                #match_arms_without_id
                            }
                            Ok(())
                        }
                    }
                }
            } else {
                // optimization: if it doesn't have data we can just do `as u32`
                quote! {
                    impl azalea_buf::McBufWritable for #ident {
                        fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            azalea_buf::McBufVarWritable::var_write_into(&(*self as u32), buf)
                        }
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
