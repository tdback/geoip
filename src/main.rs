use domain::base::name::UncertainName;
use domain::resolv::StubResolver;
use std::str::FromStr;

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
        println!("no addresses found");
    } else {
        for addr in addrs {
            println!("{addr}");
        }
    }

    Ok(())
}
