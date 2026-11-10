use std::{collections::BTreeMap, fmt::Display};

use mime::Mime;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use content_type::ContentTypeError;
pub mod content_type;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseHeaders(BTreeMap<String, Vec<String>>);

impl ResponseHeaders {
    pub fn custom_header(&self, key: impl AsRef<str>) -> Option<String> {
        self.0
            .get(&key.as_ref().to_lowercase().replace("-", "_"))
            .map(|vec| vec[0].clone())
    }

    pub fn content_type(&self) -> Result<Mime, ContentTypeError> {
        let content_type_str = self
            .custom_header("Content-Type")
            .ok_or(ContentTypeError::NotFound)?;
        let parsed: Mime = content_type_str
            .parse()
            .map_err(ContentTypeError::ParseError)?;
        Ok(parsed)
    }
}

impl Display for ResponseHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_else(|serde_err| panic!("Should be impossible to have an error serializing ResponseHeaders. Got error: {serde_err}")))
    }
}

impl From<HeaderMap> for ResponseHeaders {
    fn from(header_map: HeaderMap) -> Self {
        let mut hmap: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (key, value) in header_map {
            if let Some(key) = key {
                let key = key.as_str().replace("-", "_").to_string();
                if let Ok(value) = value.to_str() {
                    let value = value.to_string();
                    hmap.entry(key).or_default().push(value);
                }
            }
        }
        ResponseHeaders(hmap)
    }
}
