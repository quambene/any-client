use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api: Api,
    pub subscription: Subscription,
}

#[derive(Deserialize)]
pub struct Api {
    pub url: String,
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct Subscription {
    pub request: Option<Value>,
    pub query_string: Option<Value>,
}
