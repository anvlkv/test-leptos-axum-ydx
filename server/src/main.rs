mod config;
mod handlers;
mod models;
mod routes;
mod schema;

pub mod fileserv;

use app::*;
use axum::body::Body as AxumBody;
use axum::extract::{FromRef, RawQuery, State};
use axum::http::{HeaderMap, Request};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_session::{
    DatabasePool, Session, SessionConfig, SessionLayer, SessionPgPool, SessionStore,
};
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication, HasPermission};
use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};

use crate::config::config;
use crate::routes::api_router;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone, FromRef)]
pub struct AppState {
    d_pool: Pool,
    s_pool: PgPool,
    leptos_options: LeptosOptions,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let config = config().await;

    let manager = Manager::new(config.db.url.clone(), deadpool_diesel::Runtime::Tokio1);
    let d_pool = Pool::builder(manager).build().unwrap();

    {
        run_migrations(&d_pool).await;
    }

    let s_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db.url.as_str())
        .await?;

    let leptos_options = config.leptos.leptos_options.clone();

    let state = AppState {
        d_pool,
        s_pool,
        leptos_options,
    };

    let addr = state.leptos_options.site_addr;
    let routes = generate_route_list(App);
    let api_routes = api_router();

    let session_config = SessionConfig::default().with_table_name("sessioons");

    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(state.s_pool.clone().into()), session_config)
            .await?;

    let app = Router::new()
        .nest("/api", api_routes)
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(state)
        .layer(SessionLayer::new(session_store))
        .layer(
            AuthSessionLayer::<models::User, i64, SessionPgPool, PgPool>::new(Some(s_pool))
                .with_config(auth_config),
        );

    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    raw_query: RawQuery,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.d_pool.clone());
        },
        App,
    );

    handler(req).await.into_response()
}
