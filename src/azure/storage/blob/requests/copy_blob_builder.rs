use crate::azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use crate::azure::core::lease::LeaseId;
use crate::azure::core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired,
    ContainerNameSupport, CopySourceOption, CopySourceSupport, LeaseIdOption, LeaseIdSupport, MetadataOption, MetadataSupport, No,
    TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use crate::azure::storage::blob::generate_blob_uri;
use crate::azure::storage::blob::responses::CopyBlobResponse;
use crate::azure::storage::client::Client;
use futures::future::{done, ok};
use futures::prelude::*;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_body: PhantomData<BodySet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    copy_source: Option<&'a str>,
    timeout: Option<u64>,
    content_type: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    cache_control: Option<&'a str>,
    content_md5: Option<&'a [u8]>,
    content_disposition: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a> CopyBlobBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> CopyBlobBuilder<'a, No, No, No> {
        CopyBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_body: PhantomData {},
            copy_source: None,
            timeout: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            content_md5: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequired<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BodySet> ContainerNameRequired<'a> for CopyBlobBuilder<'a, Yes, BlobNameSet, BodySet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BodySet> BlobNameRequired<'a> for CopyBlobBuilder<'a, ContainerNameSet, Yes, BodySet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutOption for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> MetadataOption<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdOption<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdOption<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContainerNameSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, Yes, BlobNameSet, BodySet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            copy_source: self.copy_source,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BlobNameSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, Yes, BodySet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            copy_source: self.copy_source,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> CopySourceSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_copy_source(self, copy_source: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            copy_source: Some(copy_source),
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> CopySourceOption<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn copy_source(&self) -> Option<&'a str> {
        self.copy_source
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutSupport for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            copy_source: self.copy_source,
            timeout: Some(timeout),
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> MetadataSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            copy_source: self.copy_source,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: Some(metadata),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            copy_source: self.copy_source,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdSupport<'a> for CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            copy_source: self.copy_source,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, BodySet> CopyBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
}

impl<'a> CopyBlobBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub fn finalize(self) -> impl Future<Item = CopyBlobResponse, Error = AzureError> {
        let mut uri = generate_blob_uri(&self, None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let req = self.client().perform_request(
            &uri,
            &Method::PUT,
            |ref mut request| {
                CopySourceOption::add_header(&self, request);
                MetadataOption::add_header(&self, request);
                LeaseIdOption::add_header(&self, request);
                ClientRequestIdOption::add_header(&self, request);
            },
            Some(&[]),
        );

        done(req)
            .from_err()
            .and_then(move |response| check_status_extract_headers_and_body(response, StatusCode::ACCEPTED))
            .and_then(move |(headers, _body)| done(CopyBlobResponse::from_headers(&headers)).and_then(ok))
    }
}
