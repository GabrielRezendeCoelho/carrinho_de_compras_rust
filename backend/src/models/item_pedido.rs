use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::itens_pedido;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = itens_pedido)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemPedido {
    pub id: i32,
    pub pedido_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
    pub preco_unitario: f64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = itens_pedido)]
pub struct NovoItemPedido {
    pub pedido_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
    pub preco_unitario: f64,
}
