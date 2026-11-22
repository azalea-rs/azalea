use syn::{
    Attribute,
    parse::{Parse, ParseStream},
};

pub fn filter_codec_attrs(attrs: &[Attribute]) -> impl IntoIterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("codec"))
}

#[derive(Default, Debug)]
pub struct FieldAttrs {
    pub rename: Option<String>,
    pub flatten: bool,
    pub optional: bool,
    pub lenient: bool,
}
impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr_ident = input.parse::<proc_macro2::Ident>()?;
            match attr_ident.to_string().as_str() {
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let rename = input.parse::<syn::LitStr>()?;
                    attrs.rename = Some(rename.value());
                }
                "flatten" => attrs.flatten = true,
                "optional" => attrs.optional = true,
                "lenient" => attrs.lenient = true,
                attr => {
                    return Err(syn::Error::new(
                        attr_ident.span(),
                        format!("unknown attribute '{attr}'"),
                    ));
                }
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut a = FieldAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let FieldAttrs {
            rename,
            flatten,
            optional,
            lenient,
        } = attr.parse_args()?;
        a.rename = rename.or(a.rename);
        a.flatten |= flatten;
        a.optional |= optional;
        a.lenient |= lenient;
    }
    if a.optional && a.lenient {
        panic!("conflicting 'optional' and 'lenient'; optional already implies lenient")
    }
    if a.flatten && a.lenient {
        panic!("conflicting 'flatten' and 'lenient'")
    }
    Ok(a)
}

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub allow_unknown_keys: bool,
    pub transparent: bool,
}
impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr_ident = input.parse::<proc_macro2::Ident>()?;
            match attr_ident.to_string().as_str() {
                "allow_unknown_keys" => attrs.allow_unknown_keys = true,
                "transparent" => attrs.transparent = true,
                attr => {
                    return Err(syn::Error::new(
                        attr_ident.span(),
                        format!("unknown attribute '{attr}'"),
                    ));
                }
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_struct_attrs(attrs: &[syn::Attribute]) -> syn::Result<StructAttrs> {
    let mut a = StructAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let StructAttrs {
            allow_unknown_keys,
            transparent,
        } = attr.parse_args()?;
        a.allow_unknown_keys |= allow_unknown_keys;
        a.transparent |= transparent;
    }
    if a.transparent && a.allow_unknown_keys {
        panic!("conflicting 'transparent' and 'allow_unknown_keys'")
    }
    Ok(a)
}

#[derive(Default, Debug)]
pub struct EnumVariantAttrs {
    pub key: Option<String>,
}
impl Parse for EnumVariantAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr_ident = input.parse::<proc_macro2::Ident>()?;
            match attr_ident.to_string().as_str() {
                "key" => {
                    input.parse::<syn::Token![=]>()?;
                    let key = input.parse::<syn::LitStr>()?;
                    attrs.key = Some(key.value());
                }
                attr => {
                    return Err(syn::Error::new(
                        attr_ident.span(),
                        format!("unknown attribute '{attr}'"),
                    ));
                }
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_enum_variant_attrs(
    attrs: &[syn::Attribute],
) -> (EnumVariantAttrs, Vec<syn::Attribute>) {
    let mut a = EnumVariantAttrs::default();
    let mut unhandled = Vec::new();

    for attr in filter_codec_attrs(attrs) {
        let Ok(EnumVariantAttrs { key }) = attr.parse_args() else {
            unhandled.push(attr.clone());
            continue;
        };

        a.key = key.or(a.key);
    }
    (a, unhandled)
}

#[derive(Default, Debug)]
pub struct EnumAttrs {
    pub tag: Option<String>,
    pub default_snake_case_key: bool,
    pub is_map: bool,
}
impl Parse for EnumAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr_ident = input.parse::<proc_macro2::Ident>()?;
            match attr_ident.to_string().as_str() {
                "tag" => {
                    input.parse::<syn::Token![=]>()?;
                    let v = input.parse::<syn::LitStr>()?;
                    attrs.tag = Some(v.value());
                }
                "default_snake_case_key" => attrs.default_snake_case_key = true,
                "is_map" => attrs.is_map = true,
                attr => {
                    return Err(syn::Error::new(
                        attr_ident.span(),
                        format!("unknown attribute '{attr}'"),
                    ));
                }
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_enum_attrs(attrs: &[syn::Attribute]) -> syn::Result<EnumAttrs> {
    let mut a = EnumAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let EnumAttrs {
            tag,
            default_snake_case_key,
            is_map,
        } = attr.parse_args()?;
        a.tag = tag.or(a.tag);
        a.default_snake_case_key |= default_snake_case_key;
        a.is_map |= is_map;
    }
    Ok(a)
}
