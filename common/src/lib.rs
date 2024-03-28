pub type IdType = i32;

#[cfg(feature = "ssr")]
pub mod ctx;
pub mod handlers;
#[cfg(feature = "ssr")]
pub mod migrations;
pub mod models;
pub mod perms;
#[cfg(feature = "ssr")]
pub mod schema;
pub mod user;
