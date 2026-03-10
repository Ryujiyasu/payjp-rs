//! Plan API for managing recurring billing plans.

use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{Deleted, List, ListParams, Metadata};

/// Plan object.
#[derive(Debug, Deserialize)]
pub struct Plan {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub amount: u64,
    pub currency: String,
    pub interval: String,
    pub name: Option<String>,
    pub trial_days: Option<u32>,
    pub billing_day: Option<u32>,
    pub metadata: Option<Metadata>,
}

/// Parameters for creating a plan.
#[derive(Debug, Serialize)]
pub struct CreatePlanParams {
    pub amount: u64,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day: Option<u32>,
}

/// Parameters for updating a plan.
#[derive(Debug, Default, Serialize)]
pub struct UpdatePlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PayjpClient {
    pub async fn create_plan(&self, params: &CreatePlanParams) -> Result<Plan> {
        self.post_form("/plans", params).await
    }

    pub async fn retrieve_plan(&self, id: &str) -> Result<Plan> {
        self.get(&format!("/plans/{id}")).await
    }

    pub async fn update_plan(&self, id: &str, params: &UpdatePlanParams) -> Result<Plan> {
        self.post_form(&format!("/plans/{id}"), params).await
    }

    pub async fn delete_plan(&self, id: &str) -> Result<Deleted> {
        self.delete(&format!("/plans/{id}")).await
    }

    pub async fn list_plans(&self, params: &ListParams) -> Result<List<Plan>> {
        self.get_with_query("/plans", params).await
    }
}
