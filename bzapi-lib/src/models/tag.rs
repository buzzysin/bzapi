use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, Queryable, Selectable, Insertable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
