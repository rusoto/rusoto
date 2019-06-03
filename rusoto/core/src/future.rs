use std::time::Duration;

use futures::sync::oneshot::spawn;
use futures::{Async, Future, IntoFuture, Poll};
use tokio::runtime::Runtime;

use super::client::{SignAndDispatchError, TimeoutFuture};
use super::error::{RusotoError, RusotoResult};
use super::request::HttpResponse;

lazy_static! {
    static ref FALLBACK_RUNTIME: Runtime = Runtime::new().unwrap();
}

/// Future that is returned from all rusoto service APIs.
///
/// ## Mocking
///
/// To mock service traits, you can use [`RusotoFuture::from_future`] to create
/// [`RusotoFuture`] instance. You can also use the [`From`] implementation on
/// the [`Result`] value.
///
/// ```rust,ignore
/// # // TODO: remove ignore when the cyclic dependency issue has been fixed
/// # // https://github.com/rusoto/rusoto/pull/1141#issuecomment-421865362
/// # extern crate futures;
/// # extern crate rusoto_core;
/// # extern crate rusoto_s3;
/// # extern crate tokio_timer;
/// #
/// use std::time::{Duration, Instant};
/// use futures::prelude::*;
/// use rusoto_core::RusotoFuture;
/// use rusoto_s3::*;
/// use tokio_timer::Delay;
///
/// pub struct S3Mock;
///
/// impl S3 for S3Mock {
///     fn abort_multipart_upload(
///         &self,
///         _input: AbortMultipartUploadRequest,
///     ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError> {
///         unimplemented!();
///     }
///
///     // ...
///
///     fn put_object(&self, input: PutObjectRequest) -> RusotoFuture<PutObjectOutput, PutObjectError> {
///         if input.bucket == "foo" {
///             let deadline = Instant::now() + Duration::from_secs(3);
///             let output = PutObjectOutput {
///                 ..Default::default()
///             };
///             RusotoFuture::from_future(
///                 Delay::new(deadline)
///                     .map_err(|e| PutObjectError::Unknown(e.to_string()))
///                     .map(|_| output)
///             )
///         } else {
///             Err(PutObjectError::Validation("Invalid bucket".to_string())).into()
///         }
///     }
///
///     // ...
/// #   fn complete_multipart_upload(&self, _: CompleteMultipartUploadRequest) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError> { unimplemented!() }
/// #   fn copy_object(&self, _: CopyObjectRequest) -> RusotoFuture<CopyObjectOutput, CopyObjectError> { unimplemented!() }
/// #   fn create_bucket(&self, _: CreateBucketRequest) -> RusotoFuture<CreateBucketOutput, CreateBucketError> { unimplemented!() }
/// #   fn create_multipart_upload(&self, _: CreateMultipartUploadRequest) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError> { unimplemented!() }
/// #   fn delete_bucket(&self, _: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError> { unimplemented!() }
/// #   fn delete_bucket_analytics_configuration(&self, _: DeleteBucketAnalyticsConfigurationRequest) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError> { unimplemented!() }
/// #   fn delete_bucket_cors(&self, _: DeleteBucketCorsRequest) -> RusotoFuture<(), DeleteBucketCorsError> { unimplemented!() }
/// #   fn delete_bucket_encryption(&self, _: DeleteBucketEncryptionRequest) -> RusotoFuture<(), DeleteBucketEncryptionError> { unimplemented!() }
/// #   fn delete_bucket_inventory_configuration(&self, _: DeleteBucketInventoryConfigurationRequest) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError> { unimplemented!() }
/// #   fn delete_bucket_lifecycle(&self, _: DeleteBucketLifecycleRequest) -> RusotoFuture<(), DeleteBucketLifecycleError> { unimplemented!() }
/// #   fn delete_bucket_metrics_configuration(&self, _: DeleteBucketMetricsConfigurationRequest) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError> { unimplemented!() }
/// #   fn delete_bucket_policy(&self, _: DeleteBucketPolicyRequest) -> RusotoFuture<(), DeleteBucketPolicyError> { unimplemented!() }
/// #   fn delete_bucket_replication(&self, _: DeleteBucketReplicationRequest) -> RusotoFuture<(), DeleteBucketReplicationError> { unimplemented!() }
/// #   fn delete_bucket_tagging(&self, _: DeleteBucketTaggingRequest) -> RusotoFuture<(), DeleteBucketTaggingError> { unimplemented!() }
/// #   fn delete_bucket_website(&self, _: DeleteBucketWebsiteRequest) -> RusotoFuture<(), DeleteBucketWebsiteError> { unimplemented!() }
/// #   fn delete_object(&self, _: DeleteObjectRequest) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError> { unimplemented!() }
/// #   fn delete_object_tagging(&self, _: DeleteObjectTaggingRequest) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError> { unimplemented!() }
/// #   fn delete_objects(&self, _: DeleteObjectsRequest) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError> { unimplemented!() }
/// #   fn get_bucket_accelerate_configuration(&self, _: GetBucketAccelerateConfigurationRequest) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError> { unimplemented!() }
/// #   fn get_bucket_acl(&self, _: GetBucketAclRequest) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError> { unimplemented!() }
/// #   fn get_bucket_analytics_configuration(&self, _: GetBucketAnalyticsConfigurationRequest) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError> { unimplemented!() }
/// #   fn get_bucket_cors(&self, _: GetBucketCorsRequest) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError> { unimplemented!() }
/// #   fn get_bucket_encryption(&self, _: GetBucketEncryptionRequest) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError> { unimplemented!() }
/// #   fn get_bucket_inventory_configuration(&self, _: GetBucketInventoryConfigurationRequest) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError> { unimplemented!() }
/// #   fn get_bucket_lifecycle(&self, _: GetBucketLifecycleRequest) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError> { unimplemented!() }
/// #   fn get_bucket_lifecycle_configuration(&self, _: GetBucketLifecycleConfigurationRequest) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError> { unimplemented!() }
/// #   fn get_bucket_location(&self, _: GetBucketLocationRequest) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError> { unimplemented!() }
/// #   fn get_bucket_logging(&self, _: GetBucketLoggingRequest) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError> { unimplemented!() }
/// #   fn get_bucket_metrics_configuration(&self, _: GetBucketMetricsConfigurationRequest) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> { unimplemented!() }
/// #   fn get_bucket_notification(&self, _: GetBucketNotificationConfigurationRequest) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError> { unimplemented!() }
/// #   fn get_bucket_notification_configuration(&self, _: GetBucketNotificationConfigurationRequest) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError> { unimplemented!() }
/// #   fn get_bucket_policy(&self, _: GetBucketPolicyRequest) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError> { unimplemented!() }
/// #   fn get_bucket_replication(&self, _: GetBucketReplicationRequest) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError> { unimplemented!() }
/// #   fn get_bucket_request_payment(&self, _: GetBucketRequestPaymentRequest) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> { unimplemented!() }
/// #   fn get_bucket_tagging(&self, _: GetBucketTaggingRequest) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError> { unimplemented!() }
/// #   fn get_bucket_versioning(&self, _: GetBucketVersioningRequest) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError> { unimplemented!() }
/// #   fn get_bucket_website(&self, _: GetBucketWebsiteRequest) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError> { unimplemented!() }
/// #   fn get_object(&self, _: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError> { unimplemented!() }
/// #   fn get_object_acl(&self, _: GetObjectAclRequest) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError> { unimplemented!() }
/// #   fn get_object_tagging(&self, _: GetObjectTaggingRequest) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError> { unimplemented!() }
/// #   fn get_object_torrent(&self, _: GetObjectTorrentRequest) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError> { unimplemented!() }
/// #   fn head_bucket(&self, _: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError> { unimplemented!() }
/// #   fn head_object(&self, _: HeadObjectRequest) -> RusotoFuture<HeadObjectOutput, HeadObjectError> { unimplemented!() }
/// #   fn list_bucket_analytics_configurations(&self, _: ListBucketAnalyticsConfigurationsRequest) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError> { unimplemented!() }
/// #   fn list_bucket_inventory_configurations(&self, _: ListBucketInventoryConfigurationsRequest) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError> { unimplemented!() }
/// #   fn list_bucket_metrics_configurations(&self, _: ListBucketMetricsConfigurationsRequest) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError> { unimplemented!() }
/// #   fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError> { unimplemented!() }
/// #   fn list_multipart_uploads(&self, _: ListMultipartUploadsRequest) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError> { unimplemented!() }
/// #   fn list_object_versions(&self, _: ListObjectVersionsRequest) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError> { unimplemented!() }
/// #   fn list_objects(&self, _: ListObjectsRequest) -> RusotoFuture<ListObjectsOutput, ListObjectsError> { unimplemented!() }
/// #   fn list_objects_v2(&self, _: ListObjectsV2Request) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error> { unimplemented!() }
/// #   fn list_parts(&self, _: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError> { unimplemented!() }
/// #   fn put_bucket_accelerate_configuration(&self, _: PutBucketAccelerateConfigurationRequest) -> RusotoFuture<(), PutBucketAccelerateConfigurationError> { unimplemented!() }
/// #   fn put_bucket_acl(&self, _: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError> { unimplemented!() }
/// #   fn put_bucket_analytics_configuration(&self, _: PutBucketAnalyticsConfigurationRequest) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError> { unimplemented!() }
/// #   fn put_bucket_cors(&self, _: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError> { unimplemented!() }
/// #   fn put_bucket_encryption(&self, _: PutBucketEncryptionRequest) -> RusotoFuture<(), PutBucketEncryptionError> { unimplemented!() }
/// #   fn put_bucket_inventory_configuration(&self, _: PutBucketInventoryConfigurationRequest) -> RusotoFuture<(), PutBucketInventoryConfigurationError> { unimplemented!() }
/// #   fn put_bucket_lifecycle(&self, _: PutBucketLifecycleRequest) -> RusotoFuture<(), PutBucketLifecycleError> { unimplemented!() }
/// #   fn put_bucket_lifecycle_configuration(&self, _: PutBucketLifecycleConfigurationRequest) -> RusotoFuture<(), PutBucketLifecycleConfigurationError> { unimplemented!() }
/// #   fn put_bucket_logging(&self, _: PutBucketLoggingRequest) -> RusotoFuture<(), PutBucketLoggingError> { unimplemented!() }
/// #   fn put_bucket_metrics_configuration(&self, _: PutBucketMetricsConfigurationRequest) -> RusotoFuture<(), PutBucketMetricsConfigurationError> { unimplemented!() }
/// #   fn put_bucket_notification(&self, _: PutBucketNotificationRequest) -> RusotoFuture<(), PutBucketNotificationError> { unimplemented!() }
/// #   fn put_bucket_notification_configuration(&self, _: PutBucketNotificationConfigurationRequest) -> RusotoFuture<(), PutBucketNotificationConfigurationError> { unimplemented!() }
/// #   fn put_bucket_policy(&self, _: PutBucketPolicyRequest) -> RusotoFuture<(), PutBucketPolicyError> { unimplemented!() }
/// #   fn put_bucket_replication(&self, _: PutBucketReplicationRequest) -> RusotoFuture<(), PutBucketReplicationError> { unimplemented!() }
/// #   fn put_bucket_request_payment(&self, _: PutBucketRequestPaymentRequest) -> RusotoFuture<(), PutBucketRequestPaymentError> { unimplemented!() }
/// #   fn put_bucket_tagging(&self, _: PutBucketTaggingRequest) -> RusotoFuture<(), PutBucketTaggingError> { unimplemented!() }
/// #   fn put_bucket_versioning(&self, _: PutBucketVersioningRequest) -> RusotoFuture<(), PutBucketVersioningError> { unimplemented!() }
/// #   fn put_bucket_website(&self, _: PutBucketWebsiteRequest) -> RusotoFuture<(), PutBucketWebsiteError> { unimplemented!() }
/// #   fn put_object_acl(&self, _: PutObjectAclRequest) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError> { unimplemented!() }
/// #   fn put_object_tagging(&self, _: PutObjectTaggingRequest) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError> { unimplemented!() }
/// #   fn restore_object(&self, _: RestoreObjectRequest) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError> { unimplemented!() }
/// #   fn select_object_content(&self, _: SelectObjectContentRequest) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError> { unimplemented!() }
/// #   fn upload_part(&self, _: UploadPartRequest) -> RusotoFuture<UploadPartOutput, UploadPartError> { unimplemented!() }
/// #   fn upload_part_copy(&self, _: UploadPartCopyRequest) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError> { unimplemented!() }
/// }
/// ```
pub struct RusotoFuture<T, E> {
    state: Option<RusotoFutureState<T, E>>,
}

pub fn new<T, E>(
    future: Box<dyn TimeoutFuture<Item = HttpResponse, Error = SignAndDispatchError> + Send>,
    handler: fn(HttpResponse) -> Box<dyn Future<Item = T, Error = RusotoError<E>> + Send>,
) -> RusotoFuture<T, E> {
    RusotoFuture {
        state: Some(RusotoFutureState::SignAndDispatch { future, handler }),
    }
}

impl<T, E> RusotoFuture<T, E> {
    /// Set the timeout on the future to the provided duration.
    ///
    /// Unlike `set_timeout` this method can be easily chained:
    ///
    /// ```rust,ignore
    /// # // TODO: remove ignore when the cyclic dependency issue has been fixed
    /// # // https://github.com/rusoto/rusoto/pull/1141#issuecomment-421865362
    /// # extern crate rusoto_core;
    /// # extern crate rusoto_s3;
    /// #
    /// # use std::time::Duration;
    /// # use rusoto_core::Region;
    /// # use rusoto_s3::{S3, S3Client};
    /// #
    /// # let s3 = S3Client::new(Region::default());
    /// #
    /// let future = s3.list_buckets()
    ///     .with_timeout(Duration::from_secs(10));
    /// ```
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.set_timeout(timeout);
        self
    }

    /// Set the timeout on the future to the provided duration.
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn set_timeout(&mut self, timeout: Duration) {
        if let Some(RusotoFutureState::SignAndDispatch { ref mut future, .. }) = self.state {
            future.set_timeout(timeout);
        }
    }

    /// Clear the timeout on the future.
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn clear_timeout(&mut self) {
        if let Some(RusotoFutureState::SignAndDispatch { ref mut future, .. }) = self.state {
            future.clear_timeout();
        }
    }

    /// Blocks the current thread until the future has resolved.
    ///
    /// This is meant to provide a simple way for non-async consumers
    /// to work with rusoto.
    pub fn sync(self) -> RusotoResult<T, E>
    where
        T: Send + 'static,
        E: Send + 'static,
    {
        spawn(self, &FALLBACK_RUNTIME.executor()).wait()
    }

    /// Wraps the provided future, mainly to mock the service response.
    ///
    /// ## Caution
    ///
    /// This is not intended to be used outside of the test case.
    /// In production, [`Box`] is recommended.
    pub fn from_future<F>(fut: F) -> Self
    where
        F: IntoFuture<Item = T, Error = RusotoError<E>>,
        F::Future: Send + 'static,
    {
        let fut = fut.into_future();
        RusotoFuture {
            state: Some(RusotoFutureState::RunningResponseHandler(Box::new(fut))),
        }
    }
}

impl<T, E> Future for RusotoFuture<T, E> {
    type Item = T;
    type Error = RusotoError<E>;

    fn poll(&mut self) -> Poll<T, RusotoError<E>> {
        match self.state.take().unwrap() {
            RusotoFutureState::SignAndDispatch {
                mut future,
                handler,
            } => match future.poll() {
                Err(SignAndDispatchError::Credentials(err)) => Err(err.into()),
                Err(SignAndDispatchError::Dispatch(err)) => Err(err.into()),
                Ok(Async::Ready(response)) => {
                    self.state = Some(RusotoFutureState::RunningResponseHandler(handler(response)));
                    self.poll()
                }
                Ok(Async::NotReady) => {
                    self.state = Some(RusotoFutureState::SignAndDispatch { future, handler });
                    Ok(Async::NotReady)
                }
            },
            RusotoFutureState::RunningResponseHandler(mut future) => match future.poll()? {
                Async::Ready(value) => Ok(Async::Ready(value)),
                Async::NotReady => {
                    self.state = Some(RusotoFutureState::RunningResponseHandler(future));
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

enum RusotoFutureState<T, E> {
    SignAndDispatch {
        future: Box<dyn TimeoutFuture<Item = HttpResponse, Error = SignAndDispatchError> + Send>,
        handler: fn(HttpResponse) -> Box<dyn Future<Item = T, Error = RusotoError<E>> + Send>,
    },
    RunningResponseHandler(Box<dyn Future<Item = T, Error = RusotoError<E>> + Send>),
}

impl<T: Send + 'static, E: Send + 'static> From<RusotoResult<T, E>> for RusotoFuture<T, E> {
    fn from(value: RusotoResult<T, E>) -> Self {
        RusotoFuture::from_future(value)
    }
}

#[test]
fn rusoto_future_is_send() {
    fn is_send<T: Send>() {}

    is_send::<RusotoFuture<(), ()>>();
}

#[test]
fn rusoto_future_from_ok() {
    use std::error::Error;
    let fut: RusotoFuture<i32, Box<Error + Send + Sync>> = RusotoFuture::from(Ok(42));
    assert_eq!(fut.sync().unwrap(), 42);
}

#[test]
fn rusoto_future_from_err() {
    use std::error::Error;
    let fut: RusotoFuture<i32, Box<Error + Send + Sync>> = RusotoFuture::from(
        "ab".parse::<i32>()
            .map_err(|e| RusotoError::Service(e.into())),
    );
    assert!(fut.sync().is_err());
}

#[test]
fn rusoto_future_from_delay() {
    use std::error::Error;
    use std::time::{Duration, Instant};
    use tokio_timer::Delay;
    let deadline = Instant::now() + Duration::from_millis(500);
    let delay = Delay::new(deadline);
    let fut: RusotoFuture<i32, Box<Error + Send + Sync>> = RusotoFuture::from_future(
        delay
            .map_err(|e| RusotoError::Service(e.into()))
            .map(|_| 42),
    );
    assert_eq!(fut.sync().unwrap(), 42);
    assert!(deadline <= Instant::now());
}
