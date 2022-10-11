use reqwest::{header::HeaderMap, StatusCode, Version};
use std::fmt;

#[derive(Debug)]
pub struct Response {
    pub version: Version,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl Response {
    pub fn new(version: Version, status: StatusCode, headers: HeaderMap, body: String) -> Self {
        Self {
            version,
            status,
            headers,
            body,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", &self.status)?;
        writeln!(f, "Version: {:#?}", &self.version)?;
        writeln!(f, "Headers: {:#?}", &self.headers)?;
        writeln!(f, "Body: {}", &self.body)?;
        Ok(())
    }
}
