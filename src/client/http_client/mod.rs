pub mod config;
pub mod request;
pub mod response;

use crate::client::{
    http_client::{
        config::{Config, RequestMethod},
        request::Request,
        response::Response,
    },
    prettify_json,
};
use anyhow::Context;
use log::info;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Client, Url, Version,
};
use serde_json::Value;
use std::str::FromStr;

pub async fn use_http_client(config_file: String) -> Result<(), anyhow::Error> {
    info!("Using http client");

    let config: Config = serde_json::from_str(&config_file).context("Can't deserialize json")?;
    let url = format!("{}{}", config.api.url, config.api.endpoint);
    let url = Url::parse(&url)?;

    let client = Client::new();

    let mut header_map = HeaderMap::new();

    if let Some(headers) = config.request.headers {
        for header in headers {
            let key = HeaderName::from_str(&header.key)?;
            let value = HeaderValue::from_str(&header.value)?;
            header_map.insert(key, value);
        }
    }

    let mut request = match config.request.method {
        RequestMethod::Get => client.get(url),
        RequestMethod::Post => client.post(url),
        RequestMethod::Put => client.put(url),
        RequestMethod::Delete => client.delete(url),
    };

    request = request.version(Version::HTTP_11).headers(header_map);

    if let Some(value) = &config.request.query_string {
        request = request.query(value)
    };

    if let Some(value) = &config.request.body {
        request = request.json(value)
    };

    let request = request.build()?;
    let formatted_request = Request::new(
        request.version(),
        request.method().to_owned(),
        request.url().to_owned(),
        request.headers().to_owned(),
        match config.request.body {
            Some(body) => Some(prettify_json(body)?),
            None => None,
        },
    );

    info!("Sending request\n{}", formatted_request);

    let response = client.execute(request).await?;

    let mut formatted_response = Response::new(
        response.version(),
        response.status(),
        response.headers().to_owned(),
        response.text().await?,
    );

    if let Some(content_type) = formatted_response.headers.get(CONTENT_TYPE) {
        if content_type.to_str()?.contains("application/json") {
            if let Ok(response_body) = serde_json::from_str::<Value>(&formatted_response.body) {
                formatted_response.body = prettify_json(response_body)?;
            };
        }
    }

    info!("Receiving response\n{}", formatted_response);

    Ok(())
}
