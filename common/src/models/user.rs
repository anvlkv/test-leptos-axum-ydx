#[cfg(feature = "ssr")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::IdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, sqlx::FromRow)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct User {
    pub id: IdType,
    pub name: String,
    pub family_name: String,
    pub patronym: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, sqlx::FromRow, Identifiable, Associations)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::permissions))]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::schema::users::table, foreign_key = user_id)))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct PermissionTokens {
    pub id: IdType,
    pub token: String,
    pub user_id: IdType,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::collections::HashSet;

    use super::*;

    use crate::user::UserPasshash;

    impl User {
        pub fn into_user_with_password(
            self,
            sql_user_perms: Option<Vec<PermissionTokens>>,
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
}
