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
use common::{ctx::AppAuthSession, migrations::run_migrations, models, schema, user, IdType};
use config::Config;
use deadpool_diesel::postgres::{Manager, Pool};
use diesel::{insert_into, prelude::*};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::config;
use crate::fixture::make_fixture;

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

    initial_setup(&d_pool, config).await;

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

    let session_config = SessionConfig::default().with_table_name("sessions");

    let auth_config = AuthConfig::<IdType>::default()
        .with_anonymous_user_id(None)
        .with_max_age(chrono::Duration::try_days(2).unwrap())
        .with_session_id("public-stage.a.nvlkv.online");

    let session_store =
        SessionStore::<SessionPgPool>::new(Some(state.s_pool.clone().into()), session_config)
            .await?;

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
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

async fn initial_setup(pool: &Pool, config: &Config) {
    use common::perms::*;

    run_migrations(pool).await;

    use schema::{permissions::dsl as perm_dsl, users::dsl as users_dsl};

    // add admin user from env
    let conn = pool.get().await.unwrap();
    let admin_username = config.default_admin_user.clone();
    let admin: Vec<models::User> = conn
        .interact(move |conn| {
            let query = users_dsl::users
                .filter(users_dsl::username.eq(admin_username.as_str()))
                .limit(1)
                .select(models::User::as_select());

            query.load::<models::User>(conn).unwrap()
        })
        .await
        .unwrap();

    if admin.first().is_none() {
        let admin_username = config.default_admin_user.clone();
        let pwd =
            bcrypt::hash(config.default_admin_password.as_str(), bcrypt::DEFAULT_COST).unwrap();

        conn.interact(move |conn| {
            let admin = insert_into(users_dsl::users)
                .values((
                    users_dsl::username.eq(admin_username),
                    users_dsl::password.eq(pwd),
                    users_dsl::name.eq("Администратор"),
                    users_dsl::family_name.eq("По умолчанию"),
                ))
                .get_result::<models::User>(conn)
                .unwrap();
            _ = insert_into(perm_dsl::permissions)
                .values(vec![
                    (
                        perm_dsl::user_id.eq(admin.id),
                        perm_dsl::token.eq(MANAGE_USERS),
                    ),
                    (perm_dsl::user_id.eq(admin.id), perm_dsl::token.eq(VIEW_ALL)),
                ])
                .execute(conn)
                .unwrap();
        })
        .await
        .unwrap();

        log::info!("Added admin user");
    }

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
            provide_context(app_state.d_pool.clone());
            provide_context(app_state.s_pool.clone());
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
