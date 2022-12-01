use crate::parse_macro::{DeclareMenus, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: DeclareMenus) -> TokenStream {
    let mut match_variants = quote! {};
    for menu in input.menus {
        match_variants.extend(generate_match_variant_for_menu(menu));
    }

    quote! {
        impl Menu {
            /// Get a mutable reference to the [`Slot`] at the given protocol index. If
            /// you're trying to get an item in a menu normally, you should just
            /// `match` it and index the `[Slot]` you get
            pub fn slot_mut(&self, i: usize) -> Option<&Slot> {
                Some(match self {
                    #match_variants
                })
            }
        }
    }
}

/// Menu::Player {
///     craft_result,
///     craft,
///     armor,
///     inventory,
///     offhand,
/// } => {
///     match i {
///         0 => craft_result,
///         1..=4 => craft,
///         5..=8 => armor,
///         // ...
///         _ => return None,
///     }
/// } // ...
pub fn generate_match_variant_for_menu(menu: Menu) -> TokenStream {
    let menu_name = menu.name;
    let menu_field_names = menu.fields.into_iter().map(|f| f.name);

    quote! {
        Menu::#menu_name {
            #(#menu_field_names),*
        } => {
            match i {
                _ => return None,
            }
        }
    }
}
