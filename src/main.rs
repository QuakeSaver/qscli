mod api;
mod cli;
mod scan;
mod table;

use crate::scan::scan;
use clap::Parser;

use crate::api::{print_sensors, trigger_action};
use crate::cli::Commands;
use crate::table::print_table;
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
        Commands::Sensors {
            filter,
            sort,
            reverse,
        } => {
            print_sensors(&filter, sort, reverse).await?;
        }

        Commands::Action { action, sensor_uid } => {
            trigger_action(&action.to_string(), &sensor_uid).await?;
        }
    }
    Ok(())
}

struct Device {
    address: Ipv4Addr,
    uuid: String,
    version: String,
    device_type: String,
}

fn present_results(scan_results: Vec<(Ipv4Addr, String)>) {
    if scan_results.is_empty() {
        println!("no devices detected");
        return;
    }

    let mut devices: Vec<Device> = scan_results
        .iter()
        .map(|(address, body)| parse_device(*address, body))
        .collect();
    devices.sort_by_key(|d| d.address);

    print_device_table(&devices);
}

fn parse_device(address: Ipv4Addr, body: &str) -> Device {
    let json: serde_json::Value = serde_json::from_str(body).unwrap_or(serde_json::Value::Null);
    let field = |key: &str| {
        json.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };
    Device {
        address,
        // The sensor sets its hostname to the device UID (qs-set-hostname).
        uuid: field("hostname"),
        version: field("ringnes"),
        device_type: device_type_from_machine(json.get("machine").and_then(|v| v.as_str())),
    }
}

/// Map the sensor's `machine` string to its product name.
fn device_type_from_machine(machine: Option<&str>) -> String {
    match machine {
        Some(m) if m.starts_with("orange-pi") => "MEMS".to_string(),
        Some(m) if m.starts_with("raspberrypi") => "HiDRA".to_string(),
        Some(m) => m.to_string(),
        None => "unknown".to_string(),
    }
}

fn print_device_table(devices: &[Device]) {
    let rows: Vec<Vec<String>> = devices
        .iter()
        .map(|d| {
            vec![
                d.address.to_string(),
                d.uuid.clone(),
                d.version.clone(),
                d.device_type.clone(),
            ]
        })
        .collect();

    print_table(&["IP ADDRESS", "UUID", "VERSION", "TYPE"], &rows);
}
