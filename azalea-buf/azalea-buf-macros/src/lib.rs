mod read;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(AzaleaRead, attributes(var))]
pub fn derive_azalearead(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    read::create_impl_azalearead(&ident, &generics, &data).into()
}

#[proc_macro_derive(AzaleaWrite, attributes(var))]
pub fn derive_azaleawrite(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    write::create_impl_azaleawrite(&ident, &generics, &data).into()
}

#[proc_macro_derive(AzBuf, attributes(var, limit))]
pub fn derive_azbuf(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let writable = write::create_impl_azaleawrite(&ident, &generics, &data);
    let readable = read::create_impl_azalearead(&ident, &generics, &data);
    quote! {
        #writable
        #readable
    }
    .into()
}
