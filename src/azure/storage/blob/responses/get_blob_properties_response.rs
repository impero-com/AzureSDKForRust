use crate::azure::core::errors::AzureError;
use crate::azure::core::{date_from_headers, request_id_from_headers, RequestId};
use crate::azure::storage::blob::Blob;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetBlobPropertiesResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl GetBlobPropertiesResponse {
    pub(crate) fn from_response(headers: &HeaderMap, blob: Blob, body: &[u8]) -> Result<GetBlobPropertiesResponse, AzureError> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetBlobPropertiesResponse { blob, request_id, date })
    }
}
