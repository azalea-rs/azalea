use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, DeriveInput, FieldsNamed};

fn as_packet_derive(
    input: proc_macro::TokenStream,
    state: proc_macro2::TokenStream,
) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("#[derive(*Packet)] can only be used on structs"),
    };
    let FieldsNamed { named, .. } = match fields {
        syn::Fields::Named(f) => f,
        _ => panic!("#[derive(*Packet)] can only be used on structs with named fields"),
    };

    let write_fields = named
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_type = &f.ty;
            // do a different buf.write_* for each field depending on the type
            // if it's a string, use buf.write_string
            match field_type {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    if path.is_ident("String") {
                        quote! { buf.write_utf(&self.#field_name)?; }
                    } else if path.is_ident("ResourceLocation") {
                        quote! { buf.write_resource_location(&self.#field_name)?; }
                    // i don't know how to do this in a way that isn't terrible
                    } else if path.to_token_stream().to_string() == "Vec < u8 >" {
                        quote! { buf.write_bytes(&self.#field_name)?; }
                    } else if path.is_ident("u32") {
                        if f.attrs.iter().any(|attr| attr.path.is_ident("varint")) {
                            quote! { buf.write_varint(self.#field_name as i32)?; }
                        } else {
                            quote! { buf.write_u32(self.#field_name)?; }
                        }
                    } else if path.is_ident("u16") {
                        quote! { buf.write_short(self.#field_name as i16)?; }
                    } else if path.is_ident("ConnectionProtocol") {
                        quote! { buf.write_varint(self.#field_name.clone() as i32)?; }
                    } else {
                        if f.attrs.iter().any(|attr| attr.path.is_ident("varint")) {
                            quote! {
                                crate::mc_buf::McBufVarintWritable::varint_write_into(&self.#field_name, buf)?;
                            }
                        } else {
                            quote! {
                                crate::mc_buf::McBufVarintWritable::write_into(&self.#field_name, buf)?;
                            }
                        }

                    }
                }
                _ => panic!(
                    "Error writing field {}: {}",
                    field_name.clone().unwrap(),
                    field_type.to_token_stream()
                ),
            }
        })
        .collect::<Vec<_>>();

    let read_fields = named
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_type = &f.ty;
            // do a different buf.write_* for each field depending on the type
            // if it's a string, use buf.write_string
            match field_type {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    if path.is_ident("String") {
                        quote! { let #field_name = buf.read_utf().await?; }
                    } else if path.is_ident("ResourceLocation") {
                        quote! { let #field_name = buf.read_resource_location().await?; }
                    } else if path.to_token_stream().to_string() == "Vec < u8 >" {
                        quote! { let #field_name = buf.read_bytes().await?; }
                    } else if path.is_ident("u32") {
                        if f.attrs.iter().any(|a| a.path.is_ident("varint")) {
                            quote! { let #field_name = buf.read_varint().await? as u32; }
                        } else {
                            quote! { let #field_name = buf.read_u32().await?; }
                        }
                    } else if path.is_ident("u16") {
                        quote! { let #field_name = buf.read_short().await? as u16; }
                    } else if path.is_ident("ConnectionProtocol") {
                        quote! {
                            let #field_name = ConnectionProtocol::from_i32(buf.read_varint().await?)
                                .ok_or_else(|| "Invalid intention".to_string())?;
                        }
                    } else {
                        if f.attrs.iter().any(|a| a.path.is_ident("varint")) {
                            quote! {
                                let #field_name = crate::mc_buf::McBufVarintReadable::varint_read_into(buf).await?;
                            }
                        } else {
                            quote! {
                                let #field_name = crate::mc_buf::McBufReadable::read_into(buf).await?;
                            }
                        }
                    }
                }
                _ => panic!(
                    "Error reading field {}: {}",
                    field_name.clone().unwrap(),
                    field_type.to_token_stream()
                ),
            }
        })
        .collect::<Vec<_>>();
    let read_field_names = named.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let gen = quote! {
        impl #ident {
            pub fn get(self) -> #state {
                #state::#ident(self)
            }

            pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
                #(#write_fields)*
                Ok(())
            }

            pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
                buf: &mut T,
            ) -> Result<#state, String> {
                #(#read_fields)*
                Ok(#ident {
                    #(#read_field_names: #read_field_names),*
                }.get())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(GamePacket, attributes(varint))]
pub fn derive_game_packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::GamePacket})
}

#[proc_macro_derive(HandshakePacket, attributes(varint))]
pub fn derive_handshake_packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_packet_derive(input, quote! {crate::packets::handshake::HandshakePacket})
}

#[proc_macro_derive(LoginPacket, attributes(varint))]
pub fn derive_login_packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_packet_derive(input, quote! {crate::packets::login::LoginPacket})
}

#[proc_macro_derive(StatusPacket, attributes(varint))]
pub fn derive_status_packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_packet_derive(input, quote! {crate::packets::status::StatusPacket})
}
