use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Field, FieldsNamed, GenericParam, Generics, Ident, Visibility};

use crate::{attrs, derive_transparent_impl, utils};

pub fn derive_codec_for_named_fields_struct(
    vis: &Visibility,
    attrs_slice: &[Attribute],
    ident: &Ident,
    generics: &Generics,
    fields: FieldsNamed,
) -> TokenStream {
    let attrs = match attrs::parse_struct_attrs(attrs_slice) {
        Ok(a) => a,
        Err(e) => return syn::Error::into_compile_error(e).into(),
    };
    if attrs.transparent {
        assert_eq!(
            fields.named.len(),
            1,
            "cannot mark structs with more than one field as transparent"
        );
        return derive_transparent_impl(attrs_slice, ident, &fields.named[0]);
    }

    let incomplete = Ident::new(&format!("__Incomplete__{ident}"), ident.span());

    let mut incomplete_struct = quote! {};
    let mut incomplete_struct_new = quote! {};
    let mut incomplete_struct_build = quote! {};
    let mut incomplete_struct_build_extra = quote! {};

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

    let mut fields = fields.named.clone();
    let mut virtual_fields = HashSet::new();
    for (virt_attrs, virt_ident, virt_ty) in attrs.virt {
        virtual_fields.insert(virt_ident.clone());
        fields.push(Field {
            attrs: virt_attrs,
            vis: Visibility::Inherited,
            modifiers: Default::default(),
            ident: Some(virt_ident),
            colon_token: None,
            ty: virt_ty,
            default: None,
        });
    }

    for field in &fields {
        let field_name = field.ident.as_ref().expect("fields must be named");
        let field_type = &field.ty;
        let field_is_virtual = virtual_fields.contains(field_name);

        let attrs = match attrs::parse_field_attrs(&field.attrs) {
            Ok(a) => a,
            Err(e) => return syn::Error::into_compile_error(e).into(),
        };

        let field_name_string = attrs.rename.unwrap_or_else(|| field_name.to_string());

        if attrs.flatten {
            incomplete_struct.extend(quote! {
                #field_name: <#field_type as ::azalea_codec::Codec<O>>::Incomplete,
            });

            // cool and normal
            incomplete_struct_new.extend(quote! {
                #field_name: <<#field_type as ::azalea_codec::Codec<O>>::Incomplete as ::azalea_codec::NewOk<O>>::new_ok(),
            });
        } else {
            incomplete_struct.extend(quote! {
                #field_name: ::azalea_codec::error::ReadResult<#field_type>,
            });

            if attrs.optional || attrs.lenient {
                if let Some(default) = &attrs.default {
                    incomplete_struct_new.extend(quote! {
                        #field_name: Ok(#default),
                    });
                } else {
                    incomplete_struct_new.extend(quote! {
                        #field_name: Ok(Default::default()),
                    });
                }
            } else {
                incomplete_struct_new.extend(quote! {
                    #field_name: ::azalea_codec::Incomplete::new(#field_name_string),
                });
            }
        }

        // after making the struct, switch the field type to the one in
        // attrs.with if necessary
        let field_type = if let Some(with) = &attrs.with {
            quote! { #with }
        } else {
            quote! { #field_type }
        };

        if field_is_virtual {
            // let it error if it needs to but don't save it
            incomplete_struct_build_extra.extend(quote! {
                self.#field_name.build()?;
            })
        } else {
            incomplete_struct_build.extend(quote! {
                #field_name: self.#field_name.build()?.into(),
            });
        }

        let read_fn = quote! { ::codec_read(v) };
        let read_fn = if attrs.with.is_some() {
            quote! { #read_fn.map(|i| i.0) }
        } else {
            read_fn
        };

        if attrs.flatten {
            // formatted like this to more easily handle multiple flattened
            // fields
            codec_read_map_item_remaining_impl.extend(quote! {
                <#field_type as ::azalea_codec::MapCodec<O>>::codec_read_map_item(&mut incomplete.#field_name, k, v);
                // it's possibly fine for the flattened struct to error here
                // without it being fatal, if we have a second flattened map
                // that we're going to decode next. so, we reset back the error
                // status to Ok(()) (by calling get_and_reset_result), and
                // fallthrough only if it was an error.
                if <#field_type as ::azalea_codec::MapCodec<O>>::get_and_reset_result(&mut incomplete.#field_name).is_ok() {
                    return;
                }
            });
        } else if attrs.lenient {
            codec_read_map_item_impl.extend(quote! {
                #field_name_string => {
                    if let Ok(v) = ::azalea_codec::Codec #read_fn {
                        incomplete.#field_name = Ok(v);
                    }
                }
            });
        } else {
            codec_read_map_item_impl.extend(quote! {
                #field_name_string => {
                    incomplete.#field_name = <#field_type as ::azalea_codec::Codec<O>> #read_fn;
                }
            });
        }

        let field_ref = if field_is_virtual {
            quote! { <#field_type>::default() }
        } else {
            quote! { self.#field_name }
        };
        let field_ref = if let Some(with) = &attrs.with {
            quote! { #with(#field_ref.clone()) }
        } else {
            field_ref
        };

        if attrs.flatten {
            codec_write_to_map_impl.extend(quote! {
                ::azalea_codec::MapCodec::codec_write_to_map(&#field_ref, ctx, __codec_map)?;
            });
        } else if attrs.lenient || attrs.optional {
            let is_not_default = if let Some(compare_against) = &attrs.default {
                quote! { (#field_ref != #compare_against) }
            } else {
                quote! { !::azalea_codec::Defaultable::<O>::is_default(&#field_ref) }
            };
            codec_write_to_map_impl.extend(quote! {
                if #is_not_default {
                    __codec_map.push((
                        O::create_string(ctx, #field_name_string)?,
                        ::azalea_codec::Codec::codec_write(&#field_ref, ctx)?,
                    ));
                }
            });
        } else {
            codec_write_to_map_impl.extend(quote! {
                __codec_map.push((
                    O::create_string(ctx, #field_name_string)?,
                    ::azalea_codec::Codec::codec_write(&#field_ref, ctx)?
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
            #incomplete_struct_build_extra

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

        fn codec_write_to_map(&self, ctx: O::Context, __codec_map: &mut Vec<(O, O)>) -> ::azalea_codec::error::WriteResult<()> {
            #codec_write_to_map_impl
            Ok(())
        }

        fn get_and_reset_result(incomplete: &mut <Self as ::azalea_codec::Codec<O>>::Incomplete) -> ::azalea_codec::error::ReadResult<()> {
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
        fn codec_write(&self, ctx: O::Context) -> ::azalea_codec::error::WriteResult<O> {
            use ::azalea_codec::MapCodec;

            let mut __codec_map = Vec::new();
            Self::codec_write_to_map(&self, ctx, &mut __codec_map)?;
            O::create_map(ctx, &__codec_map)
        }
    }
    }
    .into()
}
