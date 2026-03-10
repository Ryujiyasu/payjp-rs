use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ApiErrorResponse, PayjpError, Result};

const BASE_URL: &str = "https://api.pay.jp/v1";

pub struct PayjpClient {
    secret_key: String,
    http: Client,
}

impl PayjpClient {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
            http: Client::new(),
        }
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .http
            .get(format!("{BASE_URL}{path}"))
            .basic_auth(&self.secret_key, Option::<&str>::None)
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
            .post(format!("{BASE_URL}{path}"))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .form(params)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .http
            .delete(format!("{BASE_URL}{path}"))
            .basic_auth(&self.secret_key, Option::<&str>::None)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            let err: ApiErrorResponse = serde_json::from_str(&body)?;
            return Err(PayjpError::Api(err.error));
        }

        Ok(serde_json::from_str(&body)?)
    }
}
