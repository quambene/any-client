pub mod config;
pub mod item;

use self::{config::Config, item::Item};
use crate::client::prettify_json;
use anyhow::anyhow;
use futures::{SinkExt, Stream, StreamExt};
use log::{error, info};
use std::collections::HashMap;
pub use tungstenite::{Error, Message};
use url::Url;

pub async fn use_websocket_client(config_file: String) -> Result<(), anyhow::Error> {
    info!("Using websocket client");

    let config = serde_json::from_str(&config_file)?;

    let mut stream = connect(config).await?;

    info!("Receiving stream");

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => println!("{}", Item::try_from(msg)?),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}

async fn connect(
    config: Config,
) -> Result<impl Stream<Item = Result<Message, Error>>, anyhow::Error> {
    let mut base_url = Url::parse(&format!("{}{}", &config.api.url, &config.api.endpoint))?;

    let mut key_value_pairs = HashMap::new();

    if let Some(query_string) = &config.subscription.query_string {
        if let Some(object) = query_string.as_object() {
            for (key, value) in object {
                if let Some(value) = value.as_str() {
                    key_value_pairs.insert(key, value);
                }
            }
        }
    }

    let url_encoded = if !key_value_pairs.is_empty() {
        Some(
            form_urlencoded::Serializer::new(String::new())
                .extend_pairs(key_value_pairs)
                .finish(),
        )
    } else {
        None
    };

    base_url.set_query(url_encoded.as_deref());

    info!("Connecting to websocket at url '{}'", base_url);

    let (socket, response) = tokio_tungstenite::connect_async(base_url).await?;

    // Check if protocol was changed to websocket protocol (see
    // https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
    if response.status() == 101 {
        info!("Switching to websocket protocol");
        info!("Connection to websocket established");

        let (mut sink, stream) = socket.split();

        if let Some(subscription_request) = config.subscription.request {
            let subscription_message = Message::Text(subscription_request.to_string());

            info!(
                "Sending subscription message\nMessage: {}",
                prettify_json(subscription_request)?
            );

            sink.send(subscription_message).await?;
        }

        Ok(stream)
    } else {
        Err(anyhow!(format!(
            "Invalid status code {} for websocket response",
            response.status()
        )))
    }
}
