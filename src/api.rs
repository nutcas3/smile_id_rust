use chrono::Utc;
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

use crate::auth::Auth;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::models::{ApiResponse, JobStatusRequest, JobStatusResponse};

#[derive(Debug, Clone)]
pub struct ApiClient<'a> {
    client: Client,
    auth: Auth,
    config: &'a Config,
}

impl<'a> ApiClient<'a> {
    pub fn new(config: &'a Config) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .map_err(Error::Http)?;

        let auth = Auth::new(&config.api_key, &config.partner_id);

        Ok(Self {
            client,
            auth,
            config,
        })
    }

    pub async fn get_job_status(
        &self,
        user_id: impl Into<String>,
        job_id: impl Into<String>,
        include_history: Option<bool>,
        include_image_links: Option<bool>,
    ) -> Result<JobStatusResponse> {
        let request = JobStatusRequest {
            user_id: user_id.into(),
            job_id: job_id.into(),
            include_history,
            include_image_links,
        };

        let url = format!("{}/job_status", self.base_url());
        self.post(&url, &request).await
    }

    pub async fn post<T, R>(&self, url: &str, payload: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let timestamp = Utc::now();
        let json = serde_json::to_string(payload).map_err(Error::Json)?;
        let signature = self.auth.generate_signature(&timestamp, &json)?;

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Smile-Partner-ID", self.auth.partner_id())
            .header("X-Smile-Signature", &signature)
            .header("X-Smile-Timestamp", timestamp.to_rfc3339())
            .body(json)
            .send()
            .await
            .map_err(Error::Http)?;

        let status = response.status();
        let body = response.text().await.map_err(Error::Http)?;

        if !status.is_success() {
            return Err(Error::Api {
                status_code: status.as_u16(),
                message: body,
            });
        }

        let api_response: ApiResponse<R> = serde_json::from_str(&body).map_err(Error::Json)?;

        if api_response.status_code >= 400 {
            return Err(Error::Api {
                status_code: api_response.status_code,
                message: api_response.message,
            });
        }

        Ok(api_response.data)
    }

    pub fn base_url(&self) -> String {
        format!("{}/v{}", self.config.base_url, self.config.version)
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct ApiClient<'a> {
        client: reqwest::blocking::Client,
        auth: Auth,
        config: &Config,
    }

    impl<'a> ApiClient<'a> {
        pub fn new(config: Config) -> Result<Self> {
            let client = reqwest::blocking::ClientBuilder::new()
                .timeout(Duration::from_secs(config.timeout))
                .build()
                .map_err(Error::Http)?;

            let auth = Auth::new(&config.api_key, &config.partner_id);

            Ok(Self {
                client,
                auth,
                config,
            })
        }

        pub fn get_job_status(
            &self,
            user_id: impl Into<String>,
            job_id: impl Into<String>,
            include_history: Option<bool>,
            include_image_links: Option<bool>,
        ) -> Result<JobStatusResponse> {
            let request = JobStatusRequest {
                user_id: user_id.into(),
                job_id: job_id.into(),
                include_history,
                include_image_links,
            };

            let url = format!("{}/job_status", self.base_url());
            self.post(&url, &request)
        }

        pub fn post<T, R>(&self, url: &str, payload: &T) -> Result<R>
        where
            T: Serialize + ?Sized,
            R: DeserializeOwned,
        {
            let timestamp = Utc::now();
            let json = serde_json::to_string(payload).map_err(Error::Json)?;
            let signature = self.auth.generate_signature(&timestamp, &json)?;

            let response = self
                .client
                .post(url)
                .header("Content-Type", "application/json")
                .header("X-Smile-Partner-ID", self.auth.partner_id())
                .header("X-Smile-Signature", &signature)
                .header("X-Smile-Timestamp", timestamp.to_rfc3339())
                .body(json)
                .send()
                .map_err(Error::Http)?;

            let status = response.status();
            let body = response.text().map_err(Error::Http)?;

            if !status.is_success() {
                return Err(Error::Api {
                    status_code: status.as_u16(),
                    message: body,
                });
            }

            let api_response: ApiResponse<R> = serde_json::from_str(&body).map_err(Error::Json)?;

            if api_response.status_code >= 400 {
                return Err(Error::Api {
                    status_code: api_response.status_code,
                    message: api_response.message,
                });
            }

            Ok(api_response.data)
        }

        pub fn base_url(&self) -> String {
            format!("{}/v{}", self.config.base_url, self.config.version)
        }
    }
}
