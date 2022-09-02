use aws_sdk_s3::{Error, Client};


pub async fn upload_object(
    client: &Client,
    bucket: &str,
    filename: &str,
    key: &str,
) -> Result<(), Error> {

    let resp = client.list_buckets().send().await?;

    let body = aws_sdk_s3::types::ByteStream::from_path(std::path::Path::new(filename)).await;

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;

            tracing::log::info!("Upload success. Version: {:?}", resp.version_id);

            let resp = client
                                            .get_object()
                                            .bucket(bucket)
                                            .key(key)
                                            .send()
                                            .await?;

            let data = resp.body.collect().await;

            tracing::log::info!("data: {:?}", data.unwrap().into_bytes());

            return Ok(());

        }
        Err(e) => {

            tracing::log::info!("Got an error uploading object:");
            tracing::log::info!("{}", e);
            
            // let t = aws_sdk_s3::model::Error::builder();

            // let u = t
            //         .key("body not formed".to_string())
            //         .build();

            let i = aws_sdk_s3::Error::InvalidObjectState(aws_sdk_s3::error::InvalidObjectState::builder().build());

            return Err(i);
        }
    }

    // Ok(())
}