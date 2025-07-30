use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::error::Result;
use crate::models::{ApiResponse, BusinessVerificationRequest};

/// Business Verification product
#[derive(Debug, Clone)]
pub struct BusinessVerification<'a> {
    client: ApiClient<'a>,
}

impl<'a> BusinessVerification<'a> {
    pub fn new(client: ApiClient<'a>) -> Self {
        Self { client }
    }

    pub async fn verify(
        &self,
        business_name: impl Into<String>,
        registration_number: impl Into<String>,
        country: impl Into<String>,
    ) -> Result<String> {
        let request = BusinessVerificationRequest {
            business_name: business_name.into(),
            registration_number: registration_number.into(),
            country: country.into(),
            partner_params: None,
        };

        let url = format!("{}/business_verification", self.client.base_url());
        let response: ApiResponse<VerifyResponse> = self.client.post(&url, &request).await?;

        Ok(response.data.job_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyResponse {
    pub job_id: String,
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use super::*;
    use crate::api::blocking::ApiClient;

    #[derive(Debug, Clone)]
    pub struct BusinessVerification<'a> {
        client: ApiClient<'a>,
    }

    impl<'a> BusinessVerification<'a> {
        pub fn new(client: ApiClient<'a>) -> Self {
            Self { client }
        }

        pub fn verify(
            &self,
            business_name: impl Into<String>,
            registration_number: impl Into<String>,
            country: impl Into<String>,
        ) -> Result<String> {
            let request = BusinessVerificationRequest {
                business_name: business_name.into(),
                registration_number: registration_number.into(),
                country: country.into(),
                partner_params: None,
            };

            let url = format!("{}/business_verification", self.client.base_url());
            let response: ApiResponse<VerifyResponse> = self.client.post(&url, &request)?;

            Ok(response.data.job_id)
        }
    }
}
