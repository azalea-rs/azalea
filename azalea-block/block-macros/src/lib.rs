mod utils;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Token,
};
use utils::{combinations_of, to_pascal_case};

struct PropertyDefinition {
    name: Ident,
    variants: Punctuated<Ident, Token![,]>,
}
struct PropertyDefinitions {
    properties: Vec<PropertyDefinition>,
}

struct BlockDefinition {
    name: Ident,
    behavior: Expr,
    properties: Punctuated<Ident, Token![,]>,
}
struct BlockDefinitions {
    blocks: Vec<BlockDefinition>,
}
struct MakeBlockStates {
    property_definitions: PropertyDefinitions,
    block_definitions: BlockDefinitions,
}

impl Parse for PropertyDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        // Face {
        //     Floor,
        //     Wall,
        //     Ceiling
        // };
        let name = input.parse()?;

        let content;
        braced!(content in input);
        let variants = content.parse_terminated(Ident::parse)?;

        input.parse::<Token![;]>()?;
        Ok(PropertyDefinition { name, variants })
    }
}

impl Parse for PropertyDefinitions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut property_definitions = Vec::new();
        while !input.is_empty() {
            property_definitions.push(input.parse()?);
        }

        Ok(PropertyDefinitions {
            properties: property_definitions,
        })
    }
}

impl Parse for BlockDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        //     acacia_button => BlockBehavior { has_collision: false }, {
        //         Face,
        //         Facing,
        //         Powered
        //     };
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let behavior = input.parse()?;

        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);
        let properties = content.parse_terminated(Ident::parse)?;
        input.parse::<Token![;]>()?;
        Ok(BlockDefinition {
            name,
            behavior,
            properties,
        })
    }
}

impl Parse for BlockDefinitions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut blocks = Vec::new();
        while !input.is_empty() {
            blocks.push(input.parse()?);
        }

        Ok(BlockDefinitions { blocks })
    }
}

impl Parse for MakeBlockStates {
    fn parse(input: ParseStream) -> Result<Self> {
        // PROPERTIES => { ... } BLOCKS => { ... }
        let properties_ident = input.parse::<Ident>()?;
        assert_eq!(properties_ident.to_string(), "PROPERTIES");
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let properties = content.parse()?;

        let blocks_ident = input.parse::<Ident>()?;
        assert_eq!(blocks_ident.to_string(), "BLOCKS");
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let blocks = content.parse()?;

        Ok(MakeBlockStates {
            property_definitions: properties,
            block_definitions: blocks,
        })
    }
}

#[proc_macro]
pub fn make_block_states(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MakeBlockStates);

    let mut property_enums = quote! {};
    let mut properties_map = HashMap::new();
    for property in &input.property_definitions.properties {
        let mut property_enum_variants = quote! {};
        let mut property_enum_variant_names = Vec::new();

        for variant in &property.variants {
            property_enum_variants.extend(quote! {
                #variant,
            });
            property_enum_variant_names.push(variant.to_string());
        }

        let property_name = &property.name;

        property_enums.extend(quote! {
            #[derive(Debug, Clone, Copy)]
            pub enum #property_name {
                #property_enum_variants
            }
        });
        properties_map.insert(property_name.to_string(), property_enum_variant_names);
    }

    let mut block_state_enum_variants = quote! {};
    let mut block_structs = quote! {};
    for block in &input.block_definitions.blocks {
        let block_property_names = &block
            .properties
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>();
        let mut block_properties_vec = Vec::new();
        for property_name in block_property_names {
            let property_variants = properties_map
                .get(property_name)
                .expect(format!("Property '{}' not found", property_name).as_str())
                .clone();
            block_properties_vec.push(property_variants);
        }

        //     pub face: properties::Face,
        //     pub facing: properties::Facing,
        //     pub powered: properties::Powered,
        let mut block_struct_fields = quote! {};
        for property in &block.properties {
            let property_name_snake =
                Ident::new(&property.to_string(), proc_macro2::Span::call_site());
            block_struct_fields.extend(quote! {
                pub #property_name_snake: #property,
            })
        }
        let block_struct_name = Ident::new(
            &format!("{}Block", to_pascal_case(&block.name.to_string())),
            proc_macro2::Span::call_site(),
        );

        let mut from_block_to_state_match = quote! {};

        for combination in combinations_of(&block_properties_vec) {
            let variant_name = Ident::new(
                &format!(
                    "{}_{}",
                    to_pascal_case(&block.name.to_string()),
                    combination
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                ),
                proc_macro2::Span::call_site(),
            );
            block_state_enum_variants.extend(quote! {
                #variant_name,
            });

            // 	face: properties::Face::Floor,
            // 	facing: properties::Facing::North,
            // 	powered: properties::Powered::True,
            let mut from_block_to_state_match_inner = quote! {};
            for i in 0..block_property_names.len() {
                let property_name = &block_property_names[i];
                let property_name_ident = Ident::new(property_name, proc_macro2::Span::call_site());
                let property_name_snake =
                    Ident::new(&property_name.to_string(), proc_macro2::Span::call_site());
                let variant =
                    Ident::new(&combination[i].to_string(), proc_macro2::Span::call_site());

                from_block_to_state_match_inner.extend(quote! {
                    #property_name_ident: #property_name_snake::#variant,
                });
            }

            from_block_to_state_match.extend(quote! {
                #block_struct_name {
                    #from_block_to_state_match_inner
                } => BlockState::#variant_name,
            });
        }

        let block_behavior = &block.behavior;
        let block_struct = quote! {
            #[derive(Debug)]
            pub struct #block_struct_name {
                #block_struct_fields
            }

            impl Block for #block_struct_name {
                fn behavior(&self) -> BlockBehavior {
                    #block_behavior
                }
            }

            impl From<#block_struct_name> for BlockState {
                fn from(b: #block_struct_name) -> Self {
                    match b {
                        #from_block_to_state_match
                    }
                }
            }

        };

        block_structs.extend(block_struct);
    }

    quote! {
        #property_enums

        pub enum BlockState {
            #block_state_enum_variants
        }

        #block_structs
    }
    .into()
}
