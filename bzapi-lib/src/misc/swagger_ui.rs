use utoipa_swagger_ui::SwaggerUi;

use super::api_docs;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_docs::openapi())
}
