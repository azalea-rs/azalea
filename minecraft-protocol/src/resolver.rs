use std::net::IpAddr;

use crate::{ServerAddress, ServerIpAddress};
use async_recursion::async_recursion;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

/// Resolve a Minecraft server address into an IP address and port.
/// If it's already an IP address, it's returned as-is.
#[async_recursion]
pub async fn resolve_address(address: &ServerAddress) -> Result<ServerIpAddress, String> {
    // If the address.host is already in the format of an ip address, return it.
    if let Ok(ip) = address.host.parse::<IpAddr>() {
        return Ok(ServerIpAddress {
            ip: ip,
            port: address.port,
        });
    }

    // we specify Cloudflare instead of the default resolver because trust_dns_resolver has an issue on Windows where it's really slow using the default resolver
    let resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default()).unwrap();

    // first, we do a srv lookup for _minecraft._tcp.<host>
    let srv_redirect_result = resolver
        .srv_lookup(format!("_minecraft._tcp.{}", address.host).as_str())
        .await;

    // if it resolves that means it's a redirect so we call resolve_address again with the new host
    if srv_redirect_result.is_ok() {
        let redirect_result = srv_redirect_result.unwrap();
        let redirect_srv = redirect_result
            .iter()
            .next()
            .ok_or_else(|| "No SRV record found".to_string())?;
        let redirect_address = ServerAddress {
            host: redirect_srv.target().to_utf8(),
            port: redirect_srv.port(),
        };

        println!("redirecting to {:?}", redirect_address);

        return resolve_address(&redirect_address).await;
    }

    // there's no redirect, try to resolve this as an ip address
    let lookup_ip_result = resolver.lookup_ip(address.host.clone()).await;
    let lookup_ip = lookup_ip_result.map_err(|_| "No IP found".to_string())?;

    Ok(ServerIpAddress {
        ip: lookup_ip.iter().next().unwrap(),
        port: address.port,
    })
}
