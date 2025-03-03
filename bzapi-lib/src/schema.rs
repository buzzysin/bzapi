use diesel::allow_tables_to_appear_in_same_query;
use diesel::joinable;
use diesel::table;

table! {
  users (id) {
      id -> Varchar,
      name -> Varchar,
      email -> Varchar,
  }
}

table! {
  providers (id) {
      id -> Varchar,
      name -> Varchar,
      display_name -> Varchar,
      description -> Varchar,
      icon -> Varchar,
  }
}

table! {
  categories (id) {
      id -> Varchar,
      name -> Varchar,
  }
}

table! {
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

table! {
  threads (id) {
      id -> Varchar,
      is_locked -> Bool,
      is_sticky -> Bool,
      created_at -> Varchar,
      updated_at -> Varchar,
      deleted_at -> Nullable<Varchar>,
  }
}

table! {
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

table! {
  tags (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(posts -> users (author_id));
joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    providers,
    categories,
    posts,
    comments,
    tags,
);

