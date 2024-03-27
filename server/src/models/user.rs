use std::collections::HashSet;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, sqlx::FromRow)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub family_name: String,
    pub patronym: Option<String>,
    pub username: String,
    pub password: String,
}

/// Explicitly is not Serialize/Deserialize!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPasshash(String);

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

impl User {
    pub fn into_user(
        self,
        sql_user_perms: Option<Vec<SqlPermissionTokens>>,
    ) -> (crate::user::User, UserPasshash) {
        (
            crate::user::User {
                id: self.id,
                username: self.username,
                name: self.name,
                family_name: self.family_name,
                patronym: self.patronym,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            },
            UserPasshash(self.password),
        )
    }
}
