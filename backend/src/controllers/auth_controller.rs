use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::DbPool;
use crate::errors::AppError;
use crate::middlewares::auth_guard::AuthenticatedUser;
use crate::schemas::auth_dto::{AuthResponse, LoginRequest, RegisterRequest, UsuarioPublico};
use crate::services::auth_service;

#[post("/register", data = "<body>")]
pub fn register(
    pool: &State<DbPool>,
    body: Json<RegisterRequest>,
) -> Result<(Status, Json<AuthResponse>), AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let response = auth_service::registrar(&mut conn, body.into_inner())?;
    Ok((Status::Created, Json(response)))
}

#[post("/login", data = "<body>")]
pub fn login(
    pool: &State<DbPool>,
    body: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let response = auth_service::login(&mut conn, body.into_inner())?;
    Ok(Json(response))
}

#[get("/me")]
pub fn me(
    pool: &State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Json<UsuarioPublico>, AppError> {
    let mut conn = pool.get().map_err(|e| {
        AppError::InternalError(format!("Erro de conexão: {}", e))
    })?;

    let perfil = auth_service::buscar_perfil(&mut conn, user.user_id)?;
    Ok(Json(perfil))
}
