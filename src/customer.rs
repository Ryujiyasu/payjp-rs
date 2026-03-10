use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{Card, Deleted, List, ListParams, Metadata};

/// Customer object.
#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub default_card: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub cards: Option<List<Card>>,
    pub metadata: Option<Metadata>,
}

/// Parameters for creating a customer.
#[derive(Debug, Default, Serialize)]
pub struct CreateCustomerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
}

/// Parameters for updating a customer.
#[derive(Debug, Default, Serialize)]
pub struct UpdateCustomerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
}

impl PayjpClient {
    pub async fn create_customer(&self, params: &CreateCustomerParams) -> Result<Customer> {
        self.post_form("/customers", params).await
    }

    pub async fn retrieve_customer(&self, id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{id}")).await
    }

    pub async fn update_customer(
        &self,
        id: &str,
        params: &UpdateCustomerParams,
    ) -> Result<Customer> {
        self.post_form(&format!("/customers/{id}"), params).await
    }

    pub async fn delete_customer(&self, id: &str) -> Result<Deleted> {
        self.delete(&format!("/customers/{id}")).await
    }

    pub async fn list_customers(&self, params: &ListParams) -> Result<List<Customer>> {
        self.get_with_query("/customers", params).await
    }
}
