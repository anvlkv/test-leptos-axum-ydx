use chrono::{NaiveDate, Utc};
use diesel::data_types::Cents;
#[cfg(feature = "ssr")]
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::IdType;

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, Associations)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::entries))]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::schema::users::table, foreign_key = by_user_id)))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Entry {
    pub id: IdType,
    pub address: String,
    #[serde(serialize_with = "cents_ser")]
    #[serde(deserialize_with = "cents_de")]
    pub revenue: Cents,
    pub date: NaiveDate,
    pub by_user_id: IdType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntryWithUser {
    pub id: IdType,
    pub address: String,
    #[serde(serialize_with = "cents_ser")]
    #[serde(deserialize_with = "cents_de")]
    pub revenue: Cents,
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
                revenue: entry.revenue,
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
            revenue: Cents(0),
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

fn cents_de<'de, D>(deser: D) -> Result<Cents, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Cents(i64::deserialize(deser)?))
}

fn cents_ser<S>(value: &Cents, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.0.serialize(ser)
}
