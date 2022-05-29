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

struct PropertyAndDefault {
    name: Ident,
    default: Ident,
}
struct BlockDefinition {
    name: Ident,
    behavior: Expr,
    properties_and_defaults: Vec<PropertyAndDefault>,
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
        // },
        let name = input.parse()?;

        let content;
        braced!(content in input);
        let variants = content.parse_terminated(Ident::parse)?;

        input.parse::<Token![,]>()?;
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
        //     acacia_button => BlockBehavior::default().no_collision(), {
        //         Face,
        //         Facing,
        //         Powered
        //     },
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let behavior = input.parse()?;

        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);

        let mut properties_and_defaults = Vec::new();

        loop {
            let property = match content.parse() {
                Ok(property) => property,
                Err(_) => break,
            };
            content.parse::<Token![=]>()?;
            let property_default = content.parse()?;
            properties_and_defaults.push(PropertyAndDefault {
                name: property,
                default: property_default,
            });
            if content.parse::<Token![,]>().is_err() {
                break;
            }
        }
        input.parse::<Token![,]>()?;
        Ok(BlockDefinition {
            name,
            behavior,
            properties_and_defaults,
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
        // Properties => { ... } Blocks => { ... }
        let properties_ident = input.parse::<Ident>()?;
        assert_eq!(properties_ident.to_string(), "Properties");
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let properties = content.parse()?;

        input.parse::<Token![,]>()?;

        let blocks_ident = input.parse::<Ident>()?;
        assert_eq!(blocks_ident.to_string(), "Blocks");
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

    let mut state_id = 0usize;

    for property in &input.property_definitions.properties {
        let mut property_enum_variants = quote! {};
        let mut property_from_number_variants = quote! {};
        let mut property_enum_variant_names = Vec::new();

        let property_name = &property.name;

        for i in 0..property.variants.len() {
            let variant = &property.variants[i];

            let i_lit = syn::Lit::Int(syn::LitInt::new(
                &i.to_string(),
                proc_macro2::Span::call_site(),
            ));

            property_enum_variants.extend(quote! {
                #variant = #i_lit,
            });

            // i_lit is used here instead of i because otherwise it says 0size
            // in the expansion and that looks uglier
            property_from_number_variants.extend(quote! {
                #i_lit => #property_name::#variant,
            });

            property_enum_variant_names.push(variant.to_string());
        }

        property_enums.extend(quote! {
            #[derive(Debug, Clone, Copy)]
            pub enum #property_name {
                #property_enum_variants
            }

            impl From<usize> for #property_name {
                fn from(value: usize) -> Self {
                    match value {
                        #property_from_number_variants
                        _ => panic!("Invalid property value: {}", value),
                    }
                }
            }
        });
        properties_map.insert(property_name.to_string(), property_enum_variant_names);
    }

    let mut block_state_enum_variants = quote! {};
    let mut block_structs = quote! {};
    let mut from_state_to_block_match = quote! {};
    for block in &input.block_definitions.blocks {
        let block_property_names = &block
            .properties_and_defaults
            .iter()
            .map(|p| p.name.to_string())
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
        for PropertyAndDefault { name: property, .. } in &block.properties_and_defaults {
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

        let first_state_id = state_id;

        for combination in combinations_of(&block_properties_vec) {
            state_id += 1;
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

        // 7035..=7058 => {
        //     let b = b - 7035;
        //     &AcaciaButtonBlock {
        //         Powered: Powered::from((b / 1) % 2),
        //         Facing: Facing::from((b / 2) % 4),
        //         Face: Face::from((b / 8) % 3),
        //     }
        // }
        let mut from_state_to_block_inner = quote! {};
        let mut division = 1usize;
        for i in (0..block.properties_and_defaults.len()).rev() {
            let PropertyAndDefault {
                name: property_name,
                ..
            } = &block.properties_and_defaults[i];

            let property_variants = &block_properties_vec[i];
            let property_variants_count = property_variants.len();
            from_state_to_block_inner.extend(quote! {
                #property_name: #property_name::from((b / #division) % #property_variants_count),
            });

            division *= property_variants_count;
        }

        let last_state_id = state_id - 1;
        from_state_to_block_match.extend(quote! {
            #first_state_id..=#last_state_id => {
                let b = b - #first_state_id;
                Box::new(#block_struct_name {
                    #from_state_to_block_inner
                })
            },
        });

        let mut block_default_fields = quote! {};
        for PropertyAndDefault {
            name: property,
            default: property_default,
        } in &block.properties_and_defaults
        {
            block_default_fields.extend(quote! {
                #property: #property::#property_default,
            })
        }

        let block_behavior = &block.behavior;
        let block_id = block.name.to_string();
        let block_struct = quote! {
            #[derive(Debug)]
            pub struct #block_struct_name {
                #block_struct_fields
            }

            impl Block for #block_struct_name {
                fn behavior(&self) -> BlockBehavior {
                    #block_behavior
                }
                fn id(&self) -> &'static str {
                    #block_id
                }
            }

            impl From<#block_struct_name> for BlockState {
                fn from(b: #block_struct_name) -> Self {
                    match b {
                        #from_block_to_state_match
                    }
                }
            }

            impl Default for #block_struct_name {
                fn default() -> Self {
                    Self {
                        #block_default_fields
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

        impl From<BlockState> for Box<dyn Block> {
            fn from(b: BlockState) -> Self {
                let b = b as usize;
                match b {
                    #from_state_to_block_match
                    _ => panic!("Invalid block state: {}", b),
                }
            }
        }
    }
    .into()
}
