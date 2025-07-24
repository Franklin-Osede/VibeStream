use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CDNError {
    FileTooLarge { size: u64, max_size: u64 },
    ContentNotFound { content_id: uuid::Uuid },
    UploadFailed { reason: String },
    InvalidContentType { content_type: String },
    CacheError { reason: String },
    NetworkError { reason: String },
}

impl fmt::Display for CDNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CDNError::FileTooLarge { size, max_size } => {
                write!(f, "File size {}MB exceeds maximum allowed size of {}MB", size, max_size)
            }
            CDNError::ContentNotFound { content_id } => {
                write!(f, "Content with ID {} not found", content_id)
            }
            CDNError::UploadFailed { reason } => {
                write!(f, "Upload failed: {}", reason)
            }
            CDNError::InvalidContentType { content_type } => {
                write!(f, "Invalid content type: {}", content_type)
            }
            CDNError::CacheError { reason } => {
                write!(f, "Cache error: {}", reason)
            }
            CDNError::NetworkError { reason } => {
                write!(f, "Network error: {}", reason)
            }
        }
    }
}

impl Error for CDNError {}

// Remove conflicting implementation - CDNError already implements Error trait
// which provides the From implementation automatically 