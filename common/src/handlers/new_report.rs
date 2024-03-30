use chrono::Utc;
use diesel::data_types::Cents;
use leptos::*;

use crate::models;

#[server(NewReport, "/api")]
pub async fn new_report(revenue: i64, address: String) -> Result<(), ServerFnError> {
    use crate::{
        ctx::{auth, d_pool, pool},
        perms::EDIT_OWNED,
    };
    use axum_session_auth::HasPermission;
    use diesel::{insert_into, ExpressionMethods, RunQueryDsl};

    let pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(EDIT_OWNED, &pool.as_ref()).await {
            use crate::schema::entries::dsl as entries_dsl;

            let pool = d_pool()?;
            let conn = pool.get().await?;

            let date = Utc::now().date_naive();
            let user_id = user.id;

            let entry = conn
                .interact(move |conn| {
                    insert_into(entries_dsl::entries)
                        .values((
                            entries_dsl::date.eq(date),
                            entries_dsl::revenue.eq(Cents(revenue)),
                            entries_dsl::by_user_id.eq(user_id),
                            entries_dsl::address.eq(address),
                        ))
                        .get_result::<models::Entry>(conn)
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
