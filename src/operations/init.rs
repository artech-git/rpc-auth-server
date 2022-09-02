use aws_sdk_dynamodb::{Client, Endpoint, Region};
use aws_types::Credentials;
use http::Uri;
use once_cell::sync::Lazy;

use crate::constants::KEY_MAP;

pub static DynamoDBClient: Lazy<Client> = Lazy::new(|| {
    let region = KEY_MAP
        .get(&"aws-region".to_string())
        .unwrap_or(&"ap-south-1".to_string())
        .to_owned();

    let url = KEY_MAP
        .get(&"aws-region-url".to_string())
        .unwrap_or(&format!("https://dynamodb.{}.amazonaws.com/", region))
        .to_owned()
        .parse::<Uri>()
        .unwrap();

    let db_key = match KEY_MAP.get(&"db-access-key".to_string()) {
        Some(v) => {
            tracing::log::info!("access key present");
            v
        }
        None => {
            tracing::log::warn!("access key is not present");
            panic!();
        }
    };

    let db_sec = match KEY_MAP.get(&"db-secret-access-key".to_string()) {
        Some(v) => {
            tracing::log::info!("secret key present");
            v
        }
        None => {
            tracing::log::warn!("secret key is not present");
            panic!();
        }
    };

    let creds = aws_sdk_dynamodb::Credentials::new(db_key, db_sec, None, None, "something");

    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::new()
        .credentials_provider(creds)
        .region(Region::new(region))
        .endpoint_resolver(Endpoint::immutable(url))
        .build();

    let client = Client::from_conf(dynamodb_local_config);

    return client;
});


pub static S3Client: Lazy<aws_sdk_s3::Client> = Lazy::new(|| {
    let region = KEY_MAP
        .get(&"aws-region".to_string())
        .unwrap_or(&"ap-south-1".to_string())
        .to_owned();

    let url = KEY_MAP
        .get(&"aws-region-url".to_string())
        .unwrap_or(&format!("https://dynamodb.{}.amazonaws.com/", region))
        .to_owned()
        .parse::<Uri>()
        .unwrap();

    let db_key = match KEY_MAP.get(&"db-access-key".to_string()) {
        Some(v) => {
            tracing::log::info!("access key present");
            v
        }
        None => {
            tracing::log::warn!("access key is not present");
            panic!();
        }
    };

    let db_sec = match KEY_MAP.get(&"db-secret-access-key".to_string()) {
        Some(v) => {
            tracing::log::info!("secret key present");
            v
        }
        None => {
            tracing::log::warn!("secret key is not present");
            panic!();
        }
    };

    let creds = aws_sdk_s3::Credentials::new(db_key, db_sec, None, None, "something");

    let s3_local_config = aws_sdk_s3::config::Builder::new()
        .credentials_provider(creds)
        .region(Region::new(region))
        .endpoint_resolver(Endpoint::immutable(url))
        .build();

    let client = aws_sdk_s3::Client::from_conf(s3_local_config);

    return client;
});
