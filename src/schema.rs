// @generated automatically by Diesel CLI.

diesel::table! {
    comments (uuid) {
        uuid -> Uuid,
        content -> Text,
        post_id -> Uuid,
        author_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users_profile (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        #[max_length = 16]
        handle -> Varchar,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        private -> Nullable<Bool>,
        bio -> Nullable<Text>,
        profile_image -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        posts_count -> Nullable<Int4>,
        likes_count -> Nullable<Int4>,
        comments_count -> Nullable<Int4>,
        followers_count -> Nullable<Int4>,
        following_count -> Nullable<Int4>,
        postings -> Nullable<Jsonb>,
        comments -> Nullable<Jsonb>,
        verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(users_profile -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
    users_profile,
);
