use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ApiErrorResponse, PayjpError, Result};

const DEFAULT_BASE_URL: &str = "https://api.pay.jp/v1";

/// PAY.JP API client.
///
/// Handles authentication and HTTP communication with the PAY.JP API.
///
/// # Example
///
/// ```rust,no_run
/// use payjp_rs::PayjpClient;
///
/// let client = PayjpClient::new("sk_test_your_secret_key");
/// ```
pub struct PayjpClient {
    secret_key: String,
    base_url: String,
    http: Client,
}

impl PayjpClient {
    /// Create a new client with the given secret API key.
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            http: Client::new(),
        }
    }

    /// Create a client with a custom base URL (useful for testing).
    pub fn with_base_url(secret_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
            base_url: base_url.into(),
            http: Client::new(),
        }
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .http
            .get(format!("{}{path}", self.base_url))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &impl Serialize,
    ) -> Result<T> {
        let resp = self
            .http
            .get(format!("{}{path}", self.base_url))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .query(params)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn post_form<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &impl Serialize,
    ) -> Result<T> {
        let resp = self
            .http
            .post(format!("{}{path}", self.base_url))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .form(params)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .http
            .delete(format!("{}{path}", self.base_url))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            let err: ApiErrorResponse = serde_json::from_str(&body)?;
            return Err(PayjpError::Api(err.error));
        }

        Ok(serde_json::from_str(&body)?)
    }
}
