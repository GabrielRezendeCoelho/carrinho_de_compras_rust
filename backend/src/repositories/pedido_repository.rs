use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::pedido::{NovoPedido, Pedido};
use crate::schema::pedidos;

pub fn criar(conn: &mut DbConn, novo: NovoPedido) -> Result<Pedido, AppError> {
    diesel::insert_into(pedidos::table)
        .values(&novo)
        .execute(conn)?;

    pedidos::table
        .order(pedidos::id.desc())
        .first::<Pedido>(conn)
        .map_err(AppError::from)
}

pub fn listar_por_usuario(conn: &mut DbConn, usuario_id: i32) -> Result<Vec<Pedido>, AppError> {
    pedidos::table
        .filter(pedidos::usuario_id.eq(usuario_id))
        .order(pedidos::criado_em.desc())
        .load::<Pedido>(conn)
        .map_err(AppError::from)
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<Pedido, AppError> {
    pedidos::table
        .find(id)
        .first::<Pedido>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Pedido não encontrado".to_string())
            }
            _ => AppError::from(e),
        })
}

pub fn atualizar_status(conn: &mut DbConn, id: i32, novo_status: &str) -> Result<(), AppError> {
    diesel::update(pedidos::table.find(id))
        .set(pedidos::status.eq(novo_status))
        .execute(conn)?;

    Ok(())
}
