use serde::{de, Deserialize, Deserializer};
use std::fmt;

const HTTP: &str = "http";
const WEBSOCKET: &str = "websocket";
const GRPC: &str = "grpc";

pub enum ClientType {
    Http,
    Websocket,
    Grpc,
}

impl<'de> Deserialize<'de> for ClientType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let request_method = match <&str>::deserialize(deserializer)? {
            HTTP => Self::Http,
            WEBSOCKET => Self::Websocket,
            GRPC => Self::Grpc,
            others => return Err(de::Error::unknown_variant(others, &[HTTP, WEBSOCKET, GRPC])),
        };

        Ok(request_method)
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_method = match self {
            Self::Http => HTTP,
            Self::Websocket => WEBSOCKET,
            Self::Grpc => GRPC,
        };

        write!(f, "{}", request_method)
    }
}
