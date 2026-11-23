use std::{collections::BTreeMap, fmt::Display};

use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::fetch::headers::content_type::ContentTypeError;

pub mod content_type;
pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum FluentHeaderValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct FluentHeaderMap(BTreeMap<String, FluentHeaderValue>);

impl FluentHeaderMap {
    pub fn custom_header(&self, key: impl AsRef<str>) -> Option<String> {
        self.0.get(&normalize_header_key(key)).map(|val| match val {
            FluentHeaderValue::Single(item) => item.clone(),
            FluentHeaderValue::Multiple(items) => items[0].clone(),
        })
    }

    pub fn custom_header_all(&self, key: impl AsRef<str>) -> Option<Vec<String>> {
        self.0.get(&normalize_header_key(key)).map(|val| match val {
            FluentHeaderValue::Single(item) => vec![item.clone()],
            FluentHeaderValue::Multiple(items) => items.clone(),
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
        let mut btmap: BTreeMap<String, FluentHeaderValue> = BTreeMap::new();
        for header_name in header_map.keys() {
            for value in
                header_map
                    .get_all(header_name)
                    .iter()
                    .filter_map(|header_value| {
                        header_value.to_str().ok().map(|str| str.to_string())
                    })
            {
                let key = normalize_header_key(header_name.as_str());
                let new_value = if let Some(entry) = btmap.get(&key) {
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
                btmap.insert(key, new_value);
            }
        }
        FluentHeaderMap(btmap)
    }
}

impl From<FluentHeaderMap> for HeaderMap {
    fn from(value: FluentHeaderMap) -> Self {
        let mut hmap = HeaderMap::new();
        for (header_key, fluent_header_value) in value.0 {
            let header_name: HeaderName = header_key
                .parse()
                .unwrap_or_else(|err| panic!("Should be impossible to have an error parsing a FluentHeaderKey into a HeaderName.\nAttempted to parse: {header_key}\nGot error {err}"));
            let header_values = match fluent_header_value {
                FluentHeaderValue::Single(header_value) => vec![header_value],
                FluentHeaderValue::Multiple(items) => items,
            };
            for header_value_string in header_values {
                let header_value: HeaderValue = header_value_string
                    .parse()
                    .unwrap_or_else(|err| panic!("Should be impossible to have an error parsing a FluentHeaderValue into a HeaderValue.\nAttempted to parse: {header_value_string}\nGot error {err}"));
                hmap.append(header_name.clone(), header_value);
            }
        }
        hmap
    }
}

pub(crate) fn normalize_header_key(key: impl AsRef<str>) -> String {
    key.as_ref().to_lowercase().replace("-", "_")
}
