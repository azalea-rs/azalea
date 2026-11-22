use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Field, Ident, Variant, Visibility, punctuated::Punctuated, token::Comma};

use crate::{attrs, utils};

pub fn derive_codec_for_enum(
    vis: &Visibility,
    attrs: &[Attribute],
    ident: &Ident,
    variants: Punctuated<Variant, Comma>,
) -> TokenStream {
    let mut codec_read_impl = quote! {};
    let mut codec_write_impl = quote! {};

    let mut separate_variant_struct_decls = quote! {};

    let mut incomplete_struct = quote! {};
    let mut codec_read_map_item_impl = quote! {};
    let mut codec_write_to_map_impl = quote! {};
    let mut get_and_reset_result_impl = quote! {};
    let mut incomplete_struct_new = quote! {};
    let mut incomplete_struct_build = quote! {};

    let enum_attrs = match attrs::parse_enum_attrs(&attrs) {
        Ok(a) => a,
        Err(e) => return syn::Error::into_compile_error(e).into(),
    };
    let mut used_default_snake_case_key = false;

    for variant in &variants {
        let variant_name = &variant.ident;
        let (variant_attrs, mut unhandled_attrs) = attrs::parse_enum_variant_attrs(&variant.attrs);
        let mut used_variant_key = false;

        let mut get_variant_key = || {
            let mut key = variant_attrs.key.clone();
            if key.is_some() {
                used_variant_key = true;
            } else if enum_attrs.default_snake_case_key {
                key = Some(utils::to_snake_case(&variant_name.to_string()));
                used_default_snake_case_key = true;
            }
            key
        };

        // unit variants are to be parsed as string literals
        if variant.fields.is_empty() && enum_attrs.tag.is_none() {
            let Some(key) = get_variant_key() else {
                return syn::Error::new(
                    variant.ident.span(),
                    "must provide #[codec(key = \"...\")] for unit variants, or #[codec(default_snake_case_key)] must be set on the enum declaration",
                ).to_compile_error().into();
            };
            codec_read_impl.extend(quote! {
                match <::azalea_codec::types::StringLiteral<#key>>::codec_read(__codec_ops) {
                    Ok(r) => {
                        return Ok(Self::#variant_name);
                    }
                    Err(e) => {
                        __errors.push((#key, e));
                    }
                }
            });
            codec_write_impl.extend(quote! {
                Self::#variant_name => ::azalea_codec::Ops::create_string(#key),
            });
            continue;
        }

        let first_variant_field = variant.fields.iter().next();

        // whether the fields in this variant are to be named, which would
        // require us to split it into a separate struct
        let mut is_named_enum = false;

        if enum_attrs.tag.is_some() {
            is_named_enum = true;
        } else if let Some(first) = first_variant_field
            && first.ident.is_some()
        {
            is_named_enum = true;
        }

        let field_type: syn::Type;
        let mut debug_field_name_string = variant.ident.to_string();

        fn filter_attributes_in_fields(fields: &mut Punctuated<Field, Comma>) {
            for f in fields {
                f.attrs = attrs::filter_codec_attrs(&f.attrs)
                    .into_iter()
                    .cloned()
                    .collect();
            }
        }
        let incomplete_variant_field_name = Ident::new(
            &utils::to_snake_case(&variant_name.to_string()),
            variant_name.span(),
        );

        if is_named_enum {
            // named enum

            // to make handling named enums easier, when making the Incomplete
            // struct we move out the contents of the variant into a separate
            // struct that's marked #[derive(Codec)]
            debug_field_name_string = format!("{ident}::{variant_name}");
            // field type now becomes the name of the struct
            field_type = syn::Type::Verbatim(
                Ident::new(&format!("Codec__{ident}__{variant_name}"), ident.span())
                    .to_token_stream(),
            );

            let mut field_names = Vec::new();
            let mut construct_extracted_struct = quote! {};

            let mut variant_struct_contents = variant.fields.clone();
            // remove the attributes that aren't #[codec(...)]
            match &mut variant_struct_contents {
                syn::Fields::Named(f) => filter_attributes_in_fields(&mut f.named),
                syn::Fields::Unnamed(f) => filter_attributes_in_fields(&mut f.unnamed),
                _ => {}
            }

            let variant_struct_contents = if let Some(tag) = &enum_attrs.tag {
                // if we're using a tag, modify every variant struct to include
                // a __codec_tag string literal field

                let variant_key = get_variant_key();
                let Some(variant_key) = variant_key else {
                    return syn::Error::new(
                        variant.ident.span(),
                        "when #[codec(tag = \"...\")] is set on an enum, you must either set #[codec(key = \"...\")] for all variants, or add #[codec(default_snake_case_key)] to the enum declaration"
                    ).to_compile_error().into();
                };
                used_variant_key = true;

                let fields_inner = quote! {
                    #[codec(rename = #tag)]
                    __codec_tag: ::azalea_codec::types::StringLiteral<#variant_key>,
                };
                construct_extracted_struct.extend(quote! {
                    __codec_tag: Default::default(),
                });

                match variant_struct_contents {
                    syn::Fields::Named(f) => {
                        let f = &f.named;
                        for field in f {
                            field_names.push(field.ident.clone().unwrap());
                        }
                        quote! { { #fields_inner #f } }
                    }
                    syn::Fields::Unnamed(f) => {
                        let f = &f.unnamed;
                        assert_eq!(f.len(), 1);
                        // we'll have to turn this unnamed field into a named
                        // one to be able to fit the __codec_tag in there.
                        field_names.push(Ident::new("value", ident.span()));
                        quote! { {
                            #fields_inner
                            value: #f,
                        } }
                    }
                    syn::Fields::Unit => {
                        quote! { { #fields_inner } }
                    }
                }
            } else {
                for field in variant.fields.iter() {
                    let field_name = field
                        .ident
                        .as_ref()
                        .expect("cannot mix named and unnamed fields in Codec enums");
                    field_names.push(field_name.clone());
                }
                quote! {#variant_struct_contents}
            };

            let mut variant_codec_attrs = quote! {};
            for a in unhandled_attrs.drain(..) {
                variant_codec_attrs.extend(quote! {#a});
            }

            separate_variant_struct_decls.extend(quote! {
                #[doc(hidden)]
                #[allow(nonstandard_style)]
                #[derive(::azalea_codec::Codec)]
                #variant_codec_attrs
                #vis struct #field_type #variant_struct_contents
            });

            let mut comma_separated_field_names = quote! {};
            for field_name in &field_names {
                comma_separated_field_names.extend(quote! { #field_name, });
                construct_extracted_struct.extend(quote! { #field_name: #field_name.clone(), });
            }
            let construct_extracted_struct = quote! { #field_type { #construct_extracted_struct } };
            let construct_variant = quote! { Self::#variant_name { #comma_separated_field_names } };

            codec_read_impl.extend(quote! {
                match <#field_type>::codec_read(__codec_ops) {
                    Ok(#field_type { #comma_separated_field_names .. }) => {
                        return Ok(#construct_variant);
                    }
                    Err(e) => {
                        __errors.push((#debug_field_name_string, e));
                    }
                }
            });

            codec_write_impl.extend(quote! {
                #construct_variant => {
                    ::azalea_codec::Codec::codec_write(&#construct_extracted_struct)
                },
            });

            codec_write_to_map_impl.extend(quote! {
                #construct_variant => ::azalea_codec::MapCodec::codec_write_to_map(
                    &#construct_extracted_struct,
                    __map
                ),
            });
        } else {
            // unnamed enum
            assert_eq!(
                variant.fields.len(),
                1,
                "unnamed enum variants must have exactly one field"
            );
            let first_variant_field = first_variant_field.unwrap();
            field_type = first_variant_field.ty.clone();

            codec_read_impl.extend(quote! {
                match <#field_type>::codec_read(__codec_ops) {
                    Ok(r) => {
                        return Ok(Self::#variant_name(r));
                    }
                    Err(e) => {
                        __errors.push((stringify!(#field_type), e));
                    }
                }
            });
            codec_write_impl.extend(quote! {
                Self::#variant_name(v) => ::azalea_codec::Codec::codec_write(v),
            });

            codec_write_to_map_impl.extend(quote! {
                Self::#variant_name(r) => ::azalea_codec::MapCodec::codec_write_to_map(r, __map),
            });
        }
        incomplete_struct.extend(quote! {
            #incomplete_variant_field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
        });
        codec_read_map_item_impl.extend(quote! {
            <#field_type as ::azalea_codec::MapCodec<O>>::codec_read_map_item(&mut incomplete.#incomplete_variant_field_name, k, v);
        });
        get_and_reset_result_impl.extend(quote! {
            if <#field_type as ::azalea_codec::MapCodec<O>>::get_and_reset_result(&mut incomplete.#incomplete_variant_field_name).is_ok() {
                return Ok(());
            }
        });
        incomplete_struct_new.extend(quote! {
            #incomplete_variant_field_name: ::azalea_codec::Incomplete::new(#debug_field_name_string),
        });
        incomplete_struct_build.extend(quote! {
            if let Ok(r) = self.#incomplete_variant_field_name.build() {
                return Ok(Self::Complete::#variant_name(r));
            }
        });

        if variant_attrs.key.is_some() && !used_variant_key {
            panic!("unused #[codec(key = \"...\")]")
        }
        for attr in unhandled_attrs {
            panic!("unknown attribute {}", attr.to_token_stream().to_string())
        }
    }

    if enum_attrs.default_snake_case_key && !used_default_snake_case_key {
        panic!("#[codec(used_default_snake_case_key)] was set but unused");
    }

    let (incomplete, incomplete_ty) = if enum_attrs.is_map {
        let ident = Ident::new(&format!("__Incomplete__{ident}"), ident.span());
        (ident.to_token_stream(), quote! {#ident<O>})
    } else {
        (
            quote! {},
            quote! { ::azalea_codec::error::ReadResult<Self> },
        )
    };

    let mut out = quote! {
    #separate_variant_struct_decls

    #[automatically_derived]
    impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        type Incomplete = #incomplete_ty;
        fn codec_read(__codec_ops: &O) -> ::azalea_codec::error::ReadResult<Self> {
            let mut __errors = Vec::<(&'static str, ::azalea_codec::error::ReadError)>::new();

            #codec_read_impl

            Err(::azalea_codec::error::ReadError::no_variants_matched(__errors))
        }

        fn codec_write(&self) -> ::azalea_codec::error::WriteResult<O> {
            match self {
                #codec_write_impl
            }
        }
    }

    };

    if enum_attrs.is_map {
        out.extend(quote! {
            #[doc(hidden)]
            #[allow(nonstandard_style)]
            pub struct #incomplete<O: ::azalea_codec::Ops> {
                #incomplete_struct
            }
            #[automatically_derived]
            impl<O: ::azalea_codec::Ops> ::azalea_codec::MapCodec<O> for #ident {
                fn codec_read_map_item(
                    incomplete: &mut #incomplete_ty,
                    k: &O,
                    v: &O,
                ) {
                    #codec_read_map_item_impl
                }
                fn codec_write_to_map(&self, __map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
                    match self {
                        #codec_write_to_map_impl
                        _ => Err(azalea_codec::error::WriteError::custom(
                            "used #[codec(flatten)] on something that's not a map",
                        ))
                    }
                }
                fn get_and_reset_result(
                    incomplete: &mut #incomplete_ty,
                ) -> ::azalea_codec::error::ReadResult<()> {
                    #get_and_reset_result_impl
                    Err(::azalea_codec::error::ReadError::custom(
                        "every variant failed",
                    ))
                }
            }

            #[automatically_derived]
            impl<O: ::azalea_codec::Ops> ::azalea_codec::Incomplete for #incomplete<O> {
                type Complete = #ident;
                fn new(_: &str) -> Self {
                    Self {
                        #incomplete_struct_new
                    }
                }
                fn build(self) -> ::azalea_codec::error::ReadResult<Self::Complete> {
                    #incomplete_struct_build
                    Err(::azalea_codec::error::ReadError::custom(
                        "couldn't build any variant",
                    ))
                }
            }

            #[automatically_derived]
            impl<O: ::azalea_codec::Ops> ::azalea_codec::NewOk<O> for #incomplete_ty {
                fn new_ok() -> Self {
                    <Self as ::azalea_codec::Incomplete>::new_unnamed()
                }
            }

        })
    }

    out.into()
}
