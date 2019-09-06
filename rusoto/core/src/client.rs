use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use crate::{CredentialsError, DefaultCredentialsProvider, ProvideAwsCredentials};
use crate::future::{self, RusotoFuture};
use crate::request::{DispatchSignedRequest, HttpClient, HttpDispatchError, HttpResponse};
use crate::signature::SignedRequest;

use futures::FutureExt;
use lazy_static::lazy_static;
use tokio::future::FutureExt as _;

pub(crate) type SignAndDispatchFuture = Pin<Box<dyn Future<Output = Result<HttpResponse, SignAndDispatchError>> + Send>>;

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
    pub fn sign_and_dispatch<T, E>(
        &self,
        request: SignedRequest,
        response_handler: fn(HttpResponse) -> future::RusotoHandlerFuture<T, E>,
    ) -> RusotoFuture<T, E>
        where
            T: 'static,
            E: 'static
    {
        future::new(self.inner.sign_and_dispatch(request, None), response_handler)
    }
}

#[derive(Debug, PartialEq)]
pub enum SignAndDispatchError {
    Credentials(CredentialsError),
    Dispatch(HttpDispatchError),
}

trait SignAndDispatch {
    fn sign_and_dispatch(
        &self,
        request: SignedRequest,
        timeout: Option<Duration>,
    ) -> SignAndDispatchFuture;
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
            Err(e) => {
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

impl<P, D> SignAndDispatch for ClientInner<P, D>
where
    P: ProvideAwsCredentials + Send + Sync + 'static,
    D: DispatchSignedRequest + Send + Sync + 'static,
{
    fn sign_and_dispatch(
        &self,
        request: SignedRequest,
        timeout: Option<Duration>,
    ) -> SignAndDispatchFuture {
        sign_and_dispatch(
            self.clone(),
            request,
            timeout,
        ).boxed()
    }
}

#[test]
fn client_is_send_and_sync() {
    fn is_send_and_sync<T: Send + Sync>() {}

    is_send_and_sync::<Client>();
}
