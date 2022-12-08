use crate::parse_macro::{DeclareMenus, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut match_variants = quote! {};
    for menu in &input.menus {
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
pub fn generate_match_variant_for_menu(menu: &Menu) -> TokenStream {
    let menu_name = &menu.name;
    let mut menu_field_names = quote! {};
    for field in &menu.fields {
        let field_name = &field.name;
        menu_field_names.extend(quote! { #field_name, })
    }

    let mut match_arms = quote! {};
    let mut i = 0;
    for field in &menu.fields {
        let field_name = &field.name;
        let start = i;
        i += field.length;
        let end = i - 1;
        match_arms.extend(if start == end {
            quote! { #start => #field_name, }
        } else if start == 0 {
            quote! { #start..=#end => &#field_name[i], }
        } else {
            quote! { #start..=#end => &#field_name[i - #start], }
        });
    }

    let matcher = if menu.name.to_string() == "Player" {
        quote! { (Player { #menu_field_names }) }
    } else {
        quote! { { #menu_field_names } }
    };
    quote! {
        Menu::#menu_name #matcher => {
            match i {
                #match_arms
                _ => return None,
            }
        },
    }
}
