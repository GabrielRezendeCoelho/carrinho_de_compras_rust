use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::config::app_config::AppConfig;
use crate::utils::jwt;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    "Token de autenticação não fornecido".to_string(),
                ));
            }
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    "Formato de token inválido. Use: Bearer <token>".to_string(),
                ));
            }
        };

        let config = AppConfig::carregar();

        match jwt::validar_token(token, &config.jwt_secret) {
            Ok(claims) => Outcome::Success(AuthenticatedUser {
                user_id: claims.sub,
                email: claims.email,
            }),
            Err(_) => Outcome::Error((
                Status::Unauthorized,
                "Token inválido ou expirado".to_string(),
            )),
        }
    }
}
