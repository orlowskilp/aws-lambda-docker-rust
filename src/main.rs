use lambda_runtime::{service_fn, tracing, Error};
use lambda_template::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Required for CloudWatch logging
    tracing::init_default_subscriber();

    // Wrap the handler function and dispatch it
    let lambda_function = service_fn(handler);
    lambda_runtime::run(lambda_function).await?;

    Ok(())
}
