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

/// GroupMember : A User schema to return via the API as member of a group.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMember {
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl GroupMember {
    /// A User schema to return via the API as member of a group.
    pub fn new(uid: String, username: String, email: String, name: String) -> GroupMember {
        GroupMember {
            uid,
            username,
            email,
            name,
        }
    }
}

