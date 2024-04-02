use axum_session::SessionPgPool;
use axum_session_auth::AuthSession;
use leptos::*;

use crate::IdType;

pub type AppAuthSession = AuthSession<crate::user::User, IdType, SessionPgPool, sqlx::PgPool>;

pub fn pool() -> Result<sqlx::PgPool, ServerFnError> {
    use_context::<sqlx::PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}

pub fn auth() -> Result<AppAuthSession, ServerFnError> {
    use_context::<AppAuthSession>()
        .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
}
