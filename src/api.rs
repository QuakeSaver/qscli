use chrono::prelude::*;
use chrono::{Duration, NaiveDateTime, TimeDelta};

use eyre::eyre;
use reqwest::Client;
use sensor_api::models::Sensor;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const BASE_URL: &str = "https://api.network.quakesaver.net/api/v1";
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

    // Get sensor IDs
    let sensors = get_sensors(connected_client.get_token()).await?;
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
    println!(
        "{}\t{}\t{}",
        sensor.uid,
        sensor.software_version,
        PrettyDuration(last_seen)
    );
}

async fn get_auth_token() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to read .env file");
    let username = std::env::var("SEISMIQ_USERNAME").expect("AUTH_TOKEN not set");
    let password = std::env::var("SEISMIQ_PASSWORD").expect("AUTH_TOKEN not set");
    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("username", username);
    data.insert("password", password);

    let response = client
        .post(format!("{}/user/get_token", BASE_URL))
        .form(&data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        let token_type = json["token_type"].as_str().ok_or("Missing token_type")?;
        let access_token = json["access_token"]
            .as_str()
            .ok_or("Missing access_token")?;
        Ok(format!("{} {}", token_type, access_token))
    } else {
        Err(eyre!("Failed to get auth token: {}", response.text().await?).into())
    }
}

async fn get_sensors(auth_token: &str) -> Result<Vec<Sensor>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get(format!("{}/user/me/sensors_full", BASE_URL))
        .query(&[("limit", 1000)])
        .header("Authorization", auth_token)
        .send()
        .await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        let sensors = &json["sensors"];
        if let Value::Object(map) = sensors {
            let sensors = map
                .iter()
                .map(|(_, v)| serde_json::from_value(v.clone()).unwrap())
                .collect::<Vec<Sensor>>();
            Ok(sensors)
        } else {
            Err(eyre!("Failed to parse sensors: {}", sensors).into())
        }
    } else {
        Err(eyre!("Failed to get sensor IDs: {}", response.text().await?).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let client = SMIQClient::new();
    }

    #[tokio::test]
    async fn test_get_auth_token() {
        let base_domain = "network.quakesaver.net";
        let api_base_url = format!("https://api.{}/api/v1", base_domain);
        let auth_token = get_auth_token().await.unwrap();
        assert_eq!(auth_token, "g.balaskas");
    }
}
