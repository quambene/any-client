use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    pub api: Api,
    pub proto: Proto,
}

#[derive(Deserialize)]
pub struct Api {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Proto {
    pub path: String,
    pub file: String,
    pub service: String,
    pub package: String,
    pub method: String,
    pub message: String,
    pub request: Option<Value>,
    pub metadata: Option<String>,
}
