use derive_more::From;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    TxnCantCommitNoOpenTxn,
    CannotBeginTxnWithTxnFalse,
    CannotCommitTxnWithTxnFalse,
    NoTxn,
    // // -- Externals
    #[from]
    //serde_as It tells Serde:Instead of serializing this field directly, convert it into a string using its Display implementation.
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

//impl From<sqlx::Error> for Error {
//    fn from(error: sqlx::Error) -> Self {
//        Self::Sqlx(error)
//    }
//}
// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
