use chrono::NaiveDate;
use diesel::data_types::Cents;
#[cfg(feature="ssr")]
use diesel::prelude::*;

use crate::IdType;

#[cfg_attr(feature = "ssr", derive(Queryable, Selectable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::entries))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Entry {
    pub id: IdType,
    pub address: String,
    pub revenue: Cents,
    pub date: NaiveDate,
    pub by_user_id: IdType,
}
