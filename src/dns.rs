use std::net::IpAddr;
use trust_dns_resolver::Resolver;

pub fn resolve_hostname(hostname: &str) -> Result<IpAddr, String> {
    let resolver = Resolver::from_system_conf()
        .map_err(|err| format!("Error resolving '{}' : '{}'", hostname, err))?;

    let ips = resolver
        .lookup_ip(hostname)
        .map_err(|err| format!("Error resolving '{}' : '{}'", hostname, err))?;

    let ip = ips
        .iter()
        .next()
        .ok_or(format!("Error resolving '{}': No entries found", hostname))?;

    return Ok(ip);
}
