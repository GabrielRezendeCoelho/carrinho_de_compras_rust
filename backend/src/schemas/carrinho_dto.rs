use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AdicionarItemRequest {
    pub produto_id: i32,
    #[validate(range(min = 1, message = "Quantidade deve ser pelo menos 1"))]
    pub quantidade: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AtualizarQuantidadeRequest {
    #[validate(range(min = 1, message = "Quantidade deve ser pelo menos 1"))]
    pub quantidade: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ItemCarrinhoResponse {
    pub id: i32,
    pub produto_id: i32,
    pub produto_nome: String,
    pub preco: f64,
    pub quantidade: i32,
    pub subtotal: f64,
}

#[derive(Debug, Serialize)]
pub struct CarrinhoResponse {
    pub id: i32,
    pub itens: Vec<ItemCarrinhoResponse>,
    pub total: f64,
}
