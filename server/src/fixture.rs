use chrono::{Days, Utc};

use common::perms::*;
use sqlx::{postgres::types::PgMoney, PgPool};

use crate::config::Config;

pub async fn make_fixture(pool: &PgPool, config: &Config) {
    // add admin user from env
    let admin_username = config.default_admin_user.clone();
    let admin = sqlx::query!(
        r#"
        SELECT * FROM users
        WHERE username = $1
        "#,
        admin_username
    )
    .fetch_all(pool)
    .await
    .unwrap();

    if admin.first().is_none() {
        let admin_username = config.default_admin_user.clone();
        let pwd =
            bcrypt::hash(config.default_admin_password.as_str(), bcrypt::DEFAULT_COST).unwrap();

        let admin = sqlx::query!(
            r#"
            INSERT INTO users(username, password, name, family_name, patronym)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            admin_username,
            pwd.clone(),
            "Админ",
            "По",
            "Умолчанию",
        )
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO permissions(user_id, token)
            VALUES ($1, $2), ($1, $3)
            "#,
            admin.id,
            MANAGE_USERS,
            VIEW_ALL
        )
        .execute(pool)
        .await
        .unwrap();

        log::info!("Added admin user");
    }

    // add fixture records
    let record = sqlx::query!(
        r#"
            SELECT COUNT(*) FROM entries
        "#
    )
    .fetch_one(pool)
    .await
    .unwrap();

    if record.count.map(|c| c < 100).unwrap_or(true) {
        let then = Utc::now()
            .date_naive()
            .checked_sub_days(Days::new(365 * 2))
            .unwrap();

        let pwd = config
            .demo_user_password
            .as_ref()
            .map(|d| bcrypt::hash(d, bcrypt::DEFAULT_COST).unwrap())
            .unwrap_or_default();

        let fixture_user_1 = sqlx::query!(
            r#"
            INSERT INTO users(username, password, name, family_name, patronym)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            "fixture_user_1",
            pwd.clone(),
            "Демо",
            "Пользователь",
            "Один",
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let fixture_user_2 = sqlx::query!(
            r#"
            INSERT INTO users(username, password, name, family_name, patronym)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            "fixture_user_2",
            pwd.clone(),
            "Демо 2",
            "Пользователь",
            "Два",
        )
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO permissions(user_id, token)
            VALUES ($1, $3), ($1, $4), ($2, $3), ($2, $4)
            "#,
            fixture_user_1.id,
            fixture_user_2.id,
            VIEW_OWNED,
            EDIT_OWNED
        )
        .execute(pool)
        .await
        .unwrap();

        for (i, date) in then.iter_days().take(365 * 2).enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO entries(by_user_id, date, address, revenue)
                VALUES ($1, $2, $3, $4), ($5, $6, $7, $8);
                "#,
                fixture_user_1.id,
                date,
                "Демо адрес, 1/21, г.Санкт-Петербург, 197000",
                PgMoney((i as i64 + 1) * 200 + i as i64),
                fixture_user_2.id,
                date,
                "Демо адрес, 2/42, г.Москва, 103274",
                PgMoney((i as i64 + 1) * 275 + i as i64),
            )
            .execute(pool)
            .await
            .unwrap();
        }

        log::info!("created fixtures");
    }
}
