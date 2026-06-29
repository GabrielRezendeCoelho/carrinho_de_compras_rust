use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::produto::{NovoProduto, Produto, UpdateProduto};
use crate::schema::produtos;

pub fn listar(conn: &mut DbConn, limit: i64, offset: i64) -> Result<Vec<Produto>, AppError> {
    produtos::table
        .order(produtos::id.asc())
        .limit(limit)
        .offset(offset)
        .load::<Produto>(conn)
        .map_err(AppError::from)
}

pub fn contar(conn: &mut DbConn) -> Result<i64, AppError> {
    produtos::table
        .count()
        .get_result(conn)
        .map_err(AppError::from)
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<Produto, AppError> {
    produtos::table
        .find(id)
        .first::<Produto>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Produto não encontrado".to_string())
            }
            _ => AppError::from(e),
        })
}

pub fn buscar_por_nome(conn: &mut DbConn, nome: &str, limit: i64, offset: i64) -> Result<Vec<Produto>, AppError> {
    let pattern = format!("%{}%", nome);
    produtos::table
        .filter(produtos::nome.like(&pattern))
        .order(produtos::nome.asc())
        .limit(limit)
        .offset(offset)
        .load::<Produto>(conn)
        .map_err(AppError::from)
}

pub fn contar_por_nome(conn: &mut DbConn, nome: &str) -> Result<i64, AppError> {
    let pattern = format!("%{}%", nome);
    produtos::table
        .filter(produtos::nome.like(&pattern))
        .count()
        .get_result(conn)
        .map_err(AppError::from)
}

pub fn filtrar_por_categoria(
    conn: &mut DbConn,
    categoria_id: i32,
    limit: i64,
    offset: i64,
) -> Result<Vec<Produto>, AppError> {
    produtos::table
        .filter(produtos::categoria_id.eq(categoria_id))
        .order(produtos::nome.asc())
        .limit(limit)
        .offset(offset)
        .load::<Produto>(conn)
        .map_err(AppError::from)
}

pub fn contar_por_categoria(conn: &mut DbConn, categoria_id: i32) -> Result<i64, AppError> {
    produtos::table
        .filter(produtos::categoria_id.eq(categoria_id))
        .count()
        .get_result(conn)
        .map_err(AppError::from)
}

pub fn criar(conn: &mut DbConn, novo: NovoProduto) -> Result<Produto, AppError> {
    diesel::insert_into(produtos::table)
        .values(&novo)
        .execute(conn)?;

    produtos::table
        .order(produtos::id.desc())
        .first::<Produto>(conn)
        .map_err(AppError::from)
}

pub fn atualizar(conn: &mut DbConn, id: i32, dados: UpdateProduto) -> Result<Produto, AppError> {
    diesel::update(produtos::table.find(id))
        .set(&dados)
        .execute(conn)?;

    buscar_por_id(conn, id)
}

pub fn atualizar_estoque(conn: &mut DbConn, id: i32, novo_estoque: i32) -> Result<(), AppError> {
    diesel::update(produtos::table.find(id))
        .set(produtos::estoque.eq(novo_estoque))
        .execute(conn)?;

    Ok(())
}

pub fn remover(conn: &mut DbConn, id: i32) -> Result<usize, AppError> {
    diesel::delete(produtos::table.find(id))
        .execute(conn)
        .map_err(AppError::from)
}
