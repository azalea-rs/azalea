use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{self, punctuated::Punctuated, token::Comma, Data, Field, FieldsNamed, Ident};

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

pub fn create_impl_mcbufwritable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let syn::Fields::Named(FieldsNamed { named, .. }) = fields else {
                panic!("#[derive(McBuf)] can only be used on structs with named fields")
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
                    azalea_buf::McBufVarWritable::var_write_into(&#variant_discrim, buf)?;
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
                            if f.attrs.iter().any(|attr| attr.path.is_ident("var")) {
                                writers_code.extend(quote! {
                                    azalea_buf::McBufVarWritable::var_write_into(#param_ident, buf)?;
                                });
                            } else {
                                writers_code.extend(quote! {
                                    azalea_buf::McBufWritable::write_into(#param_ident, buf)?;
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
