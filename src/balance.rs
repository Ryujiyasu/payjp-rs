use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams};
use crate::statement::Statement;

/// Bank account information.
#[derive(Debug, Deserialize)]
pub struct BankInfo {
    pub bank_code: Option<String>,
    pub bank_branch_code: Option<String>,
    pub bank_account_type: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_account_holder_name: Option<String>,
    pub bank_account_status: Option<String>,
}

/// Balance object.
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub net: i64,
    pub tenant_id: Option<String>,
    pub state: Option<String>,
    pub closed: Option<bool>,
    pub due_date: Option<String>,
    pub bank_info: Option<BankInfo>,
    pub statements: Option<List<Statement>>,
}

/// Parameters for listing balances.
#[derive(Debug, Default, Serialize)]
pub struct ListBalancesParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

impl PayjpClient {
    /// Retrieve a balance by ID.
    pub async fn retrieve_balance(&self, id: &str) -> Result<Balance> {
        self.get(&format!("/balances/{id}")).await
    }

    /// List balances.
    pub async fn list_balances(&self, params: &ListBalancesParams) -> Result<List<Balance>> {
        self.get_with_query("/balances", params).await
    }
}
