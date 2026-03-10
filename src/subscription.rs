//! Subscription API for managing recurring payments.
//!
//! Subscriptions can be paused, resumed, and canceled.

use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{List, ListParams, Metadata};
use crate::plan::Plan;

/// Subscription object.
#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub customer: String,
    pub plan: Plan,
    pub status: String,
    pub current_period_start: Option<i64>,
    pub current_period_end: Option<i64>,
    pub trial_start: Option<i64>,
    pub trial_end: Option<i64>,
    pub paused_at: Option<i64>,
    pub canceled_at: Option<i64>,
    pub resumed_at: Option<i64>,
    pub prorate: Option<bool>,
    pub metadata: Option<Metadata>,
}

/// Parameters for creating a subscription.
#[derive(Debug, Serialize)]
pub struct CreateSubscriptionParams {
    pub customer: String,
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

/// Parameters for updating a subscription.
#[derive(Debug, Default, Serialize)]
pub struct UpdateSubscriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

/// Parameters for listing subscriptions.
#[derive(Debug, Default, Serialize)]
pub struct ListSubscriptionsParams {
    #[serde(flatten)]
    pub pagination: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PayjpClient {
    pub async fn create_subscription(
        &self,
        params: &CreateSubscriptionParams,
    ) -> Result<Subscription> {
        self.post_form("/subscriptions", params).await
    }

    pub async fn retrieve_subscription(&self, id: &str) -> Result<Subscription> {
        self.get(&format!("/subscriptions/{id}")).await
    }

    pub async fn update_subscription(
        &self,
        id: &str,
        params: &UpdateSubscriptionParams,
    ) -> Result<Subscription> {
        self.post_form(&format!("/subscriptions/{id}"), params)
            .await
    }

    pub async fn pause_subscription(&self, id: &str) -> Result<Subscription> {
        self.post_form(&format!("/subscriptions/{id}/pause"), &())
            .await
    }

    pub async fn resume_subscription(&self, id: &str) -> Result<Subscription> {
        self.post_form(&format!("/subscriptions/{id}/resume"), &())
            .await
    }

    pub async fn cancel_subscription(&self, id: &str) -> Result<Subscription> {
        self.post_form(&format!("/subscriptions/{id}/cancel"), &())
            .await
    }

    pub async fn delete_subscription(&self, id: &str) -> Result<Subscription> {
        self.delete(&format!("/subscriptions/{id}")).await
    }

    pub async fn list_subscriptions(
        &self,
        params: &ListSubscriptionsParams,
    ) -> Result<List<Subscription>> {
        self.get_with_query("/subscriptions", params).await
    }
}
