use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, DeriveInput, FieldsNamed, Ident, LitInt, Token,
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
                azalea_buf::McBufWritable::write_into(self, buf)
            }

            pub fn read(
                buf: &mut impl std::io::Read,
            ) -> Result<#state, String> {
                use azalea_buf::McBufReadable;
                Ok(Self::read_from(buf)?.get())
            }
        }
    };

    contents.into()
}

#[proc_macro_derive(ServerboundGamePacket, attributes(var))]
pub fn derive_serverbound_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::ServerboundGamePacket})
}
#[proc_macro_derive(ServerboundHandshakePacket, attributes(var))]
pub fn derive_serverbound_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::handshake::ServerboundHandshakePacket},
    )
}
#[proc_macro_derive(ServerboundLoginPacket, attributes(var))]
pub fn derive_serverbound_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::login::ServerboundLoginPacket},
    )
}
#[proc_macro_derive(ServerboundStatusPacket, attributes(var))]
pub fn derive_serverbound_status_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::status::ServerboundStatusPacket},
    )
}

#[proc_macro_derive(ClientboundGamePacket, attributes(var))]
pub fn derive_clientbound_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::ClientboundGamePacket})
}
#[proc_macro_derive(ClientboundHandshakePacket, attributes(var))]
pub fn derive_clientbound_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::handshake::ClientboundHandshakePacket},
    )
}
#[proc_macro_derive(ClientboundLoginPacket, attributes(var))]
pub fn derive_clientbound_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::login::ClientboundLoginPacket},
    )
}
#[proc_macro_derive(ClientboundStatusPacket, attributes(var))]
pub fn derive_clientbound_status_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::status::ClientboundStatusPacket},
    )
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

    let serverbound_state_name =
        Ident::new(&format!("Serverbound{}", input.name), input.name.span());
    let clientbound_state_name =
        Ident::new(&format!("Clientbound{}", input.name), input.name.span());

    let state_name_litstr = syn::LitStr::new(&input.name.to_string(), input.name.span());

    let has_serverbound_packets = !input.serverbound.packets.is_empty();
    let has_clientbound_packets = !input.clientbound.packets.is_empty();

    let mut serverbound_enum_contents = quote!();
    let mut clientbound_enum_contents = quote!();
    let mut serverbound_id_match_contents = quote!();
    let mut clientbound_id_match_contents = quote!();
    let mut serverbound_write_match_contents = quote!();
    let mut clientbound_write_match_contents = quote!();
    let mut serverbound_read_match_contents = quote!();
    let mut clientbound_read_match_contents = quote!();

    for PacketIdPair { id, module, name } in input.serverbound.packets {
        serverbound_enum_contents.extend(quote! {
            #name(#module::#name),
        });
        serverbound_id_match_contents.extend(quote! {
            #serverbound_state_name::#name(_packet) => #id,
        });
        serverbound_write_match_contents.extend(quote! {
            #serverbound_state_name::#name(packet) => packet.write(buf),
        });
        serverbound_read_match_contents.extend(quote! {
            #id => #module::#name::read(buf)?,
        });
    }
    for PacketIdPair { id, module, name } in input.clientbound.packets {
        // let name_litstr = syn::LitStr::new(&name.to_string(), name.span());
        clientbound_enum_contents.extend(quote! {
            #name(#module::#name),
        });
        clientbound_id_match_contents.extend(quote! {
            #clientbound_state_name::#name(_packet) => #id,
        });
        clientbound_write_match_contents.extend(quote! {
            #clientbound_state_name::#name(packet) => packet.write(buf),
        });
        clientbound_read_match_contents.extend(quote! {
            #id => #module::#name::read(buf)?,
        });
    }

    if !has_serverbound_packets {
        serverbound_id_match_contents.extend(quote! {
            _ => panic!("This enum is empty and can't exist.")
        });
        serverbound_write_match_contents.extend(quote! {
            _ => panic!("This enum is empty and can't exist.")
        });
    }
    if !has_clientbound_packets {
        clientbound_id_match_contents.extend(quote! {
            _ => panic!("This enum is empty and can't exist.")
        });
        clientbound_write_match_contents.extend(quote! {
            _ => panic!("This enum is empty and can't exist.")
        });
    }

    let mut contents = quote! {
        #[derive(Clone, Debug)]
        pub enum #serverbound_state_name
        where
        Self: Sized,
        {
            #serverbound_enum_contents
        }
        #[derive(Clone, Debug)]
        pub enum #clientbound_state_name
        where
            Self: Sized,
        {
            #clientbound_enum_contents
        }
    };

    contents.extend(quote! {
        #[allow(unreachable_code)]
        impl crate::packets::ProtocolPacket for #serverbound_state_name {
            fn id(&self) -> u32 {
                match self {
                    #serverbound_id_match_contents
                }
            }

            fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                match self {
                    #serverbound_write_match_contents
                }
            }

            /// Read a packet by its id, ConnectionProtocol, and flow
            fn read(
                id: u32,
                buf: &mut impl std::io::Read,
            ) -> Result<#serverbound_state_name, String>
            where
                Self: Sized,
            {
                Ok(match id {
                    #serverbound_read_match_contents
                    _ => return Err(format!("Unknown Serverbound {} packet id: {}", #state_name_litstr, id)),
                })
            }
        }
    });

    contents.extend(quote! {
        #[allow(unreachable_code)]
        impl crate::packets::ProtocolPacket for #clientbound_state_name {
            fn id(&self) -> u32 {
                match self {
                    #clientbound_id_match_contents
                }
            }

            fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                match self {
                    #clientbound_write_match_contents
                }
            }

            /// Read a packet by its id, ConnectionProtocol, and flow
            fn read(
                id: u32,
                buf: &mut impl std::io::Read,
            ) -> Result<#clientbound_state_name, String>
            where
                Self: Sized,
            {
                Ok(match id {
                    #clientbound_read_match_contents
                    _ => return Err(format!("Unknown Clientbound {} packet id: {}", #state_name_litstr, id)),
                })
            }
        }
    });

    contents.into()
}
