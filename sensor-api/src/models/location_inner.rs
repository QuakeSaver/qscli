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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationInner {
}

impl LocationInner {
    pub fn new() -> LocationInner {
        LocationInner {
        }
    }
}

