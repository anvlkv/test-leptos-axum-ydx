use leptos::*;

#[server(NewReport, "/api")]
pub async fn new_report(
    revenue: String,
    address: String,
    date: chrono::NaiveDate,
) -> Result<(), ServerFnError> {
    use std::str::FromStr;

    use axum_session_auth::HasPermission;
    use chrono::{Datelike, NaiveDate, Utc};
    use diesel::data_types::Cents;
    use diesel::{insert_into, ExpressionMethods, RunQueryDsl};

    use crate::moneys::Moneys;
    use crate::schema::entries::dsl as entries_dsl;
    use crate::{
        ctx::{auth, d_pool, pool},
        perms::EDIT_OWNED,
    };

    let revenue = Moneys::from_str(revenue.as_str())?;
    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(EDIT_OWNED, &pool.as_ref()).await {
            let pool = d_pool()?;
            let conn = pool.get().await?;

            let user_id = user.id;

            let now = Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

            if date < min_date || date > now {
                return Err(ServerFnError::Request(
                    "Дата за пределами допустимой".to_string(),
                ));
            }

            _ = conn
                .interact(move |conn| {
                    insert_into(entries_dsl::entries)
                        .values((
                            entries_dsl::date.eq(date),
                            entries_dsl::revenue.eq(Cents(revenue.0)),
                            entries_dsl::by_user_id.eq(user_id),
                            entries_dsl::address.eq(address),
                        ))
                        .execute(conn)
                })
                .await??;

            leptos_axum::redirect("/");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для добавления отчетов".to_string(),
    ))
}
