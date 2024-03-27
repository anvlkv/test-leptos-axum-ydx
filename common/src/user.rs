use std::collections::HashSet;

use crate::{models, IdType};
use leptos::*;
use serde::{Deserialize, Serialize};

/// Explicitly is not Serialize/Deserialize!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPasshash(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: IdType,
    pub name: String,
    pub family_name: String,
    pub patronym: Option<String>,
    pub username: String,
    pub permissions: HashSet<String>,
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::ctx::auth;

    let auth = auth()?;

    Ok(auth.current_user)
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;

    use axum::async_trait;
    use axum_session_auth::{Authentication, HasPermission};
    use sqlx::PgPool;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    impl User {
        pub async fn get_with_passhash(id: IdType, pool: &PgPool) -> Option<(Self, UserPasshash)> {
            let sqluser = sqlx::query_as::<_, models::User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get(id: IdType, pool: &PgPool) -> Option<Self> {
            User::get_with_passhash(id, pool)
                .await
                .map(|(user, _)| user)
        }

        pub async fn get_from_username_with_passhash(
            name: String,
            pool: &PgPool,
        ) -> Option<(Self, UserPasshash)> {
            let sqluser =
                sqlx::query_as::<_, models::User>("SELECT * FROM users WHERE username = ?")
                    .bind(name)
                    .fetch_one(pool)
                    .await
                    .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(sqluser.id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_username(name: String, pool: &PgPool) -> Option<Self> {
            User::get_from_username_with_passhash(name, pool)
                .await
                .map(|(user, _)| user)
        }
    }

    #[async_trait]
    impl Authentication<User, IdType, PgPool> for User {
        async fn load_user(userid: IdType, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
            let pool = pool.ok_or_else(|| anyhow::anyhow!("No pool"))?;

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            self.permissions.contains(perm)
        }
    }
}
