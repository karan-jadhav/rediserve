use crate::models::api_types::{JsonValue, RedisValue};

pub fn redis_value_to_json(redis_value: RedisValue) -> JsonValue {
    match redis_value {
        RedisValue::Status(status) => JsonValue::String(status),
        RedisValue::Int(int) => JsonValue::Number(int.into()),
        RedisValue::Data(data) => match String::from_utf8(data) {
            Ok(string) => JsonValue::String(string),
            Err(_) => JsonValue::Null,
        },
        RedisValue::Bulk(bulk) => {
            JsonValue::Array(bulk.into_iter().map(redis_value_to_json).collect())
        }
        RedisValue::Okay => JsonValue::String("OK".to_string()),
        RedisValue::Nil => JsonValue::Null,
    }
}
