# Temple for AWS Lambda function in Rust

[![MIT License](https://img.shields.io/badge/license-MIT-green)](/LICENSE)

Quick start template to start developing Lambda functions in Rust.

The template uses Docker multistage build to compile the function using `musl` toolchain and wraps
it in an Alpine Linux container, to keep container image small.

The lambda entry point and business logic are separated in an axiomatic manner, which enables easy
testing of business logic in a Rust-native manner.

## Running the Lambda function

The process looks as follows:

1. Build the container image.
2. Tag it so that it matches the ECR registry name.
3. Push to the ECR registry.
4. Create a Lambda function with the image pushed to the registry.

Refer to the [AWS Lambda docs](https://docs.aws.amazon.com/lambda/latest/dg/images-create.html) for
details.
