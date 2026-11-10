use std::fmt::Display;

use reqwest::Url;
use url::Host;

#[derive(Debug, Clone)]
pub struct Hurl {
    host: Host<String>,
    path: Vec<String>,
    query: Option<String>,
    is_secure: bool,
}

impl Hurl {
    pub fn parse_string(input_url: impl AsRef<str>) -> Result<Self, String> {
        let url = Url::parse(input_url.as_ref())
            .map_err(|parse_error| format!("Url ParseError: {parse_error}"))?;
        Hurl::parse_url(&url)
    }

    pub fn parse_url(url: &Url) -> Result<Self, String> {
        let scheme = url.scheme();
        let is_secure = match scheme {
            "https" => true,
            "http" => false,
            _ => return Err(format!("Scheme was not http or https. Was {scheme}")),
        };
        let host = url.host().ok_or("Missing Host")?.to_owned();
        let path: Vec<String> = url
            .path_segments()
            .ok_or("Url cannot be a base")?
            .filter_map(|item| {
                if item.is_empty() {
                    return None;
                }
                Some(item.to_string())
            })
            .collect();
        let query = url.query().map(|q| q.to_string());
        Ok(Self {
            host,
            path,
            query,
            is_secure,
        })
    }

    pub fn is_secure(&self) -> bool {
        self.is_secure
    }

    pub fn host(&self) -> Host<String> {
        self.host.clone()
    }

    pub fn path_vec(&self) -> Vec<String> {
        self.path.clone()
    }

    pub fn path_str(&self) -> String {
        format!("/{}", self.path.join("/"))
    }

    pub fn set_path_vec(
        &mut self,
        pieces: impl Iterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.path = pieces
            .filter_map(|piece| {
                let piece = piece.as_ref().trim().to_string();
                if piece.is_empty() {
                    return None;
                }
                Some(percent_encode(piece))
            })
            .collect();
        self
    }

    pub fn add_path_seg(&mut self, path_segment: impl AsRef<str>) -> &mut Self {
        self.path.push(percent_encode(path_segment));
        self
    }

    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    pub fn set_query(&mut self, new_query: Option<impl AsRef<str>>) -> &mut Self {
        self.query = new_query.map(|q| q.as_ref().to_string());
        self
    }

    pub fn add_query(&mut self, added_query: impl AsRef<str>) -> &mut Self {
        let added_query = added_query.as_ref().to_string();
        self.query = match self.query() {
            Some(qs) => Some(format!("{qs}&{added_query}")),
            None => Some(added_query),
        };
        self
    }
}

impl Display for Hurl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let scheme = if self.is_secure { "https" } else { "http" };
        output.push_str(scheme);
        output.push_str("://");
        output.push_str(&self.host().to_string());
        output.push_str(&self.path_str());
        if let Some(qs) = self.query() {
            output.push('?');
            output.push_str(&qs);
        }
        write!(f, "{output}")
    }
}

impl From<Hurl> for Url {
    fn from(value: Hurl) -> Self {
        value
            .to_string()
            .parse()
            .unwrap_or_else(|err| panic!("Should be impossible to have an error parsing Hurl: {value}. Got error {err:#?}"))
    }
}

fn percent_encode(input: impl AsRef<str>) -> String {
    let mut url = Url::parse("http://example.com").expect("Can parse valid url");
    url.set_path(input.as_ref());
    url.path().trim_start_matches("/").to_string()
}
