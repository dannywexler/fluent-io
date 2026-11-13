use std::{fmt::Display, ops::Deref};

use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::fetch::headers::{FluentHeaderMap, FluentHeaderValue};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RequestHeaders(FluentHeaderMap);

impl RequestHeaders {
    pub fn append_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        let new_value = if let Some(entry) = self.0.0.get(&key) {
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
        self.0.0.insert(key, new_value);
        self
    }

    pub fn set_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.0.0.insert(
            key.as_ref().to_lowercase().replace("-", "_").to_string(),
            super::FluentHeaderValue::Single(value.as_ref().to_string()),
        );
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
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_else(|serde_err| panic!("Should be impossible to have an error serializing. Got error: {serde_err}")))
    }
}

impl From<HeaderMap> for RequestHeaders {
    fn from(value: HeaderMap) -> Self {
        RequestHeaders(FluentHeaderMap::from(value))
    }
}
