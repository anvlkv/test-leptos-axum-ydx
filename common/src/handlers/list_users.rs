use leptos::*;

#[server(ListUsers, "/api", "GetJson")]
pub async fn list_users(managers_only: bool) -> Result<Vec<crate::user::User>, ServerFnError> {
    use std::collections::HashMap;

    use axum_session_auth::HasPermission;

    use crate::ctx::{auth, pool};
    use crate::{
        models,
        perms::{EDIT_OWNED, MANAGE_USERS},
    };

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(MANAGE_USERS, &Some(&pool)).await {
            let current_user_id = user.id;

            let result_users = sqlx::query!(
                r#"
                SELECT users.*, permissions.token
                FROM users
                LEFT JOIN permissions
                ON permissions.user_id = users.id
                WHERE users.id != $1
                "#,
                current_user_id
            )
            .fetch_all(&pool)
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut acc, row| {
                let entry = acc.entry(row.id).or_insert((
                    models::User {
                        id: row.id,
                        name: row.name,
                        family_name: row.family_name,
                        patronym: row.patronym,
                        username: row.username,
                        password: row.password,
                    },
                    vec![],
                ));
                entry.1.push(row.token);

                acc
            })
            .into_iter()
            .filter_map(|(_, e)| {
                if !managers_only || e.1.contains(&EDIT_OWNED.to_string()) {
                    Some(e.0.into_user_with_password(Some(e.1)).0)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

            return Ok(result_users);
        } else {
            log::error!("no permission");
        }
    } else {
        log::error!("no user");
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для управления другими пользователями".to_string(),
    ))
}
