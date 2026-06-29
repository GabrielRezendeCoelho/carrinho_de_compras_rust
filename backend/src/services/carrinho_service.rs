use validator::Validate;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::item_carrinho::NovoItemCarrinho;
use crate::repositories::{carrinho_repository, item_carrinho_repository, produto_repository};
use crate::schemas::carrinho_dto::{
    AdicionarItemRequest, AtualizarQuantidadeRequest, CarrinhoResponse, ItemCarrinhoResponse,
};

pub fn buscar_carrinho(conn: &mut DbConn, usuario_id: i32) -> Result<CarrinhoResponse, AppError> {
    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;
    let itens = item_carrinho_repository::listar_por_carrinho(conn, carrinho.id)?;

    let mut itens_response = Vec::with_capacity(itens.len());
    let mut total = 0.0;

    for item in &itens {
        let produto = produto_repository::buscar_por_id(conn, item.produto_id)?;
        let subtotal = produto.preco * item.quantidade as f64;
        total += subtotal;

        itens_response.push(ItemCarrinhoResponse {
            id: item.id,
            produto_id: produto.id,
            produto_nome: produto.nome.clone(),
            preco: produto.preco,
            quantidade: item.quantidade,
            subtotal,
        });
    }

    Ok(CarrinhoResponse {
        id: carrinho.id,
        itens: itens_response,
        total,
    })
}

pub fn adicionar_item(
    conn: &mut DbConn,
    usuario_id: i32,
    dto: AdicionarItemRequest,
) -> Result<(), AppError> {

    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    let produto = produto_repository::buscar_por_id(conn, dto.produto_id)?;

    if produto.estoque < dto.quantidade {
        return Err(AppError::BadRequest(format!(
            "Estoque insuficiente para o produto: {}. Disponível: {}",
            produto.nome, produto.estoque
        )));
    }

    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;

    match item_carrinho_repository::buscar_por_produto(conn, carrinho.id, dto.produto_id)? {
        Some(item_existente) => {

            let nova_quantidade = item_existente.quantidade + dto.quantidade;
            if nova_quantidade > produto.estoque {
                return Err(AppError::BadRequest(format!(
                    "Estoque insuficiente para o produto: {}. Disponível: {}",
                    produto.nome, produto.estoque
                )));
            }
            item_carrinho_repository::atualizar_quantidade(conn, item_existente.id, nova_quantidade)?;
        }
        None => {

            let novo = NovoItemCarrinho {
                carrinho_id: carrinho.id,
                produto_id: dto.produto_id,
                quantidade: dto.quantidade,
            };
            item_carrinho_repository::criar(conn, novo)?;
        }
    }

    Ok(())
}

pub fn atualizar_quantidade(
    conn: &mut DbConn,
    usuario_id: i32,
    item_id: i32,
    dto: AtualizarQuantidadeRequest,
) -> Result<(), AppError> {
    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;
    let item = item_carrinho_repository::buscar_por_id(conn, item_id)?;

    if item.carrinho_id != carrinho.id {
        return Err(AppError::Forbidden("Item não pertence ao seu carrinho".to_string()));
    }

    let produto = produto_repository::buscar_por_id(conn, item.produto_id)?;
    if dto.quantidade > produto.estoque {
        return Err(AppError::BadRequest(format!(
            "Estoque insuficiente para o produto: {}. Disponível: {}",
            produto.nome, produto.estoque
        )));
    }

    item_carrinho_repository::atualizar_quantidade(conn, item_id, dto.quantidade)?;
    Ok(())
}

pub fn remover_item(conn: &mut DbConn, usuario_id: i32, item_id: i32) -> Result<(), AppError> {
    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;
    let item = item_carrinho_repository::buscar_por_id(conn, item_id)?;

    if item.carrinho_id != carrinho.id {
        return Err(AppError::Forbidden("Item não pertence ao seu carrinho".to_string()));
    }

    item_carrinho_repository::remover(conn, item_id)?;
    Ok(())
}

pub fn limpar(conn: &mut DbConn, usuario_id: i32) -> Result<(), AppError> {
    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;
    item_carrinho_repository::limpar_carrinho(conn, carrinho.id)?;
    Ok(())
}
