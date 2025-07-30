use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::error::Result;
use crate::models::{ApiResponse, BasicKycRequest};

#[derive(Debug, Clone)]
pub struct BasicKyc<'a> {
    client: ApiClient<'a>,
}

impl<'a> BasicKyc<'a> {
    pub fn new(client: ApiClient<'a>) -> Self {
        Self { client }
    }

    pub async fn verify(
        &self,
        id_type: impl Into<String>,
        id_number: impl Into<String>,
        country: impl Into<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        dob: Option<String>,
    ) -> Result<String> {
        let request = BasicKycRequest {
            id_type: id_type.into(),
            id_number: id_number.into(),
            country: country.into(),
            first_name,
            last_name,
            dob,
            partner_params: None,
        };

        let url = format!("{}/basic_kyc", self.client.base_url());
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
    pub struct BasicKyc<'a> {
        client: ApiClient<'a>,
    }

    impl<'a> BasicKyc<'a> {
        pub fn new(client: ApiClient<'a>) -> Self {
            Self { client }
        }

        pub fn verify(
            &self,
            id_type: impl Into<String>,
            id_number: impl Into<String>,
            country: impl Into<String>,
            first_name: Option<String>,
            last_name: Option<String>,
            dob: Option<String>,
        ) -> Result<String> {
            let request = BasicKycRequest {
                id_type: id_type.into(),
                id_number: id_number.into(),
                country: country.into(),
                first_name,
                last_name,
                dob,
                partner_params: None,
            };

            let url = format!("{}/basic_kyc", self.client.base_url());
            let response: ApiResponse<VerifyResponse> = self.client.post(&url, &request)?;

            Ok(response.data.job_id)
        }
    }
}
