use aws_config::Region;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_kinesis::Client;
use aws_sdk_kinesis::primitives::Blob;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct IoTDevice {
    name: String,
}

impl IoTDevice {
    pub fn new(name: String) -> Self {
        IoTDevice { name }
    }

    pub async fn send_temperature_data(&self, client: &Client, kinesis_stream_arn: &String) -> () {
        let mut rng = rand::rng();

        let temperature_reading = TemperatureReading::new(rng.random_range(10.0..25.6));

        let serialized_data = serde_json::to_string(&temperature_reading).unwrap();

        let put_res = client
            .put_record()
            .stream_arn(kinesis_stream_arn)
            .partition_key(&self.name)
            .data(Blob::new(serialized_data))
            .send()
            .await;

        match put_res {
            Ok(_) => tracing::info!("Success sending Kinesis data for device {}", &self.name),
            Err(e) => {
                tracing::error!("Failure sending kinesis data for device {}", &self.name);
                tracing::error!("{}", e.into_service_error().to_string());
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TemperatureReading {
    temperature: f32,
    reading_timestamp: f32,
}

impl TemperatureReading {
    fn new(temperature: f32) -> Self {
        Self {
            temperature,
            reading_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f32(),
        }
    }
}

pub async fn new_client(is_local: String) -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let sdk_config = aws_config::from_env().region(region_provider).load().await;
    if is_local.to_ascii_lowercase() == "true".to_string() {
        let config = aws_sdk_kinesis::config::Builder::from(&sdk_config)
            .endpoint_url("http://localhost:8000".to_string())
            .region(Region::from_static("eu-west-1"))
            .build();
        return Client::from_conf(config);
    }

    let config = aws_sdk_kinesis::config::Builder::from(&sdk_config).build();
    Client::from_conf(config)
}
