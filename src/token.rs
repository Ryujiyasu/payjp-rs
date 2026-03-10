//! Token API for tokenizing card information.
//!
//! Note: Tokens are typically created client-side using PAY.JP's JavaScript library.
//! Server-side token creation requires PCI DSS compliance.

use serde::{Deserialize, Serialize};

use crate::client::PayjpClient;
use crate::error::Result;
use crate::models::Card;

/// Token object.
#[derive(Debug, Deserialize)]
pub struct Token {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub used: bool,
    pub card: Card,
}

/// Parameters for creating a token.
#[derive(Debug, Serialize)]
pub struct CreateTokenParams {
    #[serde(rename = "card[number]")]
    pub card_number: String,
    #[serde(rename = "card[exp_month]")]
    pub card_exp_month: u32,
    #[serde(rename = "card[exp_year]")]
    pub card_exp_year: u32,
    #[serde(rename = "card[cvc]", skip_serializing_if = "Option::is_none")]
    pub card_cvc: Option<String>,
    #[serde(rename = "card[name]", skip_serializing_if = "Option::is_none")]
    pub card_name: Option<String>,
}

impl PayjpClient {
    pub async fn create_token(&self, params: &CreateTokenParams) -> Result<Token> {
        self.post_form("/tokens", params).await
    }

    pub async fn retrieve_token(&self, id: &str) -> Result<Token> {
        self.get(&format!("/tokens/{id}")).await
    }
}
