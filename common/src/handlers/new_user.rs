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
    use crate::{
        ctx::{auth, d_pool, pool},
        models,
        perms::{EDIT_ALL, EDIT_OWNED, MANAGE_USERS},
    };
    use axum_session_auth::HasPermission;
    use bcrypt::{hash, DEFAULT_COST};
    use diesel::{insert_into, ExpressionMethods, RunQueryDsl};

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(MANAGE_USERS, &pool.as_ref()).await {
            use crate::schema::permissions::dsl as perm_dsl;
            use crate::schema::users::dsl as users_dsl;

            let pool = d_pool()?;
            let conn = pool.get().await?;
            let pwd = hash(password, DEFAULT_COST)?;

            _ = conn
                .interact(move |conn| {
                    let user: models::User = insert_into(users_dsl::users)
                        .values((
                            users_dsl::name.eq(name),
                            users_dsl::family_name.eq(family_name),
                            users_dsl::patronym.eq(patronym),
                            users_dsl::username.eq(username),
                            users_dsl::password.eq(pwd),
                        ))
                        .get_result(conn)?;

                    let permissions = if is_admin.is_some() {
                        vec![MANAGE_USERS, EDIT_ALL]
                    } else {
                        vec![EDIT_OWNED]
                    };

                    _ = insert_into(perm_dsl::permissions)
                        .values(
                            permissions
                                .into_iter()
                                .map(|p| (perm_dsl::user_id.eq(user.id), perm_dsl::token.eq(p)))
                                .collect::<Vec<_>>(),
                        )
                        .execute(conn)?;

                    Result::<(), anyhow::Error>::Ok(())
                })
                .await?;

            leptos_axum::redirect("/users");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для управления другими пользователями".to_string(),
    ))
}
