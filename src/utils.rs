use base64::{engine::general_purpose, Engine};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};

pub fn encode_image_file(path: impl AsRef<Path>) -> Result<String> {
    let mut file =
        File::open(path).map_err(|e| Error::Other(format!("Failed to open image file: {e}")))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| Error::Other(format!("Failed to read image file: {e}")))?;

    Ok(general_purpose::STANDARD.encode(&buffer))
}

pub fn encode_image_bytes(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

pub fn format_date(year: u16, month: u8, day: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}
