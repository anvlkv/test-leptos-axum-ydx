// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Int4,
        address -> Varchar,
        revenue -> Int8,
        date -> Date,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 250]
        name -> Varchar,
        #[max_length = 250]
        family_name -> Varchar,
        #[max_length = 250]
        patronym -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    users,
);
