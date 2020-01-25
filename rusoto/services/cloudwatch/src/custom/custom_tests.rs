use crate::generated::{CloudWatch, CloudWatchClient, Dimension, MetricDatum, PutMetricDataInput};

use rusoto_core::param::Params;
use rusoto_core::signature::SignedRequest;
use rusoto_core::signature::SignedRequestPayload;
use rusoto_core::Region;
use rusoto_mock::*;
use serde_urlencoded;

#[tokio::test]
async fn should_serialize_complex_metric_data_params() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body("")
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);
            assert_eq!("/", request.path);
            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    params.get("Namespace"),
                    Some(&Some("TestNamespace".to_owned()))
                );
                assert_eq!(
                    params.get("MetricData.member.1.MetricName"),
                    Some(&Some("buffers".to_owned()))
                );
                assert_eq!(
                    params.get("MetricData.member.1.Unit"),
                    Some(&Some("Bytes".to_owned()))
                );
                assert_eq!(
                    params.get("MetricData.member.1.Value"),
                    Some(&Some("1".to_owned()))
                );
                assert_eq!(
                    params.get("MetricData.member.1.Dimensions.member.1.Name"),
                    Some(&Some("foo".to_owned()))
                );
                assert_eq!(
                    params.get("MetricData.member.1.Dimensions.member.1.Value"),
                    Some(&Some("bar".to_owned()))
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });
    let metric_data = vec![MetricDatum {
        dimensions: Some(vec![Dimension {
            name: "foo".to_string(),
            value: "bar".to_string(),
        }]),
        metric_name: "buffers".to_string(),
        statistic_values: None,
        timestamp: None,
        unit: Some("Bytes".to_string()),
        value: Some(1.0),
        ..Default::default()
    }];
    let request = PutMetricDataInput {
        namespace: "TestNamespace".to_string(),
        metric_data: metric_data,
    };

    let client = CloudWatchClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let response = client.put_metric_data(request).await.unwrap();
    println!("{:#?}", response);
}