use mockito::{Matcher, Server};
use payjp_rs::charge::{
    CaptureChargeParams, CreateChargeParams, RefundChargeParams, UpdateChargeParams,
};
use payjp_rs::PayjpClient;

fn mock_charge_json(id: &str) -> String {
    format!(
        r#"{{
        "id": "{id}",
        "object": "charge",
        "livemode": false,
        "created": 1700000000,
        "amount": 1000,
        "currency": "jpy",
        "paid": true,
        "captured": true,
        "captured_at": 1700000000,
        "card": null,
        "customer": null,
        "description": null,
        "failure_code": null,
        "failure_message": null,
        "refunded": false,
        "amount_refunded": 0,
        "refund_reason": null,
        "subscription": null,
        "metadata": null,
        "expired_at": null,
        "three_d_secure_status": null
    }}"#
    )
}

#[tokio::test]
async fn test_create_charge() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/charges")
        .match_header("authorization", Matcher::Regex("Basic .+".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_charge_json("ch_test_123"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());

    let charge = client
        .create_charge(&CreateChargeParams {
            amount: 1000,
            card: Some("tok_xxx".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(charge.id, "ch_test_123");
    assert_eq!(charge.amount, 1000);
    assert!(charge.paid);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_retrieve_charge() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/charges/ch_test_456")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_charge_json("ch_test_456"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let charge = client.retrieve_charge("ch_test_456").await.unwrap();

    assert_eq!(charge.id, "ch_test_456");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_charge() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/charges/ch_test_789")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_charge_json("ch_test_789"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let charge = client
        .update_charge(
            "ch_test_789",
            &UpdateChargeParams {
                description: Some("updated".to_string()),
            },
        )
        .await
        .unwrap();

    assert_eq!(charge.id, "ch_test_789");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_refund_charge() {
    let mut server = Server::new_async().await;

    let body = r#"{
        "id": "ch_refund",
        "object": "charge",
        "livemode": false,
        "created": 1700000000,
        "amount": 1000,
        "currency": "jpy",
        "paid": true,
        "captured": true,
        "captured_at": 1700000000,
        "card": null,
        "customer": null,
        "description": null,
        "failure_code": null,
        "failure_message": null,
        "refunded": true,
        "amount_refunded": 1000,
        "refund_reason": null,
        "subscription": null,
        "metadata": null,
        "expired_at": null,
        "three_d_secure_status": null
    }"#;

    let mock = server
        .mock("POST", "/charges/ch_refund/refund")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let charge = client
        .refund_charge("ch_refund", &RefundChargeParams::default())
        .await
        .unwrap();

    assert!(charge.refunded);
    assert_eq!(charge.amount_refunded, 1000);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_capture_charge() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/charges/ch_cap/capture")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_charge_json("ch_cap"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let charge = client
        .capture_charge("ch_cap", &CaptureChargeParams::default())
        .await
        .unwrap();

    assert!(charge.captured);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_api_error() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/charges")
        .with_status(402)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "error": {
                "message": "Invalid card",
                "type": "card_error",
                "code": "invalid_number",
                "status": 402
            }
        }"#,
        )
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let result = client
        .create_charge(&CreateChargeParams {
            amount: 1000,
            card: Some("tok_invalid".to_string()),
            ..Default::default()
        })
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid card"));
    mock.assert_async().await;
}
