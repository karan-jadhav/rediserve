use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::header::HeaderName,
    http::HeaderValue,
    response::Response,
};

use serde::Deserialize;

use super::api_types::JsonValue;

#[derive(Deserialize, Debug)]
pub struct PipelineApiInput(pub Vec<Vec<JsonValue>>);

#[async_trait]
impl<S> FromRequest<S> for PipelineApiInput
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // check content type
        let content_type = req.headers().get(HeaderName::from_static("content-type"));

        match content_type {
            None => {
                return Err(Response::builder()
                    .status(400)
                    .body(Body::from("invalid content type"))
                    .unwrap());
            }
            Some(content_type) => {
                if content_type == HeaderValue::from_static("application/json") {
                    let body = Bytes::from_request(req, state).await.map_err(|_| {
                        Response::builder()
                            .status(400)
                            .body(Body::from("invalid body"))
                            .unwrap()
                    })?;

                    // parse json from bytes

                    let deserialized = serde_json::from_slice(&body)
                        .map_err(|e| {
                            Response::builder()
                                .status(400)
                                .body(Body::from(format!("invalid json: {}", e)))
                                .unwrap()
                        })
                        .map(|s| PipelineApiInput(s))?;

                    return Ok(deserialized);
                } else {
                    return Err(Response::builder()
                        .status(400)
                        .body(Body::from("invalid content type"))
                        .unwrap());
                }
            }
        }
    }
}
