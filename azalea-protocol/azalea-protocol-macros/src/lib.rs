use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, DeriveInput, Ident, Token,
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
                azalea_buf::AzaleaWrite::azalea_write(self, buf)
            }

            pub fn read(
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#state, azalea_buf::BufReadError> {
                use azalea_buf::AzaleaRead;
                Ok(crate::packets::Packet::into_variant(Self::azalea_read(buf)?))
            }

        }

        impl crate::packets::Packet<#state> for #ident {
            fn into_variant(self) -> #state {
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
struct PacketList {
    packets: Vec<Ident>,
}

impl Parse for PacketList {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut packets = vec![];

        // example:
        // change_difficulty,
        // keep_alive,
        while let Ok(packet_name) = input.parse::<Ident>() {
            packets.push(packet_name);
            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        Ok(PacketList { packets })
    }
}

#[derive(Debug)]
struct DeclareStatePackets {
    name: Ident,
    clientbound: PacketList,
    serverbound: PacketList,
}

impl Parse for DeclareStatePackets {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
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
        bracketed!(content in input);
        let clientbound = content.parse()?;

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
        bracketed!(content in input);
        let serverbound = content.parse()?;

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

    let clientbound_state_name =
        Ident::new(&format!("Clientbound{}", input.name), input.name.span());
    let serverbound_state_name =
        Ident::new(&format!("Serverbound{}", input.name), input.name.span());

    let state_name_litstr = syn::LitStr::new(&input.name.to_string(), input.name.span());

    let has_clientbound_packets = !input.clientbound.packets.is_empty();
    let has_serverbound_packets = !input.serverbound.packets.is_empty();

    let mut mod_and_use_statements_contents = quote!();
    let mut clientbound_enum_contents = quote!();
    let mut serverbound_enum_contents = quote!();
    let mut clientbound_id_match_contents = quote!();
    let mut serverbound_id_match_contents = quote!();
    let mut clientbound_write_match_contents = quote!();
    let mut serverbound_write_match_contents = quote!();
    let mut clientbound_read_match_contents = quote!();
    let mut serverbound_read_match_contents = quote!();

    for (id, packet_name) in input.clientbound.packets.iter().enumerate() {
        let id = id as u32;

        let struct_name = packet_name_to_struct_name(packet_name, "clientbound");
        let module_name = packet_name_to_module_name(packet_name, "clientbound");
        let variant_name = packet_name_to_variant_name(packet_name);
        let packet_name_litstr = syn::LitStr::new(&packet_name.to_string(), packet_name.span());

        mod_and_use_statements_contents.extend(quote! {
            pub mod #module_name;
            pub use #module_name::#struct_name;
        });

        clientbound_enum_contents.extend(quote! {
            #variant_name(#module_name::#struct_name),
        });
        clientbound_id_match_contents.extend(quote! {
            #clientbound_state_name::#variant_name(_packet) => #id,
        });
        clientbound_write_match_contents.extend(quote! {
            #clientbound_state_name::#variant_name(packet) => packet.write(buf),
        });
        clientbound_read_match_contents.extend(quote! {
            #id => {
                let data = #module_name::#struct_name::read(buf).map_err(|e| crate::read::ReadPacketError::Parse {
                    source: e,
                    packet_id: #id,
                    backtrace: Box::new(std::backtrace::Backtrace::capture()),
                    packet_name: #packet_name_litstr.to_string(),
                })?;
                #[cfg(debug_assertions)]
                {
                    let mut leftover = Vec::new();
                    let _ = std::io::Read::read_to_end(buf, &mut leftover);
                    if !leftover.is_empty() {
                        return Err(
                            Box::new(
                                crate::read::ReadPacketError::LeftoverData {
                                    packet_name: #packet_name_litstr.to_string(),
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
    for (id, packet_name) in input.serverbound.packets.iter().enumerate() {
        let id = id as u32;

        let struct_name = packet_name_to_struct_name(packet_name, "serverbound");
        let module_name = packet_name_to_module_name(packet_name, "serverbound");
        let variant_name = packet_name_to_variant_name(packet_name);
        let packet_name_litstr = syn::LitStr::new(&packet_name.to_string(), packet_name.span());

        mod_and_use_statements_contents.extend(quote! {
            pub mod #module_name;
            pub use #module_name::#struct_name;
        });

        serverbound_enum_contents.extend(quote! {
            #variant_name(#module_name::#struct_name),
        });
        serverbound_id_match_contents.extend(quote! {
            #serverbound_state_name::#variant_name(_packet) => #id,
        });
        serverbound_write_match_contents.extend(quote! {
            #serverbound_state_name::#variant_name(packet) => packet.write(buf),
        });
        serverbound_read_match_contents.extend(quote! {
            #id => {
                let data = #module_name::#struct_name::read(buf).map_err(|e| crate::read::ReadPacketError::Parse {
                    source: e,
                    packet_id: #id,
                    backtrace: Box::new(std::backtrace::Backtrace::capture()),
                    packet_name: #packet_name_litstr.to_string(),
                })?;
                #[cfg(debug_assertions)]
                {
                    let mut leftover = Vec::new();
                    let _ = std::io::Read::read_to_end(buf, &mut leftover);
                    if !leftover.is_empty() {
                        return Err(Box::new(crate::read::ReadPacketError::LeftoverData { packet_name: #packet_name_litstr.to_string(), data: leftover }));
                    }
                }
                data
            },
        });
    }

    if !has_serverbound_packets {
        serverbound_id_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
        serverbound_write_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
    }
    if !has_clientbound_packets {
        clientbound_id_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
        clientbound_write_match_contents.extend(quote! {
            _ => unreachable!("This enum is empty and can't exist.")
        });
    }

    let mut contents = quote! {
        #mod_and_use_statements_contents

        #[derive(Clone, Debug)]
        pub enum #clientbound_state_name
        where
            Self: Sized,
        {
            #clientbound_enum_contents
        }
        #[derive(Clone, Debug)]
        pub enum #serverbound_state_name
        where
        Self: Sized,
        {
            #serverbound_enum_contents
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
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#serverbound_state_name, Box<crate::read::ReadPacketError>>
            where
                Self: Sized,
            {
                Ok(match id {
                    #serverbound_read_match_contents
                    _ => return Err(Box::new(crate::read::ReadPacketError::UnknownPacketId { state_name: #state_name_litstr.to_string(), id })),
                })
            }
        }

        impl crate::packets::Packet<#serverbound_state_name> for #serverbound_state_name {
            /// No-op, exists so you can pass a packet enum when a Packet<> is expected.
            fn into_variant(self) -> #serverbound_state_name {
                self
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
                buf: &mut std::io::Cursor<&[u8]>,
            ) -> Result<#clientbound_state_name, Box<crate::read::ReadPacketError>>
            where
                Self: Sized,
            {
                Ok(match id {
                    #clientbound_read_match_contents
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

fn packet_name_to_struct_name(name: &syn::Ident, direction: &str) -> syn::Ident {
    let struct_name_snake = format!("{direction}_{name}");
    let struct_name = to_camel_case(&struct_name_snake);
    syn::Ident::new(&struct_name, name.span())
}
fn packet_name_to_module_name(name: &syn::Ident, direction: &str) -> syn::Ident {
    let module_name_snake = format!("{}_{name}", direction.chars().next().unwrap());
    let module_name = to_snake_case(&module_name_snake);
    syn::Ident::new(&module_name, name.span())
}
fn packet_name_to_variant_name(name: &syn::Ident) -> syn::Ident {
    let variant_name = to_camel_case(&name.to_string());
    syn::Ident::new(&variant_name, name.span())
}

fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = true;
    for c in snake_case.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
            } else {
                camel_case.push(c);
            }
            capitalize_next = false;
        }
    }
    camel_case
}
fn to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    for c in camel_case.chars() {
        if c.is_ascii_uppercase() {
            snake_case.push('_');
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
