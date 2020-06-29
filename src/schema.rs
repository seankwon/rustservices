table! {
    sessions (id) {
        id -> Varchar,
        username -> Varchar,
        secret -> Varchar,
        created_at -> Timestamp,
        user_id -> Nullable<Varchar>,
        token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
