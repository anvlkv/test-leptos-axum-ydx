use leptos::*;

#[server(ListUsers, "/api", "GetJson")]
pub async fn list_users(managers_only: bool) -> Result<Vec<crate::user::User>, ServerFnError> {
    use axum_session_auth::HasPermission;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

    use crate::ctx::{auth, d_pool, pool};
    use crate::{
        models,
        perms::{EDIT_OWNED, MANAGE_USERS},
    };

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(MANAGE_USERS, &pool.as_ref()).await {
            let pool = d_pool()?;
            let conn = pool.get().await?;

            let current_user_id = user.id;
            let result_users = conn
                .interact(move |conn| {
                    use crate::schema::users::dsl::*;

                    users
                        .filter(id.ne(current_user_id))
                        .select(models::User::as_select())
                        .load::<models::User>(conn)
                })
                .await??;
            let result_perms = conn
                .interact({
                    let result_users = result_users.clone();
                    move |conn| {
                        use crate::schema::permissions::dsl::*;

                        let mut acc = vec![];
                        for u in result_users {
                            let perms = permissions
                                .filter(user_id.eq(u.id))
                                .select(models::PermissionTokens::as_select())
                                .load::<models::PermissionTokens>(conn)?;

                            acc.push(perms);
                        }

                        Result::<Vec<Vec<models::PermissionTokens>>, ServerFnError>::Ok(acc)
                    }
                })
                .await??;

            return Ok(result_users
                .into_iter()
                .zip(result_perms)
                .map(|(u, p)| u.into_user_with_password(Some(p)).0)
                .filter(|u| !managers_only || u.permissions.contains(EDIT_OWNED))
                .collect());
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
