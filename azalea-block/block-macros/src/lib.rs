use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Debug;
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Expr, FieldsNamed, Ident, LitInt, Token,
};

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

    for property in &input.property_definitions.properties {
        let mut property_enum_variants = quote! {};

        for variant in &property.variants {
            property_enum_variants.extend(quote! {
                #variant,
            });
        }

        let property_name = &property.name;

        property_enums.extend(quote! {
            #[derive(Debug, Clone, Copy)]
            pub enum #property_name {
                #property_enum_variants
            }
        });
    }

    // let mut block_state_enum_variants = quote! {};

    // for block in &input.block_definitions.blocks {
    //     let block_state_enum_variant = quote! {
    //         #block.name(#block.behavior, #block.properties)
    //     };
    //     block_state_enum_variants.extend(block_state_enum_variant);
    // }

    quote! {
        #property_enums
        // #block_state_enum_variants
    }
    .into()
}
