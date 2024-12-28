use clap::{Parser, Subcommand, ValueEnum};
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
        #[clap(value_enum)]
        action: ActionOptions,
        /// Sensor UId
        sensor_uid: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
/// An example option
pub enum ActionOptions {
    /// reboot the sensor
    Reboot,
    /// blink the LED
    Blink,
}
