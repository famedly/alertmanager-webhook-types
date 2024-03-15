use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub alerts: Vec<Alert>,
    pub group_labels: HashMap<String, String>,
    pub common_labels: HashMap<String, String>,
    pub common_annotations: HashMap<String, String>,
    #[serde(rename = "externalURL")]
    pub external_url: String,
    pub group_key: String,
    pub truncated_alerts: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub status: Status,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
    #[serde(rename = "generatorURL")]
    pub generator_url: String,
    pub fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Firing,
    Resolved,
}
