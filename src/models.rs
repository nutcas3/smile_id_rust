use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub status_code: u16,
    pub message: String,

    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobStatusResponse {
    pub job_id: String,
    pub job_status: JobStatus,
    pub job_type: String,
    pub job_complete: bool,
    pub job_success: bool,
    pub result_type: Option<String>,
    pub result_text: Option<String>,
    pub result_code: Option<String>,
    pub actions: Option<Vec<String>>,
    pub confidence_values: Option<HashMap<String, f64>>,
    pub history: Option<Vec<JobHistoryItem>>,
    pub image_links: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobHistoryItem {
    pub job_id: String,
    pub job_type: String,
    pub job_status: JobStatus,
    pub job_complete: bool,
    pub job_success: bool,
    pub result_type: Option<String>,
    pub result_text: Option<String>,
    pub result_code: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BasicKycRequest {
    pub id_type: String,
    pub id_number: String,
    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnhancedKycRequest {
    pub id_type: String,
    pub id_number: String,
    pub country: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BiometricKycRequest {
    pub id_type: String,
    pub id_number: String,
    pub country: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub selfie_image: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentVerificationRequest {
    pub document_type: String,
    pub country: String,
    pub document_images: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SmartSelfieAuthRequest {
    pub user_id: String,
    pub job_id: String,
    pub selfie_image: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessVerificationRequest {
    pub business_name: String,
    pub registration_number: String,
    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobStatusRequest {
    pub user_id: String,
    pub job_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_history: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_image_links: Option<bool>,
}
