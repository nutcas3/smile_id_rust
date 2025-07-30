use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::error::Result;
use crate::models::{ApiResponse, SmartSelfieAuthRequest};

#[derive(Debug, Clone)]
pub struct SmartSelfieAuth<'a> {
    client: ApiClient<'a>,
}

impl<'a> SmartSelfieAuth<'a> {
    pub fn new(client: ApiClient<'a>) -> Self {
        Self { client }
    }

    pub async fn authenticate(
        &self,
        user_id: impl Into<String>,
        job_id: impl Into<String>,
        selfie_image: impl Into<String>,
    ) -> Result<String> {
        let request = SmartSelfieAuthRequest {
            user_id: user_id.into(),
            job_id: job_id.into(),
            selfie_image: selfie_image.into(),
            partner_params: None,
        };

        let url = format!("{}/smartselfie_auth", self.client.base_url());
        let response: ApiResponse<AuthResponse> = self.client.post(&url, &request).await?;

        Ok(response.data.job_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthResponse {
    pub job_id: String,
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use super::*;
    use crate::api::blocking::ApiClient;

    #[derive(Debug, Clone)]
    pub struct SmartSelfieAuth<'a> {
        client: ApiClient<'a>,
    }

    impl<'a> SmartSelfieAuth<'a> {
        pub fn new(client: ApiClient<'a>) -> Self {
            Self { client }
        }

        pub fn authenticate(
            &self,
            user_id: impl Into<String>,
            job_id: impl Into<String>,
            selfie_image: impl Into<String>,
        ) -> Result<String> {
            let request = SmartSelfieAuthRequest {
                user_id: user_id.into(),
                job_id: job_id.into(),
                selfie_image: selfie_image.into(),
                partner_params: None,
            };

            let url = format!("{}/smartselfie_auth", self.client.base_url());
            let response: ApiResponse<AuthResponse> = self.client.post(&url, &request)?;

            Ok(response.data.job_id)
        }
    }
}
