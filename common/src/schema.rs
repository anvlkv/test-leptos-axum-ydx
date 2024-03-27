// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Int4,
        address -> Varchar,
        revenue -> Money,
        date -> Date,
        by_user_id -> Int4,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        token -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    sessioons (id) {
        #[max_length = 128]
        id -> Varchar,
        expires -> Nullable<Int4>,
        session -> Text,
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
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(entries -> users (by_user_id));
diesel::joinable!(permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    permissions,
    sessioons,
    users,
);
