use std::ops::{Deref, DerefMut};

use reqwest::Client;
use serde_json::Value;

use crate::url::Hurl;

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

    pub async fn send_get(&self) -> serde_json::Value {
        let res = self
            .client
            .get(self.hurl.to_string())
            .send()
            .await
            .expect("Can send request");
        let status = res.status();
        let status_code = status.as_u16();

        let backup_text = format!("Unknown{status_code}");
        let backup_text = backup_text.as_str();
        let status_text = status
            .canonical_reason()
            .unwrap_or(backup_text)
            .replace(" ", "")
            .replace("-", "")
            .replace("'", "");

        let response_headers = res.headers().clone();
        let bytes = res.bytes().await.expect("Can get bytes");
        if let Some(content_type) = response_headers
            .get("Content-Type")
            .and_then(|ct| ct.to_str().ok())
        {
            if content_type.starts_with("text/") {
                return serde_json::json!({
                    status_text: {
                        "status": status_code,
                        "body": String::from_utf8(bytes.to_vec()).expect("Can read body as utf8 string"),
                    }
                });
            } else if content_type.contains("json") {
                let body: Value =
                    serde_json::from_slice(&bytes).expect("Can read body as json");
                return serde_json::json!({
                    status_text: {
                        "status": status_code,
                        "body": body,
                    }
                });
            }
        }
        let bytes_vec: Vec<u8> = bytes.into();
        serde_json::json!({
            status_text: {
                "status": status_code,
                "body": bytes_vec,
            }
        })
    }
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::fetch::FluentFetch;

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    #[serde(rename_all = "camelCase")]
    struct User {
        pub first_name: String,
        pub email: String,
    }

    #[derive(Debug, Deserialize)]
    enum UserResponses {
        OK { body: User },
        NotFound {},
    }

    #[tokio::test]
    async fn test_fetch_struct() {
        let prefix = "https://dummyjson.com/users";
        let cases = vec![("1", false), ("-1", true)];
        for (id, invalid) in cases {
            let mut ff = FluentFetch::parse_base_url(prefix).expect("valid url");
            ff.add_path_seg(id);
            println!("Fetching url: {}", ff.to_string());
            let res_value = ff.send_get().await;
            println!("Got res_value:\n{res_value:#?}");
            match serde_json::from_value::<UserResponses>(res_value) {
                Ok(responses) => {
                    match responses {
                        UserResponses::OK { body } => {
                            if !invalid {
                                println!("Got user:\n{body:#?}");
                            }
                        }
                        UserResponses::NotFound {} => {
                            if invalid {
                                println!("No user found with id {id}");
                            }
                        }
                    };
                    assert!(true, "Parsing succeeded");
                }
                Err(serde_err) => {
                    let category = if serde_err.is_io() {
                        "io"
                    } else if serde_err.is_syntax() {
                        "syntax"
                    } else if serde_err.is_data() {
                        "data"
                    } else {
                        "eof"
                    };
                    println!("Serde err category: {category}");
                    println!(
                        "Serde err line: {}, column: {}",
                        serde_err.line(),
                        serde_err.column()
                    );
                    println!("Serde err msg: {serde_err}");
                    println!("Serde err msg:? {serde_err:?}");
                    println!("Serde err msg:#? {serde_err:#?}");
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
            let res_value = ff.send_get().await;
            println!("Got res_value:\n{res_value:#?}");
            let res_enum: TextResponses =
                serde_json::from_value(res_value).expect("Can parse as text");

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
        let res_value = FluentFetch::parse_base_url("https://dummyjson.com/image/10")
            .expect("Valid url")
            .send_get()
            .await;
        println!("Got res_value:\n{res_value:?}");
        let parsed_response: BytesResponses =
            serde_json::from_value(res_value).expect("Can parse as bytes");

        match parsed_response {
            BytesResponses::OK { body } => {
                println!("Got bytes:\n{body:?}");
            }
        }
    }
}
