use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Data, Field, FieldsNamed, Ident, punctuated::Punctuated, token::Comma};

pub fn create_impl_azaleawrite(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let write_fields =
                    write_named_fields(named, Some(&Ident::new("self", Span::call_site())));

                quote! {
                    impl azalea_buf::AzaleaWrite for #ident {
                        fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            #write_fields
                            Ok(())
                        }
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    impl azalea_buf::AzaleaWrite for #ident {
                        fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            Ok(())
                        }
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let write_fields = write_unnamed_fields(&fields.unnamed);

                quote! {
                    impl azalea_buf::AzaleaWrite for #ident {
                        fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            #write_fields
                            Ok(())
                        }
                    }
                }
            }
        },
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
                                // syn::Lit::Str(s) => s.value(),
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
                        if first {
                            first = false;
                        } else {
                            variant_discrim += 1;
                        }
                    }
                }

                let variant_name = &variant.ident;

                // the variant number that we're going to write
                let write_the_variant = quote! {
                    azalea_buf::AzaleaWriteVar::azalea_write_var(&#variant_discrim, buf)?;
                };
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
                                #write_the_variant
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
                                #write_the_variant
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
                            if f.attrs.iter().any(|attr| attr.path().is_ident("var")) {
                                writers_code.extend(quote! {
                                    azalea_buf::AzaleaWriteVar::azalea_write_var(#param_ident, buf)?;
                                });
                            } else {
                                writers_code.extend(quote! {
                                    azalea_buf::AzaleaWrite::azalea_write(#param_ident, buf)?;
                                });
                            }
                        }
                        match_arms.extend(quote! {
                            Self::#variant_name(#params_code) => {
                                #write_the_variant
                                #writers_code
                            }
                        });
                        match_arms_without_id.extend(quote! {
                            Self::#variant_name(data) => {
                                azalea_buf::AzaleaWrite::azalea_write(data, buf)?;
                            }
                        });
                    }
                }
            }
            if is_data_enum {
                quote! {
                    impl azalea_buf::AzaleaWrite for #ident {
                        fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
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
                    impl azalea_buf::AzaleaWrite for #ident {
                        fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                            azalea_buf::AzaleaWriteVar::azalea_write_var(&(*self as u32), buf)
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(AzBuf)] can only be used on structs"),
    }
}

fn write_named_fields(
    named: &Punctuated<Field, Comma>,
    ident_name: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let write_fields = named.iter().map(|f| {
        let field_name = &f.ident;
        let ident_dot_field = match ident_name {
            Some(ident) => quote! { &#ident.#field_name },
            None => quote! { #field_name },
        };

        make_write_call(f, ident_dot_field)
    });
    quote! { #(#write_fields)* }
}

fn write_unnamed_fields(named: &Punctuated<Field, Comma>) -> proc_macro2::TokenStream {
    let write_fields = named.iter().enumerate().map(|(i, f)| {
        let i_literal = syn::Index::from(i);
        let ident_dot_field = quote! { &self.#i_literal };

        make_write_call(f, ident_dot_field)
    });
    quote! { #(#write_fields)* }
}

fn make_write_call(
    f: &Field,
    ident_dot_field: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_type = &f.ty;
    // do a different buf.write_* for each field depending on the type
    // if it's a string, use buf.write_string
    match field_type {
        syn::Type::Path(_) | syn::Type::Array(_) => {
            if f.attrs.iter().any(|attr| attr.path().is_ident("var")) {
                quote! {
                    azalea_buf::AzaleaWriteVar::azalea_write_var(#ident_dot_field, buf)?;
                }
            } else {
                quote! {
                    azalea_buf::AzaleaWrite::azalea_write(#ident_dot_field, buf)?;
                }
            }
        }
        _ => panic!(
            "Error writing field {:?}: {}",
            f.ident,
            field_type.to_token_stream()
        ),
    }
}
