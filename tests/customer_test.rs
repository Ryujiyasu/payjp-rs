use mockito::Server;
use payjp_rs::customer::{CreateCustomerParams, UpdateCustomerParams};
use payjp_rs::models::ListParams;
use payjp_rs::PayjpClient;

fn mock_customer_json(id: &str) -> String {
    format!(
        r#"{{
        "id": "{id}",
        "object": "customer",
        "livemode": false,
        "created": 1700000000,
        "default_card": null,
        "description": "test customer",
        "email": "test@example.com",
        "cards": {{
            "object": "list",
            "count": 0,
            "has_more": false,
            "url": "/v1/customers/{id}/cards",
            "data": []
        }},
        "metadata": null
    }}"#
    )
}

#[tokio::test]
async fn test_create_customer() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/customers")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_customer_json("cus_test_1"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let customer = client
        .create_customer(&CreateCustomerParams {
            email: Some("test@example.com".to_string()),
            description: Some("test customer".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(customer.id, "cus_test_1");
    assert_eq!(customer.email.unwrap(), "test@example.com");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_retrieve_customer() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/customers/cus_test_2")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_customer_json("cus_test_2"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let customer = client.retrieve_customer("cus_test_2").await.unwrap();

    assert_eq!(customer.id, "cus_test_2");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_customer() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/customers/cus_test_3")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_customer_json("cus_test_3"))
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let customer = client
        .update_customer(
            "cus_test_3",
            &UpdateCustomerParams {
                email: Some("new@example.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(customer.id, "cus_test_3");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_customer() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("DELETE", "/customers/cus_del")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "cus_del", "deleted": true, "livemode": false}"#)
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let deleted = client.delete_customer("cus_del").await.unwrap();

    assert!(deleted.deleted);
    assert_eq!(deleted.id, "cus_del");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_customers() {
    let mut server = Server::new_async().await;

    let body = format!(
        r#"{{
        "object": "list",
        "count": 1,
        "has_more": false,
        "url": "/v1/customers",
        "data": [{}]
    }}"#,
        mock_customer_json("cus_list_1")
    );

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"^/customers\??.*$".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async()
        .await;

    let client = PayjpClient::with_base_url("sk_test_xxx", server.url());
    let list = client
        .list_customers(&ListParams {
            limit: Some(10),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(list.count, 1);
    assert_eq!(list.data[0].id, "cus_list_1");
    mock.assert_async().await;
}
