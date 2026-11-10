use mime::FromStrError;

pub enum ContentTypeError {
    NotFound,
    ParseError(FromStrError),
}
