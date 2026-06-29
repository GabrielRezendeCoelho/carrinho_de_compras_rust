use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::DbPool;
use crate::errors::AppError;
use crate::middlewares::auth_guard::AuthenticatedUser;
use crate::schemas::carrinho_dto::{
    AdicionarItemRequest, AtualizarQuantidadeRequest, CarrinhoResponse,
};
use crate::services::carrinho_service;

#[get("/")]
pub fn listar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Json<CarrinhoResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let carrinho = carrinho_service::buscar_carrinho(&mut conn, user.user_id)?;
    Ok(Json(carrinho))
}

#[post("/itens", data = "<body>")]
pub fn adicionar_item(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
    body: Json<AdicionarItemRequest>,
) -> Result<(Status, Json<crate::utils::response::MensagemResponse>), AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    carrinho_service::adicionar_item(&mut conn, user.user_id, body.into_inner())?;
    Ok((
        Status::Created,
        Json(crate::utils::response::MensagemResponse {
            mensagem: "Item adicionado ao carrinho".to_string(),
        }),
    ))
}

#[put("/itens/<item_id>", data = "<body>")]
pub fn atualizar_quantidade(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
    item_id: i32,
    body: Json<AtualizarQuantidadeRequest>,
) -> Result<Json<crate::utils::response::MensagemResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    carrinho_service::atualizar_quantidade(&mut conn, user.user_id, item_id, body.into_inner())?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Quantidade atualizada".to_string(),
    }))
}

#[delete("/itens/<item_id>")]
pub fn remover_item(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
    item_id: i32,
) -> Result<Json<crate::utils::response::MensagemResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    carrinho_service::remover_item(&mut conn, user.user_id, item_id)?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Item removido do carrinho".to_string(),
    }))
}

#[delete("/")]
pub fn limpar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Json<crate::utils::response::MensagemResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    carrinho_service::limpar(&mut conn, user.user_id)?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Carrinho limpo com sucesso".to_string(),
    }))
}
