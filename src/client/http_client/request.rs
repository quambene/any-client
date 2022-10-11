use reqwest::{header::HeaderMap, Method, Url, Version};
use std::fmt;

#[derive(Debug)]
pub struct Request {
    pub version: Version,
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: Option<String>,
}

impl Request {
    pub fn new(
        version: Version,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<String>,
    ) -> Self {
        Self {
            version,
            method,
            url,
            headers,
            body,
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Method: {}", &self.method)?;
        writeln!(f, "Url: {}", &self.url)?;
        writeln!(f, "Version: {:#?}", &self.version)?;
        writeln!(f, "Headers: {:#?}", &self.headers)?;
        match &self.body {
            Some(body) => writeln!(f, "Body: {}", body)?,
            None => writeln!(f, "Body: {{}}")?,
        }
        Ok(())
    }
}
