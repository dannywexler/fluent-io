use std::{ops::Deref, string::FromUtf8Error};

use mime::Mime;
use reqwest::{Body, Client, Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    fetch::headers::{FluentHeaderMap, RequestHeaders, ResponseHeaders},
    url::Hurl,
};

pub mod headers;

pub struct FluentFetch {
    hurl: Hurl,
    client: Client,
    request_headers: RequestHeaders,
}

impl FluentFetch {
    // === === Associated Functions === === //
    pub fn base_url(hurl: Hurl) -> Self {
        Self {
            hurl,
            client: Client::new(),
            request_headers: RequestHeaders::default(),
        }
    }

    pub fn parse_base_url(base_url: impl AsRef<str>) -> Result<Self, String> {
        Ok(Hurl::parse_string(base_url)?.into())
    }

    // === === Hurl methods === === //
    pub fn url(&self) -> Hurl {
        self.hurl.clone()
    }

    pub fn set_reqwest_client(&mut self, reqwest_client: Client) -> &mut Self {
        self.client = reqwest_client;
        self
    }

    pub fn add_path_segment(&mut self, path_segment: impl AsRef<str>) -> &mut Self {
        self.hurl.add_path_segment(path_segment);
        self
    }

    // === === RequestHeaders setter methods === === //
    pub fn set_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.request_headers.set_custom_header(key, value);
        self
    }

    pub fn append_custom_header(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.request_headers.append_custom_header(key, value);
        self
    }

    pub fn set_content_type(&mut self, content_type: Mime) -> &mut Self {
        self.request_headers.set_content_type(content_type);
        self
    }

    // === === REST method convenience helpers === === //
    pub async fn get<T: DeserializeOwned>(&self) -> Result<T, FetchError> {
        self.send_request(&Method::GET, None::<String>).await
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        body: impl Into<Body>,
    ) -> Result<T, FetchError> {
        self.send_request(&Method::POST, Some(body)).await
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        body: impl Into<Body>,
    ) -> Result<T, FetchError> {
        self.send_request(&Method::PUT, Some(body)).await
    }

    pub async fn patch<T: DeserializeOwned>(
        &self,
        body: impl Into<Body>,
    ) -> Result<T, FetchError> {
        self.send_request(&Method::PATCH, Some(body)).await
    }

    pub async fn delete<T: DeserializeOwned>(&self) -> Result<T, FetchError> {
        self.send_request(&Method::DELETE, None::<String>).await
    }

    // === === REST method impl === === //
    pub async fn send_request<T: DeserializeOwned>(
        &self,
        method: &Method,
        maybe_body: Option<impl Into<Body>>,
    ) -> Result<T, FetchError> {
        let mut request_headers_copy = self.request_headers.clone();
        let request = self.client.request(method.clone(), self.hurl.to_string());

        let request =
            resolve_request_body(request, &mut request_headers_copy, maybe_body);

        let request = request.headers(request_headers_copy.clone().into());

        let response = request.send().await.map_err(|reqwest_error| FetchError {
            request_url: self.hurl.to_string(),
            request_headers: request_headers_copy.clone(),
            method: method.clone(),
            cause: FetchErrorCause::SendRequest { reqwest_error },
        })?;
        let status_code = response.status();
        let status_text = resolve_status_text(&status_code);

        let response_url_str = response.url().to_string();
        let response_headers: ResponseHeaders = response.headers().clone().into();
        let response_body =
            resolve_response_body_value(response)
                .await
                .map_err(|cause| FetchError {
                    request_url: self.hurl.to_string(),
                    request_headers: request_headers_copy.clone(),
                    method: method.clone(),
                    cause,
                })?;
        serde_json::from_value(serde_json::json!({
            status_text.clone(): {
                "body": response_body,
                "request_url": self.hurl.to_string(),
                "request_headers": request_headers_copy,
                "response_headers": response_headers,
                "response_url": response_url_str,
            }
        }))
        .map_err(|serde_json_error| {
            let cause = if serde_json_error.to_string().starts_with("unknown variant") {
                FetchErrorCause::UnhandledStatus {
                    status_code,
                    status_text,
                }
            } else {
                FetchErrorCause::ParseResponse { serde_json_error }
            };
            FetchError {
                request_url: self.hurl.to_string(),
                request_headers: request_headers_copy,
                method: method.clone(),
                cause,
            }
        })
    }
}

impl Deref for FluentFetch {
    type Target = FluentHeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.request_headers
    }
}

impl From<Hurl> for FluentFetch {
    fn from(value: Hurl) -> Self {
        FluentFetch::base_url(value)
    }
}

fn resolve_request_body(
    request: RequestBuilder,
    request_headers: &mut RequestHeaders,
    maybe_body: Option<impl Into<Body>>,
) -> RequestBuilder {
    if let Some(body) = maybe_body {
        let body = body.into();
        if let Err(_) = request_headers.content_type()
            && let Some(bytes) = body.as_bytes()
        {
            request_headers.set_content_type(detect_mime(bytes));
        }
        return request.body(body);
    }
    request
}

fn resolve_status_text(status_code: &StatusCode) -> String {
    let status_number = status_code.as_u16();

    status_code
        .canonical_reason()
        .unwrap_or(format!("Unknown{status_number}").as_str())
        .replace(" ", "")
        .replace("-", "")
        .replace("'", "")
}

async fn resolve_response_body_value(
    response: Response,
) -> Result<serde_json::Value, FetchErrorCause> {
    let response_headers: ResponseHeaders = response.headers().clone().into();
    let bytes = response
        .bytes()
        .await
        .map_err(|reqwest_error| FetchErrorCause::ReadBodyAsBytes { reqwest_error })?;
    let content_type = response_headers
        .content_type()
        .unwrap_or_else(|_| detect_mime(&bytes));
    if content_type.type_() == "text" {
        return Ok(String::from_utf8(bytes.into())
            .map_err(|from_utf8_error| FetchErrorCause::ReadBodyAsText {
                from_utf8_error,
            })?
            .into());
    }
    if mime_is_jsonlike(&content_type) {
        return serde_json::from_slice(&bytes).map_err(|serde_json_error| {
            FetchErrorCause::ReadBodyAsJson { serde_json_error }
        });
    }
    let bytes_vec: Vec<u8> = bytes.into();
    Ok(bytes_vec.into())
}

pub(crate) fn detect_mime(bytes: &[u8]) -> Mime {
    let detected_mime_str = mimetype_detector::detect(bytes).to_string();
    detected_mime_str
        .parse::<Mime>()
        .unwrap_or_else(|err| panic!("Should be impossible to have an error parsing a 'Mime' from a 'MimeType'.\nString to parse: {detected_mime_str}\nGot error: {err}"))
}

pub(crate) fn mime_is_jsonlike(mime: &Mime) -> bool {
    if mime.subtype().as_str().ends_with("json") {
        return true;
    }
    if let Some(suffix) = mime.suffix() {
        return suffix.as_str().ends_with("json");
    }
    false
}

#[derive(Debug)]
pub struct FetchError {
    pub request_url: String,
    pub request_headers: RequestHeaders,
    pub method: Method,
    pub cause: FetchErrorCause,
}

#[derive(Debug)]
pub enum FetchErrorCause {
    SendRequest {
        reqwest_error: reqwest::Error,
    },
    ReadBodyAsBytes {
        reqwest_error: reqwest::Error,
    },
    ReadBodyAsText {
        from_utf8_error: FromUtf8Error,
    },
    ReadBodyAsJson {
        serde_json_error: serde_json::Error,
    },
    UnhandledStatus {
        status_code: StatusCode,
        status_text: String,
    },
    ParseResponse {
        serde_json_error: serde_json::Error,
    },
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Value;

    use crate::fetch::{
        FluentFetch,
        headers::{RequestHeaders, ResponseHeaders},
    };

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    #[serde(rename_all = "camelCase")]
    struct User {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
    }

    // #[derive(Debug, Deserialize)]
    // struct NotFoundHeaders {
    //     content_length: String,
    //     content_type: String,
    // }

    #[derive(Debug, Deserialize)]
    enum UserResponses {
        OK {
            body: User,
        },
        NotFound {
            request_url: String,
            response_headers: ResponseHeaders,
            // response_headers: NotFoundHeaders,
            response_url: String,
            body: Value,
        },
    }

    #[tokio::test]
    async fn test_fetch_get_struct() {
        let prefix = "https://dummyjson.com/users";
        let cases = vec![("1", false), ("-1", true)];
        for (id, invalid) in cases {
            let mut ff = FluentFetch::parse_base_url(prefix).expect("valid url");
            ff.add_path_segment(id);
            println!("Fetching url: {}", ff.url().to_string());
            match ff.get::<UserResponses>().await {
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
        OK {
            body: String,
            response_headers: ResponseHeaders,
        },
        NotFound {
            body: String,
        },
    }

    #[tokio::test]
    async fn test_fetch_get_text() {
        let prefix = "https://raw.githubusercontent.com/serde-rs/serde";
        let cases = vec![("master/README.md", false), ("DOESNTEXIST", true)];
        for (suffix, invalid) in cases {
            let mut ff = FluentFetch::parse_base_url(prefix).expect("valid url");
            ff.add_path_segment(suffix);
            println!("Fetching url: {}", ff.url().to_string());
            let res_enum: TextResponses = ff.get().await.expect("Can parse as text");

            match res_enum {
                TextResponses::OK {
                    body,
                    response_headers,
                } => {
                    if !invalid {
                        println!("Text Found successfully!");
                        println!("ResponseHeaders: {response_headers}");
                        println!("Got readme body:\n{body:?}");
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
        OK {
            body: Vec<u8>,
            response_headers: ResponseHeaders,
        },
    }

    #[tokio::test]
    async fn test_fetch_get_bytes() {
        let bytes_res: BytesResponses =
            FluentFetch::parse_base_url("https://dummyjson.com/image/10")
                .expect("Valid url")
                .get()
                .await
                .expect("Can parse as bytes");

        match bytes_res {
            BytesResponses::OK {
                body,
                response_headers,
            } => {
                println!("Fetched image successfully.");
                println!("ResponseHeaders: {response_headers}");
                println!("Got bytes:\n{body:?}");
            }
        }
    }

    #[derive(Debug, Deserialize)]
    enum AddUserResponses {
        Created {
            body: User,
            request_headers: RequestHeaders,
        },
    }

    #[tokio::test]
    async fn test_fetch_post() {
        let add_user_res: AddUserResponses =
            FluentFetch::parse_base_url("https://dummyjson.com/users")
                .expect("Valid url")
                .add_path_segment("add")
                .post(
                    serde_json::json!({
                        "firstName": "FirstName",
                        "lastName": "LastName",
                        "email": "First.Last@email.com",
                    })
                    .to_string(),
                )
                .await
                .expect("Can parse as user");

        match add_user_res {
            AddUserResponses::Created {
                body,
                request_headers,
            } => {
                println!("Added user:");
                println!("RequestHeaders: {request_headers}");
                println!("Body:\n{body:#?}");
            }
        }
    }
}
