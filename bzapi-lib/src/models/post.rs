use super::User;

use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Queryable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User, foreign_key = author_id))]
pub struct Post {
    // Fields:
    pub id: String,
    pub title: String,
    pub content: String,
    // Relations: owns:

    // Relatnions: belongs_to:
    pub author_id: String,
    // Timestamps:
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
