use diesel::Queryable;
use diesel::prelude::Identifiable;
use serde::Deserialize;
use serde::Serialize;

/// ## Provider
///
/// This struct represents an authentication provider.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::providers)]
pub struct Provider {
    // Fields:
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
}
