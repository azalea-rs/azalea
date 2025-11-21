//! Resolve a Minecraft server address into an IP address and port.

use std::{
    net::{IpAddr, SocketAddr},
    sync::LazyLock,
};

pub use hickory_resolver::ResolveError;
use hickory_resolver::{Name, TokioResolver, name_server::TokioConnectionProvider};

use crate::ServerAddress;

#[deprecated(note = "Renamed to ResolveError")]
pub type ResolverError = ResolveError;

static RESOLVER: LazyLock<TokioResolver> = LazyLock::new(|| {
    TokioResolver::builder(TokioConnectionProvider::default())
        .unwrap()
        .build()
});

/// Resolve a Minecraft server address into an IP address and port.
///
/// If it's already an IP address, it's returned as-is.
pub async fn resolve_address(mut address: &ServerAddress) -> Result<SocketAddr, ResolveError> {
    let redirect = resolve_srv_redirect(address).await;
    if let Ok(redirect_target) = &redirect {
        address = redirect_target;
    }

    resolve_ip_without_redirects(address).await
}

async fn resolve_ip_without_redirects(address: &ServerAddress) -> Result<SocketAddr, ResolveError> {
    if let Ok(ip) = address.host.parse::<IpAddr>() {
        // no need to do a lookup
        return Ok(SocketAddr::new(ip, address.port));
    }

    let name = Name::from_ascii(&address.host)?;
    let lookup_ip = RESOLVER.lookup_ip(name).await?;

    let ip = lookup_ip
        .iter()
        .next()
        .ok_or(hickory_resolver::ResolveError::from(
            "No A/AAAA record found",
        ))?;

    Ok(SocketAddr::new(ip, address.port))
}

async fn resolve_srv_redirect(address: &ServerAddress) -> Result<ServerAddress, ResolveError> {
    if address.port != 25565 {
        return Err(ResolveError::from("Port must be 25565 to do a SRV lookup"));
    }

    let query = format!("_minecraft._tcp.{}", address.host);
    let res = RESOLVER.srv_lookup(query).await?;

    let srv = res
        .iter()
        .next()
        .ok_or(ResolveError::from("No SRV record found"))?;
    Ok(ServerAddress {
        host: srv.target().to_ascii(),
        port: srv.port(),
    })
}
