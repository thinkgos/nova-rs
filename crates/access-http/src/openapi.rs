use axum::Router;
use readiness::app_state::AppState;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::misc;
use crate::handlers::passport;

pub fn route() -> impl Into<Router<AppState>> {
    SwaggerUi::new("/swagger/index.html").url("/swagger/doc.json", ApiDoc::openapi())
}

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        license(
            name = "Apache-2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0.txt",
        ),
    ),
    servers(
        (
            url = "/api", 
            description = "Local Develop Server",
        ),
        (
            url = "http://{host}:{port}/api", 
            description = "Remote Server",
            variables (
                ("host" = (default = "kata.thinkgos.cn", enum_values("kata.thinkgos.cn"), description = "Supported urls for API")),
                ("port" = (default = "9527", enum_values("9527", "9999"), description = "Supported ports for API"))
            )
        ),
    ),
    security(("Token" = []),("ApiKey" = [])),
    modifiers(&SecurityAddon),
    nest(
        (path = "/", api = misc::MiscApi),
        (path = "/", api = passport::PassportApi),
    ),
    components(schemas(), responses()),

    // paths(
    //     misc::healthy,
    // ),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert_with(|| utoipa::openapi::ComponentsBuilder::new().build());
        // Authorization: Bearer xxx
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        // Authorization: xxx
        components.add_security_scheme(
            "ApiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}
