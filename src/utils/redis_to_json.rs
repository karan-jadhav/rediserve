use crate::models::api_types::{JsonValue, RedisValue};
use base64::{engine, prelude::*};

pub fn redis_value_to_json(redis_value: RedisValue, encoding: &str) -> JsonValue {
    match redis_value {
        RedisValue::Status(status) => {
            if encoding == "base64" {
                JsonValue::String(engine::general_purpose::STANDARD.encode(status))
            } else {
                JsonValue::String(status)
            }
        }
        RedisValue::Int(int) => JsonValue::Number(int.into()),
        RedisValue::Data(data) => {
            if encoding == "base64" {
                JsonValue::String(engine::general_purpose::STANDARD.encode(data))
            } else {
                match String::from_utf8(data) {
                    Ok(s) => JsonValue::String(s),
                    Err(_) => JsonValue::Null,
                }
            }
        }
        RedisValue::Bulk(bulk) => JsonValue::Array(
            bulk.into_iter()
                .map(|v| redis_value_to_json(v, encoding))
                .collect(),
        ),
        RedisValue::Okay => JsonValue::String("OK".to_string()),
        RedisValue::Nil => JsonValue::Null,
    }
}
