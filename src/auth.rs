use crate::error::{Error, Result};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct Auth {
    api_key: String,
    partner_id: String,
}

impl Auth {
    pub fn new(api_key: impl Into<String>, partner_id: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            partner_id: partner_id.into(),
        }
    }

    pub fn generate_signature(&self, timestamp: &DateTime<Utc>, payload: &str) -> Result<String> {
        let timestamp_str = timestamp.to_rfc3339();
        let message = format!("{}{}{}", self.partner_id, timestamp_str, payload);

        let mut mac = HmacSha256::new_from_slice(self.api_key.as_bytes())
            .map_err(|e| Error::Auth(format!("Failed to create HMAC: {e}")))?;

        mac.update(message.as_bytes());
        let result = mac.finalize();
        let signature = general_purpose::STANDARD.encode(result.into_bytes());

        Ok(signature)
    }

    pub fn verify_signature(
        &self,
        signature: &str,
        timestamp: &DateTime<Utc>,
        payload: &str,
    ) -> Result<bool> {
        let expected_signature = self.generate_signature(timestamp, payload)?;
        Ok(signature == expected_signature)
    }

    pub fn partner_id(&self) -> &str {
        &self.partner_id
    }
}
