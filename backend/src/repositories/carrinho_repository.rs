use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::carrinho::{Carrinho, NovoCarrinho};
use crate::schema::carrinhos;

pub fn buscar_por_usuario(conn: &mut DbConn, usuario_id: i32) -> Result<Option<Carrinho>, AppError> {
    carrinhos::table
        .filter(carrinhos::usuario_id.eq(usuario_id))
        .first::<Carrinho>(conn)
        .optional()
        .map_err(AppError::from)
}

pub fn criar(conn: &mut DbConn, usuario_id: i32) -> Result<Carrinho, AppError> {
    let novo = NovoCarrinho { usuario_id };

    diesel::insert_into(carrinhos::table)
        .values(&novo)
        .execute(conn)?;

    carrinhos::table
        .filter(carrinhos::usuario_id.eq(usuario_id))
        .first::<Carrinho>(conn)
        .map_err(AppError::from)
}

pub fn buscar_ou_criar(conn: &mut DbConn, usuario_id: i32) -> Result<Carrinho, AppError> {
    match buscar_por_usuario(conn, usuario_id)? {
        Some(carrinho) => Ok(carrinho),
        None => criar(conn, usuario_id),
    }
}
