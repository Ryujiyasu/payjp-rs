use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams};

/// Term (billing cycle) object.
#[derive(Debug, Deserialize)]
pub struct Term {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub charge_count: Option<u32>,
    pub refund_count: Option<u32>,
    pub dispute_count: Option<u32>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
}

/// Parameters for listing terms.
#[derive(Debug, Default, Serialize)]
pub struct ListTermsParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_start_at: Option<i64>,
}

impl PayjpClient {
    /// Retrieve a term by ID.
    pub async fn retrieve_term(&self, id: &str) -> Result<Term> {
        self.get(&format!("/terms/{id}")).await
    }

    /// List terms.
    pub async fn list_terms(&self, params: &ListTermsParams) -> Result<List<Term>> {
        self.get_with_query("/terms", params).await
    }
}
