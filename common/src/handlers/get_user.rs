use leptos::*;

#[server(GetUser, "/api", "GetJson")]
pub async fn get_user(id: crate::IdType) -> Result<crate::user::User, ServerFnError> {
    use axum_session_auth::HasPermission;
    use diesel::prelude::*;

    use crate::ctx::{auth, d_pool, pool};
    use crate::schema::permissions::dsl as perm_dsl;
    use crate::schema::users::dsl as users_dsl;
    use crate::{models, perms::MANAGE_USERS};

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_manage_users = user.has(MANAGE_USERS, &pool.as_ref()).await;
        if can_manage_users || user.id == id {
            let pool = d_pool()?;
            let conn = pool.get().await?;

            let user_data = conn
                .interact(move |conn| {
                    users_dsl::users
                        .filter(users_dsl::id.eq(id))
                        .select(models::User::as_select())
                        .limit(1)
                        .load::<models::User>(conn)
                        .map(|u| u.first().cloned())
                })
                .await??;

            if let Some(data) = user_data {
                let id = data.id;
                let perms = conn
                    .interact(move |conn| {
                        perm_dsl::permissions
                            .filter(perm_dsl::user_id.eq(id))
                            .select(models::PermissionTokens::as_select())
                            .load::<models::PermissionTokens>(conn)
                    })
                    .await??;

                return Ok(data.into_user_with_password(Some(perms)).0);
            } else {
                return Err(ServerFnError::ServerError(
                    "Пользователь не найден.".to_string(),
                ));
            }
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для редактирования этого пользователя".to_string(),
    ))
}
