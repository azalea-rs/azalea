//! Generate the `enum menu` and nothing else. Implementations are in
//! impl_menu.rs

use crate::parse_macro::{DeclareMenus, Menu};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: &DeclareMenus) -> TokenStream {
    let mut variants = quote! {};
    for menu in &input.menus {
        variants.extend(generate_variant_for_menu(menu));
    }

    quote! {
        pub enum Menu {
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
    let mut fields = quote! {};

    for field in &menu.fields {
        let field_name = &field.name;

        let field_length = field.length;
        let field_type = if matches!(field_name.to_string().as_str(), "inventory" | "player") {
            quote! { std::sync::Arc<[Slot; #field_length ]> }
        } else if field.length == 1 {
            quote! { Slot }
        } else {
            quote! { [Slot; #field_length ] }
        };
        fields.extend(quote! { #field_name: #field_type, })
    }

    quote! {
        #name {
            #fields
        },
    }
}
