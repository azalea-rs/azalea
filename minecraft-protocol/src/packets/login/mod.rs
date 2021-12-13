pub mod clientbound_hello_packet;
pub mod serverbound_hello_packet;

#[derive(Clone, Debug)]
pub enum LoginPacket {
    ServerboundHelloPacket(serverbound_hello_packet::ServerboundHelloPacket),
    ClientboundHelloPacket(clientbound_hello_packet::ClientboundHelloPacket),
}
