use chrono::NaiveDate;
use diesel::data_types::Cents;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entry {
    pub id: i32,
    pub address: String,
    pub revenue: Cents,
    pub date: NaiveDate,
    pub by_user_id: i32,
}
