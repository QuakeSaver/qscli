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

/// InfluxData : A schema to describe influx data points.  The attribute naming is chosen for compatibility reasons with the frontend lib (ApexCharts).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfluxData {
    #[serde(rename = "times")]
    pub times: Vec<String>,
    #[serde(rename = "values")]
    pub values: Vec<f64>,
}

impl InfluxData {
    /// A schema to describe influx data points.  The attribute naming is chosen for compatibility reasons with the frontend lib (ApexCharts).
    pub fn new(times: Vec<String>, values: Vec<f64>) -> InfluxData {
        InfluxData {
            times,
            values,
        }
    }
}

