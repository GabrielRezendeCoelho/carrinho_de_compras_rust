use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CriarProdutoRequest {
    pub categoria_id: i32,
    #[validate(length(min = 1, message = "Nome do produto é obrigatório"))]
    pub nome: String,
    #[serde(default)]
    pub descricao: String,
    #[validate(range(min = 0.0, message = "Preço não pode ser negativo"))]
    pub preco: f64,
    #[validate(range(min = 0, message = "Estoque não pode ser negativo"))]
    pub estoque: i32,
    pub imagem: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AtualizarProdutoRequest {
    pub categoria_id: Option<i32>,
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub preco: Option<f64>,
    pub estoque: Option<i32>,
    pub imagem: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProdutoResponse {
    pub id: i32,
    pub categoria_id: i32,
    pub categoria_nome: String,
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
    pub estoque: i32,
    pub imagem: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProdutoFiltros {
    pub nome: Option<String>,
    pub categoria_id: Option<i32>,
    pub pagina: Option<i64>,
    pub por_pagina: Option<i64>,
}
