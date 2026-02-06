//! An internal crate used by `azalea_block`.

mod property;
mod utils;

use std::{collections::HashMap, fmt::Write};

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{
    Expr, Ident, Token, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
};
use utils::{combinations_of, to_pascal_case};

use crate::property::{
    generate::{
        generate_properties_code, get_property_value_type, get_property_variant_types,
        make_property_struct_names_to_names,
    },
    parse::{PropertyDefinition, PropertyWithNameAndDefault, parse_property_definitions},
};

// must be the same as the type in `azalea-block/src/lib.rs`
type BlockStateIntegerRepr = u16;

/// ```ignore
/// grass_block => BlockBehavior::default(), {
///   "snowy": false,
/// },
/// ```
struct BlockDefinition {
    name: Ident,
    behavior: Expr,
    properties_and_defaults: Vec<PropertyWithNameAndDefault>,
}

struct BlockDefinitions {
    blocks: Vec<BlockDefinition>,
}

impl Parse for BlockDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        // acacia_button => BlockBehavior::new().strength(0.5, 0.5), {
        //     "face": Face::Wall,
        //     "facing": FacingCardinal::North,
        //     "powered": Powered(false),
        // },
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;
        let behavior = input.parse()?;

        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);

        let mut properties_and_defaults = Vec::new();

        // read the things comma-separated
        let property_and_default_punctuated =
            content.parse_terminated(PropertyWithNameAndDefault::parse, Token![,])?;

        for property_and_default in property_and_default_punctuated {
            properties_and_defaults.push(property_and_default);
        }

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

        let block_definitions_punctuated =
            input.parse_terminated(BlockDefinition::parse, Token![,])?;
        for block_definition in block_definitions_punctuated {
            blocks.push(block_definition);
        }

        Ok(BlockDefinitions { blocks })
    }
}

struct MakeBlockStates {
    pub properties: Vec<PropertyDefinition>,
    pub blocks: BlockDefinitions,
}
impl Parse for MakeBlockStates {
    fn parse(input: ParseStream) -> Result<Self> {
        // Properties => { ... } Blocks => { ... }
        let properties_ident = input.parse::<Ident>()?;
        assert_eq!(properties_ident.to_string(), "Properties");
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let properties = parse_property_definitions(&content)?;

        input.parse::<Token![,]>()?;

        let blocks_ident = input.parse::<Ident>()?;
        assert_eq!(blocks_ident.to_string(), "Blocks");
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let blocks = content.parse()?;

        Ok(MakeBlockStates { properties, blocks })
    }
}

struct PropertyVariantData {
    pub block_state_ids: Vec<BlockStateIntegerRepr>,
    pub kind: PropertyKind,
    pub ident: Ident,
    pub index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PropertyKind {
    Enum,
    Bool,
}

#[derive(Clone, Debug)]
struct PropertyVariantMeta {
    pub ident: Ident,
    pub index: usize,
}

#[proc_macro]
pub fn make_block_states(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MakeBlockStates);

    let mut properties_map = HashMap::new();
    for property in &input.properties {
        let property_value_type = get_property_value_type(&property.data);
        let property_variant_types = get_property_variant_types(&property.data);
        properties_map.insert(property_value_type.to_string(), property_variant_types);
    }

    let property_struct_names_to_names = make_property_struct_names_to_names(&input.properties);

    let mut block_state_enum_variants = quote! {};
    let mut block_structs = quote! {};

    let mut from_state_to_block_match = quote! {};
    let mut from_registry_block_to_block_match = quote! {};
    let mut from_registry_block_to_blockstate_match = quote! {};
    let mut from_registry_block_to_blockstates_match = quote! {};

    // keys are enum names like Waterlogged
    let mut properties_to_state_ids = HashMap::<String, Vec<PropertyVariantData>>::new();

    let mut state_id: BlockStateIntegerRepr = 0;
    for block in &input.blocks.blocks {
        let block_property_names = &block
            .properties_and_defaults
            .iter()
            .map(|p| p.property_value_type.to_string())
            .collect::<Vec<_>>();
        let mut block_properties_vec = Vec::new();
        for property_name in block_property_names {
            let property_variants = properties_map
                .get(property_name)
                .unwrap_or_else(|| panic!("Property '{property_name}' not found"))
                .clone();
            block_properties_vec.push(property_variants);
        }

        let mut properties_with_name: Vec<PropertyWithNameAndDefault> =
            Vec::with_capacity(block.properties_and_defaults.len());
        // Used to determine the index of the property so we can optionally add a number
        // to it
        let mut previous_names: Vec<String> = Vec::new();
        for property in &block.properties_and_defaults {
            let index: Option<usize> = if block
                .properties_and_defaults
                .iter()
                .filter(|p| p.name == property.name)
                .count()
                > 1
            {
                Some(
                    previous_names
                        .iter()
                        .filter(|&p| p == &property.name)
                        .count(),
                )
            } else {
                None
            };

            let mut property_name = property_struct_names_to_names
                .get(&property.name)
                .cloned()
                .unwrap_or_else(|| property.name.clone());
            previous_names.push(property_name.clone());
            if let Some(index) = index {
                // property_name.push_str(&format!("_{}", &index.to_string()));
                write!(property_name, "_{index}").unwrap();
            }
            properties_with_name.push(PropertyWithNameAndDefault {
                name_ident: name_to_ident(&property_name),
                name: property_name,
                property_type: property.property_type.clone(),
                property_value_type: property.property_value_type.clone(),
                kind: property.kind,
                default: property.default.clone(),
            });
        }
        drop(previous_names);

        //     pub face: properties::Face,
        //     pub facing: properties::Facing,
        //     pub powered: properties::Powered,
        // or
        //     pub has_bottle_0: HasBottle,
        //     pub has_bottle_1: HasBottle,
        //     pub has_bottle_2: HasBottle,
        let mut block_struct_fields = quote! {};
        for PropertyWithNameAndDefault {
            property_value_type,
            name_ident,
            kind,
            ..
        } in &properties_with_name
        {
            block_struct_fields.extend(match kind {
                PropertyKind::Enum => {
                    quote! { pub #name_ident: properties::#property_value_type, }
                }
                PropertyKind::Bool => {
                    quote! { pub #name_ident: #property_value_type, }
                }
            });
        }

        let block_name_pascal_case = Ident::new(
            &to_pascal_case(&block.name.to_string()),
            proc_macro2::Span::call_site(),
        );
        let block_struct_name = Ident::new(
            &block_name_pascal_case.to_string(),
            proc_macro2::Span::call_site(),
        );

        let first_state_id = state_id;
        let mut default_state_id = None;

        // if there's no properties, then the block is just a single state
        if block_properties_vec.is_empty() {
            block_state_enum_variants.extend(quote! {
                #block_name_pascal_case,
            });
            default_state_id = Some(state_id);
            state_id += 1;
        }
        for combination in combinations_of(&block_properties_vec) {
            let mut is_default = true;

            // 	face: properties::Face::Floor,
            // 	facing: properties::Facing::North,
            // 	powered: properties::Powered::True,
            let mut from_block_to_state_combination_match_inner = quote! {};
            for i in 0..properties_with_name.len() {
                let property = &properties_with_name[i];
                let property_name_ident = &property.name_ident;
                let property_value_name_ident = &property.property_type;
                let variant = &combination[i];
                let variant_ident = variant.ident.clone();

                // property.default is a TokenStream, so we have to parse it like this
                let property_default_ident = property
                    .default
                    .clone()
                    .into_iter()
                    .last()
                    .and_then(|tt| match tt {
                        TokenTree::Ident(ident) => Some(ident),
                        _ => None,
                    })
                    .unwrap();
                if variant.ident != property_default_ident {
                    is_default = false;
                }

                let property_variant = match property.kind {
                    PropertyKind::Enum => {
                        quote! { properties::#property_value_name_ident::#variant_ident }
                    }
                    PropertyKind::Bool => {
                        quote! { #variant_ident }
                    }
                };

                from_block_to_state_combination_match_inner.extend(quote! {
                    #property_name_ident: #property_variant,
                });

                // add to properties_to_state_ids
                let property_variants = properties_to_state_ids
                    .entry(property_value_name_ident.to_string())
                    .or_default();
                let property_variant_data = property_variants
                    .iter_mut()
                    .find(|v| v.ident == variant_ident);
                if let Some(property_variant_data) = property_variant_data {
                    property_variant_data.block_state_ids.push(state_id);
                } else {
                    property_variants.push(PropertyVariantData {
                        block_state_ids: vec![state_id],
                        ident: variant_ident,
                        index: variant.index,
                        kind: property.kind,
                    });
                }
            }

            if is_default {
                default_state_id = Some(state_id);
            }

            state_id += 1;
        }

        let Some(default_state_id) = default_state_id else {
            let defaults = properties_with_name
                .iter()
                .map(|p| match p.default.clone().into_iter().last().unwrap() {
                    TokenTree::Ident(i) => i.to_string(),
                    _ => {
                        panic!()
                    }
                })
                .collect::<Vec<_>>();
            panic!(
                "Couldn't get default state id for {block_name_pascal_case}, combinations={block_properties_vec:?}, defaults={defaults:?}"
            )
        };

        // 7035..=7058 => {
        //     let b = b - 7035;
        //     &AcaciaButtonBlock {
        //         powered: properties::Powered::from((b / 1) % 2),
        //         facing: properties::Facing::from((b / 2) % 4),
        //         face: properties::Face::from((b / 8) % 3),
        //     }
        // }
        let mut from_state_to_block_inner = quote! {};
        let mut division: BlockStateIntegerRepr = 1;
        for i in (0..properties_with_name.len()).rev() {
            let PropertyWithNameAndDefault {
                property_type: property_struct_name_ident,
                name_ident: property_name_ident,
                property_value_type,
                ..
            } = &properties_with_name[i];

            let property_variants = &block_properties_vec[i];
            let property_variants_count = property_variants.len() as crate::BlockStateIntegerRepr;
            let conversion_code = {
                if &property_value_type.to_string() == "bool" {
                    assert_eq!(property_variants_count, 2);
                    // this is not a mistake, it starts with true for some reason
                    quote! {(b / #division) % #property_variants_count == 0}
                } else {
                    quote! {properties::#property_struct_name_ident::from((b / #division) % #property_variants_count)}
                }
            };
            from_state_to_block_inner.extend(quote! {
                #property_name_ident: #conversion_code,
            });

            division *= property_variants_count;
        }

        let mut as_block_state_inner = quote! { #first_state_id };
        let mut factor: BlockStateIntegerRepr = 1;
        for i in (0..properties_with_name.len()).rev() {
            let PropertyWithNameAndDefault {
                name_ident: property_name_ident,
                property_value_type,
                ..
            } = &properties_with_name[i];

            let property_variants = &block_properties_vec[i];
            let property_variants_count = property_variants.len() as crate::BlockStateIntegerRepr;
            if &property_value_type.to_string() == "bool" {
                // this is not a mistake, it starts with true for some reason, so invert it to
                // make `true be 0`
                as_block_state_inner.extend(
                    quote! { + (!self.#property_name_ident as BlockStateIntegerRepr) * #factor},
                );
            } else {
                as_block_state_inner.extend(
                    quote! { + (self.#property_name_ident as BlockStateIntegerRepr) * #factor},
                );
            };

            factor *= property_variants_count;
        }

        let last_state_id = state_id - 1;
        from_state_to_block_match.extend(if first_state_id == last_state_id {
            quote! {
                #first_state_id => {
                    Box::new(#block_struct_name { #from_state_to_block_inner })
                },
            }
        } else {
            quote! {
                #first_state_id..=#last_state_id => {
                    let b = b - #first_state_id;
                    Box::new(#block_struct_name { #from_state_to_block_inner })
                },
            }
        });

        from_registry_block_to_block_match.extend(quote! {
            BlockKind::#block_name_pascal_case => Box::new(#block_struct_name::default()),
        });
        from_registry_block_to_blockstate_match.extend(quote! {
            BlockKind::#block_name_pascal_case => BlockState::new_const(#default_state_id),
        });
        from_registry_block_to_blockstates_match.extend(quote! {
            BlockKind::#block_name_pascal_case => BlockStates::from(#first_state_id..=#last_state_id),
        });

        let mut property_map_inner = quote! {};
        let mut get_property_match_inner = quote! {};
        let mut set_property_match_inner = quote! {};

        for PropertyWithNameAndDefault {
            name,
            name_ident,
            kind,
            ..
        } in &properties_with_name
        {
            let variant_name_tokens = match kind {
                PropertyKind::Enum => quote! { self.#name_ident.to_static_str() },
                PropertyKind::Bool => quote! { if self.#name_ident { "true" } else { "false" } },
            };
            property_map_inner.extend(quote! {
                map.insert(#name, #variant_name_tokens);
            });
            get_property_match_inner.extend(quote! {
                #name => Some(#variant_name_tokens),
            });

            set_property_match_inner.extend(match kind {
                PropertyKind::Enum => quote! { #name => self.#name_ident = new_value.parse()?, },
                PropertyKind::Bool => {
                    quote! { #name => self.#name_ident = new_value.parse::<bool>().map_err(|_| InvalidPropertyError)?, }
                }
            });
        }
        let set_property = if set_property_match_inner.is_empty() {
            quote! {
                Err(InvalidPropertyError)
            }
        } else {
            quote! {
                match name {
                    #set_property_match_inner
                    _ => return Err(InvalidPropertyError),
                }
                Ok(())
            }
        };

        let mut block_default_fields = quote! {};
        for PropertyWithNameAndDefault {
            name_ident,
            default: property_default,
            ..
        } in properties_with_name
        {
            block_default_fields.extend(quote! { #name_ident: #property_default, });
        }

        let block_behavior = &block.behavior;
        let block_id = block.name.to_string();

        let as_block_state = quote! { BlockState::new_const(#as_block_state_inner) };

        let mut block_struct = quote! {
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct #block_struct_name
        };
        if block_struct_fields.is_empty() {
            block_struct.extend(quote! {;});
        } else {
            block_struct.extend(quote! { { #block_struct_fields } });
        }

        block_struct.extend(quote! {
            impl BlockTrait for #block_struct_name {
                fn behavior(&self) -> BlockBehavior {
                    #block_behavior
                }
                fn id(&self) -> &'static str {
                    #block_id
                }
                fn as_block_state(&self) -> BlockState {
                    #as_block_state
                }
                fn as_registry_block(&self) -> BlockKind {
                    BlockKind::#block_name_pascal_case
                }

                fn property_map(&self) -> std::collections::HashMap<&'static str, &'static str> {
                    let mut map = std::collections::HashMap::new();
                    #property_map_inner
                    map
                }
                fn get_property(&self, name: &str) -> Option<&'static str> {
                    match name {
                        #get_property_match_inner
                        _ => None,
                    }
                }
                fn set_property(&mut self, name: &str, new_value: &str) -> Result<(), InvalidPropertyError> {
                    #set_property
                }
            }

            impl From<#block_struct_name> for BlockState {
                fn from(b: #block_struct_name) -> Self {
                    b.as_block_state()
                }
            }

            impl Default for #block_struct_name {
                fn default() -> Self {
                    Self {
                        #block_default_fields
                    }
                }
            }
        });

        block_structs.extend(block_struct);
    }

    let last_state_id = state_id - 1;
    let mut generated = quote! {
        impl BlockState {
            /// The highest possible block state ID.
            pub const MAX_STATE: BlockStateIntegerRepr = #last_state_id;

            /// Get a property from this block state, or `None` if the block can't have the property.
            ///
            /// ```
            /// fn is_waterlogged(block_state: azalea_block::BlockState) -> bool {
            ///     block_state.property::<azalea_block::properties::Waterlogged>().unwrap_or_default()
            /// }
            /// ```
            pub fn property<P: Property>(self) -> Option<P::Value> {
                P::try_from_block_state(self)
            }
        }
    };

    let properties_code =
        generate_properties_code(&input.properties, &properties_to_state_ids, last_state_id);

    generated.extend(quote! {
        pub mod properties {
            use super::*;

            #properties_code
        }

        pub mod blocks {
            use super::*;
            use azalea_registry::builtin::BlockKind;

            #block_structs

            impl From<BlockState> for Box<dyn BlockTrait> {
                fn from(block_state: BlockState) -> Self {
                    let b = block_state.id();
                    match b {
                        #from_state_to_block_match
                        _ => panic!("Invalid block state: {}", b),
                    }
                }
            }
            impl From<BlockKind> for Box<dyn BlockTrait> {
                fn from(block: BlockKind) -> Self {
                    match block {
                        #from_registry_block_to_block_match
                        _ => unreachable!("There should always be a block struct for every BlockKind variant")
                    }
                }
            }
            impl From<BlockKind> for BlockState {
                fn from(block: BlockKind) -> Self {
                    match block {
                        #from_registry_block_to_blockstate_match
                        _ => unreachable!("There should always be a block state for every BlockKind variant")
                    }
                }
            }
            impl From<BlockKind> for BlockStates {
                fn from(block: BlockKind) -> Self {
                    match block {
                        #from_registry_block_to_blockstates_match
                        _ => unreachable!("There should always be a block state for every BlockKind variant")
                    }
                }
            }
        }
    });

    generated.into()
}

/// Convert a name to a Rust identifier, replacing some Rust keywords with
/// alternatives (e.g. `type` -> `kind`).
fn name_to_ident(name: &str) -> Ident {
    let ident_str = match name {
        "type" => "kind",
        _ => name,
    };
    Ident::new(ident_str, proc_macro2::Span::call_site())
}
