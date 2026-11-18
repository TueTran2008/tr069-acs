use std::borrow::Cow;

use crate::model::store::dbx;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::DatabaseError;

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

    DeviceAlreadyExist {
        device_identify: String,
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
    /// This function will transform the error into a more precise variant if it is an SQLX or PGError Unique Violation.
    /// The resolver can contain a function (table_name: &str, constraint: &str) that may return a specific Error if desired.
    /// If the resolver is None, or if the resolver function returns None, it will default to Error::UniqueViolation {table, constraint}.
    pub fn resolve_unique_violation<F>(self, resolver: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>,
    {
        match self
            .as_database_error()
            .map(|db_error| (db_error.code(), db_error.table(), db_error.constraint()))
        {
            // "23505" => postgresql "unique violation"
            Some((Some(Cow::Borrowed("23505")), Some(table), Some(constraint))) => resolver
                .and_then(|fun| fun(table, constraint))
                .unwrap_or_else(|| Error::UniqueViolation {
                    table: table.to_string(),
                    constraint: constraint.to_string(),
                }),
            _ => self,
        }
    }

    /// A convenient function to return the eventual database error (Postgres)
    /// if this Error is an SQLX Error that contains a database error.
    pub fn as_database_error(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Dbx(dbx::Error::Sqlx(sqlx_error)) => sqlx_error.as_database_error(),
            _ => None,
        }
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
