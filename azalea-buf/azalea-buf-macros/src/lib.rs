mod read;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AzaleaRead, attributes(var))]
pub fn derive_azalearead(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    read::create_impl_azalearead(&ident, &data).into()
}

#[proc_macro_derive(AzaleaWrite, attributes(var))]
pub fn derive_azaleawrite(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    write::create_impl_azaleawrite(&ident, &data).into()
}

#[proc_macro_derive(AzBuf, attributes(var, limit))]
pub fn derive_azbuf(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let writable = write::create_impl_azaleawrite(&ident, &data);
    let readable = read::create_impl_azalearead(&ident, &data);
    quote! {
        #writable
        #readable
    }
    .into()
}
