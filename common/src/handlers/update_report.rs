use leptos::*;

#[server(UpdateReport, "/api")]
pub async fn update_report(
    id: crate::IdType,
    revenue: i64,
    address: String,
    date: chrono::NaiveDate,
) -> Result<(), ServerFnError> {
    use axum_session_auth::HasPermission;
    use chrono::{Datelike, NaiveDate, Utc};
    use diesel::data_types::Cents;
    use diesel::{update, ExpressionMethods, RunQueryDsl};

    use crate::schema::entries::dsl as entries_dsl;
    use crate::{
        ctx::{auth, d_pool, pool},
        perms::EDIT_OWNED,
    };

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(EDIT_OWNED, &pool.as_ref()).await {
            let pool = d_pool()?;
            let conn = pool.get().await?;

            let now = Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

            if date < min_date || date > now {
                return Err(ServerFnError::Request(
                    "Дата за пределами допустимой".to_string(),
                ));
            }

            let user_id = user.id;

            _ = conn
                .interact(move |conn| {
                    update(entries_dsl::entries)
                        .filter(entries_dsl::by_user_id.eq(user_id))
                        .filter(entries_dsl::id.eq(id))
                        .filter(entries_dsl::date.ge(min_date))
                        .set((
                            entries_dsl::revenue.eq(Cents(revenue)),
                            entries_dsl::address.eq(address),
                        ))
                        .execute(conn)
                })
                .await??;

            leptos_axum::redirect("/reports");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для добавления отчетов".to_string(),
    ))
}
