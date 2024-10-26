use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    parse_macro::{DeclareMenus, Menu},
    utils::{to_pascal_case, to_snake_case},
};

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut slot_mut_match_variants = quote! {};
    let mut slot_match_variants = quote! {};
    let mut len_match_variants = quote! {};
    let mut kind_match_variants = quote! {};
    let mut slots_match_variants = quote! {};
    let mut contents_match_variants = quote! {};
    let mut location_match_variants = quote! {};
    let mut player_slots_range_match_variants = quote! {};

    let mut player_consts = quote! {};
    let mut menu_consts = quote! {};

    let mut hotbar_slots_start = 0;
    let mut hotbar_slots_end = 0;
    let mut inventory_without_hotbar_slots_start = 0;
    let mut inventory_without_hotbar_slots_end = 0;

    for menu in &input.menus {
        slot_mut_match_variants.extend(generate_match_variant_for_slot_mut(menu, true));
        slot_match_variants.extend(generate_match_variant_for_slot_mut(menu, false));
        len_match_variants.extend(generate_match_variant_for_len(menu));
        kind_match_variants.extend(generate_match_variant_for_kind(menu));
        slots_match_variants.extend(generate_match_variant_for_slots(menu));
        contents_match_variants.extend(generate_match_variant_for_contents(menu));
        location_match_variants.extend(generate_match_variant_for_location(menu));
        player_slots_range_match_variants
            .extend(generate_match_variant_for_player_slots_range(menu));

        // this part is only used to generate `Player::is_hotbar_slot`
        if menu.name == "Player" {
            let mut i = 0;
            for field in &menu.fields {
                let field_name = &field.name;
                let start = i;
                i += field.length;
                let end = i - 1;

                if field_name == "inventory" {
                    // it only subtracts 8 here since it's inclusive (there's 9 total hotbar slots)
                    hotbar_slots_start = end - 8;
                    hotbar_slots_end = end;

                    inventory_without_hotbar_slots_start = start;
                    inventory_without_hotbar_slots_end = end - 9;
                }

                if start == end {
                    let const_name = Ident::new(
                        &format!("{}_SLOT", field_name.to_string().to_uppercase()),
                        field_name.span(),
                    );
                    player_consts.extend(quote! {
                        pub const #const_name: usize = #start;
                    });
                } else {
                    let const_name = Ident::new(
                        &format!("{}_SLOTS", field_name.to_string().to_uppercase()),
                        field_name.span(),
                    );
                    player_consts.extend(quote! {
                        pub const #const_name: RangeInclusive<usize> = #start..=#end;
                    });
                }
            }
        } else {
            menu_consts.extend(generate_menu_consts(menu));
        }
    }

    assert!(hotbar_slots_start != 0 && hotbar_slots_end != 0);
    quote! {
        impl Player {
            pub const HOTBAR_SLOTS: RangeInclusive<usize> = #hotbar_slots_start..=#hotbar_slots_end;
            pub const INVENTORY_WITHOUT_HOTBAR_SLOTS: RangeInclusive<usize> = #inventory_without_hotbar_slots_start..=#inventory_without_hotbar_slots_end;
            #player_consts

            /// Returns whether the given protocol index is in the player's hotbar.
            ///
            /// Equivalent to `Player::HOTBAR_SLOTS.contains(&i)`.
            pub fn is_hotbar_slot(i: usize) -> bool {
                Self::HOTBAR_SLOTS.contains(&i)
            }
        }

        impl Menu {
            #menu_consts

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
            /// The indexes in this will match up with [`Menu::slot_mut`].
            ///
            /// If you don't want to include the player's inventory, use [`Menu::contents`]
            /// instead.
            ///
            /// If you *only* want to include the players inventory, then you can filter by only
            /// using the slots in [`Self::player_slots_range`].
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

            /// Get the range of slot indexes that contain the player's inventory. This may be different for each menu.
            pub fn player_slots_range(&self) -> RangeInclusive<usize> {
                match self {
                    #player_slots_range_match_variants
                }
            }

            /// Get the range of slot indexes that contain the player's hotbar. This may be different for each menu.
            ///
            /// ```
            /// # let inventory = azalea_inventory::Menu::Player(azalea_inventory::Player::default());
            /// let hotbar_items = &inventory.slots()[inventory.hotbar_slots_range()];
            /// ```
            pub fn hotbar_slots_range(&self) -> RangeInclusive<usize> {
                // hotbar is always last 9 slots in the player's inventory
                ((*self.player_slots_range().end() - 8)..=*self.player_slots_range().end())
            }

            /// Get the range of slot indexes that contain the player's inventory, not including the hotbar. This may be different for each menu.
            pub fn player_slots_without_hotbar_range(&self) -> RangeInclusive<usize> {
                (*self.player_slots_range().start()..=*self.player_slots_range().end() - 9)
            }

            /// Returns whether the given index would be in the player's hotbar.
            ///
            /// Equivalent to `self.hotbar_slots_range().contains(&i)`.
            pub fn is_hotbar_slot(&self, i: usize) -> bool {
                self.hotbar_slots_range().contains(&i)
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
        } else if mutable {
            quote! { #start..=#end => &mut #field_name[i - #start], }
        } else {
            quote! { #start..=#end => &#field_name[i - #start], }
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

pub fn generate_match_variant_for_player_slots_range(menu: &Menu) -> TokenStream {
    // Menu::Player(Player { .. }) => Player::INVENTORY_SLOTS_RANGE,,
    // Menu::Generic9x3 { .. } => Menu::GENERIC9X3_SLOTS_RANGE,
    // ..

    match menu.name.to_string().as_str() {
        "Player" => {
            quote! {
                Menu::Player(Player { .. }) => Player::INVENTORY_SLOTS,
            }
        }
        _ => {
            let menu_name = &menu.name;
            let menu_slots_range_name = Ident::new(
                &format!(
                    "{}_PLAYER_SLOTS",
                    to_snake_case(&menu.name.to_string()).to_uppercase()
                ),
                menu.name.span(),
            );
            quote! {
                Menu::#menu_name { .. } => Menu::#menu_slots_range_name,
            }
        }
    }
}

fn generate_menu_consts(menu: &Menu) -> TokenStream {
    let mut menu_consts = quote! {};

    let mut i = 0;

    for field in &menu.fields {
        let field_name_start = format!(
            "{}_{}",
            to_snake_case(&menu.name.to_string()).to_uppercase(),
            to_snake_case(&field.name.to_string()).to_uppercase()
        );
        let field_index_start = i;
        i += field.length;
        let field_index_end = i - 1;

        if field.length == 1 {
            let field_name = Ident::new(
                format!("{field_name_start}_SLOT").as_str(),
                field.name.span(),
            );
            menu_consts.extend(quote! { pub const #field_name: usize = #field_index_start; });
        } else {
            let field_name = Ident::new(
                format!("{field_name_start}_SLOTS").as_str(),
                field.name.span(),
            );
            menu_consts.extend(quote! { pub const #field_name: RangeInclusive<usize> = #field_index_start..=#field_index_end; });
        }
    }

    menu_consts
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
