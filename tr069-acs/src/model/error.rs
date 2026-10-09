use derive_more::{Error, From};
use serde::Serialize;

//use self::dbx::error::Error;
use crate::model::store::dbx;
pub type Result<T> = core::result::Result<T, Error>;

//#[serde_as]
#[derive(Debug, From)]
pub enum Error {
    // -- ModelManager
    #[from(String, &String, &str)]
    CantCreateModelManagerProvider(String),

    #[from]
    Dbx(dbx::error::Error),
}

// region:    --- Custom

impl Error {
    pub fn custom(val: impl Into<String>) -> Self {
        Self::CantCreateModelManagerProvider(val.into())
    }

    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::CantCreateModelManagerProvider(err.to_string())
    }
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// endregion: --- Error Boilerplate

//It's there so Error behaves like a normal Rust error and can integrate with libraries/functions that accept std::error::Error
impl std::error::Error for Error {}
