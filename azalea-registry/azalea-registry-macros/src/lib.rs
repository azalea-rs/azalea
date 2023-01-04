use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, LitStr, Token,
};

struct RegistryItem {
    name: Ident,
    id: String,
}

struct Registry {
    name: Ident,
    items: Vec<RegistryItem>,
}

impl Parse for RegistryItem {
    // Air => "minecraft:air"
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let id = input.parse::<LitStr>()?.value();
        Ok(RegistryItem { name, id })
    }
}

impl Parse for Registry {
    fn parse(input: ParseStream) -> Result<Self> {
        // Block, {
        //     Air => "minecraft:air",
        //     Stone => "minecraft:stone"
        // }
        let name = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);
        let items: Punctuated<RegistryItem, Token![,]> =
            content.parse_terminated(RegistryItem::parse)?;

        Ok(Registry {
            name,
            items: items.into_iter().collect(),
        })
    }
}

#[proc_macro]
pub fn registry(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Registry);
    let name = input.name;
    let mut generated = quote! {};

    // enum Block {
    //     Air = 0,
    //     Stone,
    // }
    let mut enum_items = quote! {};
    for (i, item) in input.items.iter().enumerate() {
        let name = &item.name;
        let protocol_id = i as u32;
        enum_items.extend(quote! {
            #name = #protocol_id,
        });
    }
    generated.extend(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, azalea_buf::McBuf)]
        #[repr(u32)]
        pub enum #name {
            #enum_items
        }
    });

    let max_id = input.items.len() as u32;

    let doc_0 = format!("Transmutes a u32 to a {name}.");
    let doc_1 = format!("The `id` should be at most {max_id}.");

    generated.extend(quote! {
        impl #name {
            #[doc = #doc_0]
            ///
            /// # Safety
            #[doc = #doc_1]
            #[inline]
            pub unsafe fn from_u32_unchecked(id: u32) -> Self {
                std::mem::transmute::<u32, #name>(id)
            }

            #[inline]
            pub fn is_valid_id(id: u32) -> bool {
                id <= #max_id
            }
        }
    });

    let doc_0 = format!("Safely transmutes a u32 to a {name}.");

    generated.extend(quote! {
        impl TryFrom<u32> for #name {
            type Error = ();

            #[doc = #doc_0]
            fn try_from(id: u32) -> Result<Self, Self::Error> {
                if Self::is_valid_id(id) {
                    Ok(unsafe { Self::from_u32_unchecked(id) })
                } else {
                    Err(())
                }
            }
        }
    });

    // Display that uses registry ids
    let mut display_items = quote! {};
    for item in input.items.iter() {
        let name = &item.name;
        let id = &item.id;
        display_items.extend(quote! {
            Self::#name => write!(f, #id),
        });
    }
    generated.extend(quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #display_items
                }
            }
        }
    });

    generated.into()
}
