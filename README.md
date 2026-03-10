# payjp-rs

[![Crates.io](https://img.shields.io/crates/v/payjp-rs.svg)](https://crates.io/crates/payjp-rs)
[![CI](https://github.com/Ryujiyasu/payjp-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Ryujiyasu/payjp-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Unofficial Rust SDK for [PAY.JP](https://pay.jp/) payment API.

## Installation

```toml
[dependencies]
payjp-rs = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use payjp_rs::PayjpClient;
use payjp_rs::charge::CreateChargeParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PayjpClient::new("sk_test_your_secret_key");

    // Create a charge
    let charge = client.create_charge(&CreateChargeParams {
        amount: 1000,
        card: Some("tok_xxxxx".to_string()),
        ..Default::default()
    }).await?;

    println!("Charge created: {}", charge.id);
    Ok(())
}
```

## Webhook Verification

```rust
use payjp_rs::webhook;

// Verify and parse a webhook event
let event = webhook::verify_and_parse(
    "whook_from_header",   // X-Payjp-Webhook-Token header
    "whook_from_dashboard", // your configured token
    &request_body,
)?;

println!("Event: {}", event.r#type);
```

## Supported APIs

| API | Create | Retrieve | Update | Delete | List |
|-----|--------|----------|--------|--------|------|
| Charge | ✓ | ✓ | ✓ | - | ✓ |
| Customer | ✓ | ✓ | ✓ | ✓ | ✓ |
| Card | ✓ | ✓ | ✓ | ✓ | ✓ |
| Token | ✓ | ✓ | - | - | - |
| Plan | ✓ | ✓ | ✓ | ✓ | ✓ |
| Subscription | ✓ | ✓ | ✓ | ✓ | ✓ |
| Transfer | - | ✓ | - | - | ✓ |
| Event | - | ✓ | - | - | ✓ |
| Statement | - | ✓ | - | - | ✓ |
| Balance | - | ✓ | - | - | ✓ |
| Term | - | ✓ | - | - | ✓ |

Additional operations:
- **Charge**: refund, capture
- **Subscription**: pause, resume, cancel
- **Statement**: download URL generation
- **Webhook**: token verification

## Examples

```bash
# Basic charge
PAYJP_SECRET_KEY=sk_test_xxx cargo run --example basic_charge

# Webhook handler
cargo run --example webhook_handler
```

## License

MIT
