use bson::{de, ser};
use failure_derive::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum KvError {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "Io error")]
    IoError(#[cause] io::Error),
    #[fail(display = "Serilization error")]
    SerilizationError(#[cause] ser::Error),
    #[fail(display = "Deserialization error")]
    DeserializationError(#[cause] de::Error),
}
impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::IoError(err)
    }
}

impl From<ser::Error> for KvError {
    fn from(err: ser::Error) -> KvError {
        KvError::SerilizationError(err)
    }
}

impl From<de::Error> for KvError {
    fn from(err: de::Error) -> KvError {
        KvError::DeserializationError(err)
    }
}
