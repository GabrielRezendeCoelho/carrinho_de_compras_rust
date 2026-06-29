use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket::http::Method;

pub fn configurar_cors() -> CorsOptions {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
}
