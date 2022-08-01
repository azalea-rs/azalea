mod utils;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fmt::Write;
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, LitStr, Token,
};
use utils::{combinations_of, to_pascal_case};

struct PropertyDefinition {
    name: LitStr,
    struct_name: Ident,
    variants: Punctuated<Ident, Token![,]>,
}
struct PropertyDefinitions {
    properties: Vec<PropertyDefinition>,
}

struct PropertyAndDefault {
    struct_name: Ident,
    default: Ident,
}
struct PropertyWithNameAndDefault {
    name: String,
    struct_name: Ident,
    default: Ident,
}
struct BlockDefinition {
    name: Ident,
    behavior: Expr,
    properties_and_defaults: Vec<PropertyAndDefault>,
}
impl PropertyAndDefault {
    fn as_property_with_name_and_default(&self, name: String) -> PropertyWithNameAndDefault {
        PropertyWithNameAndDefault {
            name,
            struct_name: self.struct_name.clone(),
            default: self.default.clone(),
        }
    }
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
        let struct_name = input.parse()?;

        let content;
        braced!(content in input);
        let variants = content.parse_terminated(Ident::parse)?;

        input.parse::<Token![,]>()?;
        Ok(PropertyDefinition {
            name,
            struct_name,
            variants,
        })
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
        // acacia_button => BlockBehavior::default(), {
        //     Facing=North,
        //     Powered=False,
        //     Face=Wall,
        // },
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let behavior = input.parse()?;

        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);

        let mut properties_and_defaults = Vec::new();

        while let Ok(property) = content.parse() {
            content.parse::<Token![=]>()?;
            let property_default = content.parse()?;
            properties_and_defaults.push(PropertyAndDefault {
                struct_name: property,
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
    let mut property_struct_names_to_names = HashMap::new();

    let mut state_id: usize = 0;

    for property in &input.property_definitions.properties {
        let mut property_enum_variants = quote! {};
        let mut property_from_number_variants = quote! {};
        let mut property_enum_variant_names = Vec::new();

        let property_struct_name = &property.struct_name;

        property_struct_names_to_names.insert(
            property_struct_name.to_string(),
            property.name.clone().value(),
        );

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
                #i_lit => #property_struct_name::#variant,
            });

            property_enum_variant_names.push(variant.to_string());
        }

        property_enums.extend(quote! {
            #[derive(Debug, Clone, Copy)]
            pub enum #property_struct_name {
                #property_enum_variants
            }

            impl From<usize> for #property_struct_name {
                fn from(value: usize) -> Self {
                    match value {
                        #property_from_number_variants
                        _ => panic!("Invalid property value: {}", value),
                    }
                }
            }
        });
        properties_map.insert(
            property_struct_name.to_string(),
            property_enum_variant_names,
        );
    }

    let mut block_state_enum_variants = quote! {};
    let mut block_structs = quote! {};
    let mut from_state_to_block_match = quote! {};
    for block in &input.block_definitions.blocks {
        let block_property_names = &block
            .properties_and_defaults
            .iter()
            .map(|p| p.struct_name.to_string())
            .collect::<Vec<_>>();
        let mut block_properties_vec = Vec::new();
        for property_name in block_property_names {
            let property_variants = properties_map
                .get(property_name)
                .unwrap_or_else(|| panic!("Property '{}' not found", property_name))
                .clone();
            block_properties_vec.push(property_variants);
        }

        let mut properties_with_name: Vec<PropertyWithNameAndDefault> =
            Vec::with_capacity(block.properties_and_defaults.len());
        for property in &block.properties_and_defaults {
            let index: Option<usize> = if block
                .properties_and_defaults
                .iter()
                .filter(|p| p.struct_name == property.struct_name)
                .count()
                > 1
            {
                Some(
                    properties_with_name
                        .iter()
                        .filter(|p| p.struct_name == property.struct_name)
                        .count(),
                )
            } else {
                None
            };
            let mut property_name = property_struct_names_to_names
                .get(&property.struct_name.to_string())
                .unwrap_or_else(|| panic!("Property '{}' is bad", property.struct_name))
                .clone();
            if let Some(index) = index {
                // property_name.push_str(&format!("_{}", &index.to_string()));
                write!(property_name, "_{}", index).unwrap();
            }
            properties_with_name
                .push(property.as_property_with_name_and_default(property_name.clone()));
        }

        //     pub face: properties::Face,
        //     pub facing: properties::Facing,
        //     pub powered: properties::Powered,
        // or
        //     pub has_bottle_0: HasBottle,
        //     pub has_bottle_1: HasBottle,
        //     pub has_bottle_2: HasBottle,
        let mut block_struct_fields = quote! {};
        for PropertyWithNameAndDefault {
            struct_name, name, ..
        } in &properties_with_name
        {
            // let property_name_snake =
            //     Ident::new(&property.to_string(), proc_macro2::Span::call_site());
            let name_ident = Ident::new(name, proc_macro2::Span::call_site());
            block_struct_fields.extend(quote! {
                pub #name_ident: #struct_name,
            })
        }

        let block_name_pascal_case = Ident::new(
            &to_pascal_case(&block.name.to_string()),
            proc_macro2::Span::call_site(),
        );
        let block_struct_name = Ident::new(
            &format!("{}Block", block_name_pascal_case),
            proc_macro2::Span::call_site(),
        );

        let mut from_block_to_state_match_inner = quote! {};

        let first_state_id = state_id;

        // if there's no properties, then the block is just a single state
        if block_properties_vec.is_empty() {
            block_state_enum_variants.extend(quote! {
                #block_name_pascal_case,
            });
            state_id += 1;
        }
        for combination in combinations_of(&block_properties_vec) {
            state_id += 1;
            let variant_name = Ident::new(
                &format!(
                    "{}_{}",
                    block_name_pascal_case,
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
            let mut from_block_to_state_combination_match_inner = quote! {};
            for i in 0..properties_with_name.len() {
                let property = &properties_with_name[i];
                let property_name = &property.name;
                let property_name_ident = Ident::new(property_name, proc_macro2::Span::call_site());
                let property_struct_name_ident = &property.struct_name;
                let variant =
                    Ident::new(&combination[i].to_string(), proc_macro2::Span::call_site());

                from_block_to_state_combination_match_inner.extend(quote! {
                    #property_name_ident: #property_struct_name_ident::#variant,
                });
            }

            from_block_to_state_match_inner.extend(quote! {
                #block_struct_name {
                    #from_block_to_state_combination_match_inner
                } => BlockState::#variant_name,
            });
        }

        // 7035..=7058 => {
        //     let b = b - 7035;
        //     &AcaciaButtonBlock {
        //         powered: Powered::from((b / 1) % 2),
        //         facing: Facing::from((b / 2) % 4),
        //         face: Face::from((b / 8) % 3),
        //     }
        // }
        let mut from_state_to_block_inner = quote! {};
        let mut division = 1usize;
        for i in (0..properties_with_name.len()).rev() {
            let PropertyWithNameAndDefault {
                struct_name: property_struct_name_ident,
                name: property_name,
                ..
            } = &properties_with_name[i];

            let property_variants = &block_properties_vec[i];
            let property_variants_count = property_variants.len();
            let property_name_ident = Ident::new(property_name, proc_macro2::Span::call_site());
            from_state_to_block_inner.extend(quote! {
                #property_name_ident: #property_struct_name_ident::from((b / #division) % #property_variants_count),
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
        for PropertyWithNameAndDefault {
            struct_name: struct_name_ident,
            name,
            default: property_default,
        } in properties_with_name
        {
            let name_ident = Ident::new(&name, proc_macro2::Span::call_site());
            block_default_fields.extend(quote! {
                #name_ident: #struct_name_ident::#property_default,
            })
        }

        let block_behavior = &block.behavior;
        let block_id = block.name.to_string();

        let from_block_to_state_match = if !block.properties_and_defaults.is_empty() {
            quote! {
                match b {
                    #from_block_to_state_match_inner
                }
            }
        } else {
            quote! { BlockState::#block_name_pascal_case }
        };

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
                    #from_block_to_state_match
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

    let last_state_id = (state_id - 1) as u32;
    quote! {
        #property_enums

        #[repr(u32)]
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

        impl BlockState {
            /// Returns the highest possible state
            #[inline]
            pub fn max_state() -> u32 {
                #last_state_id
            }
        }

    }
    .into()
}
