use warp::{self, path, Filter};

#[tokio::main]
async fn main() {
    let instance_profile_role =
        path!("latest" / "meta-data" / "iam" / "security-credentials").map(|| "testrole");
    let instance_profile_creds =
        path!("latest" / "meta-data" / "iam" / "security-credentials" / "testrole").map(|| {
            r#"{
  "Code" : "Success",
  "LastUpdated" : "2015-08-04T00:09:23Z",
  "Type" : "AWS-HMAC",
  "AccessKeyId" : "Access_key_id_value",
  "SecretAccessKey" : "Secret_access_key_value",
  "Token" : "AAAAA",
  "Expiration" : "2015-08-04T06:32:37Z"
}"#
        });

    let routes = warp::get()
        .and(instance_profile_creds)
        .or(instance_profile_role);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
