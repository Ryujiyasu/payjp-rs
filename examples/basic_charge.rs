//! Basic charge example.
//!
//! Run with: `PAYJP_SECRET_KEY=sk_test_xxx cargo run --example basic_charge`

use payjp_rs::charge::CreateChargeParams;
use payjp_rs::PayjpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_key =
        std::env::var("PAYJP_SECRET_KEY").expect("Set PAYJP_SECRET_KEY environment variable");

    let client = PayjpClient::new(secret_key);

    // Create a charge (requires a valid token)
    let charge = client
        .create_charge(&CreateChargeParams {
            amount: 500,
            card: Some("tok_xxxxx".to_string()),
            ..Default::default()
        })
        .await?;

    println!("Charge created: {}", charge.id);
    println!("Amount: {} {}", charge.amount, charge.currency);
    println!("Paid: {}", charge.paid);

    // Retrieve the charge
    let retrieved = client.retrieve_charge(&charge.id).await?;
    println!("Retrieved charge: {}", retrieved.id);

    Ok(())
}
