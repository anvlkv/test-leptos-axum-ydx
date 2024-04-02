mod config;
mod fixture;

pub mod fileserv;

use app::*;
use axum::body::Body as AxumBody;
use axum::extract::{FromRef, Path, State};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionPgPool, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use common::{ctx::AppAuthSession, migrations::run_migrations, user, IdType};
use config::Config;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::trace::{self, TraceLayer};

use crate::config::config;
use crate::fixture::make_fixture;

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: PgPool,
    leptos_options: LeptosOptions,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = config().await;

    let pool = PgPoolOptions::new()
        .max_lifetime(Some(core::time::Duration::new(5, 0)))
        .max_connections(50)
        .connect(config.db.url.as_str())
        .await?;

    initial_setup(&pool, config).await;

    let leptos_options = config.leptos.leptos_options.clone();

    let state = AppState {
        pool,
        leptos_options,
    };

    let addr = state.leptos_options.site_addr;
    let routes = generate_route_list(App);

    let session_config = SessionConfig::default().with_table_name("sessions");

    let auth_config = AuthConfig::<IdType>::default()
        .with_anonymous_user_id(None)
        .with_max_age(chrono::Duration::try_days(2).unwrap())
        .with_session_id("public-stage.a.nvlkv.online");

    let session_store =
        SessionStore::<SessionPgPool>::new(Some(state.pool.clone().into()), session_config).await?;

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(
            AuthSessionLayer::<user::User, IdType, SessionPgPool, PgPool>::new(Some(
                state.pool.clone(),
            ))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(state);

    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn initial_setup(pool: &PgPool, config: &Config) {
    run_migrations(pool).await;

    if config.create_fixtures {
        make_fixture(pool, config).await;
    }
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    auth_session: AppAuthSession,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        App,
    );

    handler(req).await.into_response()
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    auth_session: AppAuthSession,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log::debug!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}
