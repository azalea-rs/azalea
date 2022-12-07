mod read;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(McBufReadable, attributes(var))]
pub fn derive_mcbufreadable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    read::create_impl_mcbufreadable(&ident, &data).into()
}

#[proc_macro_derive(McBufWritable, attributes(var))]
pub fn derive_mcbufwritable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    write::create_impl_mcbufwritable(&ident, &data).into()
}

#[proc_macro_derive(McBuf, attributes(var))]
pub fn derive_mcbuf(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let writable = write::create_impl_mcbufwritable(&ident, &data);
    let readable = read::create_impl_mcbufreadable(&ident, &data);
    quote! {
        #writable
        #readable
    }
    .into()
}
