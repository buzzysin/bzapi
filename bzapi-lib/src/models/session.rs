use diesel::prelude::{Associations, Insertable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::user::User;

#[derive(Clone, Default, Queryable, Selectable, Associations, Insertable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Session {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub user_id: String,
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
