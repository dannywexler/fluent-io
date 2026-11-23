use std::{fmt::Display, ops::Deref};

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::fetch::headers::FluentHeaderMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseHeaders(FluentHeaderMap);

impl Deref for ResponseHeaders {
    type Target = FluentHeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ResponseHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HeaderMap> for ResponseHeaders {
    fn from(value: HeaderMap) -> Self {
        ResponseHeaders(value.into())
    }
}

impl From<ResponseHeaders> for HeaderMap {
    fn from(value: ResponseHeaders) -> Self {
        value.0.into()
    }
}
