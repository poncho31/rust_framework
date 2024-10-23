diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        date -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(events -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
