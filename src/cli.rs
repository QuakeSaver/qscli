use clap::{Parser, Subcommand};

/// Scan for QuakeSaver devices
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    /// The network interface to scan (defaults to primary)
    pub(crate) command: Commands,
}

/// Scan for QuakeSaver devices
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The network interface to scan (defaults to primary)
    Detect { interface: Option<String> },
    /// Get sensors
    Sensors,
}
