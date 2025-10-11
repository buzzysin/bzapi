use diesel::prelude::{Associations, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::user::User;

#[derive(Clone, Default, Queryable, Selectable, Associations, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = author_id))]
pub struct Post {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub draft: bool,
    pub author_id: String, // user_id
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
