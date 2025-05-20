mod common;
mod http_handler;
use http_handler::function_handler;
use lambda_http::{Error, run, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    common::init_tracing();

    run(service_fn(function_handler)).await
}
