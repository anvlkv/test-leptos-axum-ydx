use leptos::*;

#[server(ListDates, "/api", "GetJson")]
pub async fn list_dates(
    by_user_id: Option<crate::IdType>,
) -> Result<Vec<(i32, Vec<u32>)>, ServerFnError> {
    use std::collections::BTreeMap;

    use axum_session_auth::HasPermission;
    use chrono::Datelike;

    use crate::{
        ctx::{auth, pool},
        perms::{VIEW_ALL, VIEW_OWNED},
    };

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let user_id = user.id;

        let can_view_others = user.has(VIEW_ALL, &Some(&pool)).await;
        let can_view_owned = user.has(VIEW_OWNED, &Some(&pool)).await;

        let user_id_filter = if can_view_others {
            by_user_id
        } else if can_view_owned {
            Some(user_id)
        } else {
            return Err(ServerFnError::ServerError(
                "Пользователь не авторизован для просмотра отчетов".to_string(),
            ));
        };

        let entries = sqlx::query!(
            r#"
            SELECT * FROM entries
            WHERE ($1::UUID IS NULL) OR (entries.by_user_id = $1::UUID)
            ORDER BY entries.date ASC
            "#,
            user_id_filter
        )
        .fetch_all(&pool)
        .await?;

        let dates: Vec<(i32, Vec<u32>)> = entries
            .into_iter()
            .fold(BTreeMap::<i32, Vec<u32>>::new(), |mut acc, entry| {
                let year = entry.date.year();
                let month = entry.date.month();

                let map_entry = acc.entry(year).or_default();

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
        "Пользователь не авторизован для просмотра отчетов".to_string(),
    ))
}
