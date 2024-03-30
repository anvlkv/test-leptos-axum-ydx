use chrono::{Datelike, NaiveDate, Utc};
use diesel::data_types::Cents;
use leptos::*;

use crate::{models, IdType};

#[server(UpdateReport, "/api")]
pub async fn update_report(
    id: IdType,
    revenue: i64,
    address: String,
    date: String,
) -> Result<(), ServerFnError> {
    use crate::{
        ctx::{auth, d_pool, pool},
        perms::EDIT_OWNED,
    };
    use axum_session_auth::HasPermission;
    use diesel::{update, ExpressionMethods, RunQueryDsl};

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(EDIT_OWNED, &pool.as_ref()).await {
            use crate::schema::entries::dsl as entries_dsl;

            let pool = d_pool()?;
            let conn = pool.get().await?;

            let form_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;

            let now = Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

            if form_date < min_date || form_date > now {
                return Err(ServerFnError::Request(
                    "Дата за пределами допустимой".to_string(),
                ));
            }

            let user_id = user.id;

            let entry = conn
                .interact(move |conn| {
                    update(entries_dsl::entries)
                        .filter(entries_dsl::by_user_id.eq(user_id))
                        .filter(entries_dsl::id.eq(id))
                        .filter(entries_dsl::date.ge(min_date))
                        .set((
                            entries_dsl::revenue.eq(Cents(revenue)),
                            entries_dsl::address.eq(address),
                        ))
                        .get_result::<models::Entry>(conn)
                })
                .await??;

            leptos_axum::redirect(format!("/reports/{}", entry.id).as_str());

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для добавления отчетов".to_string(),
    ))
}
