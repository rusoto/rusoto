use rusoto_core::signature;
use rusoto_core::signature::SignedRequest;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::region::Region;
use rusoto_core::credential::AwsCredentials;
use generated::{GetObjectRequest, GetObjectError};

/// URL encodes an S3 object key. This is necessary for `copy_object` and `upload_part_copy`,
/// which require the `copy_source` field to be URL encoded.
///
/// # Examples
///
/// ```
/// use rusoto_s3::CopyObjectRequest;
///
/// let request = CopyObjectRequest {
///     bucket: "my-bucket".to_owned(),
///     key: "my-key".to_owned(),
///     copy_source: rusoto_s3::util::encode_key("other-buckét/key-to-cöpy"),
///     ..Default::default()
/// };
/// ```
pub fn encode_key<T: AsRef<str>>(key: T) -> String {
    signature::encode_uri_path(key.as_ref())
}


/// TODO: document this
pub fn generate_presigned_url(region: &Region, credentials: &AwsCredentials, input: &GetObjectRequest) -> Result<String, GetObjectError> {
    let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

    let mut request = SignedRequest::new("GET", "s3", &region, &request_uri);

    if let Some(ref if_match) = input.if_match {
        request.add_header("If-Match", &if_match.to_string());
    }

    if let Some(ref if_modified_since) = input.if_modified_since {
        request.add_header("If-Modified-Since", &if_modified_since.to_string());
    }

    if let Some(ref if_none_match) = input.if_none_match {
        request.add_header("If-None-Match", &if_none_match.to_string());
    }

    if let Some(ref if_unmodified_since) = input.if_unmodified_since {
        request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
    }

    if let Some(ref range) = input.range {
        request.add_header("Range", &range.to_string());
    }

    if let Some(ref request_payer) = input.request_payer {
        request.add_header("x-amz-request-payer", &request_payer.to_string());
    }

    if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
        request.add_header("x-amz-server-side-encryption-customer-algorithm",
                           &sse_customer_algorithm.to_string());
    }

    if let Some(ref sse_customer_key) = input.sse_customer_key {
        request.add_header("x-amz-server-side-encryption-customer-key",
                           &sse_customer_key.to_string());
    }

    if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
        request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                           &sse_customer_key_md5.to_string());
    }
    let mut params = Params::new();
    if let Some(ref x) = input.part_number {
        params.put("partNumber", x);
    }
    if let Some(ref x) = input.response_cache_control {
        params.put("response-cache-control", x);
    }
    if let Some(ref x) = input.response_content_disposition {
        params.put("response-content-disposition", x);
    }
    if let Some(ref x) = input.response_content_encoding {
        params.put("response-content-encoding", x);
    }
    if let Some(ref x) = input.response_content_language {
        params.put("response-content-language", x);
    }
    if let Some(ref x) = input.response_content_type {
        params.put("response-content-type", x);
    }
    if let Some(ref x) = input.response_expires {
        params.put("response-expires", x);
    }
    if let Some(ref x) = input.version_id {
        params.put("versionId", x);
    }
    request.set_params(params);

    Ok(request.generate_presigned_url(&credentials))
}
