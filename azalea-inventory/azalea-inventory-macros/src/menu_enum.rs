//! Generate the `enum menu` and nothing else. Implementations are in
//! impl_menu.rs

use crate::parse_macro::{DeclareMenus, Field, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut variants = quote! {};
    let mut player_fields = None;
    for menu in &input.menus {
        if menu.name.to_string() == "Player" {
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

        pub enum Menu {
            Player(Player),
            #variants
        }
    }
}

/// Player {
///     craft_result: Slot,
///     craft: [Slot; 4],
///     armor: [Slot; 4],
///     inventory: [Slot; 36],
///     offhand: Slot,
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
            quote! { Slot }
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
