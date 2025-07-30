use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::error::Result;
use crate::models::{ApiResponse, DocumentVerificationRequest};

#[derive(Debug, Clone)]
pub struct DocumentVerification<'a> {
    client: ApiClient<'a>,
}

impl<'a> DocumentVerification<'a> {
    pub fn new(client: ApiClient<'a>) -> Self {
        Self { client }
    }

    pub async fn verify(
        &self,
        document_type: impl Into<String>,
        country: impl Into<String>,
        document_images: Vec<String>,
    ) -> Result<String> {
        let request = DocumentVerificationRequest {
            document_type: document_type.into(),
            country: country.into(),
            document_images,
            partner_params: None,
        };

        let url = format!("{}/document_verification", self.client.base_url());
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
    pub struct DocumentVerification<'a> {
        client: ApiClient<'a>,
    }

    impl<'a> DocumentVerification<'a> {
        pub fn new(client: ApiClient<'a>) -> Self {
            Self { client }
        }

        pub fn verify(
            &self,
            document_type: impl Into<String>,
            country: impl Into<String>,
            document_images: Vec<String>,
        ) -> Result<String> {
            let request = DocumentVerificationRequest {
                document_type: document_type.into(),
                country: country.into(),
                document_images,
                partner_params: None,
            };

            let url = format!("{}/document_verification", self.client.base_url());
            let response: ApiResponse<VerifyResponse> = self.client.post(&url, &request)?;

            Ok(response.data.job_id)
        }
    }
}
