use rusoto_core::signature;
use rusoto_core::signature::SignedRequest;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::region::Region;
use rusoto_core::credential::AwsCredentials;
use generated::{GetObjectRequest, PutObjectRequest, DeleteObjectRequest};
use std::time::Duration;
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

macro_rules! add_headers {
    (
        $input:ident , $req:ident ; $p:ident , $e:expr ; $( $t:tt )*
    ) => (
        {
            if let Some(ref $p) = $input.$p {
                $req.add_header($e, &$p.to_string());
            }
            add_headers! { $input, $req; $( $t )* }
        }
    );
    (
        $input:pat , $req:expr ;
    ) => ({
    });
}


macro_rules! add_params {
    (
        $input:ident , $params:ident ; $p:ident , $e:expr ; $( $t:tt )*
    ) => (
        {
            if let Some(ref $p) = $input.$p {
                $params.put($e, &$p);
            }
            add_params! { $input, $params; $( $t )* }
        }
    );
    (
        $input:pat , $req:expr ;
    ) => ({
    });
}

pub struct PreSignedRequestOption {
    pub expires_in: Duration
}

impl Default for PreSignedRequestOption {
    fn default() -> Self {
        Self {
            expires_in: Duration::from_secs(3600)
        }
    }
}


pub trait PreSignedRequest {
    /// http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html
    fn get_presigned_url(&self, region: &Region, credentials: &AwsCredentials, option: &PreSignedRequestOption) -> String;
}

impl PreSignedRequest for GetObjectRequest {
    /// https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectGET.html
    fn get_presigned_url(&self, region: &Region, credentials: &AwsCredentials, option: &PreSignedRequestOption) -> String {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);
        let mut request = SignedRequest::new("GET", "s3", &region, &request_uri);
        let mut params = Params::new();

        add_headers!(
            self, request;
            range, "Range";
            if_modified_since, "If-Modified-Since";
            if_unmodified_since, "If-Unmodified-Since";
            if_match, "If-Match";
            if_none_match, "If-None-Match";
            sse_customer_algorithm, "x-amz-server-side-encryption-customer-algorithm";
            sse_customer_key, "x-amz-server-side-encryption-customer-key";
            sse_customer_key_md5, "x-amz-server-side-encryption-customer-key-MD5";
        );

        add_params!(
            self, params;
            part_number, "partNumber";
            response_content_type, "response-content-type";
            response_content_language, "response-content-language";
            response_expires, "response-expires";
            response_cache_control, "response-cache-control";
            response_content_disposition, "response-content-disposition";
            response_content_encoding, "response-content-encoding";
            version_id, "versionId";
        );

        request.set_params(params);
        request.generate_presigned_url(credentials, &option.expires_in)
    }
}

impl PreSignedRequest for PutObjectRequest {
    /// https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectPUT.html
    fn get_presigned_url(&self, region: &Region, credentials: &AwsCredentials, option: &PreSignedRequestOption) -> String {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);
        let mut request = SignedRequest::new("PUT", "s3", &region, &request_uri);

        add_headers!(
            self, request;
            cache_control, "Cache-Control";
            content_disposition, "Content-Disposition";
            content_encoding, "Content-Encoding";
            content_length, "Content-Length";
            content_md5, "Content-MD5";
            content_type, "Content-Type";
            // AWS document has Expect parameter but PutObjectRequest does'nt have it.
            //expect, "Expect";
            expires, "Expires";
            storage_class, "x-amz-storage-class";
            tagging, "x-amz-tagging";
            website_redirect_location, "x-amz-website-redirect-location";
            acl, "x-amz-acl";
            grant_read, "x-amz-grant-read";
            // AWS document has x-amz-grant-write parameter but PutObjectRequest does'nt have it.
            //grant_write, "x-amz-grant-write";
            grant_read_acp, "x-amz-grant-read-acp";
            grant_write_acp, "x-amz-grant-write-acp";
            grant_full_control, "x-amz-grant-full-control";
            server_side_encryption, "x-amz-server-side-encryption";
            ssekms_key_id, "x-amz-server-side-encryption-aws-kms-key-id";
            // AWS document has x-amz-server-side-encryption-context parameter but PutObjectRequest does'nt have it.
            //kms_context, "x-amz-server-side-encryption-context";
            sse_customer_algorithm, "x-amz-server-side-encryption-customer-algorithm";
            sse_customer_key, "x-amz-server-side-encryption-customer-key";
            sse_customer_key_md5, "x-amz-server-side-encryption-customer-key-MD5";
        );

        if let Some(ref metadata) = self.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        request.generate_presigned_url(credentials, &option.expires_in)
    }
}

impl PreSignedRequest for DeleteObjectRequest {
    /// https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectDELETE.html
    fn get_presigned_url(&self, region: &Region, credentials: &AwsCredentials, option: &PreSignedRequestOption) -> String {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);
        let mut request = SignedRequest::new("DELETE", "s3", &region, &request_uri);
        let mut params = Params::new();

        add_headers!(
            self, request;
            mfa, "x-amz-mfa";
        );

        add_params!(
            self, params;
            version_id, "versionId";
        );

        request.set_params(params);
        request.generate_presigned_url(credentials, &option.expires_in)
    }
}
