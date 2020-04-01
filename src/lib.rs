#![cfg_attr(docsrs, feature(doc_cfg))]

pub use sqlx_core::arguments;
pub use sqlx_core::connection::{Connect, Connection};
pub use sqlx_core::cursor::{self, Cursor};
pub use sqlx_core::database::{self, Database};
pub use sqlx_core::executor::{self, Execute, Executor};
pub use sqlx_core::pool::{self, Pool};
pub use sqlx_core::query::{self, query, Query};
pub use sqlx_core::query_as::{query_as, QueryAs};
pub use sqlx_core::row::{self, FromRow, Row};
pub use sqlx_core::transaction::Transaction;
pub use sqlx_core::value;

#[doc(hidden)]
pub use sqlx_core::describe;

#[doc(inline)]
pub use sqlx_core::types::{self, Type};

#[doc(inline)]
pub use sqlx_core::error::{self, Error, Result};

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub use sqlx_core::mysql::{self, MySql, MySqlConnection, MySqlPool};

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub use sqlx_core::postgres::{self, PgConnection, PgPool, Postgres};

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub use sqlx_core::sqlite::{self, Sqlite, SqliteConnection, SqlitePool};

#[cfg(feature = "mssql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mssql")))]
pub use sqlx_core::mssql::{self, MsSql, MsSqlConnection};

#[cfg(feature = "macros")]
#[doc(hidden)]
pub extern crate sqlx_macros;

#[cfg(feature = "macros")]
pub use sqlx_macros::{FromRow, Type};

#[cfg(feature = "macros")]
mod macros;

// macro support
#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod ty_match;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod result_ext;

/// Types and traits for encoding values for the database.
pub mod encode {
    pub use sqlx_core::encode::{Encode, IsNull};

    #[cfg(feature = "macros")]
    pub use sqlx_macros::Encode;
}

/// Types and traits for decoding values from the database.
pub mod decode {
    pub use sqlx_core::decode::Decode;

    #[cfg(feature = "macros")]
    pub use sqlx_macros::Decode;
}

/// Convenience re-export of common traits.
pub mod prelude {
    pub use super::Connect;
    pub use super::Connection;
    pub use super::Cursor;
    pub use super::Executor;
    pub use super::FromRow;
    pub use super::Row;

    #[cfg(feature = "postgres")]
    pub use super::postgres::PgQueryAs;

    #[cfg(feature = "mysql")]
    pub use super::mysql::MySqlQueryAs;

    #[cfg(feature = "sqlite")]
    pub use super::sqlite::SqliteQueryAs;
}
