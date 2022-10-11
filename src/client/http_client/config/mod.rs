mod request_method;

pub(crate) use self::request_method::RequestMethod;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api: Api,
    pub request: Request,
}

#[derive(Deserialize)]
pub struct Api {
    pub protocol: Option<String>,
    pub url: String,
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct Request {
    pub method: RequestMethod,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Value>,
    pub query_string: Option<Value>,
}

#[derive(Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}
