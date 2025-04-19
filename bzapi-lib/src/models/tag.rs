use diesel::prelude::{Associations, Insertable, Queryable, Selectable};

#[derive(Debug, Clone, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    #[diesel(skip_insertion)]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
