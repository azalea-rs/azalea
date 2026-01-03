use quote::{ToTokens, quote};
use syn::{Data, Field, FieldsNamed, Generics, Ident, punctuated::Punctuated, token::Comma};

pub fn create_impl_azalearead(
    ident: &Ident,
    generics: &Generics,
    data: &Data,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let (read_fields, read_field_names) = read_named_fields(named);

                quote! {
                impl #impl_generics azalea_buf::AzaleaRead for #ident #ty_generics #where_clause {
                    fn azalea_read(buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        #(#read_fields)*
                        Ok(Self {
                            #(#read_field_names: #read_field_names),*
                        })
                    }
                }
                }
            }
            syn::Fields::Unit => {
                quote! {
                impl #impl_generics azalea_buf::AzaleaRead for #ident #ty_generics #where_clause {
                    fn azalea_read(buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        Ok(Self)
                    }
                }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let read_fields = read_unnamed_fields(&fields.unnamed);

                quote! {
                impl #impl_generics azalea_buf::AzaleaRead for #ident #ty_generics #where_clause {
                    fn azalea_read(buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                        Ok(Self(
                            #(#read_fields),*
                        ))
                    }
                }
                }
            }
        },
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
                                _ => panic!("Error parsing enum discriminant as int (is {e:?})"),
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
                            let is_variable_length =
                                f.attrs.iter().any(|a| a.path().is_ident("var"));
                            let limit =
                                f.attrs
                                    .iter()
                                    .find(|a| a.path().is_ident("limit"))
                                    .map(|a| {
                                        a.parse_args::<syn::LitInt>()
                                            .unwrap()
                                            .base10_parse::<usize>()
                                            .unwrap()
                                    });

                            if is_variable_length && limit.is_some() {
                                panic!("Fields cannot have both var and limit attributes");
                            }

                            if is_variable_length {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzaleaReadVar::azalea_read_var(buf)?),
                                });
                            } else if let Some(limit) = limit {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzaleaReadLimited::azalea_read_limited(buf, #limit)?),
                                });
                            } else {
                                reader_code.extend(quote! {
                                    Self::#variant_name(azalea_buf::AzaleaRead::azalea_read(buf)?),
                                });
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
            impl #impl_generics azalea_buf::AzaleaRead for #ident #ty_generics #where_clause {
                fn azalea_read(buf: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, azalea_buf::BufReadError> {
                    let id = azalea_buf::AzaleaReadVar::azalea_read_var(buf)?;
                    Self::azalea_read_id(buf, id)
                }
            }

            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn azalea_read_id(buf: &mut std::io::Cursor<&[u8]>, id: u32) -> std::result::Result<Self, azalea_buf::BufReadError> {
                    match id {
                        #match_contents
                        // you'd THINK this throws an error, but mojang decided to make it default for some reason
                        _ => {#first_reader}
                    }
                }
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
            let field_name = &f.ident;

            let reader_call = get_reader_call(f);
            quote! { let #field_name = #reader_call; }
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
                .base10_parse::<usize>()
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
                    azalea_buf::AzaleaReadVar::azalea_read_var(buf)?
                }
            } else if let Some(limit) = limit {
                quote! {
                    azalea_buf::AzaleaReadLimited::azalea_read_limited(buf, #limit)?
                }
            } else {
                quote! {
                    azalea_buf::AzaleaRead::azalea_read(buf)?
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
