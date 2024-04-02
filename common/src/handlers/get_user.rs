use leptos::*;

#[server(GetUser, "/api", "GetJson")]
pub async fn get_user(id: crate::IdType) -> Result<crate::user::User, ServerFnError> {
    use axum_session_auth::HasPermission;

    use crate::ctx::{auth, pool};
    use crate::{models, perms::MANAGE_USERS};

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_manage_users = user.has(MANAGE_USERS, &Some(&pool)).await;
        if can_manage_users || user.id == id {
            let records = sqlx::query!(
                r#"
                SELECT users.*, permissions.token
                FROM users
                LEFT JOIN permissions
                ON permissions.user_id = users.id
                WHERE users.id = $1
                "#,
                id
            )
            .fetch_all(&pool)
            .await?;

            let perms = records.iter().map(|r| r.token.clone()).collect::<Vec<_>>();

            if let Some(data) = records.first() {
                return Ok(models::User {
                    id: data.id,
                    name: data.name.clone(),
                    family_name: data.family_name.clone(),
                    patronym: data.patronym.clone(),
                    username: data.username.clone(),
                    password: data.password.clone(),
                }
                .into_user_with_password(Some(perms))
                .0);
            } else {
                return Err(ServerFnError::ServerError(
                    "Пользователь не найден".to_string(),
                ));
            }
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для редактирования этого пользователя".to_string(),
    ))
}
