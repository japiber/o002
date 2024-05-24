use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

#[derive(Clone)]
pub enum JsonResponseError {
    NoError,
    ResponseError(Value)
}

impl Debug for JsonResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonResponseError::ResponseError(v) => write!(f, "response error: {v}"),
            JsonResponseError::NoError => write!(f, "no error"),
        }
    }
}

impl Display for JsonResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "response error: {self:?}")
    }
}

impl Error for JsonResponseError {}
