use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams};

/// Event object.
#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub r#type: String,
    pub pending_webhooks: Option<u32>,
    pub data: serde_json::Value,
}

/// Parameters for listing events.
#[derive(Debug, Default, Serialize)]
pub struct ListEventsParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
}

impl PayjpClient {
    /// Retrieve an event by ID.
    pub async fn retrieve_event(&self, id: &str) -> Result<Event> {
        self.get(&format!("/events/{id}")).await
    }

    /// List events.
    pub async fn list_events(&self, params: &ListEventsParams) -> Result<List<Event>> {
        self.get_with_query("/events", params).await
    }
}
