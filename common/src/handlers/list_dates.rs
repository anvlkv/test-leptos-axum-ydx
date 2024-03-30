use leptos::*;

#[server(ListDates, "/api", "GetJson")]
pub async fn list_dates() -> Result<Vec<(i32, Vec<u32>)>, ServerFnError> {
    use std::collections::BTreeMap;

    use axum_session_auth::HasPermission;
    use chrono::Datelike;
    use diesel::prelude::*;

    use crate::schema::entries::dsl as entries_dsl;
    use crate::{
        ctx::{auth, d_pool, pool},
        models,
        perms::{VIEW_ALL, VIEW_OWNED},
    };

    let s_pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let user_id = user.id;

        let can_view_others = user.has(VIEW_ALL, &s_pool.as_ref()).await;
        let can_view_owned = user.has(VIEW_OWNED, &s_pool.as_ref()).await;

        let pool = d_pool()?;
        let conn = pool.get().await?;

        let entries: Vec<models::Entry> = conn
            .interact(move |conn| {
                let query = entries_dsl::entries
                    .select(models::Entry::as_select())
                    .order(entries_dsl::date.asc());

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

        let dates: Vec<(i32, Vec<u32>)> = entries
            .into_iter()
            .fold(BTreeMap::<i32, Vec<u32>>::new(), |mut acc, entry| {
                let year = entry.date.year();
                let month = entry.date.month();

                let map_entry = acc.entry(year).or_insert(vec![]);

                if !map_entry.contains(&month) {
                    map_entry.push(month)
                }

                acc
            })
            .into_iter()
            .collect();

        return Ok(dates);
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для добавления отчетов".to_string(),
    ))
}
