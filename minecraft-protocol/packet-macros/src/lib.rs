use casey::{pascal, snake};
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Signature,
};

// #[derive(Clone, Debug)]
// pub enum Packet {
//     // game

//     // handshake
//     ClientIntentionPacket(handshake::client_intention_packet::ClientIntentionPacket),

//     // login

//     // status
//     ServerboundStatusRequestPacket(
//         status::serverbound_status_request_packet::ServerboundStatusRequestPacket,
//     ),
//     ClientboundStatusResponsePacket(
//         status::clientbound_status_response_packet::ClientboundStatusResponsePacket,
//     ),
// }

// // TODO: do all this with macros so it's less repetitive
// impl Packet {
//     fn get_inner_packet(&self) -> &dyn PacketTrait {
//         match self {
//             Packet::ClientIntentionPacket(packet) => packet,
//             Packet::ServerboundStatusRequestPacket(packet) => packet,
//             Packet::ClientboundStatusResponsePacket(packet) => packet,
//         }
//     }

//     pub fn id(&self) -> u32 {
//         match self {
//             Packet::ClientIntentionPacket(_packet) => 0x00,
//             Packet::ServerboundStatusRequestPacket(_packet) => 0x00,
//             Packet::ClientboundStatusResponsePacket(_packet) => 0x00,
//         }
//     }

//     /// Read a packet by its id, ConnectionProtocol, and flow
//     pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
//         id: u32,
//         protocol: &ConnectionProtocol,
//         flow: &PacketFlow,
//         buf: &mut BufReader<T>,
//     ) -> Result<Packet, String> {
//         match protocol {
//             ConnectionProtocol::Handshake => match id {
//                 0x00 => Ok(
//                     handshake::client_intention_packet::ClientIntentionPacket::read(buf).await?,
//                 ),
//                 _ => Err(format!("Unknown packet id: {}", id)),
//             },
//             ConnectionProtocol::Game => Err("Game protocol not implemented yet".to_string()),
//             ConnectionProtocol::Status => match flow {
//                 PacketFlow::ServerToClient => match id {
//                     0x00 => Ok(
//                         status::clientbound_status_response_packet::ClientboundStatusResponsePacket
//                             ::read(buf)
//                             .await?,
//                     ),
//                     _ => Err(format!("Unknown packet id: {}", id)),
//                 },
//                 PacketFlow::ClientToServer => match id {
//                     0x00 => Ok(
//                         status::serverbound_status_request_packet::ServerboundStatusRequestPacket
//                             ::read(buf)
//                             .await?,
//                     ),
//                     _ => Err(format!("Unknown packet id: {}", id)),
//                 },
//             },
//             ConnectionProtocol::Login => Err("Login protocol not implemented yet".to_string()),
//         }
//     }

//     pub fn write(&self, buf: &mut Vec<u8>) {
//         self.get_inner_packet().write(buf);
//     }
// }

struct RegisterPacket {
    name: syn::Ident,
    connection_protocol: syn::Ident,
    flow: syn::Ident,
}

struct RegisterPackets {
    packets: Vec<RegisterPacket>,
}

impl Parse for RegisterPackets {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let packets = vec![];
        loop {
            let name: syn::Ident = input.parse()?;

            input.parse::<syn::Token![=>]>()?;

            let connection_protocol: syn::Ident = input.parse()?;
            input.parse::<syn::Token![,]>()?;
            let flow: syn::Ident = input.parse()?;

            input.parse::<syn::Token![;]>()?;

            packets.push(RegisterPacket {
                name,
                connection_protocol,
                flow,
            });

            if input.is_empty() {
                break;
            }
        }

        Ok(RegisterPackets { packets })
    }
}

#[proc_macro]
pub fn register_packets(input: TokenStream) -> TokenStream {
    let RegisterPackets { packets } = syn::parse_macro_input!(input as RegisterPackets);

    // ClientIntentionPacket(handshake::client_intention_packet::ClientIntentionPacket),
    let gen = quote! {
    // #[derive(Clone, Debug)]
    // pub enum Packet {
    //     // ClientIntentionPacket(handshake::client_intention_packet::ClientIntentionPacket),
    //     // ClientboundStatusResponsePacket(
    //     //     status::clientbound_status_response_packet::ClientboundStatusResponsePacket,
    //     // ),

    // }
    };
    gen.into()
}
