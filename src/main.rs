mod cli;
mod scan;

use crate::scan::scan;
use clap::Parser;

use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = cli::Cli::parse();
    let interface = args.interface;
    let results = scan(interface).await;
    present_results(results);
}

fn present_results(scan_results: Vec<(Ipv4Addr, Option<String>)>) {
    for (address, response) in scan_results {
        format_response(address, response);
    }
}

fn format_response(address: Ipv4Addr, response: Option<String>) {
    let message = match response {
        None => {
            format!("{}\t -", address)
        }
        Some(response) => {
            format!("{}\t{}", address, response.replace("\n", "\t"))
        }
    };
    println!("{}", message);
}
