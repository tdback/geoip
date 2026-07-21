use crate::response::Response;

use std::net::IpAddr;
use tokio::net::lookup_host;

pub async fn run(arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addrs = lookup(arg).await?;
    if addrs.is_empty() {
        return Err(format!("no addresses found for {arg}").into());
    }

    for addr in addrs {
        let url = format!("http://ip-api.com/json/{addr}");
        let req = reqwest::get(url).await?.text().await?;
        let res: Response = serde_json::from_str(&req)?;
        println!("{res}");
    }

    Ok(())
}

/// Lookup a host using its domain name or IP address.
async fn lookup(query: &str) -> std::io::Result<Vec<IpAddr>> {
    Ok(lookup_host(format!("{query}:80"))
        .await?
        .map(|addr| addr.ip())
        .filter(IpAddr::is_ipv4)
        .collect())
}
