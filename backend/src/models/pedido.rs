use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::pedidos;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = pedidos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Pedido {
    pub id: i32,
    pub usuario_id: i32,
    pub valor_total: f64,
    pub status: String,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = pedidos)]
pub struct NovoPedido {
    pub usuario_id: i32,
    pub valor_total: f64,
    pub status: String,
}
