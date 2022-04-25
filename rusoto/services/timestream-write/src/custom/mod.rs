use crate::{
    DescribeEndpointsError, DescribeEndpointsResponse, TimestreamWrite, TimestreamWriteClient,
    WriteRecordsError, WriteRecordsRequest,
};
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::{Client, DispatchSignedRequest, Region, RusotoError};
use std::cmp::max;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Endpoint {
    address: String,
    not_after: Instant,
}

/// Amazon Timestream use the concept of specific data-plane endpoints for write request. Naively
/// calling [TimestreamWriteClient::write_records] does not work and will return an error as the API
/// operation [`WriteRecords`](https://docs.aws.amazon.com/timestream/latest/developerguide/API_WriteRecords.html)
/// is not available on the (default) control-plane API endpoint.
/// Instead [Endpoint Discovery](https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html)
/// must be used to obtain temporary data-plan endpoints, which are then used to call the
/// `WriteRecords` operation.
///
/// The [`TimestreamProducer`] is a wrapper from Rusoto which takes care of data-plane endpoint
/// discovery, caching and renewal and can be use for writing time series data to Amazon Timestream.
///
/// # Examples
///
/// ```
/// use rusoto_core::{Region, RusotoError};
/// use futures::{Stream, StreamExt};
/// use rusoto_timestream_write::{TimestreamProducer, Record, WriteRecordsRequest, WriteRecordsError};
///
/// async fn produce(mut records: impl Stream<Item=WriteRecordsRequest> + Unpin) -> Result<(), RusotoError<WriteRecordsError>> {
///     let mut producer = TimestreamProducer::new(Region::EuCentral1);
///     while let Some(request) = records.next().await {
///         producer.write_records(request).await?;
///     }
///     Ok(())
/// }
/// ```
pub struct TimestreamProducer {
    region: Region,
    client: TimestreamWriteClient,
    endpoint_info: Option<Endpoint>,
}

impl TimestreamProducer {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: Region) -> Self {
        Self {
            client: TimestreamWriteClient::new(region.clone()),
            region,
            endpoint_info: None,
        }
    }

    pub fn new_with<P, D>(request_dispatcher: D, credentials_provider: P, region: Region) -> Self
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Self {
            client: TimestreamWriteClient::new_with(
                request_dispatcher,
                credentials_provider,
                region.clone(),
            ),
            region,
            endpoint_info: None,
        }
    }

    pub fn new_with_client(client: Client, region: Region) -> Self {
        Self {
            client: TimestreamWriteClient::new_with_client(client, region.clone()),
            region,
            endpoint_info: None,
        }
    }

    async fn endpoint_client(
        &mut self,
    ) -> Result<TimestreamWriteClient, RusotoError<DescribeEndpointsError>> {
        match &self.endpoint_info {
            Some(e) if e.not_after > Instant::now() => {
                Ok(TimestreamWriteClient::new(Region::Custom {
                    name: self.region.name().to_string(),
                    endpoint: e.address.clone(),
                }))
            }
            _ => {
                let mut response: DescribeEndpointsResponse =
                    self.client.describe_endpoints().await?;
                if let Some(e) = response.endpoints.pop() {
                    self.endpoint_info = Some(Endpoint {
                        address: e.address.clone(),
                        not_after: Instant::now().add(Duration::from_secs(
                            // Subtract some seconds from TTL to make sure endpoint data is
                            // refreshed before it expires.
                            max(5, max(1, e.cache_period_in_minutes) as u64 * 60u64 - 60),
                        )),
                    });
                    Ok(TimestreamWriteClient::new(Region::Custom {
                        name: self.region.name().to_string(),
                        endpoint: e.address.clone(),
                    }))
                } else {
                    // This should actually never happen, as above describe_endpoints should return
                    // an error and no empty result.
                    Err(RusotoError::Service(
                        DescribeEndpointsError::InternalServer(
                            "Amazon Timestream endpoint discovery returned empty endpoint list"
                                .to_string(),
                        ),
                    ))
                }
            }
        }
    }

    /// <p>The WriteRecords operation enables you to write your time series data into Timestream. You can specify a single data point or a batch of data points to be inserted into the system. Timestream offers you with a flexible schema that auto detects the column names and data types for your Timestream tables based on the dimension names and data types of the data points you specify when invoking writes into the database. Timestream support eventual consistency read semantics. This means that when you query data immediately after writing a batch of data into Timestream, the query results might not reflect the results of a recently completed write operation. The results may also include some stale data. If you repeat the query request after a short time, the results should return the latest data. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    pub async fn write_records(
        &mut self,
        input: WriteRecordsRequest,
    ) -> Result<(), RusotoError<WriteRecordsError>> {
        let client = self.endpoint_client().await.map_err(|e| match e {
            // Remap endpoint discovery errors into WriteRecordErrors
            RusotoError::Service(DescribeEndpointsError::Throttling(s)) => {
                RusotoError::Service(WriteRecordsError::Throttling(s))
            }
            RusotoError::Service(DescribeEndpointsError::InternalServer(s)) => {
                RusotoError::Service(WriteRecordsError::InternalServer(s))
            }
            // pass on all non-service specific errors as is.
            RusotoError::Blocking => RusotoError::Blocking,
            RusotoError::Credentials(e) => RusotoError::Credentials(e),
            RusotoError::HttpDispatch(e) => RusotoError::HttpDispatch(e),
            RusotoError::ParseError(e) => RusotoError::ParseError(e),
            RusotoError::Unknown(e) => RusotoError::Unknown(e),
            RusotoError::Validation(e) => RusotoError::Validation(e),
        })?;
        client.write_records(input).await
    }
}
