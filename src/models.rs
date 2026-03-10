//! Shared types used across multiple API modules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generic list response from PAY.JP API.
#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub object: String,
    pub count: u32,
    pub has_more: bool,
    pub url: String,
    pub data: Vec<T>,
}

/// Common pagination parameters.
#[derive(Debug, Default, Serialize)]
pub struct ListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
}

/// Metadata key-value pairs.
pub type Metadata = HashMap<String, String>;

/// Deleted object response.
#[derive(Debug, Deserialize)]
pub struct Deleted {
    pub id: String,
    pub deleted: bool,
    pub livemode: bool,
}

/// Card object.
#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub name: Option<String>,
    pub last4: String,
    pub exp_month: u32,
    pub exp_year: u32,
    pub brand: String,
    pub cvc_check: Option<String>,
    pub fingerprint: Option<String>,
    pub country: Option<String>,
    pub address_state: Option<String>,
    pub address_city: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub three_d_secure_status: Option<String>,
    pub livemode: bool,
    pub metadata: Option<Metadata>,
}
