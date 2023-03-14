use crate::parse_macro::{DeclareMenus, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut slot_mut_match_variants = quote! {};
    let mut len_match_variants = quote! {};
    let mut kind_match_variants = quote! {};
    let mut contents_match_variants = quote! {};

    let mut hotbar_slot_start = 0;
    let mut hotbar_slot_end = 0;

    for menu in &input.menus {
        slot_mut_match_variants.extend(generate_match_variant_for_slot_mut(menu));
        len_match_variants.extend(generate_match_variant_for_len(menu));
        kind_match_variants.extend(generate_match_variant_for_kind(menu));
        contents_match_variants.extend(generate_match_variant_for_contents(menu));

        // this part is only used to generate `Player::is_hotbar_slot`
        if menu.name == "Player" {
            let mut i = 0;
            for field in &menu.fields {
                let field_name = &field.name;
                let start = i;
                i += field.length;
                if field_name == "inventory" {
                    hotbar_slot_start = start;
                    // it only adds 8 here since it's inclusive (there's 9
                    // total hotbar slots)
                    hotbar_slot_end = start + 8;
                }
            }
        }
    }

    assert!(hotbar_slot_start != 0 && hotbar_slot_end != 0);
    quote! {
        impl Player {
            /// Returns whether the given protocol index is in the player's hotbar.
            pub fn is_hotbar_slot(i: usize) -> bool {
                (#hotbar_slot_start..=#hotbar_slot_end).contains(&i)
            }
        }

        impl Menu {
            /// Get a mutable reference to the [`ItemSlot`] at the given protocol index. If
            /// you're trying to get an item in a menu normally, you should just
            /// `match` it and index the [`ItemSlot`] you get
            pub fn slot_mut(&mut self, i: usize) -> Option<&mut ItemSlot> {
                Some(match self {
                    #slot_mut_match_variants
                })
            }

            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                match self {
                    #len_match_variants
                }
            }

            pub fn from_kind(kind: azalea_registry::MenuKind) -> Self {
                match kind {
                    #kind_match_variants
                }
            }

            /// Return the contents of the menu, not including the player's inventory.
            pub fn contents(&self) -> Vec<ItemSlot> {
                match self {
                    #contents_match_variants
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

pub fn generate_match_variant_for_kind(menu: &Menu) -> TokenStream {
    // azalea_registry::MenuKind::Player => Menu::Player(Player::default()),
    // azalea_registry::MenuKind::Generic9x3 => Menu::Generic9x3 { contents:
    // Default::default(), player: Default::default() },

    let menu_name = &menu.name;
    let menu_field_names = if menu.name == "Player" {
        return quote! {};
    } else {
        let mut menu_field_names = quote! {};
        for field in &menu.fields {
            let field_name = &field.name;
            menu_field_names.extend(quote! { #field_name: Default::default(), })
        }
        quote! { { #menu_field_names } }
    };

    quote! {
        azalea_registry::MenuKind::#menu_name => Menu::#menu_name #menu_field_names,
    }
}

pub fn generate_match_variant_for_contents(menu: &Menu) -> TokenStream {
    // Menu::Generic9x3(m) => {
    //     let mut contents = Vec::new();
    //     contents.extend(player.m.iter().copied());
    //     ...
    //     contents
    // },
    // Menu::Generic9x3(m) => {
    //     let mut contents = Vec::new();
    //     contents.extend(m.contents.iter().copied());
    //     contents
    // },

    let mut instructions = quote! {};
    let mut length = 0;
    for field in &menu.fields {
        let field_name = &field.name;
        if field_name == "player" {
            continue;
        }
        instructions.extend(if field.length == 1 {
            quote! { items.push(#field_name.clone()); }
        } else {
            quote! { items.extend(#field_name.iter().cloned()); }
        });
        length += field.length;
    }

    generate_matcher(
        menu,
        &quote! {
            let mut items = Vec::with_capacity(#length);
            #instructions
            items
        },
        true,
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

    let matcher = if menu.name == "Player" {
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
