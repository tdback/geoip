mod response;

use domain::base::name::UncertainName;
use domain::resolv::StubResolver;
use std::str::FromStr;

const API: &str = "http://ip-api.com/json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolver = StubResolver::new();
    let name: UncertainName<Vec<u8>> = UncertainName::from_str("google.com")?;

    let answer = match name {
        UncertainName::Absolute(name) => resolver.lookup_host(name).await?,
        UncertainName::Relative(name) => resolver.search_host(name).await?,
    };

    let addrs: Vec<_> = answer.iter().filter(|addr| addr.is_ipv4()).collect();
    if addrs.is_empty() {
        eprintln!("no addresses found for {}", answer.canonical_name());
        std::process::exit(1);
    }

    for addr in addrs {
        let url = format!("{}/{}", API, addr);
        let req = reqwest::get(url).await?.text().await?;
        let res: response::Response = serde_json::from_str(&req)?;
        println!("{res}");
    }

    Ok(())
}
