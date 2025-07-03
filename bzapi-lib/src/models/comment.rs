use diesel::prelude::{Associations, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::post::Post;
use crate::models::user::User;

#[derive(Clone, Default, Queryable, Selectable, Associations, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Post, foreign_key = author_id))]
#[diesel(belongs_to(User, foreign_key = parent_id))]
pub struct Comment {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub content: String,
    pub post_id: String,
    pub author_id: String, // user_id
    pub parent_id: Option<String>,
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
