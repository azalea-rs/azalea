mod read;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(AzBuf, attributes(var, limit))]
pub fn derive_azbuf(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let writable = write::create_fn_azalea_write(&data);
    let readable = read::create_fn_azalea_read(&data);
    quote! {
        impl #impl_generics azalea_buf::AzBuf for #ident #ty_generics #where_clause {
            #writable
            #readable
        }
    }
    .into()
}
