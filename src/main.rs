mod resolve;
mod response;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        eprintln!("usage: geoip [-h] DOMAIN|IP");
        std::process::exit(1);
    }

    if let Err(e) = resolve::run(&args[1]).await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
