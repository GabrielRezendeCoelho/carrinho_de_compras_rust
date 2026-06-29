use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::usuario::{NovoUsuario, Usuario};
use crate::schema::usuarios;

pub fn criar(conn: &mut DbConn, novo_usuario: NovoUsuario) -> Result<Usuario, AppError> {
    diesel::insert_into(usuarios::table)
        .values(&novo_usuario)
        .execute(conn)?;

    usuarios::table
        .filter(usuarios::email.eq(&novo_usuario.email))
        .first::<Usuario>(conn)
        .map_err(AppError::from)
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<Usuario, AppError> {
    usuarios::table
        .find(id)
        .first::<Usuario>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Usuário não encontrado".to_string())
            }
            _ => AppError::from(e),
        })
}

pub fn buscar_por_email(conn: &mut DbConn, email: &str) -> Result<Option<Usuario>, AppError> {
    usuarios::table
        .filter(usuarios::email.eq(email))
        .first::<Usuario>(conn)
        .optional()
        .map_err(AppError::from)
}
