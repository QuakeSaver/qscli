/*
 * QuakeSaver Frontend API
 *
 * This implements an API for the Frontend to communicate with the Backend.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`search_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchEventsError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Get the requesters public IP.  Returns -------     str: The IP address of the requester.
pub async fn search_events(configuration: &configuration::Configuration, start_time: Option<String>, end_time: Option<String>, min_magnitude: Option<f64>, max_magnitude: Option<f64>, catalogue: Option<&str>) -> Result<serde_json::Value, Error<SearchEventsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/event/search", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("start_time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("end_time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_magnitude {
        local_var_req_builder = local_var_req_builder.query(&[("min_magnitude", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_magnitude {
        local_var_req_builder = local_var_req_builder.query(&[("max_magnitude", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = catalogue {
        local_var_req_builder = local_var_req_builder.query(&[("catalogue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchEventsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

