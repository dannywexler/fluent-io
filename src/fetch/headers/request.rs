use std::{fmt::Display, ops::Deref};

use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::fetch::headers::{FluentHeaderMap, FluentHeaderValue, normalize_header_key};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RequestHeaders(FluentHeaderMap);

impl RequestHeaders {
    pub fn set_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        let key = normalize_header_key(key);
        let value = value.as_ref().to_string();
        self.0.0.insert(key, FluentHeaderValue::Single(value));
        self
    }

    pub fn append_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        let key = normalize_header_key(key);
        let value = value.as_ref().to_string();
        let new_value = if let Some(existing_entry) = self.0.0.get(&key) {
            match existing_entry {
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
        self.0.0.insert(key, new_value);
        self
    }

    pub fn set_content_type(&mut self, content_type: Mime) -> &mut Self {
        self.set_custom_header(CONTENT_TYPE, content_type);
        self
    }
}

impl Deref for RequestHeaders {
    type Target = FluentHeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RequestHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HeaderMap> for RequestHeaders {
    fn from(value: HeaderMap) -> Self {
        RequestHeaders(value.into())
    }
}

impl From<RequestHeaders> for HeaderMap {
    fn from(value: RequestHeaders) -> Self {
        value.0.into()
    }
}
