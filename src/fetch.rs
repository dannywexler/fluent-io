use std::{
    ops::{Deref, DerefMut},
    string::FromUtf8Error,
};

use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{fetch::headers::ResponseHeaders, url::Hurl};

pub mod headers;

pub struct FluentFetch {
    hurl: Hurl,
    client: Client,
}

impl FluentFetch {
    pub fn base_url(hurl: Hurl) -> Self {
        Self {
            hurl,
            client: Client::new(),
        }
    }

    pub fn parse_base_url(base_url: impl AsRef<str>) -> Result<Self, String> {
        Ok(Hurl::parse_string(base_url)?.into())
    }

    pub fn set_reqwest_client(&mut self, reqwest_client: Client) -> &mut Self {
        self.client = reqwest_client;
        self
    }

    pub async fn send_get<T: DeserializeOwned>(&self) -> Result<T, FetchError> {
        let res = self
            .client
            .get(self.hurl.to_string())
            .send()
            .await
            .map_err(|reqwest_error| FetchError {
                request_url: self.hurl.to_string(),
                cause: FetchErrorCause::SendRequest { reqwest_error },
            })?;
        let status = res.status();
        let status_code = status.as_u16();

        let fallback_status_text = format!("Unknown{status_code}");
        let fallback_text = fallback_status_text.as_str();
        let status_text = status
            .canonical_reason()
            .unwrap_or(fallback_text)
            .replace(" ", "")
            .replace("-", "")
            .replace("'", "");
        let status_text_clone = status_text.clone();

        let response_url_str = res.url().to_string();
        let response_headers: ResponseHeaders = res.headers().clone().into();
        let body = resolve_body_value(res).await.map_err(|cause| FetchError {
            request_url: self.hurl.to_string(),
            cause,
        })?;
        serde_json::from_value(serde_json::json!({
            status_text: {
                "body": body,
                "request_url": self.hurl.to_string(),
                "response_headers": response_headers,
                "response_url": response_url_str,
                "status_code": status_code,
            }
        }))
        .map_err(|serde_json_error| {
            let cause = if format!("{serde_json_error}").starts_with("unknown variant") {
                FetchErrorCause::UnhandledStatus {
                    status_code,
                    status_text: status_text_clone,
                    serde_json_error,
                }
            } else {
                FetchErrorCause::JsonSchemaMismatch { serde_json_error }
            };
            FetchError {
                request_url: self.hurl.to_string(),
                cause,
            }
        })
    }
}

async fn resolve_body_value(
    response: Response,
) -> Result<serde_json::Value, FetchErrorCause> {
    let response_headers: ResponseHeaders = response.headers().clone().into();
    let bytes = response
        .bytes()
        .await
        .map_err(|reqwest_error| FetchErrorCause::ReadBody { reqwest_error })?;
    if let Ok(content_type) = response_headers.content_type() {
        if content_type.type_() == "text" {
            return Ok(String::from_utf8(bytes.to_vec())
                .map_err(|from_utf8_error| FetchErrorCause::ReadBodyAsText {
                    from_utf8_error,
                })?
                .into());
        }
        if content_type.subtype().as_str().ends_with("json")
            || if let Some(suffix) = content_type.suffix() {
                suffix == "json"
            } else {
                false
            }
        {
            return Ok(serde_json::from_slice(&bytes).map_err(|serde_json_error| {
                FetchErrorCause::ReadBodyAsJson { serde_json_error }
            })?);
        }
    }
    let bytes_vec: Vec<u8> = bytes.into();
    Ok(bytes_vec.into())
}

impl From<Hurl> for FluentFetch {
    fn from(value: Hurl) -> Self {
        FluentFetch::base_url(value)
    }
}

impl Deref for FluentFetch {
    type Target = Hurl;

    fn deref(&self) -> &Self::Target {
        &self.hurl
    }
}

impl DerefMut for FluentFetch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hurl
    }
}

#[derive(Debug)]
pub struct FetchError {
    pub request_url: String,
    pub cause: FetchErrorCause,
}

#[derive(Debug)]
pub enum FetchErrorCause {
    SendRequest {
        reqwest_error: reqwest::Error,
    },
    ReadBody {
        reqwest_error: reqwest::Error,
    },
    ReadBodyAsText {
        from_utf8_error: FromUtf8Error,
    },
    ReadBodyAsJson {
        serde_json_error: serde_json::Error,
    },
    UnhandledStatus {
        status_code: u16,
        status_text: String,
        serde_json_error: serde_json::Error,
    },
    JsonSchemaMismatch {
        serde_json_error: serde_json::Error,
    },
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Value;

    use crate::fetch::{FluentFetch, headers::ResponseHeaders};

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    #[serde(rename_all = "camelCase")]
    struct User {
        pub first_name: String,
        pub email: String,
    }

    #[derive(Debug, Deserialize)]
    enum UserResponses {
        OK {
            body: User,
        },
        NotFound {
            request_url: String,
            response_headers: ResponseHeaders,
            response_url: String,
            body: Value,
        },
    }

    #[tokio::test]
    async fn test_fetch_struct() {
        let prefix = "https://dummyjson.com/users";
        let cases = vec![("1", false), ("-1", true)];
        for (id, invalid) in cases {
            let mut ff = FluentFetch::parse_base_url(prefix).expect("valid url");
            ff.add_path_seg(id);
            println!("Fetching url: {}", ff.to_string());
            match ff.send_get::<UserResponses>().await {
                Ok(responses) => {
                    match responses {
                        UserResponses::OK { body } => {
                            if !invalid {
                                println!("Got user:\n{body:#?}");
                            }
                        }
                        UserResponses::NotFound {
                            response_headers,
                            request_url,
                            response_url,
                            body,
                        } => {
                            if invalid {
                                println!("No user found with id {id}");
                                println!("request_url: {request_url}");
                                println!("response_url: {response_url}");
                                println!("ResponseHeaders:\n{}", response_headers);
                                println!("body:");
                                println!("{body}");
                            }
                        }
                    };
                    assert!(true, "Parsing succeeded");
                }
                Err(fetch_err) => {
                    println!("{fetch_err:#?}");
                    assert!(false, "Parsing failed");
                }
            }
        }
    }

    #[derive(Debug, Deserialize)]
    enum TextResponses {
        OK { body: String },
        NotFound { body: String },
    }

    #[tokio::test]
    async fn test_fetch_text() {
        let prefix = "https://raw.githubusercontent.com/serde-rs/serde";
        let cases = vec![("master/README.md", false), ("DOESNTEXIST", true)];
        for (suffix, invalid) in cases {
            let mut ff = FluentFetch::parse_base_url(prefix).expect("valid url");
            ff.add_path_seg(suffix);
            println!("Fetching url: {}", ff.to_string());
            let res_enum: TextResponses = ff.send_get().await.expect("Can parse as text");

            match res_enum {
                TextResponses::OK { body } => {
                    if !invalid {
                        println!("Text Found body:\n{body}");
                    }
                }
                TextResponses::NotFound { body } => {
                    if invalid {
                        println!("Text NotFound body:\n{body}");
                    }
                }
            }
        }
    }

    #[derive(Debug, Deserialize)]
    enum BytesResponses {
        OK { body: Vec<u8> },
    }

    #[tokio::test]
    async fn test_fetch_bytes() {
        let bytes_res: BytesResponses =
            FluentFetch::parse_base_url("https://dummyjson.com/image/10")
                .expect("Valid url")
                .send_get()
                .await
                .expect("Can parse as bytes");

        match bytes_res {
            BytesResponses::OK { body } => {
                println!("Got bytes:\n{body:?}");
            }
        }
    }
}
