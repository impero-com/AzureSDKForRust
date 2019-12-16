use crate::azure::core::errors::AzureError;
use crate::azure::core::{date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers, RequestId};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct CopyBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl CopyBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<CopyBlobResponse, AzureError> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(CopyBlobResponse {
            etag,
            last_modified,
            request_id,
            date,
        })
    }
}
