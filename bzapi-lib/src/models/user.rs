use diesel::prelude::Identifiable;
use diesel::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// ## User model
///
/// The user is someone who uses my web app.
#[derive(Serialize, Deserialize, ToSchema, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    // Fields:
    pub id: String,
    pub name: String,
    pub email: String,
    // Relations: owns:
    // Relatnions: belongs_to:
    // Timestamps:
}

/// ## NewUser
///
/// The NewUser struct is the information needed to create a new user.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
