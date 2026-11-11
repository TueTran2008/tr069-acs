use crate::model::store::dbx;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    // -- ModelManager
    #[from(String, &String, &str)]
    CantCreateModelManagerProvider(String),

    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },

    CountFail,

    // -- DB
    UserAlreadyExists {
        username: String,
    },
    UniqueViolation {
        table: String,
        constraint: String,
    },

    #[from]
    Dbx(dbx::error::Error),

    // -- Externals
    //There are two separate mechanisms here:
    //- derive_more::From handles type conversion.
    //- serde_with::DisplayFromStr handles serialization.
    //Some external error types do not implement Serde's Serialize trait.
    //DisplayFromStr tells Serde to serialize the error using its Display implementation and deserialize it using its FromStr implementation.
    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),

    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

// region:    --- Custom

//impl From<IntoSeaError> for Error {
//    fn from(value: IntoSeaError) -> Self {}
//}
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
