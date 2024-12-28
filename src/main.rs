mod api;
mod cli;
mod scan;

use crate::scan::scan;
use clap::Parser;

use crate::api::{print_sensors, trigger_action};
use crate::cli::{ActionOptions, Commands};
use eyre::Result;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = cli::Cli::parse();
    match args.command {
        Commands::Detect { interface } => {
            let results = scan(interface).await;
            present_results(results);
        }
        Commands::Sensors => {
            print_sensors().await.expect("TODO: panic message");
        }

        Commands::Action { action, sensor_uid } => {
            let action = match action {
                ActionOptions::Reboot => "RebootTrigger",
                ActionOptions::Blink => "LEDPagerTrigger",
            };
            trigger_action(&action, &sensor_uid).await?;
        }
    }
    Ok(())
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
            let json: serde_json::Value =
                serde_json::from_str(&response).expect("JSON was not well-formatted");
            format!(
                "{}\t{} ",
                address,
                serde_json::to_string_pretty(&json).unwrap()
            )
        }
    };
    println!("{}", message);
}
