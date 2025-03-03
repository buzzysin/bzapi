use diesel::prelude::Identifiable;
use diesel::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::categories)]
pub struct Category {
    // Fields:
    pub id: String,
    pub name: String,
    // Timestamps:
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
