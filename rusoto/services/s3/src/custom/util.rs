use rusoto_core::signature;

/// URL encodes an S3 object key. This is necessary for `copy_object` and `upload_part_copy`,
/// which require the `copy_source` field to be URL encoded to be used properly.
///
/// # Examples
///
/// ```
/// # use rusoto_s3::{S3, CopyObjectRequest};
/// # fn test_fn<T: S3>(s3_client: T) {
///
/// let response = s3_client.copy_object(&CopyObjectRequest {
///     bucket: "my-bucket".to_owned(),
///     key: "my-key".to_owned(),
///     copy_source: rusoto_s3::util::encode_key("other-buckét/key-to-cöpy"),
///     ..Default::default()
/// }).unwrap();
///
/// # }
pub fn encode_key<T: AsRef<str>>(key: T) -> String {
    signature::encode_uri_path(key.as_ref())
}
