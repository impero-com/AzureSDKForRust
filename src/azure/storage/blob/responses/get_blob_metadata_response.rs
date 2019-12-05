use crate::azure::core::errors::AzureError;
use crate::azure::core::{date_from_headers, request_id_from_headers, RequestId};
use crate::azure::storage::blob::BlobMetadata;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetBlobMetadataResponse {
    pub blob_metadata: BlobMetadata,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl GetBlobMetadataResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        blob_metadata: BlobMetadata,
        body: &[u8],
    ) -> Result<GetBlobMetadataResponse, AzureError> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetBlobMetadataResponse {
            blob_metadata,
            request_id,
            date,
        })
    }
}
