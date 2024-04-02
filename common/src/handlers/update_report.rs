use leptos::*;

#[server(UpdateReport, "/api")]
pub async fn update_report(
    id: crate::IdType,
    revenue: String,
    address: String,
    date: chrono::NaiveDate,
) -> Result<(), ServerFnError> {
    use std::str::FromStr;

    use axum_session_auth::HasPermission;
    use chrono::{Datelike, NaiveDate, Utc};
    use sqlx_postgres::types::PgMoney;

    use crate::moneys::Moneys;
    use crate::{
        ctx::{auth, pool},
        perms::EDIT_OWNED,
    };

    let revenue = Moneys::from_str(revenue.as_str())?;
    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        if user.has(EDIT_OWNED, &Some(&pool)).await {
            let now = Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

            if date < min_date || date > now {
                return Err(ServerFnError::Request(
                    "Дата за пределами допустимой".to_string(),
                ));
            }

            let user_id = user.id;

            sqlx::query!(
                r#"
                UPDATE entries
                SET revenue = $1,
                address = $2
                WHERE by_user_id = $3 AND
                id = $4 AND
                date >= $5
                "#,
                PgMoney(revenue.0),
                address,
                user_id,
                id,
                min_date
            )
            .execute(&pool)
            .await?;

            leptos_axum::redirect("/reports");

            return Ok(());
        }
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для добавления отчетов".to_string(),
    ))
}
