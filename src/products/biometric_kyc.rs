use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::error::Result;
use crate::models::{ApiResponse, BiometricKycRequest};

#[derive(Debug, Clone)]
pub struct BiometricKyc<'a> {
    client: ApiClient<'a>,
}

impl<'a> BiometricKyc<'a> {
    pub fn new(client: ApiClient<'a>) -> Self {
        Self { client }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn verify(
        &self,
        id_type: impl Into<String>,
        id_number: impl Into<String>,
        country: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        dob: impl Into<String>,
        selfie_image: impl Into<String>,
    ) -> Result<String> {
        let request = BiometricKycRequest {
            id_type: id_type.into(),
            id_number: id_number.into(),
            country: country.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            dob: dob.into(),
            selfie_image: selfie_image.into(),
            partner_params: None,
        };

        let url = format!("{}/biometric_kyc", self.client.base_url());
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
    pub struct BiometricKyc<'a> {
        client: ApiClient<'a>,
    }

    impl<'a> BiometricKyc<'a> {
        pub fn new(client: ApiClient<'a>) -> Self {
            Self { client }
        }

        pub fn verify(
            &self,
            id_type: impl Into<String>,
            id_number: impl Into<String>,
            country: impl Into<String>,
            first_name: impl Into<String>,
            last_name: impl Into<String>,
            dob: impl Into<String>,
            selfie_image: impl Into<String>,
        ) -> Result<String> {
            let request = BiometricKycRequest {
                id_type: id_type.into(),
                id_number: id_number.into(),
                country: country.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                dob: dob.into(),
                selfie_image: selfie_image.into(),
                partner_params: None,
            };

            let url = format!("{}/biometric_kyc", self.client.base_url());
            let response: ApiResponse<VerifyResponse> = self.client.post(&url, &request)?;

            Ok(response.data.job_id)
        }
    }
}
