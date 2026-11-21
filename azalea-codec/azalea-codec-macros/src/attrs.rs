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
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let rename = input.parse::<syn::LitStr>()?;
                    attrs.rename = Some(rename.value());
                }
                "flatten" => attrs.flatten = true,
                "optional" => attrs.optional = true,
                "lenient" => attrs.lenient = true,
                attr => panic!("unknown attribute '{attr}'"),
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut a = FieldAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let n = attr.parse_args::<FieldAttrs>().expect("invalid codec attr");
        if let Some(rename) = n.rename {
            a.rename = Some(rename);
        }
        a.flatten |= n.flatten;
        a.optional |= n.optional;
        a.lenient |= n.lenient;
    }
    if a.optional && a.lenient {
        panic!("conflicting 'optional' and 'lenient'; optional already implies lenient")
    }
    if a.flatten && a.lenient {
        panic!("conflicting 'flatten' and 'lenient'")
    }
    a
}

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub allow_unknown_keys: bool,
}
impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();
        while !input.is_empty() {
            let attr = input.parse::<proc_macro2::Ident>()?;
            match attr.to_string().as_str() {
                "allow_unknown_keys" => attrs.allow_unknown_keys = true,
                attr => panic!("unknown attribute '{attr}'"),
            }
            let _ = input.parse::<syn::Token![,]>();
        }
        Ok(attrs)
    }
}
pub fn parse_struct_attrs(attrs: &[syn::Attribute]) -> StructAttrs {
    let mut a = StructAttrs::default();
    for attr in filter_codec_attrs(attrs) {
        let n = attr
            .parse_args::<StructAttrs>()
            .expect("invalid codec attr");
        a.allow_unknown_keys |= n.allow_unknown_keys;
    }
    a
}
