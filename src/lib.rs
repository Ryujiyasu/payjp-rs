//! # payjp-rs
//!
//! Unofficial Rust SDK for the [PAY.JP](https://pay.jp/) payment API.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use payjp_rs::PayjpClient;
//! use payjp_rs::charge::CreateChargeParams;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = PayjpClient::new("sk_test_your_secret_key");
//!
//! let charge = client.create_charge(&CreateChargeParams {
//!     amount: 1000,
//!     card: Some("tok_xxxxx".to_string()),
//!     ..Default::default()
//! }).await?;
//!
//! println!("Charge created: {}", charge.id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Webhook Verification
//!
//! ```rust
//! use payjp_rs::webhook;
//!
//! let is_valid = webhook::verify_token("whook_from_header", "whook_from_dashboard");
//! ```

pub mod client;
pub mod error;
pub mod models;

pub mod balance;
pub mod card;
pub mod charge;
pub mod customer;
pub mod event;
pub mod plan;
pub mod statement;
pub mod subscription;
pub mod term;
pub mod token;
pub mod transfer;
pub mod webhook;

pub use client::PayjpClient;
pub use error::PayjpError;
