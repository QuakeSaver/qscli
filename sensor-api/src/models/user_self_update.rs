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

/// UserSelfUpdate : A schema with information a user can update on its own.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSelfUpdate {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "frontend_setting", skip_serializing_if = "Option::is_none")]
    pub frontend_setting: Option<Box<models::FrontendSettingsUpdate>>,
    #[serde(rename = "seedlink_ipv4_networks", skip_serializing_if = "Option::is_none")]
    pub seedlink_ipv4_networks: Option<Vec<String>>,
}

impl UserSelfUpdate {
    /// A schema with information a user can update on its own.
    pub fn new() -> UserSelfUpdate {
        UserSelfUpdate {
            email: None,
            password: None,
            name: None,
            frontend_setting: None,
            seedlink_ipv4_networks: None,
        }
    }
}

