use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "BzAPI",
        version = "1.0.0",
        description = "A simple API for managing users and authentication.",
    ),
    components(
        schemas(
            crate::models::user::User,
            crate::models::post::Post,
            crate::models::comment::Comment,
            crate::models::account::Account,
            crate::models::session::Session,
            crate::models::verification_token::VerificationToken,
        ),
    ),
    tags(
        (name = "user", description = "User management"),
        (name = "auth", description = "Authentication"),
    ),
)]
pub struct OpenApiDoc;
