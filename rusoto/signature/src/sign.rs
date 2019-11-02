#[cfg(feature = "futures-01")]
use crate::stream::ByteStream;
#[cfg(feature = "futures-01")]
use futures::stream::Stream;
#[cfg(feature = "futures-01")]
use hyper::{Body, Chunk};

#[cfg(feature = "async-await")]
use futures::stream::TryStreamExt;
#[cfg(feature = "async-await")]
use hyper_alpha::{Body, Chunk};

use crate::credential::AwsCredentials;
use crate::{Region, SignedRequest};
use http::{Error, Request};
use std::convert::TryInto;

/// A trait to apply AWS' signatures. At the moment, this trait is only implemented for
/// [http::Request] with Hyper's default body.
pub trait Sign: Sized {
    type Error;
    /// Returns a copy of self, signed
    fn sign(
        self,
        service: &str,
        region: &Region,
        credentials: &AwsCredentials,
    ) -> Result<Self, Self::Error>;
}

#[cfg(feature = "futures-01")]
/// Signs an `http::Request` with a set of AWS credentials.
///
/// The endpoint should be provided as an [http::Uri] _or_ as as value
/// that can be successfully converted into an [http::Uri].
///
/// ```rust,edition2018
/// use rusoto_signature::{Sign, Region, credential::AwsCredentials};
/// use http::request;
/// # use hyper_alpha::Body;
/// # use std::error::Error;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// request::Builder::new()
///     .uri("https://xxx.execute-api.us-east-1.amazon.com/dev/?key=value")
///     .method("GET")
///     .body(Body::default())?
///     .sign(
///         "execute-api",
///         &Region::UsEast1,
///         &AwsCredentials::new("key", "secret", None, None)
///     )?;
/// # Ok(())
/// # }
/// ```
impl Sign for Request<Body> {
    type Error = Error;
    fn sign(
        self,
        service: &str,
        region: &Region,
        credentials: &AwsCredentials,
    ) -> Result<Self, Self::Error> {
        let query = self.uri().query().clone();
        let custom_region = Region::Custom {
            name: region.name().into(),
            endpoint: self
                .uri()
                .to_string()
                .splitn(2, '?')
                .next()
                .expect("invalid uri")
                .to_string(),
        };
        let mut signer = SignedRequest::new(
            self.method().as_ref(),
            service,
            &custom_region,
            Default::default(),
        );
        for (key, value) in self.headers() {
            if let Ok(value) = value.to_str() {
                signer.add_header(key.as_str(), value);
            }
        }
        if let Some(query) = query {
            for param in query.split('&') {
                match &param.splitn(2, '=').collect::<Vec<_>>()[..] {
                    [key, value] => signer.add_param(*key, *value),
                    _ => (),
                }
            }
        }
        let body = self.into_body();
        signer.set_payload_stream(ByteStream::new(body.map(Chunk::into_bytes).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid request: {}", e),
            )
        })));
        signer.sign(credentials);
        signer.try_into()
    }
}

#[cfg(feature = "async-await")]
/// Signs an `http::Request` with a set of AWS credentials.
///
/// The endpoint should be provided as an [http::Uri] _or_ as as value
/// that can be successfully converted into an [http::Uri].
///
/// ```rust,edition2018
/// use rusoto_signature::{Sign, Region, credential::AwsCredentials};
/// use http::request;
/// # use hyper_alpha::Body;
/// # use std::error::Error;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// request::Builder::new()
///     .uri("https://xxx.execute-api.us-east-1.amazon.com/dev/?key=value")
///     .method("GET")
///     .body(Body::default())?
///     .sign(
///         "execute-api",
///         &Region::UsEast1,
///         &AwsCredentials::new("key", "secret", None, None)
///     )?;
/// # Ok(())
/// # }
/// ```
impl Sign for Request<Body> {
    type Error = Error;
    fn sign(
        self,
        service: &str,
        region: &Region,
        credentials: &AwsCredentials,
    ) -> Result<Self, Self::Error> {
        let query = self.uri().query().clone();
        let custom_region = Region::Custom {
            name: region.name().into(),
            endpoint: self
                .uri()
                .to_string()
                .splitn(2, '?')
                .next()
                .expect("invalid uri")
                .to_string(),
        };
        let mut signer = SignedRequest::new(
            self.method().as_ref(),
            service,
            &custom_region,
            Default::default(),
        );
        for (key, value) in self.headers() {
            if let Ok(value) = value.to_str() {
                signer.add_header(key.as_str(), value);
            }
        }
        if let Some(query) = query {
            for param in query.split('&') {
                match &param.splitn(2, '=').collect::<Vec<_>>()[..] {
                    [key, value] => signer.add_param(*key, *value),
                    _ => (),
                }
            }
        }
        let body = self.into_body();
        let body = body.map_ok(|chunk| Chunk::into_bytes(chunk)).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid request: {}", e),
            )
            .into()
        });
        signer.set_payload_stream(Box::pin(body));
        signer.sign(credentials);
        signer.try_into()
    }
}
