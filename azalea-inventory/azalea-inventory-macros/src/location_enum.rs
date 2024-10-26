use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{parse_macro::DeclareMenus, utils::to_pascal_case};

pub fn generate(input: &DeclareMenus) -> TokenStream {
    // pub enum MenuLocation {
    //     Player(PlayerMenuLocation),
    //     ...
    // }
    // pub enum PlayerMenuLocation {
    //     CraftResult,
    //     Craft,
    //     Armor,
    //     Inventory,
    //     Offhand,
    // }
    // ...

    let mut menu_location_variants = quote! {};
    let mut enums = quote! {};
    for menu in &input.menus {
        let name_snake_case = &menu.name;
        let variant_name = Ident::new(
            &to_pascal_case(&name_snake_case.to_string()),
            name_snake_case.span(),
        );
        let enum_name = Ident::new(&format!("{variant_name}MenuLocation"), variant_name.span());
        menu_location_variants.extend(quote! {
            #variant_name(#enum_name),
        });
        let mut individual_menu_location_variants = quote! {};
        for field in &menu.fields {
            let field_name = &field.name;
            let variant_name =
                Ident::new(&to_pascal_case(&field_name.to_string()), field_name.span());
            individual_menu_location_variants.extend(quote! {
                #variant_name,
            });
        }
        enums.extend(quote! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub enum #enum_name {
                #individual_menu_location_variants
            }
        });
    }

    quote! {
        pub enum MenuLocation {
            #menu_location_variants
        }

        #enums
    }
}
