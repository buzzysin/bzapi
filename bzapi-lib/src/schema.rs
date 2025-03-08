diesel::table! {
  users (id) {
    id -> Varchar,
    name -> Varchar,
    email -> Varchar,
  }
}

diesel::table! {
  providers (id) {
    id -> Varchar,
    name -> Varchar,
    display_name -> Varchar,
    description -> Varchar,
    icon -> Varchar,
  }
}

diesel::table! {
  categories (id) {
    id -> Varchar,
    name -> Varchar,
  }
}

diesel::table! {
  posts (id) {
    id -> Varchar,
    title -> Varchar,
    content -> Varchar,
    author_id -> Varchar,
    created_at -> Varchar,
    updated_at -> Varchar,
    deleted_at -> Nullable<Varchar>,
  }
}

diesel::table! {
  threads (id) {
    id -> Varchar,
    is_locked -> Bool,
    is_sticky -> Bool,
    created_at -> Varchar,
    updated_at -> Varchar,
    deleted_at -> Nullable<Varchar>,
  }
}

diesel::table! {
  comments (id) {
    id -> Varchar,
    content -> Varchar,
    thread_id -> Varchar,
    post_id -> Varchar,
    user_id -> Varchar,
    created_at -> Varchar,
    updated_at -> Varchar,
    deleted_at -> Nullable<Varchar>,
  }
}

diesel::table! {
  tags (id) {
    id -> Varchar,
    name -> Varchar,
    description -> Varchar,
    created_at -> Varchar,
    updated_at -> Varchar,
    deleted_at -> Nullable<Varchar>,
  }
}

// Define the relationships between the tables
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(comments -> threads (thread_id));

// Just add all the tables here
diesel::allow_tables_to_appear_in_same_query!(users, providers, categories, posts, comments, tags,);
