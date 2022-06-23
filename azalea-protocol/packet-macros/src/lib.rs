use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Data, DeriveInput, FieldsNamed, Ident, LitInt, Token,
};

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

    let contents = quote! {
        impl #ident {
            pub fn get(self) -> #state {
                #state::#ident(self)
            }

            pub fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                crate::mc_buf::McBufWritable::write_into(self, buf)
            }

            pub fn read(
                buf: &mut impl std::io::Read,
            ) -> Result<#state, String> {
                use crate::mc_buf::McBufReadable;
                Ok(Self::read_into(buf)?.get())
            }
        }
    };

    contents.into()
}

#[proc_macro_derive(GamePacket, attributes(var))]
pub fn derive_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::GamePacket})
}

#[proc_macro_derive(HandshakePacket, attributes(var))]
pub fn derive_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::handshake::HandshakePacket})
}

#[proc_macro_derive(LoginPacket, attributes(var))]
pub fn derive_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::login::LoginPacket})
}

#[proc_macro_derive(StatusPacket, attributes(var))]
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

        // example:
        // 0x0e: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,

        // 0x0e
        while let Ok(packet_id) = input.parse::<LitInt>() {
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
        // let name_litstr = syn::LitStr::new(&name.to_string(), name.span());
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

            fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
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
