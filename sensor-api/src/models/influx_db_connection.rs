/*
 * QuakeSaver Frontend API
 *
 * This implements an API for the Frontend to communicate with the Backend.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InfluxDbConnection : A InfluxDB Connection schema to return via the API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfluxDbConnection {
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "data_expiration_after_s")]
    pub data_expiration_after_s: i32,
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    #[serde(rename = "org")]
    pub org: String,
}

impl InfluxDbConnection {
    /// A InfluxDB Connection schema to return via the API.
    pub fn new(protocol: String, hostname: String, port: i32, data_expiration_after_s: i32, bucket_name: String, org: String) -> InfluxDbConnection {
        InfluxDbConnection {
            protocol,
            hostname,
            port,
            data_expiration_after_s,
            bucket_name,
            org,
        }
    }
}

