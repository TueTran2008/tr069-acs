use derive_more::From;
use serde::Serialize;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(From, Debug)]
pub enum Error {
    // -- ModelManager
    //CantCreateModelManagerProvider(String),
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
