use diesel::prelude::*;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::item_carrinho::{ItemCarrinho, NovoItemCarrinho};
use crate::schema::itens_carrinho;

pub fn listar_por_carrinho(conn: &mut DbConn, carrinho_id: i32) -> Result<Vec<ItemCarrinho>, AppError> {
    itens_carrinho::table
        .filter(itens_carrinho::carrinho_id.eq(carrinho_id))
        .load::<ItemCarrinho>(conn)
        .map_err(AppError::from)
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<ItemCarrinho, AppError> {
    itens_carrinho::table
        .find(id)
        .first::<ItemCarrinho>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Item do carrinho não encontrado".to_string())
            }
            _ => AppError::from(e),
        })
}

pub fn buscar_por_produto(
    conn: &mut DbConn,
    carrinho_id: i32,
    produto_id: i32,
) -> Result<Option<ItemCarrinho>, AppError> {
    itens_carrinho::table
        .filter(itens_carrinho::carrinho_id.eq(carrinho_id))
        .filter(itens_carrinho::produto_id.eq(produto_id))
        .first::<ItemCarrinho>(conn)
        .optional()
        .map_err(AppError::from)
}

pub fn criar(conn: &mut DbConn, novo: NovoItemCarrinho) -> Result<ItemCarrinho, AppError> {
    diesel::insert_into(itens_carrinho::table)
        .values(&novo)
        .execute(conn)?;

    itens_carrinho::table
        .order(itens_carrinho::id.desc())
        .first::<ItemCarrinho>(conn)
        .map_err(AppError::from)
}

pub fn atualizar_quantidade(conn: &mut DbConn, id: i32, quantidade: i32) -> Result<(), AppError> {
    diesel::update(itens_carrinho::table.find(id))
        .set(itens_carrinho::quantidade.eq(quantidade))
        .execute(conn)?;

    Ok(())
}

pub fn remover(conn: &mut DbConn, id: i32) -> Result<usize, AppError> {
    diesel::delete(itens_carrinho::table.find(id))
        .execute(conn)
        .map_err(AppError::from)
}

pub fn limpar_carrinho(conn: &mut DbConn, carrinho_id: i32) -> Result<usize, AppError> {
    diesel::delete(itens_carrinho::table.filter(itens_carrinho::carrinho_id.eq(carrinho_id)))
        .execute(conn)
        .map_err(AppError::from)
}
