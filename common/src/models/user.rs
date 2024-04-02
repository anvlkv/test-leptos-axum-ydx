use serde::{Deserialize, Serialize};

use crate::IdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: IdType,
    pub name: String,
    pub family_name: String,
    pub patronym: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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
            sql_user_perms: Option<Vec<String>>,
        ) -> (crate::user::User, UserPasshash) {
            (
                crate::user::User {
                    id: self.id,
                    username: self.username,
                    name: self.name,
                    family_name: self.family_name,
                    patronym: self.patronym,
                    permissions: if let Some(user_perms) = sql_user_perms {
                        user_perms.into_iter().collect::<HashSet<String>>()
                    } else {
                        HashSet::<String>::new()
                    },
                },
                UserPasshash(self.password),
            )
        }
    }
}
