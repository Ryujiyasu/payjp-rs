use crate::error::{PayjpError, Result};
use crate::event::Event;

/// Header name for the PAY.JP webhook token.
pub const WEBHOOK_TOKEN_HEADER: &str = "X-Payjp-Webhook-Token";

/// Verify a webhook request from PAY.JP.
///
/// PAY.JP sends an `X-Payjp-Webhook-Token` header with each webhook request.
/// This token should match the webhook token configured in your PAY.JP dashboard.
///
/// # Example
///
/// ```rust,no_run
/// use payjp_rs::webhook;
///
/// let token_header = "whook_xxxxx"; // from request header
/// let expected_token = "whook_xxxxx"; // from your PAY.JP dashboard
/// let body = r#"{"id":"evnt_xxx","object":"event","type":"charge.succeeded","data":{},"created":0,"livemode":false}"#;
///
/// let event = webhook::verify_and_parse(token_header, expected_token, body).unwrap();
/// println!("Event type: {}", event.r#type);
/// ```
pub fn verify_and_parse(token_header: &str, expected_token: &str, body: &str) -> Result<Event> {
    if !constant_time_eq(token_header.as_bytes(), expected_token.as_bytes()) {
        return Err(PayjpError::WebhookVerification(
            "webhook token mismatch".to_string(),
        ));
    }

    let event: Event = serde_json::from_str(body)?;
    Ok(event)
}

/// Verify the webhook token without parsing the body.
///
/// Returns `true` if the token matches, `false` otherwise.
pub fn verify_token(token_header: &str, expected_token: &str) -> bool {
    constant_time_eq(token_header.as_bytes(), expected_token.as_bytes())
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_token_valid() {
        assert!(verify_token("whook_abc123", "whook_abc123"));
    }

    #[test]
    fn test_verify_token_invalid() {
        assert!(!verify_token("whook_abc123", "whook_wrong"));
    }

    #[test]
    fn test_verify_and_parse_valid() {
        let body = r#"{
            "id": "evnt_test",
            "object": "event",
            "livemode": false,
            "created": 1700000000,
            "type": "charge.succeeded",
            "pending_webhooks": 1,
            "data": {"id": "ch_xxx"}
        }"#;

        let event = verify_and_parse("whook_test", "whook_test", body).unwrap();
        assert_eq!(event.id, "evnt_test");
        assert_eq!(event.r#type, "charge.succeeded");
    }

    #[test]
    fn test_verify_and_parse_token_mismatch() {
        let body = r#"{"id":"evnt_test","object":"event","livemode":false,"created":0,"type":"test","data":{}}"#;
        let result = verify_and_parse("whook_bad", "whook_good", body);
        assert!(result.is_err());
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"short", b"longer"));
    }
}
