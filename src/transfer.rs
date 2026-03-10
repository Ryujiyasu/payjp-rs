use serde::{Deserialize, Serialize};

use crate::charge::Charge;
use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams};

/// Transfer object.
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub charges: Option<List<Charge>>,
    pub scheduled_date: Option<String>,
    pub summary: Option<TransferSummary>,
    pub description: Option<String>,
    pub term_id: Option<String>,
}

/// Transfer summary.
#[derive(Debug, Deserialize)]
pub struct TransferSummary {
    pub charge_count: Option<u32>,
    pub charge_fee: Option<i64>,
    pub charge_gross: Option<i64>,
    pub net: Option<i64>,
    pub refund_amount: Option<i64>,
    pub refund_count: Option<u32>,
}

/// Parameters for listing transfers.
#[derive(Debug, Default, Serialize)]
pub struct ListTransfersParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PayjpClient {
    /// Retrieve a transfer by ID.
    pub async fn retrieve_transfer(&self, id: &str) -> Result<Transfer> {
        self.get(&format!("/transfers/{id}")).await
    }

    /// List transfers.
    pub async fn list_transfers(&self, params: &ListTransfersParams) -> Result<List<Transfer>> {
        self.get_with_query("/transfers", params).await
    }
}
