use leptos::*;

#[server(GetReport, "/api", "GetJson")]
pub async fn get_report(id: crate::IdType) -> Result<crate::models::Entry, ServerFnError> {
    use axum_session_auth::HasPermission;
    use sqlx::Postgres;

    use crate::ctx::{auth, pool};
    use crate::{
        models,
        perms::{VIEW_ALL, VIEW_OWNED},
    };

    let pool = pool()?;
    let auth = auth()?;

    if let Some(user) = auth.current_user.as_ref() {
        let can_view_others = user.has(VIEW_ALL, &Some(&pool)).await;
        let can_view_owned = user.has(VIEW_OWNED, &Some(&pool)).await;

        let user_id_filter = if can_view_others {
            None
        } else if can_view_owned {
            Some(user.id)
        } else {
            return Err(ServerFnError::ServerError(
                "Пользователь не авторизован для просмотра отчетов".to_string(),
            ));
        };

        let report = sqlx::query_as::<Postgres, models::Entry>(
            r#"
            SELECT * FROM entries
            WHERE id = $1 AND (($2::UUID IS NULL) OR (by_user_id = $2::UUID))
            "#,
        )
        .bind(id)
        .bind(user_id_filter)
        .fetch_one(&pool)
        .await?;

        return Ok(report);
    }

    Err(ServerFnError::ServerError(
        "Пользователь не авторизован для просмотра данного отчета".to_string(),
    ))
}
