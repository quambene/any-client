use crate::client::{prettify, JsonError};
use std::{fmt, str};
use thiserror::Error;
use tungstenite::Message;

pub(crate) struct Item {
    content: String,
    kind: String,
}

#[derive(Error, Debug)]
pub(crate) enum ItemError {
    #[error("Can't convert to string: {err} in payload {payload}")]
    Utf8Error { err: String, payload: String },
    #[error("Can't prettify json: {0}")]
    PrettifyError(String),
}

impl<'a> From<JsonError<'a>> for ItemError {
    fn from(err: JsonError) -> Self {
        Self::PrettifyError(err.to_string())
    }
}

impl Item {
    pub fn new(content: String, kind: String) -> Self {
        Self { content, kind }
    }
}

impl TryFrom<Message> for Item {
    type Error = ItemError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Text(data) => {
                let content = prettify(&data)?;
                let item = Item::new(content, "Text".to_string());
                Ok(item)
            }
            Message::Binary(ref data) => {
                let content = str::from_utf8(data).map_err(|err| ItemError::Utf8Error {
                    err: err.to_string(),
                    payload: String::from_utf8_lossy(data).to_string(),
                })?;
                let content = prettify(content)?;
                let item = Item::new(content, "Binary".to_string());
                Ok(item)
            }
            Message::Ping(ref data) => {
                let content = str::from_utf8(data).map_err(|err| ItemError::Utf8Error {
                    err: err.to_string(),
                    payload: String::from_utf8_lossy(data).to_string(),
                })?;
                let content = prettify(content)?;
                let item = Item::new(content, "Ping".to_string());
                Ok(item)
            }
            Message::Pong(ref data) => {
                let content = str::from_utf8(data).map_err(|err| ItemError::Utf8Error {
                    err: err.to_string(),
                    payload: String::from_utf8_lossy(data).to_string(),
                })?;
                let content = prettify(content)?;
                let item = Item::new(content, "Pong".to_string());
                Ok(item)
            }
            Message::Frame(frame) => {
                let content =
                    str::from_utf8(frame.payload()).map_err(|err| ItemError::Utf8Error {
                        err: err.to_string(),
                        payload: String::from_utf8_lossy(frame.payload()).to_string(),
                    })?;
                let content = prettify(content)?;
                let item = Item::new(content, "Frame".to_string());
                Ok(item)
            }
            Message::Close(Some(frame)) => {
                let content = frame.reason;
                let content = prettify(&content)?;
                let item = Item::new(content, "Close".to_string());
                Ok(item)
            }
            Message::Close(None) => Ok(Item::new("Close".to_string(), "Close".to_string())),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.content)
    }
}
