mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, FieldsNamed, Ident, Variant,
    parse::{Parse, ParseStream},
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
        __result: Result<()>,
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
            #field_name: <#field_type as Codec<O>>::Incomplete,
        });

        if attrs.default || attrs.lenient {
            incomplete_struct_new.extend(quote! {
                #field_name: Ok(Default::default()),
            });
        } else {
            incomplete_struct_new.extend(quote! {
                #field_name: Incomplete::new(#field_name_string),
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
                // it's possibly fine for the flattened struct to error here,
                // because we could have a second flattened struct that does
                // match. so, we track __result only in the parent struct.
                incomplete.#field_name.__result = Ok(());
                if incomplete.#field_name.__result.is_ok() {
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
                self.#field_name.codec_write_to_map(ops, map);
            });
        } else if attrs.lenient || attrs.default {
            codec_write_to_map_impl.extend(quote! {
                if Defaultable::<O>::is_default(&self.#field_name) {
                    map.push((
                        ops.create_string(#field_name_string),
                        self.#field_name.codec_write(ops),
                    ));
                }
            });
        } else {
            codec_write_to_map_impl.extend(quote! {
                map.push((
                    ops.create_string(#field_name_string),
                    self.#field_name.codec_write(ops)
                ));
            })
        }
    }

    quote! {
    #[doc(hidden)]
    pub struct #incomplete<O: Ops> {
        #incomplete_struct
    }
    #[automatically_derived]
    impl<O: Ops> Incomplete for #incomplete<O> {
        type Complete = #ident;
        fn new(_: &str) -> Self {
            Self {
                #incomplete_struct_new
            }
        }
        fn build(self) -> Result<Self::Complete> {
            if let Err(e) = self.__result {
                return Err(e);
            }

            Ok(#ident {
                #incomplete_struct_build
            })
        }
    }


    #[automatically_derived]
    impl<O: Ops> MapCodec<O> for #ident {
        fn codec_read_map_item(
            incomplete: &mut <Self as Codec<O>>::Incomplete,
            k: &O,
            v: &O,
        ) {
            match k.get_string().unwrap_or_default().as_str() {
                #codec_read_map_item_impl
                name => {
                    #codec_read_map_item_remaining_impl

                    incomplete.__result = Err(::azalea_codec::Error::unknown_key(name));
                }
            }
        }

        fn codec_write_to_map(self, ops: &O, map: &mut Vec<(O, O)>) {
            #codec_write_to_map_impl
        }
    }

    #[automatically_derived]
    impl<O: Ops> Codec<O> for #ident {
        type Incomplete = #incomplete<O>;
        fn codec_read(ops: &O) -> Result<Self> {
            let map = ops.get_map()?;
            let mut incomplete = Self::Incomplete::new_unnamed();
            map.for_each(|k, v| {
                Self::codec_read_map_item(&mut incomplete, k, v);
            });
            incomplete.build()
        }
        fn codec_write(self, ops: &O) -> O {
            let mut map = Vec::new();
            Self::codec_write_to_map(self, ops, &mut map);
            ops.create_map(&map)
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

    for variant in &variants {
        assert_eq!(variant.fields.len(), 1, "enum variants must have one field");

        let variant_name = &variant.ident;
        let field_name = Ident::new(
            &to_snake_case(&variant_name.to_string()),
            variant_name.span(),
        );
        let variant_field = variant.fields.iter().next().unwrap();
        assert!(
            variant_field.ident.is_none(),
            "enum variants must not have named fields"
        );
        let field_type = &variant_field.ty;

        let field_name_string = field_name.to_string();

        incomplete_struct.extend(quote! {
            #field_name: <#field_type as Codec<O>>::Incomplete,
        });

        incomplete_struct_new.extend(quote! {
            #field_name: Incomplete::new(#field_name_string),
        });

        incomplete_struct_build.extend(quote! {
            if let Ok(b) = self.#field_name.build() {
                return Ok(Self::Complete::#variant_name(b));
            }
        });

        codec_read_map_item_impl.extend(quote! {
            #field_type::codec_read_map_item(&mut incomplete.#field_name, k, v);
        });

        codec_write_to_map_impl.extend(quote! {
            Self::#variant_name(c) => c.codec_write_to_map(ops, map),
        })
    }

    quote! {
    #[doc(hidden)]
    #[allow(nonstandard_style)]
    pub struct #incomplete<O: Ops> {
        #incomplete_struct
    }
    #[automatically_derived]
    impl<O: Ops> Incomplete for #incomplete<O> {
        type Complete = #ident;
        fn new(_: &str) -> Self {
            Self {
                #incomplete_struct_new
            }
        }
        fn build(self) -> Result<Self::Complete> {
            #incomplete_struct_build

            Err(::azalea_codec::Error::custom("no variants matched"))
        }
    }


    #[automatically_derived]
    impl<O: Ops> MapCodec<O> for #ident {
        fn codec_read_map_item(
            incomplete: &mut <Self as Codec<O>>::Incomplete,
            k: &O,
            v: &O,
        ) {
            #codec_read_map_item_impl
        }

        fn codec_write_to_map(self, ops: &O, map: &mut Vec<(O, O)>) {
            match self {
                #codec_write_to_map_impl
            }
        }
    }

    #[automatically_derived]
    impl<O: Ops> Codec<O> for #ident {
        type Incomplete = #incomplete<O>;
        fn codec_read(ops: &O) -> Result<Self> {
            let map = ops.get_map()?;
            let mut incomplete = Self::Incomplete::new_unnamed();
            map.for_each(|k, v| {
                Self::codec_read_map_item(&mut incomplete, k, v);
            });
            incomplete.build()
        }
        fn codec_write(self, ops: &O) -> O {
            let mut map = Vec::new();
            Self::codec_write_to_map(self, ops, &mut map);
            ops.create_map(&map)
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
