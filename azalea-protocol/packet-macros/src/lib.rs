use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Data, DeriveInput, FieldsNamed, Ident, LitInt, Token,
};

fn create_impl_mcbufreadable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let FieldsNamed { named, .. } = match fields {
                syn::Fields::Named(f) => f,
                _ => panic!("#[derive(*Packet)] can only be used on structs with named fields"),
            };

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
                                        let #field_name = crate::mc_buf::McBufVarintReadable::varint_read_into(buf)?;
                                    }
                                } else {
                                    quote! {
                                        let #field_name = crate::mc_buf::McBufReadable::read_into(buf)?;
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

            quote! {
            impl crate::mc_buf::McBufReadable for #ident {
                fn read_into(buf: &mut impl std::io::Read) -> Result<Self, String> {
                    #(#read_fields)*
                    Ok(#ident {
                        #(#read_field_names: #read_field_names),*
                    })
                }
            }
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut match_contents = quote!();
            for variant in variants {
                let variant_name = &variant.ident;
                let variant_discrim = &variant
                    .discriminant
                    .as_ref()
                    .expect("enum variant must have a discriminant")
                    .1;
                match_contents.extend(quote! {
                    #variant_discrim => Ok(Self::#variant_name),
                });
            }

            quote! {
            impl crate::mc_buf::McBufReadable for #ident {
                fn read_into(buf: &mut impl std::io::Read) -> Result<Self, String>
                {
                    let id = buf.read_varint()?;
                    match id {
                        #match_contents
                        _ => Err(format!("Unknown enum variant {}", id)),
                    }
                }
            }
            }
        }
        _ => panic!("#[derive(*Packet)] can only be used on structs"),
    }
}

fn create_impl_mcbufwritable(ident: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
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

            quote! {
                impl crate::mc_buf::McBufWritable for #ident {
                    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
                        #(#write_fields)*
                        Ok(())
                    }
                }
            }
        }
        syn::Data::Enum(syn::DataEnum { .. }) => {
            quote! {
                impl crate::mc_buf::McBufWritable for #ident {
                    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
                        crate::mc_buf::Writable::write_varint(buf, *self as i32)
                    }
                }
            }
        }
        _ => panic!("#[derive(*Packet)] can only be used on structs"),
    }
}

#[proc_macro_derive(McBufReadable, attributes(varint))]
pub fn derive_mcbufreadable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    create_impl_mcbufreadable(&ident, &data).into()
}

#[proc_macro_derive(McBufWritable, attributes(varint))]
pub fn derive_mcbufwritable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    create_impl_mcbufwritable(&ident, &data).into()
}

fn as_packet_derive(input: TokenStream, state: proc_macro2::TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match &data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("#[derive(*Packet)] can only be used on structs"),
    };
    let FieldsNamed { named: _, .. } = match fields {
        syn::Fields::Named(f) => f,
        _ => panic!("#[derive(*Packet)] can only be used on structs with named fields"),
    };

    let mcbufreadable_impl = create_impl_mcbufreadable(&ident, &data);
    let mcbufwritable_impl = create_impl_mcbufwritable(&ident, &data);

    let contents = quote! {
        impl #ident {
            pub fn get(self) -> #state {
                #state::#ident(self)
            }

            pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
                crate::mc_buf::McBufWritable::write_into(self, buf)
            }

            pub fn read(
                buf: &mut impl std::io::Read,
            ) -> Result<#state, String> {
                use crate::mc_buf::McBufReadable;
                Ok(Self::read_into(buf)?.get())
            }
        }

        #mcbufreadable_impl

        #mcbufwritable_impl
    };

    contents.into()
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

            packets.push(PacketIdPair {
                id: packet_id,
                module,
                name,
            });

            if input.parse::<Token![,]>().is_err() {
                break;
            }
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

    let state_name = input.name;
    let state_name_litstr = syn::LitStr::new(&state_name.to_string(), state_name.span());

    let mut enum_contents = quote!();
    let mut id_match_contents = quote!();
    let mut write_match_contents = quote!();
    let mut serverbound_read_match_contents = quote!();
    let mut clientbound_read_match_contents = quote!();
    for PacketIdPair { id, module, name } in input.serverbound.packets {
        enum_contents.extend(quote! {
            #name(#module::#name),
        });
        id_match_contents.extend(quote! {
            #state_name::#name(_packet) => #id,
        });
        write_match_contents.extend(quote! {
            #state_name::#name(packet) => packet.write(buf),
        });
        serverbound_read_match_contents.extend(quote! {
            #id => #module::#name::read(buf)?,
        });
    }
    for PacketIdPair { id, module, name } in input.clientbound.packets {
        enum_contents.extend(quote! {
            #name(#module::#name),
        });
        id_match_contents.extend(quote! {
            #state_name::#name(_packet) => #id,
        });
        write_match_contents.extend(quote! {
            #state_name::#name(packet) => packet.write(buf),
        });
        clientbound_read_match_contents.extend(quote! {
            #id => #module::#name::read(buf)?,
        });
    }

    quote! {
        #[derive(Clone, Debug)]
        pub enum #state_name
        where
            Self: Sized,
        {
            #enum_contents
        }

        impl crate::packets::ProtocolPacket for #state_name {
            fn id(&self) -> u32 {
                match self {
                    #id_match_contents
                }
            }

            fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
                match self {
                    #write_match_contents
                }
            }

            /// Read a packet by its id, ConnectionProtocol, and flow
            fn read(
                id: u32,
                flow: &crate::connect::PacketFlow,
                buf: &mut impl std::io::Read,
            ) -> Result<#state_name, String>
            where
                Self: Sized,
            {
                Ok(match flow {
                    crate::connect::PacketFlow::ServerToClient => match id {
                        #clientbound_read_match_contents
                        _ => return Err(format!("Unknown ServerToClient {} packet id: {}", #state_name_litstr, id)),
                    },
                    crate::connect::PacketFlow::ClientToServer => match id {
                        #serverbound_read_match_contents
                        _ => return Err(format!("Unknown ClientToServer {} packet id: {}", #state_name_litstr, id)),
                    },
                })
            }
        }
    }
    .into()
}
