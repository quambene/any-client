pub mod client_type;
pub mod grpc_client;
pub mod http_client;
pub mod websocket_client;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{io, string::FromUtf8Error};
use thiserror::Error;

use self::client_type::ClientType;

#[derive(Deserialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub client_type: ClientType,
}

pub fn prettify(obj: &str) -> Result<String, JsonError> {
    let obj: Value = serde_json::from_str(obj).map_err(|err| JsonError::DeserializeError {
        err: err.to_string(),
        payload: obj,
    })?;
    let buf = Vec::new();
    let formatter = pretty_json::Formatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)
        .map_err(|err| JsonError::SerializeError(err.to_string()))?;
    Ok(String::from_utf8(ser.into_inner())?)
}

pub fn prettify_json(obj: Value) -> Result<String, anyhow::Error> {
    let buf = Vec::new();
    let formatter = pretty_json::Formatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)?;
    Ok(String::from_utf8(ser.into_inner())?)
}

#[derive(Error, Debug)]
pub enum JsonError<'a> {
    #[error("Can't serialize object: {0}")]
    SerializeError(String),
    #[error("Can't deserialize string: {err} in payload {payload}")]
    DeserializeError { err: String, payload: &'a str },
    #[error("Can't convert to string: {0}")]
    Utf8Errror(String),
}

impl<'a> From<io::Error> for JsonError<'a> {
    fn from(err: std::io::Error) -> Self {
        Self::SerializeError(err.to_string())
    }
}

impl<'a> From<FromUtf8Error> for JsonError<'a> {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8Errror(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prettify() {
        let obj = json!({"foo":1,"bar":{"innerBar":[["100","200"],["300","400"]]}});
        let buf = Vec::new();
        let formatter = pretty_json::Formatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        obj.serialize(&mut ser).unwrap();

        let actual_json = String::from_utf8(ser.into_inner()).unwrap();
        let expected_json = r#"{
  "bar": {
    "innerBar": [["100","200"],["300","400"]]
  },
  "foo": 1
}"#;

        println!("{}", actual_json);

        assert_eq!(actual_json, expected_json);
    }
}
