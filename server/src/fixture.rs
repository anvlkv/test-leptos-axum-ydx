use chrono::{Days, Utc};
use common::schema::{
    entries::dsl as entries_dsl, permissions::dsl as perm_dsl, users::dsl as users_dsl,
};
use common::{models, perms::*};
use deadpool_diesel::postgres::Pool;
use diesel::data_types::Cents;
use diesel::{insert_into, prelude::*};

use crate::config::Config;

pub async fn make_fixture(pool: &Pool, config: &Config) {
    let conn = pool.get().await.unwrap();
    let count = conn
        .interact(|conn| entries_dsl::entries.count().get_result::<i64>(conn))
        .await
        .unwrap()
        .unwrap();

    if count < 100 {
        let then = Utc::now()
            .date_naive()
            .checked_sub_days(Days::new(365 * 2))
            .unwrap();

        let pwd = config
            .demo_user_password
            .as_ref()
            .map(|d| bcrypt::hash(d, bcrypt::DEFAULT_COST).unwrap())
            .unwrap_or_default();

        conn.interact(move |conn| {
            let fixture_user_1 = insert_into(users_dsl::users)
                .values((
                    users_dsl::username.eq("fixture_user_1"),
                    users_dsl::password.eq(pwd.clone()),
                    users_dsl::name.eq("Демо"),
                    users_dsl::family_name.eq("Пользователь"),
                    users_dsl::patronym.eq("Один"),
                ))
                .get_result::<models::User>(conn)
                .unwrap();
            _ = insert_into(perm_dsl::permissions)
                .values(vec![
                    (
                        perm_dsl::user_id.eq(fixture_user_1.id),
                        perm_dsl::token.eq(VIEW_OWNED),
                    ),
                    (
                        perm_dsl::user_id.eq(fixture_user_1.id),
                        perm_dsl::token.eq(EDIT_OWNED),
                    ),
                ])
                .execute(conn)
                .unwrap();

            let fixture_user_2 = insert_into(users_dsl::users)
                .values((
                    users_dsl::username.eq("fixture_user_2"),
                    users_dsl::password.eq(pwd.clone()),
                    users_dsl::name.eq("Демо 2"),
                    users_dsl::family_name.eq("Пользователь"),
                    users_dsl::patronym.eq("Два"),
                ))
                .get_result::<models::User>(conn)
                .unwrap();
            _ = insert_into(perm_dsl::permissions)
                .values(vec![
                    (
                        perm_dsl::user_id.eq(fixture_user_2.id),
                        perm_dsl::token.eq(VIEW_OWNED),
                    ),
                    (
                        perm_dsl::user_id.eq(fixture_user_2.id),
                        perm_dsl::token.eq(EDIT_OWNED),
                    ),
                ])
                .execute(conn)
                .unwrap();

            for (i, date) in then.iter_days().take(365 * 2).enumerate() {
                insert_into(entries_dsl::entries)
                    .values(vec![
                        (
                            entries_dsl::by_user_id.eq(fixture_user_1.id),
                            entries_dsl::date.eq(date),
                            entries_dsl::address.eq("Демо адрес, 1/21, г.Санкт-Петербург, 197000"),
                            entries_dsl::revenue.eq(Cents((i as i64 + 1) * 200 + i as i64)),
                        ),
                        (
                            entries_dsl::by_user_id.eq(fixture_user_2.id),
                            entries_dsl::date.eq(date),
                            entries_dsl::address.eq("Демо адрес, 2/42, г.Москва, 103274"),
                            entries_dsl::revenue.eq(Cents((i as i64 + 1) * 275 + i as i64)),
                        ),
                    ])
                    .execute(conn)
                    .unwrap();
            }
        })
        .await
        .unwrap();

        log::info!("created fixtures");
    }
}
