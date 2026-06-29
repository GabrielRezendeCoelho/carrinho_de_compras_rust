use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::DbPool;
use crate::errors::AppError;
use crate::middlewares::auth_guard::AuthenticatedUser;
use crate::schemas::pedido_dto::{PedidoDetalheResponse, PedidoResumoResponse};
use crate::services::pedido_service;

#[post("/")]
pub fn criar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
) -> Result<(Status, Json<PedidoDetalheResponse>), AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let pedido = pedido_service::criar_pedido(&mut conn, user.user_id)?;
    Ok((Status::Created, Json(pedido)))
}

#[get("/")]
pub fn listar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<PedidoResumoResponse>>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let pedidos = pedido_service::listar_pedidos(&mut conn, user.user_id)?;
    Ok(Json(pedidos))
}

#[get("/<id>")]
pub fn detalhar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
    id: i32,
) -> Result<Json<PedidoDetalheResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let pedido = pedido_service::detalhar_pedido(&mut conn, user.user_id, id)?;
    Ok(Json(pedido))
}

#[put("/<id>/cancelar")]
pub fn cancelar(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
    id: i32,
) -> Result<Json<crate::utils::response::MensagemResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    pedido_service::cancelar_pedido(&mut conn, user.user_id, id)?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Pedido cancelado com sucesso".to_string(),
    }))
}
