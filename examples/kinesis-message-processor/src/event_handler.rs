use aws_lambda_events::kinesis::KinesisEvent;
use aws_lambda_events::streams::{KinesisBatchItemFailure, KinesisEventResponse};
use lambda_runtime::{Error, LambdaEvent};
use shared::{self};

pub async fn function_handler(
    event: LambdaEvent<KinesisEvent>,
) -> Result<KinesisEventResponse, Error> {
    let mut batch_item_failures = Vec::new();

    for message in &event.payload.records {
        let kinesis_sequence_number = message.kinesis.sequence_number.clone();

        let new_message: Result<shared::NewSensorReading, shared::MessageParseError> =
            shared::InternalKinesisMessage::new(message.clone()).try_into();

        if new_message.is_err() {
            batch_item_failures.push(KinesisBatchItemFailure {
                item_identifier: Some(kinesis_sequence_number),
            });
            continue;
        }

        // Business logic goes here
        let handle_result = shared::NewSensorReadingHandler::handle(&new_message.unwrap()).await;

        if handle_result.is_err() {
            batch_item_failures.push(KinesisBatchItemFailure {
                item_identifier: Some(kinesis_sequence_number),
            });
            continue;
        }
    }

    Ok(KinesisEventResponse {
        batch_item_failures,
    })
}
