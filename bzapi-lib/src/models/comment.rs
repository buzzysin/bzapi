use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::Post;
use crate::models::User;
use crate::models::Thread;

/// ## Comment model
///   
/// The comment is a response to a post.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Thread))]
pub struct Comment {
    // Fields:
    pub id: String,
    pub content: String,
    // Relations: belongs_to:
    pub thread_id: Option<String>,
    pub post_id: String,
    pub user_id: String,
    // Timestamps:
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
