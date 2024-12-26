use chrono::prelude::*;
use chrono::{Duration, NaiveDateTime, TimeDelta};

use sensor_api::apis::configuration::Configuration;
use sensor_api::apis::users_api::{get_access_token_by_login, get_sensors};
use sensor_api::models::Sensor;
use std::fmt;
use std::str::FromStr;
use log::warn;

const BASE_URL: &str = "https://api.network.quakesaver.net";
const OFFLINE_THRESHOLD: TimeDelta = Duration::hours(1);

struct StateDisconnected {}

struct StateConnected {
    token: String,
}

struct SMIQClient<S> {
    state: S,
}

impl SMIQClient<StateDisconnected> {
    fn new() -> Self {
        SMIQClient::<StateDisconnected> {
            state: StateDisconnected {},
        }
    }

    async fn authenticate(self) -> SMIQClient<StateConnected> {
        let _token = get_auth_token().await.unwrap();
        let state = StateConnected { token: _token };
        SMIQClient::<StateConnected> { state }
    }
}

impl SMIQClient<StateConnected> {
    fn get_token(&self) -> &str {
        &self.state.token
    }

    fn api_configuration(self) -> Configuration {
        Configuration {
            base_path: BASE_URL.to_string(),
            oauth_access_token: Some(self.get_token().to_string()),
            ..Default::default()
        }
    }
}

struct PrettyDuration(Duration);

impl fmt::Display for PrettyDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.0.num_seconds();
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = (total_seconds / 3600) % 24;
        let days = total_seconds / 86400;

        if days > 9 {
            write!(f, "{}d", days)
        } else if days > 0 {
            write!(f, "{}d {}h", days, hours)
        } else if hours > 0 {
            write!(f, "{}h {}m", hours, minutes)
        } else if minutes > 0 {
            write!(f, "{}m {}s", minutes, seconds)
        } else {
            write!(f, "{}s", seconds)
        }
    }
}

pub(crate) async fn print_sensors() -> Result<(), Box<dyn std::error::Error>> {
    let client = SMIQClient::new();
    let connected_client = client.authenticate().await;
    let sensors = get_sensors_internal(connected_client.api_configuration()).await?;
    sensors.iter().for_each(present_sensor);
    Ok(())
}

fn present_sensor(sensor: &Sensor) {
    let last_updated = NaiveDateTime::from_str(&sensor.last_updated).unwrap();
    let new = Local::now().naive_utc();
    let last_seen = new - last_updated;
    if last_seen > OFFLINE_THRESHOLD {
        return;
    }
    let sensor_icon = match &*sensor.hardware_revision {
        "OPI0_ADXL_1.0" => "▣",
        "RPI4_HIDRA_1.0" => "🌀",
        "RPI0_BMA_0.6" => "🌋",
        _ => "?",
    };

    println!(
        "{}\t{}\t{}\t{}",
        sensor_icon,
        sensor.uid,
        sensor.software_version,
        PrettyDuration(last_seen)
    );
}

async fn get_auth_token() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to read .env file");
    let username = std::env::var("SEISMIQ_USERNAME").expect("AUTH_TOKEN not set");
    let password = std::env::var("SEISMIQ_PASSWORD").expect("AUTH_TOKEN not set");
    let configuration = Configuration {
        base_path: BASE_URL.to_string(),
        ..Default::default()
    };
    let token =
        get_access_token_by_login(&configuration, &username, &password, None, None, None, None)
            .await?;
    Ok(token.access_token)
}

async fn get_sensors_internal(
    configuration: Configuration,
) -> Result<Vec<Sensor>, Box<dyn std::error::Error>> {
    let response = get_sensors(&configuration, None, Some(1000), None).await?;
    let sensors: Vec<Sensor> = response.sensors.into_values().collect();
    if sensors.len() == 1000 {
        warn!("hit sensor request limit");
    }
    Ok(sensors)
}
