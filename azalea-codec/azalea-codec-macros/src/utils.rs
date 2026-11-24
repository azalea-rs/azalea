use quote::quote;
use syn::GenericParam;

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_uppercase = true;
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !prev_was_uppercase {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_uppercase = true;
        } else {
            result.push(c);
            prev_was_uppercase = false;
        }
    }
    result
}

pub fn generic_names<'a>(
    params: impl IntoIterator<Item = &'a GenericParam>,
) -> proc_macro2::TokenStream {
    let mut generic_param_names = quote! {};
    for v in params {
        generic_param_names.extend(match v {
            GenericParam::Lifetime(p) => {
                let p = &p.lifetime;
                quote! {#p}
            }
            GenericParam::Type(p) => {
                let p = &p.ident;
                quote! {#p}
            }
            GenericParam::Const(p) => {
                let p = &p.ident;
                quote! {#p}
            }
        });
        generic_param_names.extend(quote! {,});
    }
    generic_param_names
}
