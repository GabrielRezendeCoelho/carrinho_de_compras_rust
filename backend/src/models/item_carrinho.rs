use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::itens_carrinho;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = itens_carrinho)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemCarrinho {
    pub id: i32,
    pub carrinho_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = itens_carrinho)]
pub struct NovoItemCarrinho {
    pub carrinho_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
}
