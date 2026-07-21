mod api;
mod cli;
mod scan;

use crate::scan::scan;
use clap::Parser;

use crate::api::{print_sensors, trigger_action};
use crate::cli::Commands;
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
            print_sensors().await?;
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
    // One (header, cell-values) pair per column, in display order.
    let columns: [(&str, Vec<String>); 4] = [
        (
            "IP ADDRESS",
            devices.iter().map(|d| d.address.to_string()).collect(),
        ),
        ("UUID", devices.iter().map(|d| d.uuid.clone()).collect()),
        ("VERSION", devices.iter().map(|d| d.version.clone()).collect()),
        (
            "TYPE",
            devices.iter().map(|d| d.device_type.clone()).collect(),
        ),
    ];

    let widths: Vec<usize> = columns
        .iter()
        .map(|(header, cells)| cells.iter().map(String::len).max().unwrap_or(0).max(header.len()))
        .collect();

    let print_row = |cells: &[&str]| {
        let row: Vec<String> = cells
            .iter()
            .zip(&widths)
            .map(|(cell, w)| format!("{:<width$}", cell, width = w))
            .collect();
        println!("{}", row.join("  "));
    };

    print_row(&columns.iter().map(|(h, _)| *h).collect::<Vec<_>>());
    for i in 0..devices.len() {
        let row: Vec<&str> = columns.iter().map(|(_, cells)| cells[i].as_str()).collect();
        print_row(&row);
    }
}
