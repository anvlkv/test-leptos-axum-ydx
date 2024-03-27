use std::collections::HashSet;

use axum::async_trait;
use axum_session_auth::{Authentication, HasPermission};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::{self, SqlPermissionTokens, UserPasshash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub family_name: String,
    pub patronym: Option<String>,
    pub username: String,
    pub permissions: HashSet<String>,
}

impl User {
    pub async fn get_with_passhash(id: i64, pool: &PgPool) -> Option<(Self, UserPasshash)> {
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

    pub async fn get(id: i64, pool: &PgPool) -> Option<Self> {
        User::get_with_passhash(id, pool)
            .await
            .map(|(user, _)| user)
    }
}

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
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
