use diesel::prelude::Identifiable;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// ## Thread model
///
/// The thread is a collection of comments.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::threads)]
pub struct Thread {
    // Fields:
    pub id: String,
    pub is_locked: bool,
    // Relations: belongs_to:
    // pub author: User,
    // pub forum: Forum,
    // pub parent: Option<Thread>,
    // pub children: Vec<Thread>,
    // Timestamps:
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
