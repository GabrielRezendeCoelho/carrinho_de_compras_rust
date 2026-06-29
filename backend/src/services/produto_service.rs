use validator::Validate;

use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::produto::{NovoProduto, UpdateProduto};
use crate::repositories::{categoria_repository, produto_repository};
use crate::schemas::produto_dto::{
    AtualizarProdutoRequest, CriarProdutoRequest, ProdutoFiltros, ProdutoResponse,
};
use crate::utils::pagination::{PaginacaoParams, RespostaPaginada};

fn produto_para_response(
    produto: &crate::models::produto::Produto,
    categoria_nome: String,
) -> ProdutoResponse {
    ProdutoResponse {
        id: produto.id,
        categoria_id: produto.categoria_id,
        categoria_nome,
        nome: produto.nome.clone(),
        descricao: produto.descricao.clone(),
        preco: produto.preco,
        estoque: produto.estoque,
        imagem: produto.imagem.clone(),
    }
}

pub fn listar(
    conn: &mut DbConn,
    filtros: ProdutoFiltros,
) -> Result<RespostaPaginada<ProdutoResponse>, AppError> {
    let paginacao = PaginacaoParams {
        pagina: filtros.pagina,
        por_pagina: filtros.por_pagina,
    };

    let limit = paginacao.por_pagina();
    let offset = paginacao.offset();

    let (produtos, total) = match (&filtros.nome, filtros.categoria_id) {
        (Some(nome), _) if !nome.is_empty() => {
            let prods = produto_repository::buscar_por_nome(conn, nome, limit, offset)?;
            let total = produto_repository::contar_por_nome(conn, nome)?;
            (prods, total)
        }
        (_, Some(cat_id)) => {
            let prods = produto_repository::filtrar_por_categoria(conn, cat_id, limit, offset)?;
            let total = produto_repository::contar_por_categoria(conn, cat_id)?;
            (prods, total)
        }
        _ => {
            let prods = produto_repository::listar(conn, limit, offset)?;
            let total = produto_repository::contar(conn)?;
            (prods, total)
        }
    };

    let mut respostas = Vec::with_capacity(produtos.len());
    for produto in &produtos {
        let categoria = categoria_repository::buscar_por_id(conn, produto.categoria_id)?;
        respostas.push(produto_para_response(produto, categoria.nome));
    }

    Ok(RespostaPaginada::new(
        respostas,
        total,
        paginacao.pagina(),
        limit,
    ))
}

pub fn buscar_por_id(conn: &mut DbConn, id: i32) -> Result<ProdutoResponse, AppError> {
    let produto = produto_repository::buscar_por_id(conn, id)?;
    let categoria = categoria_repository::buscar_por_id(conn, produto.categoria_id)?;

    Ok(produto_para_response(&produto, categoria.nome))
}

pub fn criar(conn: &mut DbConn, dto: CriarProdutoRequest) -> Result<ProdutoResponse, AppError> {

    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    if dto.nome.trim().is_empty() {
        return Err(AppError::BadRequest("Nome do produto é obrigatório".to_string()));
    }

    if dto.preco < 0.0 {
        return Err(AppError::BadRequest("Preço não pode ser negativo".to_string()));
    }

    if dto.estoque < 0 {
        return Err(AppError::BadRequest("Estoque não pode ser negativo".to_string()));
    }

    categoria_repository::buscar_por_id(conn, dto.categoria_id)?;

    let novo = NovoProduto {
        categoria_id: dto.categoria_id,
        nome: dto.nome,
        descricao: dto.descricao,
        preco: dto.preco,
        estoque: dto.estoque,
        imagem: dto.imagem,
    };

    let produto = produto_repository::criar(conn, novo)?;
    let categoria = categoria_repository::buscar_por_id(conn, produto.categoria_id)?;

    Ok(produto_para_response(&produto, categoria.nome))
}

pub fn atualizar(
    conn: &mut DbConn,
    id: i32,
    dto: AtualizarProdutoRequest,
) -> Result<ProdutoResponse, AppError> {

    produto_repository::buscar_por_id(conn, id)?;

    if let Some(ref nome) = dto.nome {
        if nome.trim().is_empty() {
            return Err(AppError::BadRequest("Nome do produto é obrigatório".to_string()));
        }
    }

    if let Some(preco) = dto.preco {
        if preco < 0.0 {
            return Err(AppError::BadRequest("Preço não pode ser negativo".to_string()));
        }
    }

    if let Some(estoque) = dto.estoque {
        if estoque < 0 {
            return Err(AppError::BadRequest("Estoque não pode ser negativo".to_string()));
        }
    }

    if let Some(cat_id) = dto.categoria_id {
        categoria_repository::buscar_por_id(conn, cat_id)?;
    }

    let update = UpdateProduto {
        categoria_id: dto.categoria_id,
        nome: dto.nome,
        descricao: dto.descricao,
        preco: dto.preco,
        estoque: dto.estoque,
        imagem: dto.imagem,
    };

    let produto = produto_repository::atualizar(conn, id, update)?;
    let categoria = categoria_repository::buscar_por_id(conn, produto.categoria_id)?;

    Ok(produto_para_response(&produto, categoria.nome))
}

pub fn remover(conn: &mut DbConn, id: i32) -> Result<(), AppError> {
    produto_repository::buscar_por_id(conn, id)?;
    produto_repository::remover(conn, id)?;
    Ok(())
}
