use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {

    pub sub: i32,

    pub email: String,

    pub exp: usize,

    pub iat: usize,
}

pub fn gerar_token(
    user_id: i32,
    email: &str,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, AppError> {
    let agora = Utc::now();
    let expiracao = agora + Duration::hours(expiration_hours);

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: expiracao.timestamp() as usize,
        iat: agora.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        log::error!("Erro ao gerar token JWT: {:?}", e);
        AppError::InternalError("Erro ao gerar token de autenticação".to_string())
    })
}

pub fn validar_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| {
        log::warn!("Token JWT inválido: {:?}", e);
        AppError::Unauthorized("Token inválido ou expirado".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test_secret_key_for_unit_tests";

    #[test]
    fn test_gerar_e_validar_token() {
        let token = gerar_token(1, "teste@email.com", TEST_SECRET, 24).unwrap();
        let claims = validar_token(&token, TEST_SECRET).unwrap();

        assert_eq!(claims.sub, 1);
        assert_eq!(claims.email, "teste@email.com");
    }

    #[test]
    fn test_token_invalido() {
        let resultado = validar_token("token.invalido.aqui", TEST_SECRET);
        assert!(resultado.is_err());
    }

    #[test]
    fn test_token_com_secret_errado() {
        let token = gerar_token(1, "teste@email.com", TEST_SECRET, 24).unwrap();
        let resultado = validar_token(&token, "secret_errado");
        assert!(resultado.is_err());
    }
}
