use aws_config::Region;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_kinesis::Client;
use aws_sdk_kinesis::operation::put_record::{PutRecordError, PutRecordOutput};
use aws_sdk_kinesis::primitives::Blob;
use clap::{ArgAction, Command, arg, command, value_parser};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env::var;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // requires `cargo` feature
    let matches = command!()
        .arg(arg!([kinesis_stream] "Kinesis stream to operate on").required(true))
        .get_matches();

    let stream_arn = matches.get_one::<String>("kinesis_stream").unwrap();

    let kinesis_client = new_client("false".to_string()).await;

    let device_1 = IotDevice::new("device1".to_string());
    let device_2 = IoTDevice::new("device2".to_string());
    let device_3 = IoTDevice::new("device3".to_string());

    loop {
        device_1
            .send_temperature_data(&kinesis_client, stream_arn)
            .await;
        device_2
            .send_temperature_data(&kinesis_client, stream_arn)
            .await;
        device_3
            .send_temperature_data(&kinesis_client, stream_arn)
            .await;

        sleep(Duration::from_secs(1)).await;
    }
}
