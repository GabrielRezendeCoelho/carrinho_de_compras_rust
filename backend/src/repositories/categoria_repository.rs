use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::categoria::{Categoria, NovaCategoria, UpdateCategoria};
use crate::schema::categorias;

pub fn listar(conn: &mut DbConn) -> Result<Vec<Categoria>, AppError> {
    categorias::table
        .order(categorias::nome.asc())
        .load::<Categoria>(conn)
        .map_err(AppError::from)
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<Categoria, AppError> {
    categorias::table
        .find(id)
        .first::<Categoria>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Categoria não encontrada".to_string())
            }
            _ => AppError::from(e),
        })
}

pub fn criar(conn: &mut DbConn, nova: NovaCategoria) -> Result<Categoria, AppError> {
    diesel::insert_into(categorias::table)
        .values(&nova)
        .execute(conn)?;

    categorias::table
        .filter(categorias::nome.eq(&nova.nome))
        .first::<Categoria>(conn)
        .map_err(AppError::from)
}

pub fn atualizar(conn: &mut DbConn, id: i32, dados: UpdateCategoria) -> Result<Categoria, AppError> {
    diesel::update(categorias::table.find(id))
        .set(&dados)
        .execute(conn)?;

    buscar_por_id(conn, id)
}

pub fn remover(conn: &mut DbConn, id: i32) -> Result<usize, AppError> {
    diesel::delete(categorias::table.find(id))
        .execute(conn)
        .map_err(AppError::from)
}
