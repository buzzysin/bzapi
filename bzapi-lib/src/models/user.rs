use diesel::prelude::{Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Default, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
