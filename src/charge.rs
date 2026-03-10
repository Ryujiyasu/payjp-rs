use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{Card, List, ListParams, Metadata};

/// Charge object.
#[derive(Debug, Deserialize)]
pub struct Charge {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub amount: u64,
    pub currency: String,
    pub paid: bool,
    pub captured: bool,
    pub captured_at: Option<i64>,
    pub card: Option<Card>,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub refunded: bool,
    pub amount_refunded: u64,
    pub refund_reason: Option<String>,
    pub subscription: Option<String>,
    pub metadata: Option<Metadata>,
    pub expired_at: Option<i64>,
    pub three_d_secure_status: Option<String>,
}

/// Parameters for creating a charge.
#[derive(Debug, Default, Serialize)]
pub struct CreateChargeParams {
    pub amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<bool>,
}

/// Parameters for updating a charge.
#[derive(Debug, Default, Serialize)]
pub struct UpdateChargeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for refunding a charge.
#[derive(Debug, Default, Serialize)]
pub struct RefundChargeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,
}

/// Parameters for capturing a charge.
#[derive(Debug, Default, Serialize)]
pub struct CaptureChargeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
}

/// Parameters for listing charges.
#[derive(Debug, Default, Serialize)]
pub struct ListChargesParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}

impl PayjpClient {
    /// Create a new charge.
    pub async fn create_charge(&self, params: &CreateChargeParams) -> Result<Charge> {
        self.post_form("/charges", params).await
    }

    /// Retrieve a charge by ID.
    pub async fn retrieve_charge(&self, id: &str) -> Result<Charge> {
        self.get(&format!("/charges/{id}")).await
    }

    /// Update a charge.
    pub async fn update_charge(&self, id: &str, params: &UpdateChargeParams) -> Result<Charge> {
        self.post_form(&format!("/charges/{id}"), params).await
    }

    /// List charges.
    pub async fn list_charges(&self, params: &ListChargesParams) -> Result<List<Charge>> {
        self.post_form("/charges", params).await
    }

    /// Refund a charge.
    pub async fn refund_charge(&self, id: &str, params: &RefundChargeParams) -> Result<Charge> {
        self.post_form(&format!("/charges/{id}/refund"), params).await
    }

    /// Capture an authorized charge.
    pub async fn capture_charge(&self, id: &str, params: &CaptureChargeParams) -> Result<Charge> {
        self.post_form(&format!("/charges/{id}/capture"), params).await
    }
}
