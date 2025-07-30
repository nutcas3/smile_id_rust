# Smile ID Rust Client

A Rust client library for Smile ID's identity verification services across Africa. This library provides a simple interface to verify identities, prevent fraud, and comply with KYC regulations.

## Features

- **Basic KYC**: Verify identity information against government databases
- **Enhanced KYC**: More comprehensive identity verification with additional data points
- **Biometric KYC**: Identity verification with biometric data (selfie)
- **Document Verification**: Verify identity documents like passports, ID cards, etc.
- **SmartSelfie™ Authentication**: Authenticate users with facial biometrics
- **Business Verification**: Verify business registration information
- **Job Status Tracking**: Track the progress and outcome of verification jobs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smile_id = "0.1.0"
```

## Usage

### Configuration

```rust
use smile_id::Config;
use smile_id::ApiClient;

// Create a configuration
let config = Config::new("your-api-key", "your-partner-id")
    .with_timeout(60); // Optional: Set a custom timeout in seconds

// Create an API client
let client = ApiClient::new(config).expect("Failed to create API client");
```

### Basic KYC

```rust
use smile_id::products::BasicKyc;

// Create a Basic KYC instance
let basic_kyc = BasicKyc::new(&client());

// Submit a Basic KYC verification request
let job_id = basic_kyc.verify(
    "PASSPORT", // ID type
    "AB123456", // ID number
    "NG",       // Country code (ISO 3166-1 alpha-2)
    Some("John".to_string()), // First name (optional)
    Some("Doe".to_string()),  // Last name (optional)
    Some("1990-01-01".to_string()), // Date of birth (optional)
).await.expect("Failed to submit Basic KYC verification");

println!("Job ID: {}", job_id);
```

### Enhanced KYC

```rust
use smile_id::products::EnhancedKyc;

// Create an Enhanced KYC instance
let enhanced_kyc = EnhancedKyc::new(&client());

// Submit an Enhanced KYC verification request
let job_id = enhanced_kyc.verify(
    "PASSPORT", // ID type
    "AB123456", // ID number
    "NG",       // Country code
    "John",     // First name
    "Doe",      // Last name
    "1990-01-01", // Date of birth
).await.expect("Failed to submit Enhanced KYC verification");

println!("Job ID: {}", job_id);
```

### Biometric KYC

```rust
use smile_id::products::BiometricKyc;
use smile_id::utils;

// Create a Biometric KYC instance
let biometric_kyc = BiometricKyc::new(&client());

// Encode a selfie image as base64
let selfie_image = utils::encode_image_file("path/to/selfie.jpg")
    .expect("Failed to encode selfie image");

// Submit a Biometric KYC verification request
let job_id = biometric_kyc.verify(
    "PASSPORT", // ID type
    "AB123456", // ID number
    "NG",       // Country code
    "John",     // First name
    "Doe",      // Last name
    "1990-01-01", // Date of birth
    selfie_image, // Selfie image (base64 encoded)
).await.expect("Failed to submit Biometric KYC verification");

println!("Job ID: {}", job_id);
```

### Document Verification

```rust
use smile_id::products::DocumentVerification;
use smile_id::utils;

// Create a Document Verification instance
let document_verification = DocumentVerification::new(&client());

// Encode document images as base64
let front_image = utils::encode_image_file("path/to/front.jpg")
    .expect("Failed to encode front image");
let back_image = utils::encode_image_file("path/to/back.jpg")
    .expect("Failed to encode back image");

// Submit a Document Verification request
let job_id = document_verification.verify(
    "PASSPORT", // Document type
    "NG",       // Country code
    vec![front_image, back_image], // Document images (base64 encoded)
).await.expect("Failed to submit Document Verification");

println!("Job ID: {}", job_id);
```

### SmartSelfie™ Authentication

```rust
use smile_id::products::SmartSelfieAuth;
use smile_id::utils;

// Create a SmartSelfie™ Authentication instance
let smartselfie_auth = SmartSelfieAuth::new(&client());

// Encode a selfie image as base64
let selfie_image = utils::encode_image_file("path/to/selfie.jpg")
    .expect("Failed to encode selfie image");

// Submit a SmartSelfie™ Authentication request
let job_id = smartselfie_auth.authenticate(
    "user-123", // User ID
    "job-456",  // Job ID (from a previous verification)
    selfie_image, // Selfie image (base64 encoded)
).await.expect("Failed to submit SmartSelfie™ Authentication");

println!("Job ID: {}", job_id);
```

### Business Verification

```rust
use smile_id::products::BusinessVerification;

// Create a Business Verification instance
let business_verification = BusinessVerification::new(&client());

// Submit a Business Verification request
let job_id = business_verification.verify(
    "Acme Inc", // Business name
    "RC123456", // Registration number
    "NG",       // Country code
).await.expect("Failed to submit Business Verification");

println!("Job ID: {}", job_id);
```

### Job Status

```rust
// Get the status of a job
let job_status = client.get_job_status(
    "user-123", // User ID
    "job-456",  // Job ID
    Some(true), // Include history (optional)
    Some(true), // Include image links (optional)
).await.expect("Failed to get job status");

println!("Job status: {:?}", job_status.job_status);
println!("Job complete: {}", job_status.job_complete);
println!("Job success: {}", job_status.job_success);
```

### Blocking API

The library also provides a blocking API for use in synchronous contexts:

```rust
use smile_id::config::Config;
use smile_id::api::blocking::ApiClient;
use smile_id::products::basic_kyc::blocking::BasicKyc;

// Create a configuration
let config = Config::new("your-api-key", "your-partner-id");

// Create a blocking API client
let client = ApiClient::new(config).expect("Failed to create API client");

// Create a blocking Basic KYC instance
let basic_kyc = BasicKyc::new(&client());

// Submit a Basic KYC verification request
let job_id = basic_kyc.verify(
    "PASSPORT", // ID type
    "AB123456", // ID number
    "NG",       // Country code
    Some("John".to_string()), // First name (optional)
    Some("Doe".to_string()),  // Last name (optional)
    Some("1990-01-01".to_string()), // Date of birth (optional)
).expect("Failed to submit Basic KYC verification");

println!("Job ID: {}", job_id);
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Smile ID](https://usesmileid.com) for their identity verification services
- [Smile Identity Core JS](https://github.com/smileidentity/smile-identity-core-js) for the reference implementation
