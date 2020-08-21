use crate::{region::Region, signature::SignedRequest};
use chrono::Datelike;
use chrono::{DateTime, Utc};
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use time::Date;

// Policy explanation:
// http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-HTTPPOSTConstructPolicy.html

#[derive(Default)]
pub struct PostPolicy<'a> {
    expiration: Option<&'a DateTime<Utc>>,
    content_length_range: Option<(u64, u64)>,
    conditions: Vec<Condition<'a>>,
    form_data: HashMap<String, String>,
    bucket_name: Option<&'a str>,
    key: Option<&'a str>,
    region: Option<&'a Region>,
    access_key_id: Option<&'a str>,
    secret_access_key: Option<&'a str>,
}

#[derive(Serialize)]
pub struct SerializablePolicy<'a> {
    expiration: &'a str,
    conditions: &'a Vec<Condition<'a>>,
}

struct Condition<'a>((&'a str, &'a str, &'a str));

impl<'a> Serialize for Condition<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        let v = &self.0;
        seq.serialize_element(v.0)?;

        if v.0 == "content-length-range" {
            seq.serialize_element(&v.1.parse::<u64>().map_err(|_| {
                serde::ser::Error::custom("expected u64 value, the minimum content length")
            })?)?;
            seq.serialize_element(&v.2.parse::<u64>().map_err(|_| {
                serde::ser::Error::custom("expected u64 value, the maximum content length")
            })?)?;
        } else {
            seq.serialize_element(v.1)?;
            seq.serialize_element(v.2)?;
        }

        seq.end()
    }
}

impl<'a> PostPolicy<'a> {
    /// Set expiration time
    pub fn set_expiration(mut self, t: &'a DateTime<Utc>) -> Self {
        self.expiration = Some(t);
        self
    }

    /// Set key policy condition
    pub fn set_key(mut self, key: &'a str) -> Self {
        if key.is_empty() {
            return self;
        }

        self = self.append_policy("eq", "$key", &key);
        self.key = Some(key);
        self.form_data.insert("key".to_string(), key.to_string());
        self
    }

    /// Set key startswith policy condition
    #[allow(dead_code)]
    pub fn set_key_startswith(mut self, key: &'a str) -> Self {
        if key.is_empty() {
            return self;
        }

        self.key = Some(key);

        self = self.append_policy("starts-with", "$key", &key);
        self.form_data.insert("key".to_string(), key.to_string());
        self
    }

    /// Set bucket name
    pub fn set_bucket_name(mut self, bucket_name: &'a str) -> Self {
        self.form_data
            .insert("bucket".to_string(), bucket_name.to_string());
        self = self.append_policy("eq", "$bucket", bucket_name);
        self.bucket_name = Some(bucket_name);
        self
    }

    /// Set region
    pub fn set_region(mut self, region: &'a Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set access key id
    pub fn set_access_key_id(mut self, access_key_id: &'a str) -> Self {
        if access_key_id.is_empty() {
            return self;
        }

        self.access_key_id = Some(access_key_id);
        self
    }

    /// Set secret access key
    pub fn set_secret_access_key(mut self, secret_access_key: &'a str) -> Self {
        if secret_access_key.is_empty() {
            return self;
        }

        self.secret_access_key = Some(secret_access_key);
        self
    }

    /// Set content-type policy condition
    #[allow(dead_code)]
    pub fn set_content_type(mut self, content_type: &'a str) -> Self {
        self.form_data
            .insert("Content-Type".to_string(), content_type.to_string());
        self = self.append_policy("eq", "$Content-Type", content_type);
        self
    }

    /// Set content length range policy condition
    pub fn set_content_length_range(mut self, min_length: u64, max_length: u64) -> Self {
        self.content_length_range = Some((min_length, max_length));
        // We should append the policy here, but ownership it's tricky,
        // so we'll do it inside build_form_data()
        self
    }

    /// Append policy condition
    pub fn append_policy(mut self, match_type: &'a str, target: &'a str, value: &'a str) -> Self {
        self.conditions.push(Condition((match_type, target, value)));
        self
    }

    /// Create the form data using the policy
    pub fn build_form_data(mut self) -> Result<(String, HashMap<String, String>), String> {
        match self.content_length_range {
            Some((min_length, max_length)) if min_length > max_length => {
                return Err(format!(
                    "Min-length ({}) must be <= Max-length ({})",
                    min_length, max_length
                ));
            }
            _ => (),
        }

        if self.expiration.is_none() {
            return Err("Expiration date must be specified".to_string());
        }

        if self.key.is_none() {
            return Err("Object key must be specified".to_string());
        }

        if self.bucket_name.is_none() {
            return Err("Bucket name must be specified".to_string());
        }

        if self.region.is_none() {
            return Err("Region must be specified".to_string());
        }

        if self.access_key_id.is_none() {
            return Err("Access key id must be specified".to_string());
        }

        if self.secret_access_key.is_none() {
            return Err("Secret access key must be specified".to_string());
        }

        let bucket_name = self.bucket_name.unwrap();
        let secret_access_key = self.secret_access_key.unwrap();

        let expiration = self
            .expiration
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();

        let current_time = if cfg!(test) {
            use chrono::TimeZone;
            Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)
        } else {
            Utc::now()
        };
        let current_time_fmted = current_time.format("%Y%m%dT%H%M%SZ").to_string();
        let current_date = current_time.format("%Y%m%d").to_string();

        let access_key_id = self.access_key_id.unwrap();
        let region = self.region.unwrap();
        let region_name = region.name();

        let x_amz_credential = format!(
            "{}/{}/{}/{}/aws4_request",
            &access_key_id, &current_date, &region_name, "s3",
        );

        let mut conditions: Vec<Condition> = self.conditions.into_iter().collect();

        conditions.push(Condition(("eq", "$x-amz-date", &current_time_fmted)));
        conditions.push(Condition(("eq", "$x-amz-algorithm", "AWS4-HMAC-SHA256")));
        conditions.push(Condition(("eq", "$x-amz-credential", &x_amz_credential)));

        let min_length_as_string: String;
        let max_length_as_string: String;
        if let Some((min, max)) = self.content_length_range {
            min_length_as_string = min.to_string();
            max_length_as_string = max.to_string();
            conditions.push(Condition((
                "content-length-range",
                &min_length_as_string,
                &max_length_as_string,
            )))
        }

        let policy_to_serialize = SerializablePolicy {
            expiration: &expiration,
            conditions: &conditions,
        };

        let policy_as_json =
            serde_json::to_string(&policy_to_serialize).map_err(|e| format!("{:?}", e))?;

        let policy_as_base64 = base64::encode(policy_as_json);

        let signature_date = Date::try_from_ymd(
            current_time.date().year() as i32,
            current_time.date().month() as u8,
            current_time.date().day() as u8,
        )
        .unwrap();

        let x_amz_signature = crate::signature::sign_string(
            &policy_as_base64,
            &secret_access_key,
            signature_date,
            &region_name,
            "s3",
        );

        self.form_data
            .insert("policy".to_string(), policy_as_base64);
        self.form_data
            .insert("x-amz-date".to_string(), current_time_fmted);
        self.form_data.insert(
            "x-amz-algorithm".to_string(),
            "AWS4-HMAC-SHA256".to_string(),
        );
        self.form_data
            .insert("x-amz-credential".to_string(), x_amz_credential);
        self.form_data
            .insert("x-amz-signature".to_string(), x_amz_signature);

        let signed_request = SignedRequest::new("GET", "s3", &region, "/");

        let upload_url = format!(
            "{}://{}.{}",
            signed_request.scheme(),
            bucket_name,
            signed_request.hostname()
        );

        Ok((upload_url, self.form_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    const BUCKET: &str = "the-bucket";
    const REGION: Region = Region::EuCentral1;
    const ACCESS_KEY_ID: &str = "foo_access_key";
    const SECRET_ACCESS_KEY: &str = "foo_secret_key";
    const OBJECT_KEY: &str = "the-object-key";

    #[test]
    fn bucket_name_is_required() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        assert_eq!(res, Err("Bucket name must be specified".to_string()));
    }

    #[test]
    fn region_is_required() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_bucket_name(&BUCKET)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        assert_eq!(res, Err("Region must be specified".to_string()));
    }
    #[test]
    fn access_key_id_is_required() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_bucket_name(&BUCKET)
            .set_region(&REGION)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        assert_eq!(res, Err("Access key id must be specified".to_string()));
    }

    #[test]
    fn secret_access_key_is_required() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_bucket_name(&BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        assert_eq!(res, Err("Secret access key must be specified".to_string()));
    }

    #[test]
    fn expiration_is_required() {
        let res = PostPolicy::default()
            .set_bucket_name(&BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_key(OBJECT_KEY)
            .build_form_data();

        assert_eq!(res, Err("Expiration date must be specified".to_string()));
    }
    #[test]
    fn build_successfully() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_bucket_name(BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .set_content_length_range(123, 456)
            .build_form_data();

        assert!(res.is_ok());
        let (upload_url, form_data) = res.unwrap();
        assert_eq!(
            upload_url,
            "https://the-bucket.s3.eu-central-1.amazonaws.com"
        );
        assert_eq!(form_data.get("key").unwrap(), "the-object-key");

        assert_eq!(form_data.get("bucket").unwrap(), "the-bucket");
        assert_eq!(
            form_data.get("x-amz-algorithm").unwrap(),
            "AWS4-HMAC-SHA256"
        );
        assert_eq!(
            form_data.get("x-amz-credential").unwrap(),
            "foo_access_key/20200101/eu-central-1/s3/aws4_request"
        );
        assert_eq!(form_data.get("x-amz-date").unwrap(), "20200101T000000Z");

        let expected_policy = serde_json::json!({
            "expiration": "2020-01-01T01:02:03.000Z",
            "conditions": [
                ["eq", "$bucket", "the-bucket"],
                ["eq", "$key", "the-object-key"],
                ["eq", "$x-amz-date", "20200101T000000Z"],
                ["eq", "$x-amz-algorithm", "AWS4-HMAC-SHA256"],
                ["eq", "$x-amz-credential", "foo_access_key/20200101/eu-central-1/s3/aws4_request"],
                ["content-length-range", 123, 456]
            ]
        });

        let policy_as_base64 = form_data.get("policy").unwrap();
        let policy_as_vec_u8 = base64::decode(policy_as_base64).unwrap();
        let policy: serde_json::Value = serde_json::from_slice(&policy_as_vec_u8).unwrap();
        assert_eq!(policy, expected_policy);
    }

    #[test]
    fn set_content_type() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_content_type("some/type")
            .set_bucket_name(BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        assert!(res.is_ok());

        let (_, form_data) = res.unwrap();
        dbg!(&form_data);
        assert_eq!(form_data.get("Content-Type").unwrap(), "some/type");

        let policy_as_base64 = form_data.get("policy").unwrap();
        let policy_as_vec_u8 = base64::decode(policy_as_base64).unwrap();
        let policy: serde_json::Value = serde_json::from_slice(&policy_as_vec_u8).unwrap();
        let conditions = policy["conditions"].as_array().unwrap();
        assert!(conditions.contains(&serde_json::json!(["eq", "$Content-Type", "some/type"])));
    }

    #[test]
    fn append_policy() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .append_policy("a", "b", "c")
            .set_bucket_name(BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_key(OBJECT_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        let (_, form_data) = res.unwrap();

        assert_eq!(form_data.get("a"), None);

        let policy_as_base64 = form_data.get("policy").unwrap();
        let policy_as_vec_u8 = base64::decode(policy_as_base64).unwrap();
        let policy: serde_json::Value = serde_json::from_slice(&policy_as_vec_u8).unwrap();
        let conditions = policy["conditions"].as_array().unwrap();
        assert!(conditions.contains(&serde_json::json!(["a", "b", "c"])));
    }

    #[test]
    fn set_key_startswith() {
        let expiration_date = Utc.ymd(2020, 1, 1).and_hms(1, 2, 3);

        let res = PostPolicy::default()
            .set_key_startswith("foo")
            .set_bucket_name(BUCKET)
            .set_region(&REGION)
            .set_access_key_id(ACCESS_KEY_ID)
            .set_secret_access_key(SECRET_ACCESS_KEY)
            .set_expiration(&expiration_date)
            .build_form_data();

        let (_, form_data) = res.unwrap();
        dbg!(&form_data);
        assert_eq!(form_data.get("key").unwrap(), "foo");

        let policy_as_base64 = form_data.get("policy").unwrap();
        let policy_as_vec_u8 = base64::decode(policy_as_base64).unwrap();
        let policy: serde_json::Value = serde_json::from_slice(&policy_as_vec_u8).unwrap();
        let conditions = policy["conditions"].as_array().unwrap();
        assert!(conditions.contains(&serde_json::json!(["starts-with", "$key", "foo"])));
    }
}
