use loro::{LoroEncodeError, LoroError, LoroTreeError};

use crate::structs::messages::Error;

pub trait Conversor {
    fn to_error(&self) -> Error;
}

impl Conversor for LoroError {
    fn to_error(&self) -> Error {
        Error { status: 409, message: format!("{:?}", self), fatal: false }
    }
}

impl Conversor for LoroEncodeError {
    fn to_error(&self) -> Error {
        Error { status: 409, message: format!("{:?}", self), fatal: false }
    }
}

impl Conversor for LoroTreeError {
    fn to_error(&self) -> Error {
        Error { status: 409, message: format!("{:?}", self), fatal: false }
    }
}