use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CriarCategoriaRequest {
    #[validate(length(min = 1, message = "Nome da categoria é obrigatório"))]
    pub nome: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AtualizarCategoriaRequest {
    #[validate(length(min = 1, message = "Nome da categoria é obrigatório"))]
    pub nome: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoriaResponse {
    pub id: i32,
    pub nome: String,
}
