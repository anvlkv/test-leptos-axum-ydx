use leptos::*;

#[server(ListReports, "/api", "GetJson")]
pub async fn list_reports(
    year: i32,
    month: u32,
    owner_id: Option<crate::IdType>,
) -> Result<Vec<crate::models::EntryWithUser>, ServerFnError> {
    use crate::{
        ctx::{auth, pool},
        models::{self, entry::month_range},
        perms::{VIEW_ALL, VIEW_OWNED},
    };
    use axum_session_auth::HasPermission;

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let user_id_filter = if user.has(VIEW_OWNED, &Some(&pool)).await {
            if let Some(owner_id) = owner_id.as_ref() {
                if *owner_id != user.id {
                    return Err(ServerFnError::ServerError(
                        "Пользователь не авторизован для просмотра отчетов других пользователей"
                            .to_string(),
                    ));
                } else {
                    Some(*owner_id)
                }
            } else {
                Some(user.id)
            }
        } else if user.has(VIEW_ALL, &Some(&pool)).await {
            owner_id
        } else {
            Some(user.id)
        };

        let (min_date, max_date) = month_range(year, month);

        let records = sqlx::query!(
            r#"
            SELECT entries.address, entries.revenue, entries.date, entries.by_user_id, entries.id as entry_id, users.*
            FROM entries
            INNER JOIN users ON entries.by_user_id = users.id
            WHERE entries.date >= $1
            AND entries.date <= $2
            AND (($3::UUID IS NULL) OR (users.id = $3::UUID))
            ORDER BY entries.date DESC
            "#,
            min_date,
            max_date,
            user_id_filter
        ).fetch_all(&pool).await?;

        let entries_w_users = records.into_iter().map(|r| {
            (
                models::Entry {
                    id: r.entry_id,
                    address: r.address,
                    revenue: r.revenue.into(),
                    date: r.date,
                    by_user_id: r.by_user_id,
                },
                models::User {
                    id: r.id,
                    name: r.name,
                    family_name: r.family_name,
                    patronym: r.patronym,
                    username: r.username,
                    password: r.password,
                },
            )
        });

        return Ok(entries_w_users.map(|d| d.into()).collect());
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для просмотра отчетов".to_string(),
    ))
}
