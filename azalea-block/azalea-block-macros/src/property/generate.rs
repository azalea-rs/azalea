use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Lit, LitInt};

use crate::{
    BlockStateIntegerRepr, PropertyKind, PropertyVariantData, PropertyVariantMeta,
    property::parse::{PropertyData, PropertyDefinition},
};

pub fn generate_properties_code(
    properties: &[PropertyDefinition],
    properties_to_state_ids: &HashMap<String, Vec<PropertyVariantData>>,
    last_state_id: BlockStateIntegerRepr,
) -> TokenStream {
    let mut properties_code = quote! {};
    for property in properties {
        generate_property_code(
            property,
            properties_to_state_ids,
            last_state_id,
            &mut properties_code,
        );
    }

    properties_code
}

fn generate_property_code(
    property: &PropertyDefinition,
    properties_to_state_ids: &HashMap<String, Vec<PropertyVariantData>>,
    last_state_id: BlockStateIntegerRepr,
    properties_code: &mut TokenStream,
) {
    let property_struct_name = get_property_type_name(&property.data);

    let mut to_static_str_inner = quote! {};

    match &property.data {
        PropertyData::Enum { variants, .. } => {
            let mut property_enum_variants = quote! {};
            let mut property_from_number_variants = quote! {};

            for (i, variant) in variants.iter().enumerate() {
                let variant_str = variant.name.value();
                let variant_ident = variant.ident.clone();
                let i_lit = Lit::Int(LitInt::new(&i.to_string(), proc_macro2::Span::call_site()));

                property_enum_variants.extend(quote! {
                    #variant_ident = #i_lit,
                });
                property_from_number_variants.extend(quote! {
                    #i_lit => Self::#variant_ident,
                });
                to_static_str_inner.extend(quote! {
                    Self::#variant_ident => #variant_str,
                });
            }

            properties_code.extend(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum #property_struct_name {
                    #property_enum_variants
                }

                impl From<BlockStateIntegerRepr> for #property_struct_name {
                    fn from(value: BlockStateIntegerRepr) -> Self {
                        match value {
                            #property_from_number_variants
                            _ => panic!("Invalid property value: {value}"),
                        }
                    }
                }
            });
        }
        PropertyData::Bool { .. } => {
            to_static_str_inner.extend(quote! {
                Self(true) => "true",
                Self(false) => "false",
            });

            properties_code.extend(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct #property_struct_name(pub bool);

                impl From<BlockStateIntegerRepr> for #property_struct_name {
                    /// In Minecraft, `0` = `true` and `1` = `false`.
                    fn from(value: BlockStateIntegerRepr) -> Self {
                        match value {
                            0 => Self(true),
                            1 => Self(false),
                            _ => panic!("Invalid property value: {value}"),
                        }
                    }
                }
            });
        }
    }

    let property_values = properties_to_state_ids
        .get(&property_struct_name.to_string())
        .expect("Property values not found for property");

    let try_from_block_state = generate_try_from_block_state(property_values, last_state_id);

    let value_tokens = match get_property_kind(property_values) {
        PropertyKind::Enum => quote! { Self },
        PropertyKind::Bool => quote! { bool },
    };
    let property_impl = quote! {
        impl Property for #property_struct_name {
            type Value = #value_tokens;

            fn try_from_block_state(block_state: BlockState) -> Option<Self::Value> {
                #try_from_block_state
            }

            fn to_static_str(&self) -> &'static str {
                match self {
                    #to_static_str_inner
                }
            }
        }

        impl std::fmt::Display for #property_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_static_str())
            }
        }
    };

    properties_code.extend(property_impl);
}

// generates a match statement or lookup table that's able to convert the
// `block_state` variable to an Option<property value kind>
pub fn generate_try_from_block_state(
    property_values: &[PropertyVariantData],
    last_state_id: BlockStateIntegerRepr,
) -> proc_macro2::TokenStream {
    let mut some_block_states_count = 0;
    for variant in property_values {
        some_block_states_count += variant.block_state_ids.len();
    }

    let property_kind = get_property_kind(property_values);

    let mut try_from_block_state;
    // do a simpler lookup if there's few block states
    if some_block_states_count > 2048 {
        // create a lookup table - 0 indicates None
        let table_size = last_state_id as usize + 1;
        let mut table = vec![0; table_size];
        for PropertyVariantData {
            block_state_ids,
            index: variant_index,
            ..
        } in property_values
        {
            for &block_state_id in block_state_ids {
                // add 1 since we're offsetting for zero
                table[block_state_id as usize] = variant_index + 1;
            }
        }

        let mut table_inner = quote! {};
        for entry in table {
            // this makes it not put the "usize" after the number like 0usize
            let literal_int = syn::Lit::Int(syn::LitInt::new(
                &entry.to_string(),
                proc_macro2::Span::call_site(),
            ));
            table_inner.extend(quote! { #literal_int, });
        }

        try_from_block_state = quote! {
            static TABLE: &[BlockStateIntegerRepr; #table_size] = &[#table_inner];
            let res = TABLE[block_state.id() as usize];
            if res == 0 { return None };
        };
        try_from_block_state.extend(match property_kind {
            PropertyKind::Enum => {
                quote! { Some(Self::from(res - 1)) }
            }
            PropertyKind::Bool => {
                quote! { Some(res != 2) }
            }
        })
    } else {
        // ```
        // match state_id {
        //     // this is just an example of how it might look, these state ids are definitely not correct
        //     0 | 3 | 6 => Some(Self::Axis::X),
        //     1 | 4 | 7 => Some(Self::Axis::Y),
        //     2 | 5 | 8 => Some(Self::Axis::Z),
        //     _ => None,
        // }
        // ```

        let mut enum_inner_generated = quote! {};
        for PropertyVariantData {
            block_state_ids,
            ident,
            ..
        } in property_values
        {
            enum_inner_generated.extend(match property_kind {
                PropertyKind::Enum => {
                    quote! { #(#block_state_ids)|* => Some(Self::#ident), }
                }
                PropertyKind::Bool => {
                    quote! { #(#block_state_ids)|* => Some(#ident), }
                }
            });
        }

        try_from_block_state = quote! {
            match block_state.id() {
                #enum_inner_generated
                _ => None
            }
        };
    }

    try_from_block_state
}

pub fn get_property_kind(property_values: &[PropertyVariantData]) -> PropertyKind {
    property_values
        .first()
        .map(|v| v.kind)
        .unwrap_or(PropertyKind::Enum)
}

pub fn get_property_variant_types(data: &PropertyData) -> Vec<PropertyVariantMeta> {
    match &data {
        PropertyData::Enum { variants, .. } => {
            let mut property_variant_types = Vec::new();

            for (index, variant) in variants.iter().enumerate() {
                let variant_ident = variant.ident.clone();
                property_variant_types.push(PropertyVariantMeta {
                    ident: variant_ident,
                    index,
                });
            }

            property_variant_types
        }
        PropertyData::Bool { .. } => {
            vec![
                PropertyVariantMeta {
                    ident: Ident::new("true", proc_macro2::Span::call_site()),
                    index: 0,
                },
                PropertyVariantMeta {
                    ident: Ident::new("false", proc_macro2::Span::call_site()),
                    index: 1,
                },
            ]
        }
    }
}

/// Returns either `bool` or the enum name.
pub fn get_property_value_type(data: &PropertyData) -> Ident {
    match data {
        PropertyData::Enum { enum_name, .. } => enum_name.clone(),
        PropertyData::Bool { .. } => Ident::new("bool", proc_macro2::Span::call_site()),
    }
}
/// Returns the enum or struct name of the property.
fn get_property_type_name(data: &PropertyData) -> Ident {
    match data {
        PropertyData::Enum { enum_name, .. } => enum_name.clone(),
        PropertyData::Bool { struct_name } => struct_name.clone(),
    }
}

pub fn make_property_struct_names_to_names(
    properties: &[PropertyDefinition],
) -> HashMap<String, String> {
    let mut property_struct_names_to_names = HashMap::new();

    for property in properties {
        if let PropertyData::Enum { enum_name, .. } = &property.data {
            let property_struct_name = enum_name.clone();

            property_struct_names_to_names.insert(
                property_struct_name.to_string(),
                property.name.clone().value(),
            );
        }
    }

    property_struct_names_to_names
}
