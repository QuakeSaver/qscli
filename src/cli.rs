use clap::{Args, Parser, Subcommand};

/// Scan for QuakeSaver devices
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The network interface to scan (defaults to primary)
    #[command(subcommand)]
    pub(crate) command: Commands,
}

/// Scan for QuakeSaver devices
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The network interface to scan (defaults to primary)
    Detect { interface: Option<String> },
    /// Get sensors
    Sensors,
    /// Send an action
    Action {
        /// Sensor action
        action: String,
        /// Sensor UId
        sensor_uid: String,
    },
}

#[derive(Debug, Args)]
/// An example option
pub struct ActionOptions {
    /// reboot
    #[arg(short, long)]
    reboot: String,
}
