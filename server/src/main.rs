mod config;

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
use deadpool_diesel::postgres::{Manager, Pool};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::config;

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

    let session_config = SessionConfig::default().with_table_name("sessioons");

    let auth_config = AuthConfig::<IdType>::default().with_anonymous_user_id(Some(1));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(state.s_pool.clone().into()), session_config)
            .await?;

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler)
                .post(server_fn_handler)
                .patch(server_fn_handler)
                .delete(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(
            AuthSessionLayer::<user::User, IdType, SessionPgPool, PgPool>::new(Some(
                state.s_pool.clone(),
            ))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(state);

    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    auth_session: AppAuthSession,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.d_pool.clone());
            provide_context(auth_session.clone());
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
            provide_context(app_state.d_pool.clone());
            provide_context(app_state.s_pool.clone());
        },
        request,
    )
    .await
}
