// @generated automatically by Diesel CLI.

diesel::table! {
    posts (uuid) {
        uuid -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        author_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        #[max_length = 320]
        email -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
