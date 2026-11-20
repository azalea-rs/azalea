mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, FieldsNamed, Ident, Variant,
    parse::{Parse, ParseStream, discouraged::AnyDelimiter},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

use crate::utils::to_snake_case;

#[proc_macro_derive(Codec, attributes(codec))]
pub fn derive_codec(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let syn::Fields::Named(fields) = fields else {
                panic!("#[derive(Codec)] can only be used on structs with named fields")
            };
            derive_codec_for_struct(&ident, fields)
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => derive_codec_for_enum(&ident, variants),
        _ => {
            panic!("#[derive(Codec)] can only be used on structs")
        }
    }
}

fn derive_codec_for_struct(ident: &Ident, fields: FieldsNamed) -> TokenStream {
    let incomplete = Ident::new(&format!("__Incomplete{ident}"), ident.span());

    let mut incomplete_struct = quote! {};
    let mut incomplete_struct_new = quote! {};
    let mut incomplete_struct_build = quote! {};

    let mut codec_read_map_item_impl = quote! {};
    let mut codec_read_map_item_remaining_impl = quote! {};
    let mut codec_write_to_map_impl = quote! {};

    // used for error tracking in individual variants
    incomplete_struct.extend(quote! {
        __result: ::azalea_codec::error::ReadResult<()>,
    });
    incomplete_struct_new.extend(quote! {
        __result: Ok(()),
    });

    for field in &fields.named {
        let field_name = field.ident.as_ref().expect("fields must be named");
        let field_type = &field.ty;

        let attrs = parse_field_attrs(&field.attrs);

        let field_name_string = attrs.rename.unwrap_or_else(|| field_name.to_string());

        incomplete_struct.extend(quote! {
            #field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
        });

        if attrs.default || attrs.lenient {
            incomplete_struct_new.extend(quote! {
                #field_name: Ok(Default::default()),
            });
        } else {
            incomplete_struct_new.extend(quote! {
                #field_name: ::azalea_codec::Incomplete::new(#field_name_string),
            });
        }

        incomplete_struct_build.extend(quote! {
            #field_name: self.#field_name.build()?,
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
                    incomplete.#field_name = Ok(Codec::codec_read(v).ok());
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
                self.#field_name.codec_write_to_map(__codec_ops, __codec_map)?;
            });
        } else if attrs.lenient || attrs.default {
            codec_write_to_map_impl.extend(quote! {
                if ::azalea_codec::Defaultable::<O>::is_default(&self.#field_name) {
                    __codec_map.push((
                        __codec_ops.create_string(#field_name_string)?,
                        self.#field_name.codec_write(__codec_ops)?,
                    ));
                }
            });
        } else {
            codec_write_to_map_impl.extend(quote! {
                __codec_map.push((
                    __codec_ops.create_string(#field_name_string)?,
                    self.#field_name.codec_write(__codec_ops)?
                ));
            })
        }
    }

    quote! {
    #[doc(hidden)]
    #[allow(nonstandard_style)]
    pub struct #incomplete<O: ::azalea_codec::Ops> {
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
            if let Err(e) = self.__result {
                return Err(e);
            }

            Ok(#ident {
                #incomplete_struct_build
            })
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

        fn codec_write_to_map(self, __codec_ops: &O, __codec_map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
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
        fn codec_write(self, __codec_ops: &O) -> ::azalea_codec::error::WriteResult<O> {
            use ::azalea_codec::MapCodec;

            let mut __codec_map = Vec::new();
            Self::codec_write_to_map(self, __codec_ops, &mut __codec_map)?;
            __codec_ops.create_map(&__codec_map)
        }
    }
    }
    .into()
}

fn derive_codec_for_enum(ident: &Ident, variants: Punctuated<Variant, Comma>) -> TokenStream {
    let incomplete = Ident::new(&format!("__Incomplete{ident}"), ident.span());

    let mut incomplete_struct = quote! {};
    let mut incomplete_struct_new = quote! {};
    let mut incomplete_struct_build = quote! {};

    let mut codec_read_map_item_impl = quote! {};
    let mut codec_write_to_map_impl = quote! {};

    let mut separate_variant_struct_decls = quote! {};

    for variant in &variants {
        assert_ne!(variant.fields.len(), 0, "enum variants must not be empty");

        let variant_name = &variant.ident;
        let field_name = Ident::new(
            &to_snake_case(&variant_name.to_string()),
            variant_name.span(),
        );

        let first_variant_field = variant.fields.iter().next().unwrap();

        let field_type: syn::Type;
        let field_name_string: String;
        let build_variant_code: proc_macro2::TokenStream;

        if first_variant_field.ident.is_some() {
            // named enum

            // to make handling named enums easier, when making the Incomplete
            // struct we move out the contents of the variant into a separate
            // struct that's marked #[derive(Codec)]
            field_name_string = format!("Codec__{ident}__{variant_name}");
            let variant_struct_name_ident = Ident::new(&field_name_string, ident.span());
            field_type = syn::Type::Verbatim(quote! { #variant_struct_name_ident });

            let variant_struct_contents = &variant.fields;
            separate_variant_struct_decls.extend(quote! {
                #[doc(hidden)]
                #[allow(nonstandard_style)]
                #[derive(::azalea_codec::Codec)]
                struct #variant_struct_name_ident #variant_struct_contents
            });

            let mut build_fields = quote! {};
            let mut comma_separated_field_names = quote! {};
            for field in variant_struct_contents.iter() {
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("cannot mix named and unnamed fields in Codec enums");
                build_fields.extend(quote! {
                    #field_name: b.#field_name,
                });

                comma_separated_field_names.extend(quote! { #field_name, });
            }

            build_variant_code = quote! { { #build_fields } };

            codec_write_to_map_impl.extend(quote! {
                Self::#variant_name { #comma_separated_field_names } => {
                    #variant_struct_name_ident { #comma_separated_field_names }.codec_write_to_map(__codec_ops, __codec_map)
                }
            })
        } else {
            // unnamed enum
            assert_eq!(
                variant.fields.len(),
                1,
                "unnamed enum variants must have exactly one field"
            );
            field_type = first_variant_field.ty.clone();
            field_name_string = field_name.to_string();
            build_variant_code = quote! { (b) };

            codec_write_to_map_impl.extend(quote! {
                Self::#variant_name(c) => c.codec_write_to_map(__codec_ops, __codec_map),
            })
        }

        incomplete_struct.extend(quote! {
            #field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
        });

        incomplete_struct_new.extend(quote! {
            #field_name: ::azalea_codec::Incomplete::new(#field_name_string),
        });

        incomplete_struct_build.extend(quote! {
            if let Ok(b) = self.#field_name.build() {
                return Ok(Self::Complete::#variant_name #build_variant_code);
            }
        });

        codec_read_map_item_impl.extend(quote! {
            #field_type::codec_read_map_item(&mut incomplete.#field_name, k, v);
        });
    }

    quote! {
    #separate_variant_struct_decls

    #[doc(hidden)]
    #[allow(nonstandard_style)]
    pub struct #incomplete<O: ::azalea_codec::Ops> {
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
            #incomplete_struct_build

            Err(::azalea_codec::error::ReadError::custom("no variants matched"))
        }
    }


    #[automatically_derived]
    impl<O: ::azalea_codec::Ops> ::azalea_codec::MapCodec<O> for #ident {
        fn codec_read_map_item(
            incomplete: &mut <Self as ::azalea_codec::Codec<O>>::Incomplete,
            k: &O,
            v: &O,
        ) {
            #codec_read_map_item_impl
        }

        fn codec_write_to_map(self, __codec_ops: &O, __codec_map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
            match self {
                #codec_write_to_map_impl
            }
        }

        fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ::azalea_codec::error::ReadResult<()> {
            unimplemented!();
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
        fn codec_write(self, __codec_ops: &O) -> ::azalea_codec::error::WriteResult<O> {
            use ::azalea_codec::MapCodec;

            let mut __codec_map = Vec::new();
            Self::codec_write_to_map(self, __codec_ops, &mut __codec_map)?;
            __codec_ops.create_map(&__codec_map)
        }
    }
    }
    .into()
}

#[derive(Default, Debug)]
struct FieldAttrs {
    pub rename: Option<String>,
    pub flatten: bool,
    pub default: bool,
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
                "default" => {
                    attrs.default = true;
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
        a.default |= n.default;
        a.lenient |= n.lenient;
    }

    if a.default && a.lenient {
        panic!("conflicting 'default' and 'lenient'; default already implies lenient")
    }
    if a.flatten && a.lenient {
        panic!("conflicting 'flatten' and 'lenient'")
    }

    a
}
