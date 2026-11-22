mod attrs;
mod utils;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Field, FieldsNamed, GenericParam, Generics, Ident, Variant, Visibility,
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma,
};

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
            syn::Fields::Named(fields) => {
                derive_codec_for_named_fields_struct(&vis, &attrs, &ident, &generics, fields)
            }
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
            derive_codec_for_enum(&vis, &attrs, &ident, variants)
        }
        _ => {
            panic!("#[derive(Codec)] can only be used on structs")
        }
    }
}

fn derive_codec_for_named_fields_struct(
    vis: &Visibility,
    attrs: &[Attribute],
    ident: &Ident,
    generics: &Generics,
    fields: FieldsNamed,
) -> TokenStream {
    let attrs = match attrs::parse_struct_attrs(attrs) {
        Ok(a) => a,
        Err(e) => return syn::Error::into_compile_error(e).into(),
    };
    if attrs.transparent {
        assert_eq!(
            fields.named.len(),
            1,
            "cannot mark structs with more than one field as transparent"
        );
        return derive_transparent_impl(ident, &fields.named[0]);
    }

    let incomplete = Ident::new(&format!("__Incomplete{ident}"), ident.span());

    let mut incomplete_struct = quote! {};
    let mut incomplete_struct_new = quote! {};
    let mut incomplete_struct_build = quote! {};

    let mut codec_read_map_item_impl = quote! {};
    let mut codec_read_map_item_remaining_impl = quote! {};
    let mut codec_write_to_map_impl = quote! {};

    incomplete_struct.extend(quote! {
        // used for error tracking in individual variants
        __result: ::azalea_codec::error::ReadResult<()>,
        // O isn't used if we're not flattening anything
        __unused: ::std::marker::PhantomData<O>,
    });
    incomplete_struct_new.extend(quote! {
        __result: Ok(()),
        __unused: Default::default(),
    });

    for field in &fields.named {
        let field_name = field.ident.as_ref().expect("fields must be named");
        let field_type = &field.ty;

        let attrs = match attrs::parse_field_attrs(&field.attrs) {
            Ok(a) => a,
            Err(e) => return syn::Error::into_compile_error(e).into(),
        };

        let field_name_string = attrs.rename.unwrap_or_else(|| field_name.to_string());

        if attrs.optional || attrs.lenient || attrs.flatten {
            incomplete_struct.extend(quote! {
                #field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
            });

            // cool and normal
            incomplete_struct_new.extend(quote! {
                #field_name: <<#field_type as ::azalea_codec::Codec<O>>::Incomplete as ::azalea_codec::NewOk<_>>::new_ok(),
            });
        } else {
            incomplete_struct.extend(quote! {
                #field_name: ::azalea_codec::error::ReadResult<#field_type>,
            });

            incomplete_struct_new.extend(quote! {
                #field_name: ::azalea_codec::Incomplete::new(#field_name_string),
            });
        }

        incomplete_struct_build.extend(quote! {
            #field_name: self.#field_name.build()?.into(),
        });

        if attrs.flatten {
            // formatted like this to more easily handle multiple flattened
            // fields
            codec_read_map_item_remaining_impl.extend(quote! {
                <#field_type>::codec_read_map_item(&mut incomplete.#field_name, k, v);
                // it's possibly fine for the flattened struct to error here
                // without it being fatal, if we have a second flattened map
                // that we're going to decode next. so, we reset back the error
                // status to Ok(()) (by calling get_and_reset_result), and
                // fallthrough only if it was an error
                if <#field_type as ::azalea_codec::MapCodec<O>>::get_and_reset_result(&mut incomplete.#field_name).is_ok() {
                    return;
                }
            });
        } else if attrs.lenient {
            codec_read_map_item_impl.extend(quote! {
                #field_name_string => {
                    incomplete.#field_name = Ok(Codec::codec_read(v).unwrap_or_default());
                }
            });
        } else {
            codec_read_map_item_impl.extend(quote! {
                #field_name_string => {
                    incomplete.#field_name = Codec::codec_read(v);
                }
            });
        }

        if attrs.flatten {
            codec_write_to_map_impl.extend(quote! {
                self.#field_name.codec_write_to_map(__codec_map)?;
            });
        } else if attrs.lenient || attrs.optional {
            codec_write_to_map_impl.extend(quote! {
                if !::azalea_codec::Defaultable::<O>::is_default(&self.#field_name) {
                    __codec_map.push((
                        O::create_string(#field_name_string)?,
                        self.#field_name.codec_write()?,
                    ));
                }
            });
        } else {
            codec_write_to_map_impl.extend(quote! {
                __codec_map.push((
                    O::create_string(#field_name_string)?,
                    self.#field_name.codec_write()?
                ));
            })
        }
    }

    if !attrs.allow_unknown_keys {
        codec_read_map_item_remaining_impl.extend(
            quote! { incomplete.__result = Err(::azalea_codec::error::ReadError::unknown_key(name)); },
        );
    }

    let generic_param_names = utils::generic_names(generics.params.iter());
    let (_, generic_param_names_full, _) = generics.split_for_impl();

    let mut generic_decls = quote! {};
    for p in &generics.params {
        generic_decls.extend(quote! { #p });
        match p {
            // append `::azalea_codec::Codec<O>` as a bound
            GenericParam::Type(p) => {
                if p.bounds.is_empty() {
                    generic_decls.extend(quote! { : ::azalea_codec::Codec<O> });
                } else {
                    generic_decls.extend(quote! { + ::azalea_codec::Codec<O> });
                }
            }
            _ => {}
        }
        generic_decls.extend(quote! { , });
    }

    quote! {
    #[doc(hidden)]
    #[allow(nonstandard_style)]
    #vis struct #incomplete<O: ::azalea_codec::Ops, #generic_decls> {
        #incomplete_struct
    }
    #[automatically_derived]
    impl<O: ::azalea_codec::Ops, #generic_decls> ::azalea_codec::Incomplete for #incomplete<O, #generic_param_names> {
        type Complete = #ident #generic_param_names_full;
        fn new(_: &str) -> Self {
            Self {
                #incomplete_struct_new
            }
        }
        fn build(self) -> ::azalea_codec::error::ReadResult<Self::Complete> {
            self.__result?;

            Ok(#ident {
                #incomplete_struct_build
            })
        }
    }
    #[automatically_derived]
    impl<O: ::azalea_codec::Ops, #generic_decls> ::azalea_codec::NewOk<O> for #incomplete<O, #generic_param_names> {
        fn new_ok() -> Self {
            <Self as ::azalea_codec::Incomplete>::new_unnamed()
        }
    }

    #[automatically_derived]
    impl<O: ::azalea_codec::Ops, #generic_decls> ::azalea_codec::MapCodec<O> for #ident #generic_param_names_full {
        fn codec_read_map_item(
            incomplete: &mut <Self as ::azalea_codec::Codec<O>>::Incomplete,
            k: &O,
            v: &O,
        ) {
            match k.get_string().unwrap_or_default().as_str() {
                #codec_read_map_item_impl
                name => {
                    #codec_read_map_item_remaining_impl
                }
            }
        }

        fn codec_write_to_map(&self, __codec_map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
            #codec_write_to_map_impl
            Ok(())
        }

        fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ::azalea_codec::error::ReadResult<()> {
            let mut result = Ok(());
            ::std::mem::swap(&mut incomplete.__result, &mut result);
            result
        }
    }

    #[automatically_derived]
    impl<O: ::azalea_codec::Ops, #generic_decls> ::azalea_codec::Codec<O> for #ident #generic_param_names_full {
        type Incomplete = #incomplete<O, #generic_param_names>;
        fn codec_read(__codec_ops: &O) -> ::azalea_codec::error::ReadResult<Self> {
            use ::azalea_codec::{Incomplete, MapCodec, MapLike};

            let __codec_map = __codec_ops.get_map()?;
            let mut incomplete = Self::Incomplete::new_unnamed();
            __codec_map.for_each(|k, v| {
                Self::codec_read_map_item(&mut incomplete, k, v);
            });
            incomplete.build()
        }
        fn codec_write(&self) -> ::azalea_codec::error::WriteResult<O> {
            use ::azalea_codec::MapCodec;

            let mut __codec_map = Vec::new();
            Self::codec_write_to_map(&self, &mut __codec_map)?;
            O::create_map(&__codec_map)
        }
    }
    }
    .into()
}

fn derive_transparent_impl(ident: &Ident, field: &Field) -> TokenStream {
    if let Some(field_ident) = &field.ident {
        quote! {
        impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
            fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
                Ok(Self { #field_ident: <_>::codec_read(ops)? })
            }

            fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
                self.#field_ident.codec_write()
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
                self.0.codec_write()
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
            ().codec_write()
        }
    }
    }
    .into()
}

fn derive_codec_for_enum(
    vis: &Visibility,
    attrs: &[Attribute],
    ident: &Ident,
    variants: Punctuated<Variant, Comma>,
) -> TokenStream {
    let mut codec_read = quote! {};
    let mut codec_write = quote! {};

    let mut separate_variant_struct_decls = quote! {};

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

        // unit variants must have a key
        if variant.fields.is_empty() && enum_attrs.tag.is_none() {
            let key = get_variant_key()
                .expect("must provide #[codec(key = \"...\")] for unit variants, or #[codec(default_snake_case_key)] must be set on the enum declaration");
            codec_read.extend(quote! {
                match <::azalea_codec::types::StringLiteral<#key>>::codec_read(__codec_ops) {
                    Ok(r) => {
                        return Ok(Self::#variant_name);
                    }
                    Err(e) => {
                        __errors.push((#key, e));
                    }
                }
            });
            codec_write.extend(quote! {
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
        let field_name_string: String;

        fn filter_attributes_in_fields(fields: &mut Punctuated<Field, Comma>) {
            for f in fields {
                f.attrs = attrs::filter_codec_attrs(&f.attrs)
                    .into_iter()
                    .cloned()
                    .collect();
            }
        }

        if is_named_enum {
            // named enum

            // to make handling named enums easier, when making the Incomplete
            // struct we move out the contents of the variant into a separate
            // struct that's marked #[derive(Codec)]
            field_name_string = format!("Codec__{ident}__{variant_name}");
            let variant_struct_name_ident = Ident::new(&field_name_string, ident.span());

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
                let variant_key = get_variant_key();
                let Some(variant_key) = variant_key else {
                    return syn::Error::new(
                        variant.ident.span(),
                        "when #[codec(tag = \"...\")] is set on an enum, you must either set #[codec(key = \"...\")] for all variants, or add #[codec(default_snake_case_key)] to the enum declaration"
                    ).to_compile_error().into();
                };
                used_variant_key = true;
                // if `#[codec(tag="")]` is set on the enum, then every variant
                // struct that we make here must have a
                // `#[codec(rename="")] __codec_tag: StringLiteral<"">`
                // field

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
                #vis struct #variant_struct_name_ident #variant_struct_contents
            });

            let mut comma_separated_field_names = quote! {};
            for field_name in &field_names {
                comma_separated_field_names.extend(quote! { #field_name, });
                construct_extracted_struct.extend(quote! { #field_name: #field_name.clone(), });
            }

            codec_read.extend(quote! {
                match <#variant_struct_name_ident>::codec_read(__codec_ops) {
                    Ok(#variant_struct_name_ident { #comma_separated_field_names .. }) => {
                        return Ok(Self::#variant_name { #comma_separated_field_names });
                    }
                    Err(e) => {
                        __errors.push((#field_name_string, e));
                    }
                }
            });
            codec_write.extend(quote! {
                Self::#variant_name { #comma_separated_field_names } => {
                    #variant_struct_name_ident { #construct_extracted_struct }.codec_write()
                },
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

            codec_read.extend(quote! {
                match <#field_type>::codec_read(__codec_ops) {
                    Ok(r) => {
                        return Ok(Self::#variant_name(r));
                    }
                    Err(e) => {
                        __errors.push((stringify!(#field_type), e));
                    }
                }
            });
            codec_write.extend(quote! {
                Self::#variant_name(v) => v.codec_write(),
            });
        }

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

    quote! {
    #separate_variant_struct_decls

    #[automatically_derived]
    impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        type Incomplete = ::azalea_codec::error::ReadResult<Self>;
        fn codec_read(__codec_ops: &O) -> ::azalea_codec::error::ReadResult<Self> {
            let mut __errors = Vec::<(&'static str, ::azalea_codec::error::ReadError)>::new();

            #codec_read

            Err(::azalea_codec::error::ReadError::no_variants_matched(__errors))
        }

        fn codec_write(&self) -> ::azalea_codec::error::WriteResult<O> {
            match self {
                #codec_write
            }
        }
    }
    }
    .into()
}
