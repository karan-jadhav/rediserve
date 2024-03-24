use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, FromRequestParts, Request},
    http::{header::HeaderName, request::Parts, HeaderValue},
    response::Response,
};

use serde::Deserialize;

use super::api_types::JsonValue;

#[derive(Deserialize, Debug)]
pub enum ApiInputValue {
    Single(JsonValue),
    List(Vec<JsonValue>),
    None,
}

pub struct ApiInput(pub ApiInputValue);

#[async_trait]
impl<S> FromRequest<S> for ApiInput
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
                return Ok(ApiInput(ApiInputValue::None));
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
                        .map(|s| ApiInput(ApiInputValue::List(s)))?;

                    return Ok(deserialized);
                } else {
                    let body = Bytes::from_request(req, state).await.map_err(|_| {
                        Response::builder()
                            .status(400)
                            .body(Body::from("invalid body"))
                            .unwrap()
                    })?;

                    let deserialized: ApiInput = String::from_utf8(body.to_vec())
                        .map_err(|_| {
                            Response::builder()
                                .status(400)
                                .body(Body::from("invalid utf8"))
                                .unwrap()
                        })
                        .map(|s| ApiInput(ApiInputValue::Single(JsonValue::String(s))))?;

                    return Ok(deserialized);
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ExtractEncoding(String);

impl ExtractEncoding {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractEncoding
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // check of Upstash-Encoding or Rediserve-Encoding header else return utf-8
        let encoding = parts
            .headers
            .get(HeaderName::from_static("upstash-encoding"))
            .or_else(|| {
                parts
                    .headers
                    .get(HeaderName::from_static("rediserve-encoding"))
            });

        match encoding {
            None => Ok(ExtractEncoding("utf-8".to_string())),
            Some(encoding) => {
                let encoding = encoding.to_str().map_err(|_| {
                    Response::builder()
                        .status(400)
                        .body(Body::from("invalid encoding"))
                        .unwrap()
                })?;

                // match for base64
                if encoding == "base64" {
                    return Ok(ExtractEncoding("base64".to_string()));
                } else {
                    return Result::Err(
                        Response::builder()
                            .status(400)
                            .body(Body::from("invalid encoding"))
                            .unwrap(),
                    );
                }
            }
        }
    }
}
