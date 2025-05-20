use aws_sdk_s3::Client;
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncBufReadExt;

#[derive(Serialize)]
pub(crate) struct Response {
    req_id: String,
    msg: String,
}

#[derive(Deserialize)]
pub(crate) struct Request {
    bucket: String,
    key: String,
}

pub(crate) async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let bucket = &event.payload.bucket;
    let key = &event.payload.key;

    let started_at = std::time::Instant::now();

    // Setup S3 client
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Initiate a GetObject request to S3
    let output = client
        .get_object()
        .bucket(bucket.to_string())
        .key(key.to_string())
        .send()
        .await?;
    let body = output.body;

    // Begin streaming the contents down, decompressing on the fly
    // and iterating over each chunk split by newlines.
    let body = body.into_async_read();
    let body = tokio::io::BufReader::new(body);
    let decoder = async_compression::tokio::bufread::ZstdDecoder::new(body);
    let reader = tokio::io::BufReader::new(decoder);
    let mut lines = reader.lines();

    // For each line we encounter while asynchronously streaming
    // down the S3 data, parse the JSON object.
    let mut num_log_events = 0;
    while let Some(line) = lines.next_line().await? {
        let _value: serde_json::Value = serde_json::from_str(&line)?;
        num_log_events += 1;
        if num_log_events % 1000 == 0 {
            println!("num_log_events={}", num_log_events);
        }
    }

    let msg = format!(
        "elapsed={:?} num_log_events={}",
        started_at.elapsed(),
        num_log_events
    );

    let resp = Response {
        req_id: event.context.request_id,
        msg,
    };

    Ok(resp)
}
