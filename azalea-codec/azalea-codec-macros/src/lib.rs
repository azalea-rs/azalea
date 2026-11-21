use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Variant, Visibility,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

#[proc_macro_derive(Codec, attributes(codec))]
pub fn derive_codec(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, vis, ..
    } = parse_macro_input!(input);

    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(fields) => {
                derive_codec_for_named_fields_struct(&vis, &ident, fields)
            }
            syn::Fields::Unnamed(fields) => derive_codec_for_unnamed_fields_struct(&ident, fields),
            syn::Fields::Unit => panic!("#[derive(Codec)] cannot be used on unit structs."),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            derive_codec_for_enum(&vis, &ident, variants)
        }
        _ => {
            panic!("#[derive(Codec)] can only be used on structs")
        }
    }
}

fn derive_codec_for_named_fields_struct(
    vis: &Visibility,
    ident: &Ident,
    fields: FieldsNamed,
) -> TokenStream {
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

        let attrs = parse_field_attrs(&field.attrs);

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

    quote! {
    #[doc(hidden)]
    #[allow(nonstandard_style)]
    #vis struct #incomplete<O: ::azalea_codec::Ops> {
        #incomplete_struct
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
            self.__result?;

            Ok(#ident {
                #incomplete_struct_build
            })
        }
    }
    #[automatically_derived]
    impl<O: ::azalea_codec::Ops> ::azalea_codec::NewOk<O> for #incomplete<O> {
        fn new_ok() -> Self {
            <Self as ::azalea_codec::Incomplete>::new_unnamed()
        }
    }

    #[automatically_derived]
    impl<O: ::azalea_codec::Ops> ::azalea_codec::MapCodec<O> for #ident {
        fn codec_read_map_item(
            incomplete: &mut <Self as ::azalea_codec::Codec<O>>::Incomplete,
            k: &O,
            v: &O,
        ) {
            match k.get_string().unwrap_or_default().as_str() {
                #codec_read_map_item_impl
                name => {
                    #codec_read_map_item_remaining_impl

                    incomplete.__result = Err(::azalea_codec::error::ReadError::unknown_key(name));
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
    impl<O: ::azalea_codec::Ops> ::azalea_codec::Codec<O> for #ident {
        type Incomplete = #incomplete<O>;
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

fn derive_codec_for_unnamed_fields_struct(ident: &Ident, fields: FieldsUnnamed) -> TokenStream {
    assert_eq!(
        fields.unnamed.len(),
        1,
        "#[derive(Codec)] cannot be used on structs with more than one unnamed field"
    );

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

fn derive_codec_for_enum(
    vis: &Visibility,
    ident: &Ident,
    variants: Punctuated<Variant, Comma>,
) -> TokenStream {
    let mut codec_read = quote! {};
    let mut codec_write = quote! {};

    let mut separate_variant_struct_decls = quote! {};

    for variant in &variants {
        assert_ne!(variant.fields.len(), 0, "enum variants must not be empty");

        let variant_name = &variant.ident;

        let first_variant_field = variant.fields.iter().next().unwrap();

        let field_type: syn::Type;
        let field_name_string: String;

        if first_variant_field.ident.is_some() {
            // named enum

            // to make handling named enums easier, when making the Incomplete
            // struct we move out the contents of the variant into a separate
            // struct that's marked #[derive(Codec)]
            field_name_string = format!("Codec__{ident}__{variant_name}");
            let variant_struct_name_ident = Ident::new(&field_name_string, ident.span());

            let variant_struct_contents = &variant.fields;
            separate_variant_struct_decls.extend(quote! {
                #[doc(hidden)]
                #[allow(nonstandard_style)]
                #[derive(::azalea_codec::Codec)]
                #vis struct #variant_struct_name_ident #variant_struct_contents
            });

            let mut comma_separated_field_names = quote! {};
            let mut clone_fields = quote! {};
            for field in variant_struct_contents.iter() {
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("cannot mix named and unnamed fields in Codec enums");
                comma_separated_field_names.extend(quote! { #field_name, });
                clone_fields.extend(quote! { #field_name: #field_name.clone(), });
            }

            codec_read.extend(quote! {
                match <#variant_struct_name_ident>::codec_read(__codec_ops) {
                    Ok(#variant_struct_name_ident { #comma_separated_field_names }) => {
                        return Ok(Self::#variant_name { #comma_separated_field_names });
                    }
                    Err(e) => {
                        __errors.push((#field_name_string, e));
                    }
                }
            });
            codec_write.extend(quote! {
                Self::#variant_name { #comma_separated_field_names } => {
                    #variant_struct_name_ident { #clone_fields }.codec_write()
                },
            });
        } else {
            // unnamed enum
            assert_eq!(
                variant.fields.len(),
                1,
                "unnamed enum variants must have exactly one field"
            );
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

#[derive(Default, Debug)]
struct FieldAttrs {
    pub rename: Option<String>,
    pub flatten: bool,
    pub optional: bool,
    pub lenient: bool,
}
impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();

        while !input.is_empty() {
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let rename = input.parse::<syn::LitStr>()?;

                    attrs.rename = Some(rename.value());
                }
                "flatten" => {
                    attrs.flatten = true;
                }
                "optional" => {
                    attrs.optional = true;
                }
                "lenient" => {
                    attrs.lenient = true;
                }
                attr => panic!("unknown attribute '{attr}'"),
            }

            let _ = input.parse::<syn::Token![,]>();
        }

        Ok(attrs)
    }
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut a = FieldAttrs::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("codec")) {
        let n = attr.parse_args::<FieldAttrs>().expect("invalid codec attr");

        if let Some(rename) = n.rename {
            a.rename = Some(rename);
        }
        a.flatten |= n.flatten;
        a.optional |= n.optional;
        a.lenient |= n.lenient;
    }

    if a.optional && a.lenient {
        panic!("conflicting 'optional' and 'lenient'; optional already implies lenient")
    }
    if a.flatten && a.lenient {
        panic!("conflicting 'flatten' and 'lenient'")
    }

    a
}
