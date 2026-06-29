use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub database_url: String,
}

impl AppConfig {

    pub fn carregar() -> Self {
        AppConfig {
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default_secret_change_me".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse::<i64>()
                .unwrap_or(24),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "db.sqlite".to_string()),
        }
    }
}
