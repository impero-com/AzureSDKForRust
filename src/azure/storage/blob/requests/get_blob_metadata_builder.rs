use crate::azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use crate::azure::core::lease::LeaseId;
use crate::azure::core::util::format_header_value;
use crate::azure::core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired,
    ContainerNameSupport, LeaseIdOption, LeaseIdSupport, No, SnapshotOption, SnapshotSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use crate::azure::storage::blob::responses::GetBlobMetadataResponse;
use crate::azure::storage::blob::{generate_blob_uri, BlobMetadata};
use crate::azure::storage::client::Client;
use chrono::{DateTime, Utc};
use futures::future::done;
use futures::prelude::*;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    snapshot: Option<DateTime<Utc>>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a> GetBlobMetadataBuilder<'a, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> GetBlobMetadataBuilder<'a, No, No> {
        GetBlobMetadataBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            snapshot: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequired<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet> ContainerNameRequired<'a> for GetBlobMetadataBuilder<'a, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> BlobNameRequired<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SnapshotOption for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn snapshot(&self) -> Option<DateTime<Utc>> {
        self.snapshot
    }
}

impl<'a, ContainerNameSet, BlobNameSet> TimeoutOption for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdOption<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdOption<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContainerNameSupport<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, Yes, BlobNameSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BlobNameSupport<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, ContainerNameSet, Yes>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SnapshotSupport for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> TimeoutSupport for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a> for GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetBlobMetadataBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet> GetBlobMetadataBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
}

impl<'a> GetBlobMetadataBuilder<'a, Yes, Yes> {
    #[inline]
    pub fn finalize(self) -> impl Future<Item = GetBlobMetadataResponse, Error = AzureError> {
        let container_name = self.container_name().to_owned();
        let blob_name = self.blob_name().to_owned();
        let snapshot_time = self.snapshot();

        let mut uri = generate_blob_uri(&self, Some("comp=metadata"));

        if let Some(snapshot) = SnapshotOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, snapshot);
        }
        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let req = self.client().perform_request(
            &uri,
            &Method::HEAD,
            |ref mut request| {
                LeaseIdOption::add_header(&self, request);
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::OK))
            .and_then(move |(headers, body)| {
                done(BlobMetadata::from_headers(&blob_name, &container_name, snapshot_time, &headers))
                    .and_then(move |blob_metadata| done(GetBlobMetadataResponse::from_response(&headers, blob_metadata, &body)))
            })
    }
}
