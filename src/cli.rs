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
    Sensors {
        /// Only list sensors matching these filters (repeat or comma-separate).
        ///
        /// Filters of the same kind widen the selection, filters of different
        /// kinds narrow it: `-f mems,hidra -f online` lists the online MEMS and
        /// HiDRA sensors. Without any filter every sensor is listed.
        #[arg(short, long, value_enum, value_delimiter = ',')]
        filter: Vec<SensorFilter>,
        /// Column to sort the list by
        #[arg(short, long, value_enum, default_value_t = SensorSort::Uid)]
        sort: SensorSort,
        /// Sort in descending order
        #[arg(short, long)]
        reverse: bool,
    },
    /// Send an action
    Action {
        #[clap(value_enum)]
        action: ActionOptions,
        /// Sensor UId
        sensor_uid: String,
    },
}

/// A selectable filter for the sensor list.
#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum SensorFilter {
    /// Seen within the last hour
    Online,
    /// Not seen within the last hour
    Offline,
    /// MEMS sensors (ADXL or BMA accelerometer)
    Mems,
    /// HiDRA sensors
    Hidra,
    /// Sensors with an unrecognised hardware revision
    Unknown,
    /// Sensors carrying at least one warning
    Warnings,
}

/// The property a filter selects on.
///
/// Filters sharing a facet are OR-ed together, separate facets are AND-ed, so
/// picking two hardware families widens the list while adding a status narrows
/// it.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Facet {
    Status,
    Family,
    Warnings,
}

/// Every facet a filter can belong to, for walking the AND-ed groups.
pub const FACETS: [Facet; 3] = [Facet::Status, Facet::Family, Facet::Warnings];

impl SensorFilter {
    pub fn facet(self) -> Facet {
        match self {
            SensorFilter::Online | SensorFilter::Offline => Facet::Status,
            SensorFilter::Mems | SensorFilter::Hidra | SensorFilter::Unknown => Facet::Family,
            SensorFilter::Warnings => Facet::Warnings,
        }
    }
}

/// A column the sensor list can be ordered by.
#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum SensorSort {
    /// Sensor UID
    Uid,
    /// How long ago the sensor was last seen, most recent first
    LastSeen,
    /// When the sensor was first seen, oldest first
    FirstSeen,
    /// Software version
    Version,
    /// Hardware family
    Type,
    /// Number of warnings, fewest first
    Warnings,
}

impl Display for SensorSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Spelled the way clap accepts it on the command line.
        let key = match self {
            SensorSort::Uid => "uid",
            SensorSort::LastSeen => "last-seen",
            SensorSort::FirstSeen => "first-seen",
            SensorSort::Version => "version",
            SensorSort::Type => "type",
            SensorSort::Warnings => "warnings",
        };
        f.write_str(key)
    }
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
