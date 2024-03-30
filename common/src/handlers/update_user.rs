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
    use diesel::{update, ExpressionMethods, RunQueryDsl};

    use crate::ctx::{auth, d_pool, pool};
    use crate::perms::MANAGE_USERS;
    use crate::schema::users::dsl as users_dsl;

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_manage_users = user.has(MANAGE_USERS, &pool.as_ref()).await;
        if can_manage_users || user.id == id {
            // use crate::schema::permissions::dsl as perm_dsl;

            let pool = d_pool()?;
            let conn = pool.get().await?;

            if let Some(password) = password {
                let pwd = hash(password, DEFAULT_COST)?;
                _ = conn
                    .interact(move |conn| {
                        update(users_dsl::users)
                            .filter(users_dsl::id.eq(id))
                            .set(users_dsl::password.eq(pwd))
                            .execute(conn)?;

                        Result::<(), ServerFnError>::Ok(())
                    })
                    .await??;
            }

            _ = conn
                .interact(move |conn| {
                    _ = update(users_dsl::users)
                        .filter(users_dsl::id.eq(id))
                        .set((
                            users_dsl::name.eq(name),
                            users_dsl::family_name.eq(family_name),
                            users_dsl::patronym.eq(patronym),
                            users_dsl::username.eq(username),
                        ))
                        .execute(conn)?;

                    // if can_manage_users && id != user.id {
                    //     // FIXME: admin can unadmin other admins...
                    //     let permissions = if is_admin.is_some() {
                    //         vec![MANAGE_USERS, EDIT_ALL]
                    //     } else {
                    //         vec![EDIT_OWNED]
                    //     };

                    //     _ = insert_into(perm_dsl::permissions)
                    //         .values(
                    //             permissions
                    //                 .into_iter()
                    //                 .map(|p| (perm_dsl::user_id.eq(user.id), perm_dsl::token.eq(p)))
                    //                 .collect::<Vec<_>>(),
                    //         )
                    //         .execute(conn)?;
                    // }

                    Result::<(), anyhow::Error>::Ok(())
                })
                .await?;

            leptos_axum::redirect("/users");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для редактирования этого пользователя".to_string(),
    ))
}
