use aws_config::meta::region::RegionProviderChain;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::Client;
use image::io::Reader as ImageReader;
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;
use std::env;
use std::io::Cursor;

pub async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let serialized_payload = json!(event.payload).to_string();
    println!("{}", serialized_payload);

    let records = event.payload.records;
    let source_bucket = records[0].s3.bucket.name.clone().unwrap();
    let destination_bucket = env::var("destination_bucket").unwrap();
    let key = records[0].s3.object.key.clone().unwrap();
    let decoded_key = str::replace(key.as_str(), "%3A", ":");
    let destination_key = decoded_key.clone();

    let bytes = client
        .get_object()
        .bucket(source_bucket)
        .key(decoded_key)
        .send()
        .await
        .unwrap()
        .body
        .collect()
        .await
        .unwrap()
        .into_bytes();

    let cursor_bytes = Cursor::new(bytes);

    let image = ImageReader::new(cursor_bytes)
        .with_guessed_format()
        .decode()
        .unwrap();
    let transformed_image = image.rotate180();

    let mut as_bytes: Vec<u8> = Vec::new();
    transformed_image
        .write_to(
            &mut Cursor::new(&mut as_bytes),
            image::ImageOutputFormat::Tiff,
        )
        .expect("Error writing to vector");

    let as_stream = as_bytes.into();

    client
        .put_object()
        .bucket(destination_bucket)
        .key(destination_key)
        .body(as_stream)
        .send()
        .await
        .unwrap();

    Ok(())
}
