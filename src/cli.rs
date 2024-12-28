use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::Display;
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
    /// Check for an update
    CheckUpdate,
}

impl Display for ActionOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            ActionOptions::Reboot => "RebootTrigger",
            ActionOptions::Blink => "LEDPagerTrigger",
            ActionOptions::CheckUpdate => "MenderCheckUpdate",
        };
        f.write_str(action)
    }
}
