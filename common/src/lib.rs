pub type IdType = uuid::Uuid;

#[cfg(feature = "ssr")]
pub mod ctx;
pub mod handlers;
#[cfg(feature = "ssr")]
pub mod migrations;
pub mod models;
pub mod moneys;
pub mod perms;
#[cfg(feature = "ssr")]
pub mod schema;
pub mod user;

pub use chrono::Datelike;
