//! Resolve IPs from hostnames.

use std::net::{IpAddr, SocketAddr};

use async_recursion::async_recursion;
use thiserror::Error;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Name, TokioAsyncResolver,
};

use crate::ServerAddress;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("No SRV record found")]
    NoSrvRecord,
    #[error("No IP found")]
    NoIp,
}

/// Resolve a Minecraft server address into an IP address and port.
/// If it's already an IP address, it's returned as-is.
#[must_use]
#[async_recursion]
pub async fn resolve_address(address: &ServerAddress) -> Result<SocketAddr, ResolverError> {
    // If the address.host is already in the format of an ip address, return it.
    if let Ok(ip) = address.host.parse::<IpAddr>() {
        return Ok(SocketAddr::new(ip, address.port));
    }

    // we specify Cloudflare instead of the default resolver because
    // trust_dns_resolver has an issue on Windows where it's really slow using the
    // default resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());

    // first, we do a srv lookup for _minecraft._tcp.<host>
    let srv_redirect_result = resolver
        .srv_lookup(format!("_minecraft._tcp.{}", address.host).as_str())
        .await;

    // if it resolves that means it's a redirect so we call resolve_address again
    // with the new host
    if let Ok(redirect_result) = srv_redirect_result {
        let redirect_srv = redirect_result
            .iter()
            .next()
            .ok_or(ResolverError::NoSrvRecord)?;
        let redirect_address = ServerAddress {
            host: redirect_srv.target().to_ascii(),
            port: redirect_srv.port(),
        };

        if redirect_address.host == address.host {
            let lookup_ip_result = resolver.lookup_ip(redirect_address.host).await;
            let lookup_ip = lookup_ip_result.map_err(|_| ResolverError::NoIp)?;
            return Ok(SocketAddr::new(
                lookup_ip.iter().next().unwrap(),
                redirect_address.port,
            ));
        }

        return resolve_address(&redirect_address).await;
    }

    // there's no redirect, try to resolve this as an ip address
    let name = Name::from_ascii(&address.host).map_err(|_| ResolverError::NoIp)?;
    let lookup_ip_result = resolver.lookup_ip(name).await;
    let lookup_ip = lookup_ip_result.map_err(|_| ResolverError::NoIp)?;

    Ok(SocketAddr::new(
        lookup_ip.iter().next().unwrap(),
        address.port,
    ))
}
