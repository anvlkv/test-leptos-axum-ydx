use leptos::*;

#[server(GetReport, "/api", "GetJson")]
pub async fn get_report(id: crate::IdType) -> Result<crate::models::Entry, ServerFnError> {
    use axum_session_auth::HasPermission;
    use diesel::prelude::*;

    use crate::ctx::{auth, d_pool, pool};
    use crate::schema::entries::dsl as entries_dsl;
    use crate::{
        models,
        perms::{VIEW_ALL, VIEW_OWNED},
    };

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_view_others = user.has(VIEW_ALL, &pool.as_ref()).await;
        let can_view_owned = user.has(VIEW_OWNED, &pool.as_ref()).await;

        let pool = d_pool()?;
        let conn = pool.get().await?;
        let user_id = user.id;

        let report = conn
            .interact(move |conn| {
                let query = entries_dsl::entries
                    .select(models::Entry::as_select())
                    .limit(1)
                    .filter(entries_dsl::id.eq(id));

                if can_view_others {
                    query.load::<models::Entry>(conn)
                } else if can_view_owned {
                    query
                        .filter(entries_dsl::by_user_id.eq(user_id))
                        .load::<models::Entry>(conn)
                } else {
                    Ok(vec![])
                }
            })
            .await??;

        if let Some(report) = report.first().cloned() {
            return Ok(report);
        } else {
            return Err(ServerFnError::ServerError("Отчет не найден".to_string()));
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для просмотра данного отчета".to_string(),
    ))
}
