use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::DbPool;
use crate::errors::AppError;
use crate::middlewares::auth_guard::AuthenticatedUser;
use crate::schemas::categoria_dto::{AtualizarCategoriaRequest, CategoriaResponse, CriarCategoriaRequest};
use crate::services::categoria_service;

#[get("/")]
pub fn listar(pool: &State<DbPool>) -> Result<Json<Vec<CategoriaResponse>>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let categorias = categoria_service::listar(&mut conn)?;
    Ok(Json(categorias))
}

#[get("/<id>")]
pub fn buscar(pool: &State<DbPool>, id: i32) -> Result<Json<CategoriaResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let categoria = categoria_service::buscar_por_id(&mut conn, id)?;
    Ok(Json(categoria))
}

#[post("/", data = "<body>")]
pub fn criar(
    pool: &State<DbPool>,
    _user: AuthenticatedUser,
    body: Json<CriarCategoriaRequest>,
) -> Result<(Status, Json<CategoriaResponse>), AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let categoria = categoria_service::criar(&mut conn, body.into_inner())?;
    Ok((Status::Created, Json(categoria)))
}

#[put("/<id>", data = "<body>")]
pub fn atualizar(
    pool: &State<DbPool>,
    _user: AuthenticatedUser,
    id: i32,
    body: Json<AtualizarCategoriaRequest>,
) -> Result<Json<CategoriaResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let categoria = categoria_service::atualizar(&mut conn, id, body.into_inner())?;
    Ok(Json(categoria))
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

    categoria_service::remover(&mut conn, id)?;
    Ok(Json(crate::utils::response::MensagemResponse {
        mensagem: "Categoria removida com sucesso".to_string(),
    }))
}
