use mockito::Server;
use payjp_rs::subscription::CreateSubscriptionParams;
use payjp_rs::PayjpClient;

fn mock_subscription_json(id: &str, status: &str) -> String {
    format!(
        r#"{{
        "id": "{id}",
        "object": "subscription",
        "livemode": false,
        "created": 1700000000,
        "customer": "cus_xxx",
        "plan": {{
            "id": "pln_xxx",
            "object": "plan",
            "livemode": false,
            "created": 1700000000,
            "amount": 500,
            "currency": "jpy",
            "interval": "month",
            "name": "basic",
            "trial_days": null,
            "billing_day": null,
            "metadata": null
        }},
        "status": "{status}",
        "current_period_start": 1700000000,
        "current_period_end": 1702592000,
        "trial_start": null,
        "trial_end": null,
        "paused_at": null,
        "canceled_at": null,
        "resumed_at": null,
        "prorate": null,
        "metadata": null
    }}"#
    )
}

#[tokio::test]
async fn test_create_subscription() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/subscriptions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_subscription_json("sub_1", "active"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let sub = client
        .create_subscription(&CreateSubscriptionParams {
            customer: "cus_xxx".to_string(),
            plan: "pln_xxx".to_string(),
            trial_end: None,
            prorate: None,
        })
        .await
        .unwrap();

    assert_eq!(sub.id, "sub_1");
    assert_eq!(sub.status, "active");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_pause_subscription() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/subscriptions/sub_2/pause")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_subscription_json("sub_2", "paused"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let sub = client.pause_subscription("sub_2").await.unwrap();

    assert_eq!(sub.status, "paused");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_resume_subscription() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/subscriptions/sub_3/resume")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_subscription_json("sub_3", "active"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let sub = client.resume_subscription("sub_3").await.unwrap();

    assert_eq!(sub.status, "active");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cancel_subscription() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/subscriptions/sub_4/cancel")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_subscription_json("sub_4", "canceled"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let sub = client.cancel_subscription("sub_4").await.unwrap();

    assert_eq!(sub.status, "canceled");
    mock.assert_async().await;
}
