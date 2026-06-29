use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::DbPool;
use crate::errors::AppError;
use crate::middlewares::auth_guard::AuthenticatedUser;
use crate::schemas::produto_dto::{
    AtualizarProdutoRequest, CriarProdutoRequest, ProdutoFiltros, ProdutoResponse,
};
use crate::services::produto_service;
use crate::utils::pagination::RespostaPaginada;

#[get("/?<pagina>&<por_pagina>&<nome>&<categoria_id>")]
pub fn listar(
    pool: &State<DbPool>,
    pagina: Option<i64>,
    por_pagina: Option<i64>,
    nome: Option<String>,
    categoria_id: Option<i32>,
) -> Result<Json<RespostaPaginada<ProdutoResponse>>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let filtros = ProdutoFiltros {
        nome,
        categoria_id,
        pagina,
        por_pagina,
    };

    let resultado = produto_service::listar(&mut conn, filtros)?;
    Ok(Json(resultado))
}

#[get("/<id>")]
pub fn buscar(pool: &State<DbPool>, id: i32) -> Result<Json<ProdutoResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let produto = produto_service::buscar_por_id(&mut conn, id)?;
    Ok(Json(produto))
}

#[post("/", data = "<body>")]
pub fn criar(
    pool: &State<DbPool>,
    _user: AuthenticatedUser,
    body: Json<CriarProdutoRequest>,
) -> Result<(Status, Json<ProdutoResponse>), AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let produto = produto_service::criar(&mut conn, body.into_inner())?;
    Ok((Status::Created, Json(produto)))
}

#[put("/<id>", data = "<body>")]
pub fn atualizar(
    pool: &State<DbPool>,
    _user: AuthenticatedUser,
    id: i32,
    body: Json<AtualizarProdutoRequest>,
) -> Result<Json<ProdutoResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let produto = produto_service::atualizar(&mut conn, id, body.into_inner())?;
    Ok(Json(produto))
}

#[delete("/<id>")]
pub fn remover(
    pool: &State<DbPool>,
    _user: AuthenticatedUser,
    id: i32,
) -> Result<Json<crate::utils::response::MensagemResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    produto_service::remover(&mut conn, id)?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Produto removido com sucesso".to_string(),
    }))
}
