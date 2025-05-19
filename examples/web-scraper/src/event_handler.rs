use lambda_http::{Error, LambdaEvent};
use serde_json::{Value, json};

pub async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (_event, _context) = event.into_parts();

    Ok(json!({ "message": "Hello Ferris!" }))
}
