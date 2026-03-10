use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams};

/// Statement item object.
#[derive(Debug, Deserialize)]
pub struct StatementItem {
    pub subject: Option<String>,
    pub name: Option<String>,
    pub amount: i64,
    pub tax: i64,
}

/// Statement object.
#[derive(Debug, Deserialize)]
pub struct Statement {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub title: Option<String>,
    pub tenant_id: Option<String>,
    pub term: Option<StatementTerm>,
    pub balance_id: Option<String>,
    pub items: Vec<StatementItem>,
    pub updated: Option<i64>,
}

/// Term reference within a statement.
#[derive(Debug, Deserialize)]
pub struct StatementTerm {
    pub id: String,
    pub object: String,
}

/// Statement download URL response.
#[derive(Debug, Deserialize)]
pub struct StatementDownload {
    pub object: String,
    pub url: String,
    pub expires: i64,
}

/// Parameters for listing statements.
#[derive(Debug, Default, Serialize)]
pub struct ListStatementsParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

impl PayjpClient {
    /// Retrieve a statement by ID.
    pub async fn retrieve_statement(&self, id: &str) -> Result<Statement> {
        self.get(&format!("/statements/{id}")).await
    }

    /// List statements.
    pub async fn list_statements(&self, params: &ListStatementsParams) -> Result<List<Statement>> {
        self.get_with_query("/statements", params).await
    }

    /// Get a download URL for a statement.
    pub async fn download_statement(&self, id: &str) -> Result<StatementDownload> {
        self.post_form(&format!("/statements/{id}/statement_urls"), &())
            .await
    }
}
