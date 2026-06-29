use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, message = "Nome deve ter pelo menos 2 caracteres"))]
    pub nome: String,
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 6, message = "Senha deve ter pelo menos 6 caracteres"))]
    pub senha: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 1, message = "Senha é obrigatória"))]
    pub senha: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub usuario: UsuarioPublico,
}

#[derive(Debug, Serialize, Clone)]
pub struct UsuarioPublico {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub criado_em: String,
}
