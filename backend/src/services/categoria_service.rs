use validator::Validate;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::categoria::{NovaCategoria, UpdateCategoria};
use crate::repositories::categoria_repository;
use crate::schemas::categoria_dto::{AtualizarCategoriaRequest, CategoriaResponse, CriarCategoriaRequest};

pub fn listar(conn: &mut DbConn) -> Result<Vec<CategoriaResponse>, AppError> {
    let categorias = categoria_repository::listar(conn)?;

    Ok(categorias
        .into_iter()
        .map(|c| CategoriaResponse {
            id: c.id,
            nome: c.nome,
        })
        .collect())
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<CategoriaResponse, AppError> {
    let categoria = categoria_repository::buscar_por_id(conn, id)?;

    Ok(CategoriaResponse {
        id: categoria.id,
        nome: categoria.nome,
    })
}

pub fn criar(conn: &mut DbConn, dto: CriarCategoriaRequest) -> Result<CategoriaResponse, AppError> {
    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    let nova = NovaCategoria { nome: dto.nome };
    let categoria = categoria_repository::criar(conn, nova)?;

    Ok(CategoriaResponse {
        id: categoria.id,
        nome: categoria.nome,
    })
}

pub fn atualizar(
    conn: &mut DbConn,
    id: i32,
    dto: AtualizarCategoriaRequest,
) -> Result<CategoriaResponse, AppError> {
    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    categoria_repository::buscar_por_id(conn, id)?;

    let update = UpdateCategoria {
        nome: Some(dto.nome),
    };
    let categoria = categoria_repository::atualizar(conn, id, update)?;

    Ok(CategoriaResponse {
        id: categoria.id,
        nome: categoria.nome,
    })
}

pub fn remover(conn: &mut DbConn, id: i32) -> Result<(), AppError> {
    categoria_repository::buscar_por_id(conn, id)?;
    categoria_repository::remover(conn, id)?;
    Ok(())
}
