mod parse_error;

use std::str;
use std::convert::TryFrom;
use parse_error::ParseError;
use super::method::Method;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

impl Request {
    fn get_next_word(string: &str) -> Option<(&str, &str)> {
        for (idx, ch) in string.chars().enumerate() {
            if ch == ' ' || ch == '\r' {
                return Some((&string[..idx], &string[idx + 1..]));
            }
        }

        None
    }
}

/*
the reason why TryFrom takes a slice ref is because
we want to pass in an array, but since arrays need both size and type
we must pass it a slice, but slices can only be passed by reference
 */
impl TryFrom<&[u8]> for Request {
    // type alias ParseError (custom type) to Error for this trait
    type Error = ParseError;

    // buf is a fat pointer (slice ref)
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        // Option<T> ---> Result<T, E> using ok_or()
        // just pass the Error type (in this case ParseError) to ok_or
        // and either Some(T) or Err(e) will be returned, so that `?` operator can be used on it
        let (method, request) = Self::get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (query, request) = Self::get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (protocol, _) = Self::get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        todo!()
    }
}