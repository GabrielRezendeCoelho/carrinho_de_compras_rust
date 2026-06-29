use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::item_pedido::{ItemPedido, NovoItemPedido};
use crate::schema::itens_pedido;

pub fn criar(conn: &mut DbConn, novo: NovoItemPedido) -> Result<ItemPedido, AppError> {
    diesel::insert_into(itens_pedido::table)
        .values(&novo)
        .execute(conn)?;

    itens_pedido::table
        .order(itens_pedido::id.desc())
        .first::<ItemPedido>(conn)
        .map_err(AppError::from)
}

pub fn listar_por_pedido(conn: &mut DbConn, pedido_id: i32) -> Result<Vec<ItemPedido>, AppError> {
    itens_pedido::table
        .filter(itens_pedido::pedido_id.eq(pedido_id))
        .load::<ItemPedido>(conn)
        .map_err(AppError::from)
}
