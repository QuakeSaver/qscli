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

/// BrokerConfig : This class contains information about token.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerConfig {
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "gzip_compression", skip_serializing_if = "Option::is_none")]
    pub gzip_compression: Option<bool>,
}

impl BrokerConfig {
    /// This class contains information about token.
    pub fn new(hostname: String, port: i32, protocol: Protocol, token: String) -> BrokerConfig {
        BrokerConfig {
            hostname,
            port,
            protocol,
            token,
            gzip_compression: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "mqtts")]
    Mqtts,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Mqtt
    }
}

