use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{LambdaEvent, service_fn};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: serde_json::to_string(&event.payload)
            .ok()
            .map(|v| Body::Text(v)),
        is_base64_encoded: None,
    })
}
