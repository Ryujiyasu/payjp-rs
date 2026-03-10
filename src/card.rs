//! Card API for managing customer payment cards.

use serde::Serialize;

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::{Card, Deleted, List, ListParams};

/// Parameters for creating a card.
#[derive(Debug, Default, Serialize)]
pub struct CreateCardParams {
    pub card: String,
}

/// Parameters for updating a card.
#[derive(Debug, Default, Serialize)]
pub struct UpdateCardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl PayjpClient {
    pub async fn create_card(&self, customer_id: &str, params: &CreateCardParams) -> Result<Card> {
        self.post_form(&format!("/customers/{customer_id}/cards"), params)
            .await
    }

    pub async fn retrieve_card(&self, customer_id: &str, card_id: &str) -> Result<Card> {
        self.get(&format!("/customers/{customer_id}/cards/{card_id}"))
            .await
    }

    pub async fn update_card(
        &self,
        customer_id: &str,
        card_id: &str,
        params: &UpdateCardParams,
    ) -> Result<Card> {
        self.post_form(&format!("/customers/{customer_id}/cards/{card_id}"), params)
            .await
    }

    pub async fn delete_card(&self, customer_id: &str, card_id: &str) -> Result<Deleted> {
        self.delete(&format!("/customers/{customer_id}/cards/{card_id}"))
            .await
    }

    pub async fn list_cards(&self, customer_id: &str, params: &ListParams) -> Result<List<Card>> {
        self.get_with_query(&format!("/customers/{customer_id}/cards"), params)
            .await
    }
}
