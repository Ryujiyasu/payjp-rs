//! Webhook verification example.
//!
//! Shows how to verify and parse incoming PAY.JP webhook events.

use payjp_rs::webhook;

fn main() {
    // In a real application, these come from the HTTP request
    let token_header = "whook_xxxxx"; // from X-Payjp-Webhook-Token header
    let expected_token = "whook_xxxxx"; // from your PAY.JP dashboard
    let body = r#"{
        "id": "evnt_abc123",
        "object": "event",
        "livemode": false,
        "created": 1700000000,
        "type": "charge.succeeded",
        "pending_webhooks": 1,
        "data": {
            "id": "ch_xxx",
            "object": "charge",
            "amount": 1000
        }
    }"#;

    // Verify and parse in one step
    match webhook::verify_and_parse(token_header, expected_token, body) {
        Ok(event) => {
            println!("Event ID: {}", event.id);
            println!("Event type: {}", event.r#type);
            println!("Event data: {}", event.data);
        }
        Err(e) => {
            eprintln!("Webhook verification failed: {}", e);
        }
    }

    // Or just verify the token
    if webhook::verify_token(token_header, expected_token) {
        println!("Token is valid!");
    }
}
