# payjp-rs

Unofficial Rust SDK for [PAY.JP](https://pay.jp/) payment API.

## Installation

```toml
[dependencies]
payjp-rs = "0.1"
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

## Supported APIs

| API | Create | Retrieve | Update | Delete | List |
|-----|--------|----------|--------|--------|------|
| Charge | ✓ | ✓ | ✓ | - | ✓ |
| Customer | ✓ | ✓ | ✓ | ✓ | ✓ |
| Card | ✓ | ✓ | ✓ | ✓ | ✓ |
| Token | ✓ | ✓ | - | - | - |
| Plan | ✓ | ✓ | ✓ | ✓ | ✓ |
| Subscription | ✓ | ✓ | ✓ | ✓ | ✓ |

Additional operations:
- **Charge**: refund, capture
- **Subscription**: pause, resume, cancel

## License

MIT
