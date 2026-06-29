use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::carrinhos;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = carrinhos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Carrinho {
    pub id: i32,
    pub usuario_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = carrinhos)]
pub struct NovoCarrinho {
    pub usuario_id: i32,
}
