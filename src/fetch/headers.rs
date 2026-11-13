use std::{collections::BTreeMap, fmt::Display};

use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::fetch::headers::content_type::ContentTypeError;

pub mod content_type;
pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FluentHeaderValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FluentHeaderMap(BTreeMap<String, FluentHeaderValue>);

impl FluentHeaderMap {
    pub fn custom_header(&self, key: impl AsRef<str>) -> Option<String> {
        self.0
            .get(&key.as_ref().to_lowercase().replace("-", "_"))
            .map(|val| match val {
                FluentHeaderValue::Single(item) => item.clone(),
                FluentHeaderValue::Multiple(items) => items[0].clone(),
            })
    }

    pub fn content_type(&self) -> Result<Mime, ContentTypeError> {
        let content_type_str = self
            .custom_header(CONTENT_TYPE)
            .ok_or(ContentTypeError::NotFound)?;
        let parsed: Mime = content_type_str
            .parse()
            .map_err(ContentTypeError::ParseError)?;
        Ok(parsed)
    }
}

impl Display for FluentHeaderMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_else(|serde_err| panic!("Should be impossible to have an error serializing. Got error: {serde_err}")))
    }
}

impl From<HeaderMap> for FluentHeaderMap {
    fn from(header_map: HeaderMap) -> Self {
        let mut hmap: BTreeMap<String, FluentHeaderValue> = BTreeMap::new();
        for header_name in header_map.keys() {
            for value in
                header_map
                    .get_all(header_name)
                    .iter()
                    .filter_map(|header_value| {
                        header_value.to_str().ok().map(|str| str.to_string())
                    })
            {
                let key = header_name.as_str().replace("-", "_").to_string();
                let new_value = if let Some(entry) = hmap.get(&key) {
                    match entry {
                        FluentHeaderValue::Single(item) => {
                            FluentHeaderValue::Multiple(vec![item.clone(), value])
                        }
                        FluentHeaderValue::Multiple(items) => {
                            let mut new_items = items.clone();
                            new_items.push(value);
                            FluentHeaderValue::Multiple(new_items)
                        }
                    }
                } else {
                    FluentHeaderValue::Single(value)
                };
                hmap.insert(key, new_value);
            }
        }
        FluentHeaderMap(hmap)
    }
}
