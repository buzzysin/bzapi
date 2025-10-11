use diesel::prelude::{Associations, Insertable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::user::User;

#[derive(Clone, Default, Queryable, Selectable, Associations, Insertable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::verification_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = email))]
pub struct VerificationToken {
    // #[diesel(skip_insertion)]
    pub id: String,
    pub email: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}
