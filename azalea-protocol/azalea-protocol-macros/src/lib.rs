use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, DeriveInput, Ident, LitInt, Token,
};

fn as_packet_derive(input: TokenStream, state: proc_macro2::TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = &data else {
        panic!("#[derive(*Packet)] can only be used on structs")
    };

    let (syn::Fields::Named(_) | syn::Fields::Unit) = fields else {
        panic!("#[derive(*Packet)] can only be used on structs with named fields")
    };
    let variant_name = variant_name_from(&ident);

    let contents = quote! {
        impl #ident {
            pub fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                azalea_buf::McBufWritable::write_into(self, buf)
            }

            pub fn read(
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#state, azalea_buf::BufReadError> {
                use azalea_buf::McBufReadable;
                Ok(Self::read_from(buf)?.into_variant())
            }

            /// Convert this packet into an variant for the enum of the state and direction.
            pub fn into_variant(self) -> #state {
                #state::#variant_name(self)
            }
        }
    };

    contents.into()
}

#[proc_macro_derive(ServerboundGamePacket, attributes(var))]
pub fn derive_s_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::ServerboundGamePacket})
}
#[proc_macro_derive(ServerboundHandshakePacket, attributes(var))]
pub fn derive_s_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::handshake::ServerboundHandshakePacket},
    )
}
#[proc_macro_derive(ServerboundLoginPacket, attributes(var))]
pub fn derive_s_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::login::ServerboundLoginPacket},
    )
}
#[proc_macro_derive(ServerboundStatusPacket, attributes(var))]
pub fn derive_s_status_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::status::ServerboundStatusPacket},
    )
}
#[proc_macro_derive(ServerboundConfigPacket, attributes(var))]
pub fn derive_s_config_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::config::ServerboundConfigPacket},
    )
}

#[proc_macro_derive(ClientboundGamePacket, attributes(var))]
pub fn derive_c_game_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(input, quote! {crate::packets::game::ClientboundGamePacket})
}
#[proc_macro_derive(ClientboundHandshakePacket, attributes(var))]
pub fn derive_c_handshake_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::handshake::ClientboundHandshakePacket},
    )
}
#[proc_macro_derive(ClientboundLoginPacket, attributes(var))]
pub fn derive_c_login_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::login::ClientboundLoginPacket},
    )
}
#[proc_macro_derive(ClientboundStatusPacket, attributes(var))]
pub fn derive_c_status_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::status::ClientboundStatusPacket},
    )
}
#[proc_macro_derive(ClientboundConfigPacket, attributes(var))]
pub fn derive_c_config_packet(input: TokenStream) -> TokenStream {
    as_packet_derive(
        input,
        quote! {crate::packets::config::ClientboundConfigPacket},
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
        // 0x0e: c_change_difficulty::ClientboundChangeDifficultyPacket,

        // 0x0e
        while let Ok(packet_id) = input.parse::<LitInt>() {
            let packet_id = packet_id.base10_parse::<u32>()?;
            // :
            input.parse::<Token![:]>()?;
            // c_change_difficulty_packet
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

        let s_token: Ident = input.parse()?;
        if s_token != "Serverbound" {
            return Err(syn::Error::new(s_token.span(), "Expected `Serverbound`"));
        }
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let serverbound = content.parse()?;

        input.parse::<Token![,]>()?;

        let c_token: Ident = input.parse()?;
        if c_token != "Clientbound" {
            return Err(syn::Error::new(c_token.span(), "Expected `Clientbound`"));
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

    let s_state_name = Ident::new(&format!("Serverbound{}", input.name), input.name.span());
    let c_state_name = Ident::new(&format!("Clientbound{}", input.name), input.name.span());

    let state_name_litstr = syn::LitStr::new(&input.name.to_string(), input.name.span());

    let has_s_packets = !input.serverbound.packets.is_empty();
    let has_c_packets = !input.clientbound.packets.is_empty();

    let mut s_enum_contents = quote!();
    let mut c_enum_contents = quote!();
    let mut s_id_match_contents = quote!();
    let mut c_id_match_contents = quote!();
    let mut s_write_match_contents = quote!();
    let mut c_write_match_contents = quote!();
    let mut s_read_match_contents = quote!();
    let mut c_read_match_contents = quote!();

    for PacketIdPair { id, module, name } in input.serverbound.packets {
        let variant_name = variant_name_from(&name);

        let name_litstr = syn::LitStr::new(&name.to_string(), name.span());
        s_enum_contents.extend(quote! {
            #variant_name(#module::#name),
        });
        s_id_match_contents.extend(quote! {
            #s_state_name::#variant_name(_packet) => #id,
        });
        s_write_match_contents.extend(quote! {
            #s_state_name::#variant_name(packet) => packet.write(buf),
        });
        s_read_match_contents.extend(quote! {
            #id => {
                let data = #module::#name::read(buf).map_err(|e| crate::read::ReadPacketError::Parse {
                    source: e,
                    packet_id: #id,
                    backtrace: Box::new(std::backtrace::Backtrace::capture()),
                    packet_name: #name_litstr.to_string(),
                })?;
                #[cfg(debug_assertions)]
                {
                    let mut leftover = Vec::new();
                    let _ = std::io::Read::read_to_end(buf, &mut leftover);
                    if !leftover.is_empty() {
                        return Err(Box::new(crate::read::ReadPacketError::LeftoverData { packet_name: #name_litstr.to_string(), data: leftover }));
                    }
                }
                data
            },
        });
    }
    for PacketIdPair { id, module, name } in input.clientbound.packets {
        let name_litstr = syn::LitStr::new(&name.to_string(), name.span());
        let variant_name = variant_name_from(&name);

        c_enum_contents.extend(quote! {
            #variant_name(#module::#name),
        });
        c_id_match_contents.extend(quote! {
            #c_state_name::#variant_name(_packet) => #id,
        });
        c_write_match_contents.extend(quote! {
            #c_state_name::#variant_name(packet) => packet.write(buf),
        });
        c_read_match_contents.extend(quote! {
            #id => {
                let data = #module::#name::read(buf).map_err(|e| crate::read::ReadPacketError::Parse {
                    source: e,
                    packet_id: #id,
                    backtrace: Box::new(std::backtrace::Backtrace::capture()),
                    packet_name: #name_litstr.to_string(),
                })?;
                #[cfg(debug_assertions)]
                {
                    let mut leftover = Vec::new();
                    let _ = std::io::Read::read_to_end(buf, &mut leftover);
                    if !leftover.is_empty() {
                        return Err(
                            Box::new(
                                crate::read::ReadPacketError::LeftoverData {
                                    packet_name: #name_litstr.to_string(),
                                    data: leftover
                                }
                            )
                        );
                    }
                }
                data
            },
        });
    }

    if !has_s_packets {
        s_id_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
        s_write_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
    }
    if !has_c_packets {
        c_id_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
        c_write_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
    }

    let mut contents = quote! {
        #[derive(Clone, Debug)]
        pub enum #s_state_name
        where
        Self: Sized,
        {
            #s_enum_contents
        }
        #[derive(Clone, Debug)]
        pub enum #c_state_name
        where
            Self: Sized,
        {
            #c_enum_contents
        }
    };

    contents.extend(quote! {
        #[allow(unreachable_code)]
        impl crate::packets::ProtocolPacket for #s_state_name {
            fn id(&self) -> u32 {
                match self {
                    #s_id_match_contents
                }
            }

            fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                match self {
                    #s_write_match_contents
                }
            }

            /// Read a packet by its id, ConnectionProtocol, and flow
            fn read(
                id: u32,
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#s_state_name, Box<crate::read::ReadPacketError>>
            where
                Self: Sized,
            {
                Ok(match id {
                    #s_read_match_contents
                    _ => return Err(Box::new(crate::read::ReadPacketError::UnknownPacketId { state_name: #state_name_litstr.to_string(), id })),
                })
            }
        }
    });

    contents.extend(quote! {
        #[allow(unreachable_code)]
        impl crate::packets::ProtocolPacket for #c_state_name {
            fn id(&self) -> u32 {
                match self {
                    #c_id_match_contents
                }
            }

            fn write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                match self {
                    #c_write_match_contents
                }
            }

            /// Read a packet by its id, ConnectionProtocol, and flow
            fn read(
                id: u32,
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#c_state_name, Box<crate::read::ReadPacketError>>
            where
                Self: Sized,
            {
                Ok(match id {
                    #c_read_match_contents
                    _ => return Err(Box::new(crate::read::ReadPacketError::UnknownPacketId { state_name: #state_name_litstr.to_string(), id })),
                })
            }
        }
    });

    contents.into()
}

fn variant_name_from(name: &syn::Ident) -> syn::Ident {
    // remove "<direction>Bound" from the start and "Packet" from the end
    let mut variant_name = name.to_string();
    if variant_name.starts_with("Clientbound") {
        variant_name = variant_name["Clientbound".len()..].to_string();
    } else if variant_name.starts_with("Serverbound") {
        variant_name = variant_name["Serverbound".len()..].to_string();
    }
    syn::Ident::new(&variant_name, name.span())
}
