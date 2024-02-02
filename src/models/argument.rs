use redis::ToRedisArgs;
use serde::Deserialize;

use super::api_types::JsonValue;

#[derive(Debug, Deserialize)]
pub struct Argument(pub JsonValue);

impl From<&JsonValue> for Argument {
    fn from(arg: &JsonValue) -> Self {
        match arg {
            JsonValue::String(string) => Argument(JsonValue::String(string.to_string())),
            JsonValue::Number(number) => {
                // if float then convert to string else NumberArg
                if let Some(int) = number.as_i64() {
                    Argument(JsonValue::Number(int.into()))
                } else {
                    Argument(JsonValue::String(number.to_string()))
                }
            }
            JsonValue::Bool(boolean) => Argument(JsonValue::Bool(*boolean)),
            _ => Argument(JsonValue::Null),
        }
    }
}
impl From<&String> for Argument {
    fn from(arg: &String) -> Self {
        // check for types
        if let Ok(int) = arg.parse::<i64>() {
            Argument(JsonValue::Number(int.into()))
        } else if let Ok(boolean) = arg.parse::<bool>() {
            Argument(JsonValue::Bool(boolean))
        } else {
            Argument(JsonValue::String(arg.to_string()))
        }
    }
}

impl ToRedisArgs for Argument {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match &self.0 {
            JsonValue::String(string) => string.write_redis_args(out),
            JsonValue::Number(number) => {
                // if float then convert to string else NumberArg
                if let Some(int) = number.as_i64() {
                    int.write_redis_args(out)
                } else {
                    number.to_string().write_redis_args(out)
                }
            }
            JsonValue::Null => "".write_redis_args(out),
            JsonValue::Bool(boolean) => boolean.write_redis_args(out),
            JsonValue::Object(_) => "".write_redis_args(out),
            JsonValue::Array(_) => "".write_redis_args(out),
        }
    }
}
