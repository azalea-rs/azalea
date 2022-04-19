use std::collections::{BTreeMap, HashMap};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, DeriveInput, Expr, FieldsNamed, Ident, LitInt, Token, Type, Visibility,
};

fn as_packet_derive(input: TokenStream, state: proc_macro2::TokenStream) -> TokenStream {
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
                syn::Type::Path(_) => {
                    if f.attrs.iter().any(|attr| attr.path.is_ident("varint")) {
                        quote! {
                            crate::mc_buf::McBufVarintWritable::varint_write_into(&self.#field_name, buf)?;
                        }
                    } else {
                        quote! {
                            crate::mc_buf::McBufWritable::write_into(&self.#field_name, buf)?;
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
                syn::Type::Path(_) => {
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
pub fn derive_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::GamePacket})
}

#[proc_macro_derive(HandshakePacket, attributes(varint))]
pub fn derive_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::handshake::HandshakePacket})
}

#[proc_macro_derive(LoginPacket, attributes(varint))]
pub fn derive_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::login::LoginPacket})
}

#[proc_macro_derive(StatusPacket, attributes(varint))]
pub fn derive_status_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::status::StatusPacket})
}

#[derive(Debug)]
struct PacketIdPair {
    id: u32,
    module: Ident,
    name: Ident,
}
#[derive(Debug)]
struct PacketIdMap {
    packets: Vec<PacketIdPair>,
}

impl Parse for PacketIdMap {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut packets = vec![];
        loop {
            // 0x0e: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
            // 0x0e
            let packet_id: LitInt = match input.parse() {
                Ok(i) => i,
                Err(_) => break,
            };
            let packet_id = packet_id.base10_parse::<u32>()?;
            // :
            input.parse::<Token![:]>()?;
            // clientbound_change_difficulty_packet
            let module: Ident = input.parse()?;
            // ::
            input.parse::<Token![::]>()?;
            // ClientboundChangeDifficultyPacket
            let name: Ident = input.parse()?;
            input.parse::<Token![,]>()?;

            packets.push(PacketIdPair {
                id: packet_id,
                module,
                name,
            });
        }

        Ok(PacketIdMap { packets })
    }
}

#[derive(Debug)]
struct DeclareStatePackets {
    name: Ident,
    serverbound: PacketIdMap,
    clientbound: PacketIdMap,
}

impl Parse for DeclareStatePackets {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let serverbound_token: Ident = input.parse()?;
        if serverbound_token != "Serverbound" {
            return Err(syn::Error::new(
                serverbound_token.span(),
                "Expected `Serverbound`",
            ));
        }
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let serverbound = content.parse()?;

        input.parse::<Token![,]>()?;

        let clientbound_token: Ident = input.parse()?;
        if clientbound_token != "Clientbound" {
            return Err(syn::Error::new(
                clientbound_token.span(),
                "Expected `Clientbound`",
            ));
        }
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let clientbound = content.parse()?;

        Ok(DeclareStatePackets {
            name,
            serverbound,
            clientbound,
        })
    }
}
#[proc_macro]
pub fn declare_state_packets(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeclareStatePackets);

    let name = input.name;

    let mut enum_contents = quote!();
    for PacketIdPair { id, module, name } in input.serverbound.packets {
        enum_contents.extend(quote! {#name(#module::#name)});
    }

    quote! {
        #[derive(Clone, Debug)]
        pub enum #name
        where
            Self: Sized,
        {
            #enum_contents
        }
    }
    .into()
}
