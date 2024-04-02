use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{moneys::Moneys, IdType};

#[derive(Deserialize, Serialize, Clone)]
pub struct Entry {
    pub id: IdType,
    pub address: String,
    pub revenue: Moneys,
    pub date: NaiveDate,
    pub by_user_id: IdType,
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Entry {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        let revenue: sqlx_postgres::types::PgMoney = row.try_get("revenue")?;

        Ok(Self {
            id: row.try_get("id")?,
            address: row.try_get("address")?,
            revenue: revenue.into(),
            date: row.try_get("date")?,
            by_user_id: row.try_get("by_user_id")?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntryWithUser {
    pub id: IdType,
    pub address: String,
    pub revenue: Moneys,
    pub date: NaiveDate,
    pub user: crate::user::User,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;

    impl From<(Entry, crate::models::User)> for EntryWithUser {
        fn from((entry, user): (Entry, crate::models::User)) -> Self {
            Self {
                id: entry.id,
                address: entry.address,
                revenue: entry.revenue.into(),
                date: entry.date,
                user: user.into_user_with_password(None).0,
            }
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::nil(),
            address: Default::default(),
            revenue: Default::default(),
            date: Utc::now().date_naive(),
            by_user_id: Default::default(),
        }
    }
}

pub fn month_range(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    let min_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_default();
    let max_date = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or_default())
        .pred_opt()
        .unwrap_or_default();
    (min_date, max_date)
}
