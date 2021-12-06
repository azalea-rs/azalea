use std::net::IpAddr;
use std::net::TcpStream;
use std::str::FromStr;

use tokio::runtime::Runtime;

pub mod connection;
pub mod friendly_byte_buf;
pub mod packets;
pub mod resolver;
pub mod server_status_pinger;

#[derive(Debug)]
pub struct ServerAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct ServerIpAddress {
    pub ip: IpAddr,
    pub port: u16,
}

impl ServerAddress {
    /// Convert a Minecraft server address (host:port, the port is optional) to a ServerAddress
    pub fn parse(string: &String) -> Result<ServerAddress, String> {
        if string.is_empty() {
            return Err("Empty string".to_string());
        }
        let mut parts = string.split(':');
        let host = parts.next().ok_or("No host specified")?.to_string();
        // default the port to 25565
        let port = parts.next().unwrap_or("25565");
        let port = u16::from_str(port).map_err(|_| "Invalid port specified")?;
        Ok(ServerAddress { host, port })
    }
}

pub async fn connect(address: ServerAddress) -> Result<(), Box<dyn std::error::Error>> {
    let resolved_address = resolver::resolve_address(&address).await;
    println!("Resolved address: {:?}", resolved_address);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
