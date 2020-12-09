use crate::{
    CancelQueryError, CancelQueryRequest, CancelQueryResponse, DescribeEndpointsError, QueryError,
    QueryRequest, QueryResponse, TimestreamQuery, TimestreamQueryClient,
};
use rusoto_core::{
    credential::ProvideAwsCredentials, Client, DispatchSignedRequest, Region, RusotoError,
};
use std::cmp::max;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Endpoint {
    address: String,
    expiry: Instant,
}

/// An endpoint-discovery-aware client for the Timestream Query API.
///
/// Amazon Timestream utilizes a segregated architecture to ensure better scaling and traffic isolation properties.
/// Each system segment is managed through multiple endpoints, and your applications must use the correct endpoint
/// while accessing the service. When using this client, these endpoint management tasks are transparently handled for you.
/// However, when accessing the Timestream Query API using the default [`TimestreamQuery`] methods,
/// you will need to manage and map the correct endpoints yourself. This process is called the endpoint discovery pattern,
/// and is described [here](https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html).
///
/// # Examples
///
/// ```
/// use rusoto_core::{Region, RusotoError};
/// use rusoto_timestream_query::{QueryError, QueryResponse, QueryRequest, TimestreamQueryEndpointClient};
///
/// async fn query(query: QueryRequest) -> Result<QueryResponse, RusotoError<QueryError>> {
///     let mut client = TimestreamQueryEndpointClient::new(Region::UsEast1);
///     client.query(query).await
/// }
/// ```
pub struct TimestreamQueryEndpointClient {
    client: TimestreamQueryClient,
    endpoint: Option<Endpoint>,
    region: Region,
}

impl TimestreamQueryEndpointClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: Region) -> Self {
        Self {
            client: TimestreamQueryClient::new(region.clone()),
            endpoint: None,
            region,
        }
    }

    pub fn new_with<P, D>(request_dispatcher: D, credentials_provider: P, region: Region) -> Self
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Self {
            client: TimestreamQueryClient::new_with(
                request_dispatcher,
                credentials_provider,
                region.clone(),
            ),
            endpoint: None,
            region,
        }
    }

    pub fn new_with_client(client: Client, region: Region) -> Self {
        Self {
            client: TimestreamQueryClient::new_with_client(client, region.clone()),
            endpoint: None,
            region,
        }
    }

    async fn update_endpoint(&mut self) -> Result<(), RusotoError<DescribeEndpointsError>> {
        match self.endpoint {
            Some(ref e) if e.expiry > Instant::now() => Ok(()),
            _ => {
                let client = TimestreamQueryClient::new(self.region.clone());
                let endpoint = client.describe_endpoints().await?.endpoints.pop().ok_or(
                    RusotoError::Service(DescribeEndpointsError::InternalServer(
                        "DescribeEndpoints API returned empty endpoint list".to_string(),
                    )),
                )?;

                self.endpoint = Some(Endpoint {
                    address: endpoint.address.clone(),
                    // Subtract 1 minute from the TTL to ensure the endpoint data is refreshed before it expires.
                    expiry: Instant::now().add(Duration::from_secs(max(
                        5,
                        endpoint.cache_period_in_minutes as u64 * 60 - 60,
                    ))),
                });
                self.client = TimestreamQueryClient::new(Region::Custom {
                    name: self.region.name().to_string(),
                    endpoint: endpoint.address,
                });

                Ok(())
            }
        }
    }

    /// <p> Cancels a query that has been issued. Cancellation is guaranteed only if the query has not completed execution before the cancellation request was issued. Because cancellation is an idempotent operation, subsequent cancellation requests will return a <code>CancellationMessage</code>, indicating that the query has already been canceled. </p>
    pub async fn cancel_query(
        &mut self,
        input: CancelQueryRequest,
    ) -> Result<CancelQueryResponse, RusotoError<CancelQueryError>> {
        self.update_endpoint().await.map_err(|e| match e {
            // Remap any DescribeEndpointError to WriteRecordErrors
            RusotoError::Service(DescribeEndpointsError::Throttling(s)) => {
                RusotoError::Service(CancelQueryError::Throttling(s))
            }
            RusotoError::Service(DescribeEndpointsError::InternalServer(s)) => {
                RusotoError::Service(CancelQueryError::InternalServer(s))
            }
            // Pass on all non-service related errors as is.
            RusotoError::Blocking => RusotoError::Blocking,
            RusotoError::Credentials(e) => RusotoError::Credentials(e),
            RusotoError::HttpDispatch(e) => RusotoError::HttpDispatch(e),
            RusotoError::ParseError(e) => RusotoError::ParseError(e),
            RusotoError::Unknown(e) => RusotoError::Unknown(e),
            RusotoError::Validation(e) => RusotoError::Validation(e),
        })?;

        self.client.cancel_query(input).await
    }

    /// <p> Query is a synchronous operation that enables you to execute a query. Query will timeout after 60 seconds. You must update the default timeout in the SDK to support a timeout of 60 seconds. The result set will be truncated to 1MB. Service quotas apply. For more information, see Quotas in the Timestream Developer Guide. </p>
    pub async fn query(
        &mut self,
        input: QueryRequest,
    ) -> Result<QueryResponse, RusotoError<QueryError>> {
        self.update_endpoint().await.map_err(|e| match e {
            // Remap any DescribeEndpointError to WriteRecordErrors
            RusotoError::Service(DescribeEndpointsError::Throttling(s)) => {
                RusotoError::Service(QueryError::Throttling(s))
            }
            RusotoError::Service(DescribeEndpointsError::InternalServer(s)) => {
                RusotoError::Service(QueryError::InternalServer(s))
            }
            // Pass on all non-service related errors as is.
            RusotoError::Blocking => RusotoError::Blocking,
            RusotoError::Credentials(e) => RusotoError::Credentials(e),
            RusotoError::HttpDispatch(e) => RusotoError::HttpDispatch(e),
            RusotoError::ParseError(e) => RusotoError::ParseError(e),
            RusotoError::Unknown(e) => RusotoError::Unknown(e),
            RusotoError::Validation(e) => RusotoError::Validation(e),
        })?;

        self.client.query(input).await
    }
}
