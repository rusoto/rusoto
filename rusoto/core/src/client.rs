use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use crate::credential::{CredentialsError, DefaultCredentialsProvider, ProvideAwsCredentials};
use crate::request::{DispatchSignedRequest, HttpClient, HttpDispatchError, HttpResponse};
use crate::signature::SignedRequest;

use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::future::FutureExt as _;

lazy_static! {
    static ref SHARED_CLIENT: Mutex<Weak<ClientInner<DefaultCredentialsProvider, HttpClient>>> =
        Mutex::new(Weak::new());
}

/// Re-usable logic for all clients.
#[derive(Clone)]
pub struct Client {
    inner: Arc<dyn SignAndDispatch + Send + Sync>,
}

impl Client {
    /// Return the shared default client.
    pub fn shared() -> Self {
        let mut lock = SHARED_CLIENT.lock().unwrap();
        if let Some(inner) = lock.upgrade() {
            return Client { inner };
        }
        let credentials_provider =
            DefaultCredentialsProvider::new().expect("failed to create credentials provider");
        let dispatcher = HttpClient::new().expect("failed to create request dispatcher");
        let inner = Arc::new(ClientInner {
            credentials_provider: Arc::new(credentials_provider),
            dispatcher: Arc::new(dispatcher),
        });
        *lock = Arc::downgrade(&inner);
        Client { inner }
    }

    /// Create a client from a credentials provider and request dispatcher.
    pub fn new_with<P, D>(credentials_provider: P, dispatcher: D) -> Self
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        let inner = ClientInner {
            credentials_provider: Arc::new(credentials_provider),
            dispatcher: Arc::new(dispatcher),
        };
        Client {
            inner: Arc::new(inner),
        }
    }

    /// Fetch credentials, sign the request and dispatch it.
    pub async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, SignAndDispatchError>
    {
        self.inner.sign_and_dispatch(request, None).await
    }
}

/// Error that occurs during `sign_and_dispatch`
#[derive(Debug, PartialEq)]
pub enum SignAndDispatchError {
    /// Error was due to bad credentials
    Credentials(CredentialsError),
    /// Error was due to http dispatch
    Dispatch(HttpDispatchError),
}

#[async_trait]
trait SignAndDispatch {
    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
        timeout: Option<Duration>,
    ) -> Result<HttpResponse, SignAndDispatchError>;
}

struct ClientInner<P, D> {
    credentials_provider: Arc<P>,
    dispatcher: Arc<D>,
}

impl<P, D> Clone for ClientInner<P, D> {
    fn clone(&self) -> Self {
        ClientInner {
            credentials_provider: self.credentials_provider.clone(),
            dispatcher: self.dispatcher.clone(),
        }
    }
}

async fn sign_and_dispatch<P, D>(
    client: ClientInner<P, D>,
    request: SignedRequest,
    timeout: Option<Duration>
) -> Result<HttpResponse, SignAndDispatchError>
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
{
    let f = client.credentials_provider.credentials();
    let credentials = if let Some(to) = timeout {
        match f.timeout(to).await {
            Err(_e) => {
                let err = CredentialsError {
                    message: "Timeout getting credentials".to_owned(),
                };
                return Err(SignAndDispatchError::Credentials(err));
            },
            Ok(try_creds) => try_creds,
        }
    } else {
        f.await
    };
    let credentials = credentials.map_err(SignAndDispatchError::Credentials)?;
    let mut request = request;
    request.sign_with_plus(&credentials, true);
    client.dispatcher.dispatch(request, timeout).await.map_err(SignAndDispatchError::Dispatch)
}

#[async_trait]
impl<P, D> SignAndDispatch for ClientInner<P, D>
where
    P: ProvideAwsCredentials + Send + Sync + 'static,
    D: DispatchSignedRequest + Send + Sync + 'static,
{
    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
        timeout: Option<Duration>,
    ) -> Result<HttpResponse, SignAndDispatchError> {
        sign_and_dispatch(
            self.clone(),
            request,
            timeout,
        ).await
    }
}

#[test]
fn client_is_send_and_sync() {
    fn is_send_and_sync<T: Send + Sync>() {}

    is_send_and_sync::<Client>();
}
