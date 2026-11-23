use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Ident, Path, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
};

pub fn filter_codec_attrs(attrs: &[Attribute]) -> impl IntoIterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("codec"))
}

#[derive(Default)]
pub struct FieldAttrs {
    pub rename: Option<String>,
    pub flatten: bool,
    pub optional: bool,
    /// If this is set, then we compare against this value instead of using
    /// Defaultable.
    pub default: Option<TokenStream>,
    pub lenient: bool,
    /// Use the given wrapper type to read and write this field.
    ///
    /// E.g. `#[codec(with = CompactListCodec)]`
    pub with: Option<Path>,
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
                "default" => {
                    input.parse::<syn::Token![=]>()?;
                    let default = input.parse::<syn::Expr>()?;
                    attrs.default = Some(default.to_token_stream());
                }
                "lenient" => attrs.lenient = true,
                "with" => {
                    input.parse::<syn::Token![=]>()?;
                    let with = input.parse::<syn::Path>()?;
                    attrs.with = Some(with);
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
pub fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut a = FieldAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let FieldAttrs {
            rename,
            flatten,
            optional,
            default,
            lenient,
            with,
        } = attr.parse_args()?;
        a.rename = rename.or(a.rename);
        a.flatten |= flatten;
        a.optional |= optional;
        a.default = default.or(a.default);
        a.lenient |= lenient;
        a.with = with.or(a.with);
    }
    if a.optional && a.lenient {
        panic!("conflicting 'optional' and 'lenient'; optional already implies lenient")
    }
    if a.flatten && a.lenient {
        panic!("conflicting 'flatten' and 'lenient'")
    }
    if a.default.is_some() && !a.optional && !a.lenient {
        panic!("if 'default' is set then 'optional' or 'lenient' must be set too")
    }

    Ok(a)
}

#[derive(Default)]
pub struct StructAttrs {
    pub allow_unknown_keys: bool,
    pub transparent: bool,
    /// When reading, pretend as if this field were there.
    pub virt: Vec<(Vec<Attribute>, Ident, Type)>,
}
impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr_ident = input.parse::<proc_macro2::Ident>()?;
            match attr_ident.to_string().as_str() {
                "allow_unknown_keys" => attrs.allow_unknown_keys = true,
                "transparent" => attrs.transparent = true,
                "virt" => {
                    let content;
                    parenthesized!(content in input);
                    let virt_attrs = content.call(Attribute::parse_outer)?;
                    let ident = content.parse::<Ident>()?;
                    content.parse::<Token![:]>()?;
                    let ty = content.parse::<Type>()?;
                    attrs.virt.push((virt_attrs, ident, ty));
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
pub fn parse_struct_attrs(attrs: &[syn::Attribute]) -> syn::Result<StructAttrs> {
    let mut a = StructAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let StructAttrs {
            allow_unknown_keys,
            transparent,
            virt,
        } = attr.parse_args()?;
        a.allow_unknown_keys |= allow_unknown_keys;
        a.transparent |= transparent;
        a.virt.extend(virt);
    }
    if a.transparent && a.allow_unknown_keys {
        panic!("conflicting 'transparent' and 'allow_unknown_keys'")
    }
    Ok(a)
}

#[derive(Default, Debug)]
pub struct EnumVariantAttrs {
    pub key: Option<String>,
    /// Whether this variant should match regardless of key or extra fields.
    /// Setting this implies `allow_unknown_keys`.
    pub fallback: bool,
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
                "fallback" => attrs.fallback = true,
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
        let Ok(EnumVariantAttrs { key, fallback }) = attr.parse_args() else {
            unhandled.push(attr.clone());
            continue;
        };

        a.key = key.or(a.key);
        a.fallback |= fallback;
    }

    if a.key.is_some() && a.fallback {
        panic!("conflicting 'key' and 'fallback'")
    }

    (a, unhandled)
}

#[derive(Default, Debug)]
pub struct EnumAttrs {
    pub tag: Option<String>,
    /// Encode the tag as a separate type which has `From<str> + ToString +
    /// Codec<O>`.
    ///
    /// This is equivalent to adding `#[codec(virt(FromLiteral<tag, tag_as>))]`
    /// on your variants.
    pub tag_as: Option<Type>,
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
                "tag_as" => {
                    input.parse::<syn::Token![=]>()?;
                    let v = input.parse::<syn::Type>()?;
                    attrs.tag_as = Some(v);
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
            tag_as,
            default_snake_case_key,
            is_map,
        } = attr.parse_args()?;
        a.tag = tag.or(a.tag);
        a.tag_as = tag_as.or(a.tag_as);
        a.default_snake_case_key |= default_snake_case_key;
        a.is_map |= is_map;
    }
    Ok(a)
}
