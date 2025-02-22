mod location_enum;
mod menu_enum;
mod menu_impl;
mod parse_macro;
mod utils;

use parse_macro::{DeclareMenus, Field};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, parse_macro_input};

#[proc_macro]
pub fn declare_menus(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeclareMenus);

    // implicitly add a `player` field at the end unless an `inventory` field
    // is present
    for menu in &mut input.menus {
        let mut inventory_field_missing = true;
        for field in &menu.fields {
            if matches!(field.name.to_string().as_str(), "inventory" | "player") {
                inventory_field_missing = false;
            }
        }
        if inventory_field_missing {
            menu.fields.push(Field {
                name: Ident::new("player", Span::call_site()),
                length: 36,
            })
        }
    }

    let menu_enum = menu_enum::generate(&input);
    let menu_impl = menu_impl::generate(&input);
    let location_enum = location_enum::generate(&input);

    quote! {
        #menu_enum
        #menu_impl

        #location_enum
    }
    .into()
}
