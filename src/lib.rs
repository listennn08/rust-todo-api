use rocket_okapi::{
    openapi,
    openapi_get_routes_spec,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
    settings::OpenApiSettings,
    mount_endpoints_and_merged_docs,
};

#[macro_use] extern crate rocket;

mod routes;
pub mod models;
pub mod schema;
pub mod db;
pub mod utils;
pub mod middleware;

#[openapi]
#[get("/ping")]
fn health_check() -> &'static str {
    "pong"
}

#[launch]
pub fn rocket() -> _ {
    let mut server = rocket::build()
        .attach(routes::todo::stage())
        .attach(routes::auth::stage())
        .mount("/", routes![health_check])
        .mount("/api/doc", make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }));
    
    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        server,
        "/api".to_owned(),
        openapi_settings,
        "/general" => openapi_get_routes_spec![health_check],
        "/auth" => routes::auth::get_routes_and_spec(),
        "/token" => routes::todo::get_routes_and_spec(),
    };

    server
}
