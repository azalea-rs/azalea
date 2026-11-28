use std::assert_matches;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Field, GenericParam, Generics, Ident, Variant, Visibility, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::{attrs, utils};

pub fn derive_codec_for_enum(
    vis: &Visibility,
    attrs: &[Attribute],
    ident: &Ident,
    generics: &Generics,
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

    let enum_attrs = match attrs::parse_enum_attrs(attrs) {
        Ok(a) => a,
        Err(e) => return syn::Error::into_compile_error(e).into(),
    };
    let mut used_default_snake_case_key = false;

    let mut used_variant_key_anywhere: bool = false;

    for variant in &variants {
        let variant_name = &variant.ident;
        let (variant_attrs, mut unhandled_attrs) = attrs::parse_enum_variant_attrs(&variant.attrs);

        if variant_attrs.skip {
            let panic_message = format!("writing {ident}::{variant_name} is not allowed");
            codec_write_impl.extend(quote! {
                Self::#variant_name { .. } => return Err(::azalea_codec::error::WriteError::custom(#panic_message)),
            });
            continue;
        }

        let mut used_variant_key = false;

        let mut get_variant_key_type = || {
            let mut key = variant_attrs.key.clone();
            if key.is_some() {
                used_variant_key = true;
            } else if enum_attrs.default_snake_case_key {
                key = Some(utils::to_snake_case(&variant_name.to_string()));
                used_default_snake_case_key = true;
            }

            if let Some(tag_as) = &enum_attrs.tag_as {
                key.map(|key| quote! { ::azalea_codec::types::FromLiteral<#key, #tag_as> })
            } else {
                key.map(|key| quote! { ::azalea_codec::types::StringLiteral<#key> })
            }
        };

        // unit variants are to be parsed as string literals
        if variant.fields.is_empty() && enum_attrs.tag.is_none() && !variant_attrs.unit {
            let Some(key_type) = get_variant_key_type() else {
                return syn::Error::new(
                    variant.ident.span(),
                    "Got a unit variant without a key.
You must do one of the following:
- Add #[codec(key = \"...\")] to this unit variant.
- To set default keys for all fields, add #[codec(default_snake_case_key)] to the enum declaration.
- Otherwise, if this variant was deliberately meant to match on an empty map, use #[codec(unit)].",
                )
                .to_compile_error()
                .into();
            };
            codec_read_impl.extend(quote! {
                match <#key_type>::codec_read(__codec_ops, ctx) {
                    Ok(v) => {
                        return Ok(Self::#variant_name);
                    }
                    Err(e) => {
                        __errors.push((stringify!(#variant_name), e));
                    }
                }
            });
            codec_write_impl.extend(quote! {
                Self::#variant_name => ::azalea_codec::Codec::codec_write(&<#key_type>::default(), ctx),
            });
            continue;
        }

        let first_variant_field = variant.fields.iter().next();

        // whether the fields in this variant are to be named, which would
        // require us to split it into a separate struct
        let mut is_treated_as_named_enum = false;
        let mut is_actually_named_enum = false;

        if let Some(first) = first_variant_field
            && first.ident.is_some()
        {
            is_treated_as_named_enum = true;
            is_actually_named_enum = true;
        } else if enum_attrs.tag.is_some() {
            // rust lets us pretend that a unit enum is named
            is_actually_named_enum = first_variant_field.is_none();
            is_treated_as_named_enum = true;
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

        if variant_attrs.fallback && enum_attrs.tag.is_none() {
            return syn::Error::new(
                variant.attrs[0].span(),
                "#[codec(fallback)] is set, but it wouldn't do anything since the enum isn't marked `#[codec(tag = \"...\")]`.",
            ).to_compile_error().into();
        }

        if is_treated_as_named_enum {
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
            let mut variant_codec_attrs = quote! {};

            let variant_struct_contents = if let Some(tag) = &enum_attrs.tag {
                // if we're using a tag, modify every variant struct to include
                // a __codec_tag string literal field

                let key_type = get_variant_key_type();
                let Some(key_type) = key_type else {
                    return syn::Error::new(
                        variant.ident.span(),
                        "when #[codec(tag = \"...\")] is set on an enum, you must either set #[codec(key = \"...\")] for all variants, or add #[codec(default_snake_case_key)] to the enum declaration"
                    ).to_compile_error().into();
                };
                used_variant_key = true;

                if !variant_attrs.fallback {
                    variant_codec_attrs.extend(quote! {
                        #[codec(virt(
                            #[codec(rename = #tag)]
                            __codec_tag: #key_type
                        ))]
                    });
                }

                match variant_struct_contents {
                    syn::Fields::Named(f) => {
                        let f = &f.named;
                        for field in f {
                            field_names.push(field.ident.clone().unwrap());
                        }
                        quote! { { #f } }
                    }
                    syn::Fields::Unnamed(f) => {
                        let f = &f.unnamed;
                        assert_eq!(f.len(), 1);
                        // we'll have to turn this unnamed field into a named
                        // one to be able to fit the __codec_tag in there.
                        field_names.push(Ident::new("value", ident.span()));
                        quote! { { #[codec(flatten)] value: #f, } }
                    }
                    syn::Fields::Unit => {
                        quote! { {} }
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

            for a in unhandled_attrs.drain(..) {
                variant_codec_attrs.extend(quote! {#a});
            }
            if variant_attrs.fallback {
                variant_codec_attrs.extend(quote! {
                    #[codec(allow_unknown_keys)]
                });
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
            let construct_variant = if is_actually_named_enum {
                quote! { Self::#variant_name { #comma_separated_field_names } }
            } else {
                quote! { Self::#variant_name(value) }
            };

            codec_read_impl.extend(quote! {
                match <#field_type>::codec_read(__codec_ops, ctx) {
                    Ok(#field_type { #comma_separated_field_names }) => {
                        return Ok(#construct_variant);
                    }
                    Err(e) => {
                        __errors.push((#debug_field_name_string, e));
                    }
                }
            });

            codec_write_impl.extend(quote! {
                #construct_variant => {
                    ::azalea_codec::Codec::codec_write(&#construct_extracted_struct, ctx)
                },
            });

            codec_write_to_map_impl.extend(quote! {
                #construct_variant => ::azalea_codec::MapCodec::codec_write_to_map(
                    &#construct_extracted_struct,
                    ctx,
                    __map
                ),
            });
        } else {
            // unnamed enum
            assert_matches!(
                variant.fields.len(),
                // 0 is only possible when using #[codec(unit)]
                0 | 1,
                "unnamed enum variants must have exactly one field"
            );
            if let Some(first_variant_field) = first_variant_field {
                field_type = first_variant_field.ty.clone();
            } else {
                field_type = syn::Type::Tuple(syn::TypeTuple {
                    attrs: Default::default(),
                    paren_token: Default::default(),
                    elems: Default::default(),
                });
            }

            let construct_variant;
            let value_ident;
            let value_ref;
            if variant_attrs.unit {
                value_ident = quote! { () };
                value_ref = quote! { &() };
                construct_variant = quote! { Self::#variant_name };
            } else {
                value_ident = quote! { v };
                value_ref = quote! { v };
                construct_variant = quote! { Self::#variant_name(#value_ident) };
            };

            codec_read_impl.extend(quote! {
                match <#field_type>::codec_read(__codec_ops, ctx) {
                    Ok(#value_ident) => {
                        return Ok(#construct_variant);
                    }
                    Err(e) => {
                        __errors.push((stringify!(#field_type), e));
                    }
                }
            });
            codec_write_impl.extend(quote! {
                #construct_variant => ::azalea_codec::Codec::codec_write(#value_ref, ctx),
            });

            codec_write_to_map_impl.extend(quote! {
                #construct_variant => ::azalea_codec::MapCodec::codec_write_to_map(#value_ref, ctx, __map),
            });
        }
        incomplete_struct.extend(quote! {
            #incomplete_variant_field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
        });
        codec_read_map_item_impl.extend(quote! {
            <#field_type as ::azalea_codec::MapCodec<O>>::codec_read_map_item(&mut incomplete.#incomplete_variant_field_name, ctx, k, v);
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
            if let Ok(v) = self.#incomplete_variant_field_name.build() {
                return Ok(Self::Complete::#variant_name(v));
            }
        });

        if variant_attrs.key.is_some() && !used_variant_key {
            panic!("unused #[codec(key = \"...\")]")
        }
        used_variant_key_anywhere |= used_variant_key;
        if let Some(attr) = unhandled_attrs.first() {
            panic!("unknown attribute {}", attr.to_token_stream())
        }
    }

    if enum_attrs.default_snake_case_key && !used_default_snake_case_key {
        panic!("unused #[codec(used_default_snake_case_key)]");
    }
    if enum_attrs.tag_as.is_some() && !used_variant_key_anywhere {
        panic!("unused #[codec(tag_as = ...)]");
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

    let (_, ty_generics, _) = generics.split_for_impl();

    let mut impl_generics_inner = quote! { O: ::azalea_codec::Ops };
    for param in &generics.params {
        impl_generics_inner.extend(quote! { , #param });
        if let GenericParam::Type(ty) = param {
            if ty.colon_token.is_some() {
                impl_generics_inner.extend(quote! { + ::azalea_codec::Codec<O> });
            } else {
                impl_generics_inner.extend(quote! { : ::azalea_codec::Codec<O> });
            }
        }
    }

    let mut out = quote! {
    #separate_variant_struct_decls

    #[automatically_derived]
    impl <#impl_generics_inner> ::azalea_codec::Codec<O> for #ident #ty_generics {
        type Incomplete = #incomplete_ty;
        fn codec_read(__codec_ops: &O, ctx: ::azalea_codec::Ctx<O>) -> ::azalea_codec::error::ReadResult<Self> {
            let mut __errors = Vec::<(&'static str, ::azalea_codec::error::ReadError)>::new();

            #codec_read_impl

            Err(::azalea_codec::error::ReadError::no_variants_matched(__errors))
        }

        fn codec_write(&self, ctx: ::azalea_codec::Ctx<O>) -> ::azalea_codec::error::WriteResult<O> {
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
                    ctx: ::azalea_codec::Ctx<O>,
                    k: &O,
                    v: &O,
                ) {
                    #codec_read_map_item_impl
                }
                fn codec_write_to_map(&self, ctx: ::azalea_codec::Ctx<O>, __map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
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
