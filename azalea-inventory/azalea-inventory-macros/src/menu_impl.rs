use crate::parse_macro::{DeclareMenus, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut slot_mut_match_variants = quote! {};
    let mut len_match_variants = quote! {};
    for menu in &input.menus {
        slot_mut_match_variants.extend(generate_match_variant_for_slot_mut(menu));
        len_match_variants.extend(generate_match_variant_for_len(menu));
    }

    quote! {
        impl Menu {
            /// Get a mutable reference to the [`Slot`] at the given protocol index. If
            /// you're trying to get an item in a menu normally, you should just
            /// `match` it and index the `[Slot]` you get
            pub fn slot_mut(&mut self, i: usize) -> Option<&mut Slot> {
                Some(match self {
                    #slot_mut_match_variants
                })
            }

            pub fn len(&self) -> usize {
                match self {
                    #len_match_variants
                }
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
pub fn generate_match_variant_for_slot_mut(menu: &Menu) -> TokenStream {
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
            quote! { #start..=#end => &mut #field_name[i], }
        } else {
            quote! { #start..=#end => &mut #field_name[i - #start], }
        });
    }

    generate_matcher(
        menu,
        &quote! {
            match i {
                #match_arms
                _ => return None
            }
        },
        true,
    )
}

pub fn generate_match_variant_for_len(menu: &Menu) -> TokenStream {
    let length = menu.fields.iter().map(|f| f.length).sum::<usize>();
    generate_matcher(
        menu,
        &quote! {
            #length
        },
        false,
    )
}

fn generate_matcher(menu: &Menu, match_arms: &TokenStream, needs_fields: bool) -> TokenStream {
    let menu_name = &menu.name;
    let menu_field_names = if needs_fields {
        let mut menu_field_names = quote! {};
        for field in &menu.fields {
            let field_name = &field.name;
            menu_field_names.extend(quote! { #field_name, })
        }
        menu_field_names
    } else {
        quote! { .. }
    };

    let matcher = if menu.name.to_string() == "Player" {
        quote! { (Player { #menu_field_names }) }
    } else {
        quote! { { #menu_field_names } }
    };
    quote! {
        Menu::#menu_name #matcher => {
            #match_arms
        },
    }
}
