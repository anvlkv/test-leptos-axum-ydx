use leptos::*;

#[server(NewUser, "/api")]
pub async fn new_user(
    username: String,
    password: String,
    name: String,
    family_name: String,
    patronym: Option<String>,
    is_admin: Option<String>,
) -> Result<(), ServerFnError> {
    use axum_session_auth::HasPermission;
    use bcrypt::{hash, DEFAULT_COST};

    use crate::{
        ctx::{auth, pool},
        perms::{EDIT_OWNED, MANAGE_USERS, VIEW_ALL, VIEW_OWNED},
    };

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(MANAGE_USERS, &Some(&pool)).await {
            let pwd = hash(password, DEFAULT_COST)?;

            let user_id = sqlx::query!(
                r#"
                INSERT INTO users (name, family_name, patronym, username, password)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id
                "#,
                name,
                family_name,
                patronym,
                username,
                pwd,
            )
            .fetch_one(&pool)
            .await?
            .id;

            let permissions = if is_admin.is_some() {
                [MANAGE_USERS, VIEW_ALL]
            } else {
                [EDIT_OWNED, VIEW_OWNED]
            };

            sqlx::query!(
                r#"
                INSERT INTO permissions (user_id, token)
                VALUES ($1, $2), ($1, $3)
                "#,
                user_id,
                permissions[0],
                permissions[1],
            )
            .execute(&pool)
            .await?;

            leptos_axum::redirect("/users");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для управления другими пользователями".to_string(),
    ))
}
