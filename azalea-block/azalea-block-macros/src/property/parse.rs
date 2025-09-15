use proc_macro2::Ident;
use quote::quote;
use syn::{
    LitStr, Token, braced,
    ext::IdentExt,
    parenthesized,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    token,
};

use crate::{PropertyKind, name_to_ident};

/// `"snowy": Snowy(false)` or `"axis": properties::Axis::Y`
#[derive(Debug)]
pub struct PropertyWithNameAndDefault {
    // "snowy" "axis"
    pub name: String,
    /// The property name, potentially modified so it works better as a struct
    /// field.
    pub name_ident: Ident,
    // Snowy / Axis
    pub property_type: Ident,
    pub property_value_type: Ident,
    /// Whether it's an enum or a boolean.
    pub kind: PropertyKind,
    // false / properties::Axis::Y
    pub default: proc_macro2::TokenStream,
}
impl Parse for PropertyWithNameAndDefault {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        // `"snowy": Snowy(false)` or `"axis": properties::Axis::Y`
        let property_name = input.parse::<LitStr>()?.value();
        input.parse::<Token![:]>()?;

        let first_ident = input.call(Ident::parse_any)?;
        let mut property_default = quote! { #first_ident };

        let property_type: Ident;
        let property_value_type: Ident;
        let mut kind = PropertyKind::Bool;

        if input.parse::<Token![::]>().is_ok() {
            // enum
            kind = PropertyKind::Enum;
            property_type = first_ident.clone();
            property_value_type = first_ident;
            let variant = input.parse::<Ident>()?;
            property_default = quote! { properties::#property_default::#variant };
        } else {
            // must be a unit struct if it's not an enum
            let content;
            let _paren_token: token::Paren = parenthesized!(content in input);
            // we use this instead of .parse so it works with rust keywords like true and
            // false
            let unit_struct_inner = content.call(Ident::parse_any)?;
            let unit_struct_inner_string = unit_struct_inner.to_string();

            if matches!(unit_struct_inner_string.as_str(), "true" | "false") {
                property_value_type = Ident::new("bool", first_ident.span());
                property_type = first_ident;
                property_default = quote! { #unit_struct_inner };
            } else {
                return Err(input.error("Expected a boolean or an enum variant"));
            }
        };

        let property_name_ident = name_to_ident(&property_name);

        Ok(PropertyWithNameAndDefault {
            name: property_name,
            name_ident: property_name_ident,
            property_type,
            property_value_type,
            kind,
            default: property_default,
        })
    }
}

/// `"snowy" => Snowy(bool)`
pub struct PropertyDefinition {
    pub name: LitStr,
    pub data: PropertyData,
}
impl Parse for PropertyDefinition {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        // "face" => Face {
        //     Floor,
        //     Wall,
        //     Ceiling
        // },

        // if you're wondering, the reason it's in quotes is because `type` is
        // a keyword in rust so if we don't put it in quotes it results in a
        // syntax error
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let property_type = input.parse()?;

        input.parse::<Token![,]>()?;
        Ok(PropertyDefinition {
            name,
            data: property_type,
        })
    }
}

pub enum PropertyData {
    /// `Axis { X = "x", Y = "y", Z = "z" }`
    Enum {
        enum_name: Ident,
        variants: Vec<PropertyVariant>,
    },
    /// `Snowy(bool)`
    Bool { struct_name: Ident },
}
impl Parse for PropertyData {
    // like `Axis { X = "x", Y = "y", Z = "z" }` or `Waterlogged(bool)`
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let keyword = Ident::parse(input)?;

        fn parse_braced(
            input: ParseStream,
        ) -> parse::Result<Punctuated<PropertyVariant, Token![,]>> {
            let content;
            braced!(content in input);
            let variants = content.parse_terminated(parse_variant, Token![,])?;
            Ok(variants)
        }

        /// Parses something like `X = "x"`
        fn parse_variant(input: ParseStream) -> parse::Result<PropertyVariant> {
            let ident = Ident::parse(input)?;
            input.parse::<Token![=]>()?;
            let name = input.parse::<syn::LitStr>()?;
            Ok(PropertyVariant { ident, name })
        }

        fn parse_paren(input: ParseStream) -> parse::Result<Ident> {
            let content;
            parenthesized!(content in input);
            let inner = content.parse::<Ident>()?;
            Ok(inner)
        }

        if let Ok(variants) = parse_braced(input) {
            Ok(Self::Enum {
                enum_name: keyword,
                variants: variants.into_iter().collect(),
            })
        } else if let Ok(inner) = parse_paren(input) {
            assert_eq!(
                inner.to_string(),
                "bool",
                "Currently only bool unit structs are supported"
            );
            Ok(Self::Bool {
                struct_name: keyword,
            })
        } else {
            Err(input.error("Expected a unit struct or an enum"))
        }
    }
}

pub struct PropertyVariant {
    /// The Rust identifier for the property variant, like `X` or `_1`.
    pub ident: Ident,
    /// The Minecraft name for the property variant, like `"x"` or `"1"`.
    pub name: LitStr,
}

/// Parses comma separated `PropertyDefinition`s like `"snowy" => Snowy(bool),`
pub fn parse_property_definitions(input: ParseStream) -> parse::Result<Vec<PropertyDefinition>> {
    let mut property_definitions = Vec::new();
    while !input.is_empty() {
        property_definitions.push(input.parse()?);
    }
    Ok(property_definitions)
}
