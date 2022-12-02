use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field,
    FieldsNamed, Ident,
};

fn read_named_fields(
    named: &Punctuated<Field, Comma>,
) -> (Vec<proc_macro2::TokenStream>, Vec<&Option<Ident>>) {
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

    (read_fields, read_field_names)
}

fn create_impl_mcbufreadable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let FieldsNamed { named, .. } = match fields {
                syn::Fields::Named(f) => f,
                _ => panic!("#[derive(McBuf)] can only be used on structs with named fields"),
            };

            let (read_fields, read_field_names) = read_named_fields(named);

            quote! {
            impl azalea_buf::McBufReadable for #ident {
                fn read_from(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
                    println!("Reading struct {}", stringify!(#ident));
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
                let reader = match &variant.fields {
                    syn::Fields::Named(f) => {
                        let (read_fields, read_field_names) = read_named_fields(&f.named);

                        quote! {
                            #(#read_fields)*
                            Ok(#ident::#variant_name {
                                #(#read_field_names: #read_field_names),*
                            })
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let mut reader_code = quote! {};
                        for f in &fields.unnamed {
                            if f.attrs.iter().any(|attr| attr.path.is_ident("var")) {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::McBufVarReadable::var_read_from(buf)?),
                                })
                            } else {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::McBufReadable::read_from(buf)?),
                                })
                            }
                        }
                        quote! { Ok(#reader_code) }
                    }
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
                    println!("Reading enum {}", stringify!(#ident));
                    let id = azalea_buf::McBufVarReadable::var_read_from(buf)?;
                    Self::read_from_id(buf, id)
                }
            }

            impl #ident {
                pub fn read_from_id(buf: &mut std::io::Cursor<&[u8]>, id: u32) -> Result<Self, azalea_buf::BufReadError> {
                    match id {
                        #match_contents
                        // you'd THINK this throws an error, but mojang decided to make it default for some reason
                        _ => {#first_reader}
                    }
                }
            }
            }
        }
        _ => panic!("#[derive(McBuf)] can only be used on structs"),
    }
}

fn write_named_fields(
    named: &Punctuated<Field, Comma>,
    ident_name: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let write_fields = named.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        let ident_dot_field = match ident_name {
            Some(ident) => quote! { &#ident.#field_name },
            None => quote! { #field_name },
        };
        // do a different buf.write_* for each field depending on the type
        // if it's a string, use buf.write_string
        match field_type {
            syn::Type::Path(_) | syn::Type::Array(_) => {
                if f.attrs.iter().any(|attr| attr.path.is_ident("var")) {
                    quote! {
                        azalea_buf::McBufVarWritable::var_write_into(#ident_dot_field, buf)?;
                    }
                } else {
                    quote! {
                        azalea_buf::McBufWritable::write_into(#ident_dot_field, buf)?;
                    }
                }
            }
            _ => panic!(
                "Error writing field {}: {}",
                field_name.clone().unwrap(),
                field_type.to_token_stream()
            ),
        }
    });
    quote! { #(#write_fields)* }
}

fn create_impl_mcbufwritable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let FieldsNamed { named, .. } = match fields {
                syn::Fields::Named(f) => f,
                _ => panic!("#[derive(McBuf)] can only be used on structs with named fields"),
            };

            let write_fields =
                write_named_fields(named, Some(&Ident::new("self", Span::call_site())));

            quote! {
                impl azalea_buf::McBufWritable for #ident {
                    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                        #write_fields
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

                let variant_name = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(f) => {
                        is_data_enum = true;
                        let field_names = f
                            .named
                            .iter()
                            .map(|f| f.ident.clone().unwrap())
                            .collect::<Vec<_>>();
                        let write_fields = write_named_fields(&f.named, None);
                        match_arms.extend(quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
                                #write_fields
                            }
                        });
                        match_arms_without_id.extend(quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                #write_fields
                            }
                        });
                    }
                    syn::Fields::Unit => {
                        match_arms.extend(quote! {
                            Self::#variant_name => {
                                azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
                            }
                        });
                        match_arms_without_id.extend(quote! {
                            Self::#variant_name => {}
                        });
                    }
                    syn::Fields::Unnamed(fields) => {
                        is_data_enum = true;
                        let mut writers_code = quote! {};
                        let mut params_code = quote! {};
                        for (i, f) in fields.unnamed.iter().enumerate() {
                            let param_ident = Ident::new(&format!("data{i}"), Span::call_site());
                            params_code.extend(quote! { #param_ident, });
                            if f.attrs.iter().any(|attr| attr.path.is_ident("var")) {
                                writers_code.extend(quote! {
                                    azalea_buf::McBufVarWritable::var_write_into(#param_ident, buf)?;
                                })
                            } else {
                                writers_code.extend(quote! {
                                    azalea_buf::McBufWritable::write_into(#param_ident, buf)?;
                                })
                            }
                        }
                        match_arms.extend(quote! {
                            Self::#variant_name(#params_code) => {
                                azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
                                #writers_code
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
