use std::env;

use dotenvy::dotenv;
use leptos::{get_configuration, leptos_config::ConfFile};
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Config {
    pub leptos: ConfFile,
    pub db: DatabaseConfig,
    pub(crate) default_admin_user: String,
    pub(crate) default_admin_password: String,
    pub(crate) demo_user_password: Option<String>,
    pub(crate) create_fixtures: bool,
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        name: env::var("DB_NAME").expect("DB_NAME must be set"),
    };

    Config {
        leptos: conf,
        db: database_config,
        default_admin_user: env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set"),
        default_admin_password: env::var("ADMIN_PWD").expect("ADMIN_PWD must be set"),
        create_fixtures: env::var("FIXTURES").map(|f| f == "true").unwrap_or(false),
        demo_user_password: env::var("DEMO_PWD").ok(),
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
