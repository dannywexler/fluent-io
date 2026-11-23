pub enum ContentTypeError {
    NotFound,
    ParseError(mime::FromStrError),
}
