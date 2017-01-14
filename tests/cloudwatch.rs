#![cfg(feature = "cloudwatch")]

extern crate rusoto;

use rusoto::cloudwatch::{CloudWatchClient, PutMetricDataInput, Dimension, MetricDatum};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_put_metric_data() {
	let client = CloudWatchClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1).unwrap();

	let metric_data = vec![
		MetricDatum {
	        dimensions: Some(vec![Dimension {name: "foo".to_string(), value: "bar".to_string()}]),
	        metric_name: "buffers".to_string(),
	        statistic_values: None,
	        timestamp: None,
	        unit: Some("Bytes".to_string()),
	        value: Some(1.0),
    	}
    ];
    let request = PutMetricDataInput {
        namespace: "TestNamespace".to_string(),
        metric_data: metric_data,
    };

	let response = client.put_metric_data(&request).unwrap();
	println!("{:#?}", response);
}


