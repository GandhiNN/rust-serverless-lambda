use aws_lambda_events::kinesis::KinesisEvent;
use aws_lambda_events::streams::{KinesisBatchItemFailure, KinesisEventResponse};
use lambda_runtime::{Error, LambdaEvent};
use shared;

pub async fn function_handler(
    event: LambdaEvent<KinesisEvent>,
) -> Result<KinesisEventResponse, Error> {
    let mut batch_item_failures = Vec::new();

    for message in &event.payload.records {
        let kinesis_sequence_number = message.kinesis.sequence_number.clone();

        let new_message: Result<shared::NewSensorReading, shared::MessageParseError> =
            shared::InternalKinesisMessage::new(message.clone()).try_into();
    }

    Ok(KinesisEventResponse {
        batch_item_failures,
    })
}
