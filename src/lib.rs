use lambda_runtime::{
    tracing::{info, warn},
    Error, LambdaEvent,
};
use serde::{Deserialize, Serialize};
use std::env;

// Constant literals to avoid repetition
const RESPONSE_MSG_PREFIX: &str = "Message received:";
const ENV_VAR_NAME: &str = "ENV";

#[derive(Deserialize)]
pub struct Request {
    message: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Response {
    request_id: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
}

// Helper function for reading environment variables
fn get_env(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => {
            info!("Read {} environment variable successfully", env_var_name);

            Some(value)
        }
        Err(_) => {
            warn!(
                "Couldn't read {} environment variable. Skipping...",
                env_var_name
            );

            None
        }
    }
}

// Handler function executing the business logic
pub async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Retrieve request ID from the context
    let request_id = event.context.request_id;
    info!("Read request ID from context");

    // Retrieve message from the request payload and format it
    let message = format!("{} {}", RESPONSE_MSG_PREFIX, event.payload.message);
    info!("Read event payload successfully");

    // Read the environment variable ENV, if it exists and is accessible
    let environment = get_env(ENV_VAR_NAME);

    Ok(Response {
        request_id,
        message,
        environment,
    })
}

#[cfg(test)]
mod test {
    use super::{handler, Request, Response};
    use lambda_runtime::{Context, LambdaEvent};

    // Declare constants in the test module for reusability
    const DUMMY_REQUEST_ID: &str = "b84f52c0-7a30-46e2-898b-114674dbbea2";
    const DUMMY_MESSAGE: &str = "test";

    #[tokio::test]
    async fn test_valid_request_succeed() {
        // Assemble
        let message = DUMMY_MESSAGE.to_string();
        let payload = Request { message };
        let mut context = Context::default();
        context.request_id = DUMMY_REQUEST_ID.to_string();

        let event = LambdaEvent { payload, context };

        let request_id = DUMMY_REQUEST_ID.to_string();
        let message = format!("{} {}", super::RESPONSE_MSG_PREFIX, DUMMY_MESSAGE);

        let left = Response {
            request_id,
            message,
            environment: None,
        };

        // Act
        let right = handler(event).await.unwrap();

        // Assert
        assert_eq!(left, right);
    }
}
