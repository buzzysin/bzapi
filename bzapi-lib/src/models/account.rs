use diesel::prelude::{Associations, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::user::User;

#[derive(Clone, Default, Queryable, Selectable, Associations, Serialize,ToSchema)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Account {
    #[diesel(skip_insertion)]
    pub id: String,
    pub provider_id: String,
    pub provider_account_id: String,
    pub user_id: String,
    pub provider_type: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    #[diesel(skip_insertion)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
