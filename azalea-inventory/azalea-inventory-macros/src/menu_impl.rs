use crate::{
    parse_macro::{DeclareMenus, Menu},
    utils::to_pascal_case,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut slot_mut_match_variants = quote! {};
    let mut slot_match_variants = quote! {};
    let mut len_match_variants = quote! {};
    let mut kind_match_variants = quote! {};
    let mut slots_match_variants = quote! {};
    let mut contents_match_variants = quote! {};
    let mut location_match_variants = quote! {};

    let mut hotbar_slot_start = 0;
    let mut hotbar_slot_end = 0;

    for menu in &input.menus {
        slot_mut_match_variants.extend(generate_match_variant_for_slot_mut(menu, true));
        slot_match_variants.extend(generate_match_variant_for_slot_mut(menu, false));
        len_match_variants.extend(generate_match_variant_for_len(menu));
        kind_match_variants.extend(generate_match_variant_for_kind(menu));
        slots_match_variants.extend(generate_match_variant_for_slots(menu));
        contents_match_variants.extend(generate_match_variant_for_contents(menu));
        location_match_variants.extend(generate_match_variant_for_location(menu));

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
            /// Get a mutable reference to the [`ItemSlot`] at the given protocol index.
            ///
            /// If you're trying to get an item in a menu without caring about
            /// protocol indexes, you should just `match` it and index the
            /// [`ItemSlot`] you get.
            ///
            /// Use [`Menu::slot`] if you don't need a mutable reference to the slot.
            ///
            /// # Errors
            ///
            /// Returns `None` if the index is out of bounds.
            #[inline]
            pub fn slot_mut(&mut self, i: usize) -> Option<&mut ItemSlot> {
                Some(match self {
                    #slot_mut_match_variants
                })
            }

            /// Get a reference to the [`ItemSlot`] at the given protocol index.
            ///
            /// If you're trying to get an item in a menu without caring about
            /// protocol indexes, you should just `match` it and index the
            /// [`ItemSlot`] you get.
            ///
            /// Use [`Menu::slot_mut`] if you need a mutable reference to the slot.
            ///
            /// # Errors
            ///
            /// Returns `None` if the index is out of bounds.
            pub fn slot(&self, i: usize) -> Option<&ItemSlot> {
                Some(match self {
                    #slot_match_variants
                })
            }

            /// Returns the number of slots in the menu.
            #[allow(clippy::len_without_is_empty)]
            pub const fn len(&self) -> usize {
                match self {
                    #len_match_variants
                }
            }

            pub fn from_kind(kind: azalea_registry::MenuKind) -> Self {
                match kind {
                    #kind_match_variants
                }
            }

            /// Return the contents of the menu, including the player's inventory.
            ///
            /// The indexes in this will match up with [`Menu::slot_mut`]
            ///
            /// If you don't want to include the player's inventory, use [`Menu::contents`] instead.
            pub fn slots(&self) -> Vec<ItemSlot> {
                match self {
                    #slots_match_variants
                }
            }

            /// Return the contents of the menu, not including the player's inventory.
            ///
            /// If you want to include the player's inventory, use [`Menu::slots`] instead.
            pub fn contents(&self) -> Vec<ItemSlot> {
                match self {
                    #contents_match_variants
                }
            }

            pub fn location_for_slot(&self, i: usize) -> Option<MenuLocation> {
                Some(match self {
                    #location_match_variants
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
pub fn generate_match_variant_for_slot_mut(menu: &Menu, mutable: bool) -> TokenStream {
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
            if mutable {
                quote! { #start..=#end => &mut #field_name[i], }
            } else {
                quote! { #start..=#end => &#field_name[i], }
            }
        } else {
            if mutable {
                quote! { #start..=#end => &mut #field_name[i - #start], }
            } else {
                quote! { #start..=#end => &#field_name[i - #start], }
            }
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
    // azalea_registry::MenuKind::Generic9x3 => Menu::Generic9x3 { contents:
    // Default::default(), player: Default::default() },

    let menu_name = &menu.name;
    let menu_field_names = if menu.name == "Player" {
        // player isn't in MenuKind
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

pub fn generate_match_variant_for_slots(menu: &Menu) -> TokenStream {
    let mut instructions = quote! {};
    let mut length = 0;
    for field in &menu.fields {
        let field_name = &field.name;
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

pub fn generate_match_variant_for_contents(menu: &Menu) -> TokenStream {
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

pub fn generate_match_variant_for_location(menu: &Menu) -> TokenStream {
    let mut match_arms = quote! {};
    let mut i = 0;

    let menu_name = Ident::new(&to_pascal_case(&menu.name.to_string()), menu.name.span());
    let menu_enum_name = Ident::new(&format!("{menu_name}MenuLocation"), menu_name.span());

    for field in &menu.fields {
        let field_name = Ident::new(&to_pascal_case(&field.name.to_string()), field.name.span());
        let start = i;
        i += field.length;
        let end = i - 1;
        match_arms.extend(if start == end {
            quote! { #start => #menu_enum_name::#field_name, }
        } else if start == 0 {
            quote! { #start..=#end => #menu_enum_name::#field_name, }
        } else {
            quote! { #start..=#end => #menu_enum_name::#field_name, }
        });
    }

    generate_matcher(
        menu,
        &quote! {
            MenuLocation::#menu_name(match i {
                #match_arms
                _ => return None
            })
        },
        false,
    )
}

pub fn generate_matcher(menu: &Menu, match_arms: &TokenStream, needs_fields: bool) -> TokenStream {
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
