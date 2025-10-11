// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        provider_id -> Text,
        provider_account_id -> Text,
        user_id -> Text,
        provider_type -> Text,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamp>,
        token_type -> Nullable<Text>,
        scope -> Nullable<Text>,
        id_token -> Nullable<Text>,
        session_state -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Text,
        content -> Text,
        post_id -> Text,
        author_id -> Text,
        parent_id -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Nullable<Text>,
        draft -> Bool,
        author_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        token -> Text,
        expires_at -> Timestamp,
        user_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        email_verified -> Nullable<Timestamp>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    verification_tokens (id) {
        id -> Text,
        email -> Text,
        token -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    comments,
    posts,
    sessions,
    tags,
    users,
    verification_tokens,
);
