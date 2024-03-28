use crate::{models, perms::MANAGE_USERS, user::User};
use leptos::*;

#[server(ListUsers, "/api")]
pub async fn list_users() -> Result<Vec<User>, ServerFnError> {
    use crate::ctx::{auth, d_pool, pool};
    use axum_session_auth::HasPermission;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(MANAGE_USERS, &pool.as_ref()).await {
            let pool = d_pool()?;
            let conn = pool.get().await?;

            let current_user_id = user.id.clone();
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
                .map(|(u, p)| u.into_user(Some(p)).0)
                .collect());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для управления другими пользователями".to_string(),
    ))
}
