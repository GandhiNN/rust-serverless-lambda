mod common;
use lambda_runtime::{Error, run, service_fn};
mod event_handler;
use event_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    common::init_tracing();

    run(service_fn(function_handler)).await
}
