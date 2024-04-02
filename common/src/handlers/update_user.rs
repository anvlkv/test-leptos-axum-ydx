use leptos::*;

#[server(UpdateUser, "/api")]
pub async fn update_user(
    id: crate::IdType,
    username: String,
    password: Option<String>,
    name: String,
    family_name: String,
    patronym: Option<String>,
    // is_admin: Option<String>,
) -> Result<(), ServerFnError> {
    use axum_session_auth::HasPermission;
    use bcrypt::{hash, DEFAULT_COST};

    use crate::ctx::{auth, pool};
    use crate::perms::MANAGE_USERS;

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_manage_users = user.has(MANAGE_USERS, &Some(&pool)).await;
        let is_updating_self = user.id == id;
        if can_manage_users || is_updating_self {
            if let Some(password) = password {
                let pwd = hash(password, DEFAULT_COST)?;

                sqlx::query!(
                    r#"UPDATE users
                    SET password = $1
                    WHERE id = $2"#,
                    pwd,
                    user.id
                )
                .execute(&pool)
                .await?;
            }

            sqlx::query!(
                r#"UPDATE users
                SET name = $1,
                family_name = $2,
                patronym = $3,
                username = $4
                WHERE id = $5"#,
                name,
                family_name,
                patronym,
                username,
                user.id
            )
            .execute(&pool)
            .await?;

            if is_updating_self {
                leptos_axum::redirect("/");
            } else {
                leptos_axum::redirect("/users");
            }

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для редактирования этого пользователя".to_string(),
    ))
}
