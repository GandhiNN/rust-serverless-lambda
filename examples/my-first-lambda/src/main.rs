use lambda_http::{run, service_fn};
use lambda_runtime::{Error, tracing};
mod event_handler;
use event_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
