pub mod client_intention_packet;

#[derive(Clone, Debug)]
pub enum HandshakePacket {
    ClientIntentionPacket(client_intention_packet::ClientIntentionPacket),
}
