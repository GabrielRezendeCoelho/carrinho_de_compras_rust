use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::categorias;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = categorias)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Categoria {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = categorias)]
pub struct NovaCategoria {
    pub nome: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = categorias)]
pub struct UpdateCategoria {
    pub nome: Option<String>,
}
