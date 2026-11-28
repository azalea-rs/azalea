use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, LitStr, Token, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
};

struct Registry {
    ident: Ident,
    name: String,
    items: Vec<RegistryItem>,
    attrs: Vec<Attribute>,
}

struct RegistryItem {
    attrs: Vec<Attribute>,
    ident: Ident,
    name: String,
}

impl Parse for RegistryItem {
    // Air => "minecraft:air"
    fn parse(input: ParseStream) -> Result<Self> {
        // parse annotations like #[default]
        let attrs = input.call(Attribute::parse_outer).unwrap_or_default();

        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let id = input.parse::<LitStr>()?.value();
        Ok(RegistryItem {
            attrs,
            ident: name,
            name: id,
        })
    }
}

impl Parse for Registry {
    fn parse(input: ParseStream) -> Result<Self> {
        // enum BlockKind => "block" {
        //     Air => "minecraft:air",
        //     Stone => "minecraft:stone"
        // }

        // this also includes docs
        let attrs = input.call(Attribute::parse_outer).unwrap_or_default();

        input.parse::<Token![enum]>()?;
        let ident = input.parse()?;

        input.parse::<Token![=>]>()?;
        let name = input.parse::<LitStr>()?.value();

        let content;
        braced!(content in input);
        let items: Punctuated<RegistryItem, _> =
            content.parse_terminated(RegistryItem::parse, Token![,])?;

        Ok(Registry {
            ident,
            name,
            items: items.into_iter().collect(),
            attrs,
        })
    }
}

#[proc_macro]
pub fn registry(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Registry);
    let name = input.ident;
    let mut generated = quote! {};

    // enum BlockKind {
    //     Air = 0,
    //     Stone,
    // }
    let mut enum_items = quote! {};
    for (i, item) in input.items.iter().enumerate() {
        let attrs = &item.attrs;
        let name = &item.ident;
        let protocol_id = i as u32;
        enum_items.extend(quote! {
            #(#attrs)*
            #name = #protocol_id,
        });
    }
    let attributes = input.attrs;
    generated.extend(quote! {
        #(#attributes)*
        #[derive(azalea_buf::AzBuf, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(u32)]
        pub enum #name {
            #enum_items
        }
    });

    let max_id = input.items.len() as u32 - 1;

    let doc_0 = format!("Transmutes a u32 to a {name}.");
    let doc_1 = format!("The `id` should be at most {max_id}.");

    let registry_name = input.name;

    generated.extend(quote! {
        impl #name {
            pub const MAX_ID: u32 = #max_id;

            #[doc = #doc_0]
            ///
            /// # Safety
            #[doc = #doc_1]
            #[inline]
            pub unsafe fn from_u32_unchecked(id: u32) -> Self {
                unsafe { std::mem::transmute::<u32, #name>(id) }
            }
            #[inline]
            pub const fn is_valid_id(id: u32) -> bool {
                id <= Self::MAX_ID
            }
        }
        impl azalea_registry_builtin::BuiltinRegistry for #name {
            const NAME: &'static str = #registry_name;
            fn builtin_from_id(id: u32) -> Option<Self> {
                if Self::is_valid_id(id) {
                    Some(unsafe { Self::from_u32_unchecked(id) })
                } else {
                    None
                }
            }
            fn builtin_into_id(&self) -> u32 {
                *self as u32
            }
        }
    });

    let doc_0 = format!("Safely transmutes a u32 to a {name}.");

    generated.extend(quote! {
        impl TryFrom<u32> for #name {
            type Error = ();

            #[doc = #doc_0]
            fn try_from(id: u32) -> Result<Self, Self::Error> {
                if let Some(value) = azalea_registry_builtin::BuiltinRegistry::builtin_from_id(id) {
                    Ok(value)
                } else {
                    Err(())
                }
            }
        }
    });

    // Display that uses registry ids
    let mut to_str_items = quote! {};
    let mut from_str_items = quote! {};
    for item in &input.items {
        let name = &item.ident;
        let id = &item.name;
        let namespaced_id = format!("minecraft:{id}");

        to_str_items.extend(quote! {
            Self::#name => #namespaced_id,
        });
        from_str_items.extend(quote! {
            #id | #namespaced_id => Self::#name,
        });
    }

    generated.extend(quote! {
        impl #name {
            /// Convert the value to a stringified identifier, formatted like
            /// `"minecraft:air"`.
            pub fn to_str(&self) -> &'static str {
                match self {
                    #to_str_items
                }
            }
        }
        const impl azalea_codec::types::ConstFromStr for #name {
            fn const_from_str(s: &str) -> Option<Self> {
                Some(match s {
                    #from_str_items
                    _ => return None
                })
            }
        }
        impl std::fmt::Display for #name {
            /// Convert the value to a stringified identifier, formatted like
            /// `"minecraft:air"`.
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
        impl<'a> TryFrom<&'a ::azalea_core::identifier::Identifier> for #name {
            type Error = ();
            fn try_from(ident: &'a ::azalea_core::identifier::Identifier) -> Result<Self, Self::Error> {
                azalea_codec::types::ConstFromStr::const_from_str(ident.as_ref().1).ok_or(())
            }
        }
        impl TryFrom<::azalea_core::identifier::Identifier> for #name {
            type Error = ();
            fn try_from(ident: ::azalea_core::identifier::Identifier) -> Result<Self, Self::Error> {
                Self::try_from(&ident)
            }
        }

        impl<'a> From<#name> for ::azalea_core::identifier::Identifier {
            fn from(value: #name) -> Self {
                Self::new(value.to_str())
            }
        }
        /// Parse the value from a stringified identifier, formatted like
        /// either `"air"` or `"minecraft:air"`.
        impl std::str::FromStr for #name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(&::azalea_core::identifier::Identifier::new(s))
            }
        }

        impl<O: azalea_codec::Ops> azalea_codec::Codec<O> for #name {
            fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
                let s = ops.get_string()?;
                azalea_codec::types::ConstFromStr::const_from_str(&s)
                    .ok_or_else(|| azalea_codec::error::ReadError::custom(format!("invalid variant '{s}'")))
            }
            fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
                self.to_string().codec_write()
            }
        }
    });

    generated.into()
}
