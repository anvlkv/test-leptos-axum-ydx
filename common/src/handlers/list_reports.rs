use chrono::NaiveDate;
use leptos::*;

use crate::{models, IdType};

#[server(ListReports, "/api")]
pub async fn list_reports(
    year: i32,
    month: u32,
    owner_id: Option<IdType>,
) -> Result<Vec<models::Entry>, ServerFnError> {
    use crate::schema::entries::dsl as entries_dsl;
    use crate::schema::users::table as users_tabel;
    use crate::{
        ctx::{auth, d_pool, pool},
        models::entry::month_range,
        perms::{VIEW_ALL, VIEW_OWNED},
    };
    use axum_session_auth::HasPermission;
    use diesel::prelude::*;

    let s_pool = pool().ok();
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let user_id = if user.has(VIEW_OWNED, &s_pool.as_ref()).await {
            if let Some(owner_id) = owner_id.as_ref() {
                if *owner_id != user.id {
                    return Err(ServerFnError::ServerError(
                        "Пользователь не авторизован для просмотра отчетов других пользователей"
                            .to_string(),
                    ));
                } else {
                    *owner_id
                }
            } else {
                user.id
            }
        } else if user.has(VIEW_ALL, &s_pool.as_ref()).await {
            match owner_id {
                Some(id) => id,
                None => user.id,
            }
        } else {
            user.id
        };

        let pool = d_pool()?;
        let conn = pool.get().await?;

        let (min_date, max_date) = month_range(year, month);

        let entries_w_users = conn
            .interact(move |conn| {
                entries_dsl::entries
                    // .inner_join(users_tabel)
                    // .select((models::Entry::as_select(), models::User::as_select()))
                    .select(models::Entry::as_select())
                    .filter(entries_dsl::by_user_id.eq(user_id))
                    .filter(entries_dsl::date.ge(min_date))
                    .filter(entries_dsl::date.le(max_date))
                    .order(entries_dsl::date.asc())
                    // .load::<(models::Entry, models::User)>(conn)
                    .load::<models::Entry>(conn)
            })
            .await??;

        // return Ok(vec![]);
        // return Ok(entries_w_users.into_iter().map(|d| d.into()).collect());
        return Ok(entries_w_users);
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для просмотра отчетов".to_string(),
    ))
}
