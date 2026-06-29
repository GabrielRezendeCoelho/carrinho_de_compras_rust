use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::produtos;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = produtos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Produto {
    pub id: i32,
    pub categoria_id: i32,
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
    pub estoque: i32,
    pub imagem: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = produtos)]
pub struct NovoProduto {
    pub categoria_id: i32,
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
    pub estoque: i32,
    pub imagem: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = produtos)]
pub struct UpdateProduto {
    pub categoria_id: Option<i32>,
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub preco: Option<f64>,
    pub estoque: Option<i32>,
    pub imagem: Option<String>,
}
