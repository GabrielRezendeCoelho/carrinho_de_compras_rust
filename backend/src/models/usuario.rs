use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::usuarios;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = usuarios)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = usuarios)]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
}
