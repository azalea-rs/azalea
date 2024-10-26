//! Generate the `enum menu` and nothing else. Implementations are in
//! impl_menu.rs

use proc_macro2::TokenStream;
use quote::quote;

use crate::parse_macro::{DeclareMenus, Field, Menu};

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut variants = quote! {};
    let mut player_fields = None;
    for menu in &input.menus {
        if menu.name == "Player" {
            player_fields = Some(generate_fields(&menu.fields, true));
        } else {
            variants.extend(generate_variant_for_menu(menu));
        }
    }
    let player_fields = player_fields.expect("Player variant must be present");

    quote! {
        #[derive(Clone, Debug, Default)]
        pub struct Player {
            #player_fields
        }

        /// A menu, which is a fixed collection of slots.
        #[derive(Clone, Debug)]
        pub enum Menu {
            Player(Player),
            #variants
        }
    }
}

/// Player {
///     craft_result: ItemSlot,
///     craft: [ItemSlot; 4],
///     armor: [ItemSlot; 4],
///     inventory: [ItemSlot; 36],
///     offhand: ItemSlot,
/// },
fn generate_variant_for_menu(menu: &Menu) -> TokenStream {
    let name = &menu.name;
    let fields = generate_fields(&menu.fields, false);

    quote! {
        #name {
            #fields
        },
    }
}

fn generate_fields(fields: &[Field], public: bool) -> TokenStream {
    let mut generated_fields = quote! {};
    for field in fields {
        let field_length = field.length;
        let field_type = if field.length == 1 {
            quote! { ItemSlot }
        } else {
            quote! { SlotList<#field_length> }
        };
        let field_name = &field.name;
        if public {
            generated_fields.extend(quote! { pub #field_name: #field_type, })
        } else {
            generated_fields.extend(quote! { #field_name: #field_type, })
        }
    }
    generated_fields
}
