mod api;
mod cli;
mod mseed;
mod output;
mod samples;
mod scan;
mod seedlink;
mod table;
mod timespec;
mod tui;
mod waveforms;

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

        Commands::Waveforms {
            sensors,
            filter,
            start,
            end,
            duration,
            output,
            quality,
            minimum_length,
            longest_only,
            chunk,
        } => {
            waveforms::download(waveforms::Request {
                sensors: &sensors,
                filters: &filter,
                start: start.as_deref(),
                end: end.as_deref(),
                duration: duration.as_deref(),
                output: &output,
                quality,
                minimum_length,
                longest_only,
                chunk: chunk.as_deref(),
            })
            .await?;
        }

        Commands::Tui {
            sensor,
            window,
            port,
            server,
        } => {
            watch(&sensor, window, port, server).await?;
        }

        Commands::Action { action, sensor_uid } => {
            trigger_action(&action.to_string(), &sensor_uid).await?;
        }
    }
    Ok(())
}

/// How many records may queue up before the reader waits for the view. A
/// sensor sends a handful a second, so this is seconds of slack, not samples.
const UPDATE_QUEUE: usize = 256;

/// Open the live view on one sensor.
async fn watch(sensor: &str, window: f64, port: u16, server: String) -> Result<()> {
    let source = seedlink::Source::from_arg(sensor)?;

    // Logging goes to standard error, which is the same screen the view draws
    // on, so it is silenced for as long as the view is up.
    log::set_max_level(log::LevelFilter::Off);

    let (sender, receiver) = tokio::sync::mpsc::channel(UPDATE_QUEUE);
    let reader = tokio::spawn(seedlink::stream(source.clone(), server, port, sender));

    let mut terminal = ratatui::init();
    let outcome = tui::run(source, window, receiver, &mut terminal).await;
    ratatui::restore();

    reader.abort();
    log::set_max_level(log::LevelFilter::Info);
    outcome
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
