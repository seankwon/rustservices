table! {
    sessions (id) {
        id -> Text,
        username -> Text,
        secret -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        id -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
