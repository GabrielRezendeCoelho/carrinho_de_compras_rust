use crate::config::database::DbConn;
use crate::entities::enums::StatusPedido;
use crate::errors::AppError;
use crate::models::item_pedido::NovoItemPedido;
use crate::models::pedido::NovoPedido;
use crate::repositories::{
    carrinho_repository, item_carrinho_repository, item_pedido_repository, pedido_repository,
    produto_repository,
};
use crate::schemas::pedido_dto::{ItemPedidoResponse, PedidoDetalheResponse, PedidoResumoResponse};

pub fn criar_pedido(conn: &mut DbConn, usuario_id: i32) -> Result<PedidoDetalheResponse, AppError> {

    let carrinho = carrinho_repository::buscar_ou_criar(conn, usuario_id)?;
    let itens_carrinho = item_carrinho_repository::listar_por_carrinho(conn, carrinho.id)?;

    if itens_carrinho.is_empty() {
        return Err(AppError::BadRequest("Carrinho vazio. Adicione itens antes de criar um pedido.".to_string()));
    }

    let mut valor_total = 0.0;
    let mut itens_verificados = Vec::with_capacity(itens_carrinho.len());

    for item in &itens_carrinho {
        let produto = produto_repository::buscar_por_id(conn, item.produto_id)?;

        if produto.estoque < item.quantidade {
            return Err(AppError::BadRequest(format!(
                "Estoque insuficiente para o produto: {}. Disponível: {}, Solicitado: {}",
                produto.nome, produto.estoque, item.quantidade
            )));
        }

        let subtotal = produto.preco * item.quantidade as f64;
        valor_total += subtotal;

        itens_verificados.push((produto, item.quantidade));
    }

    let novo_pedido = NovoPedido {
        usuario_id,
        valor_total,
        status: StatusPedido::Pendente.as_str().to_string(),
    };
    let pedido = pedido_repository::criar(conn, novo_pedido)?;

    let mut itens_response = Vec::with_capacity(itens_verificados.len());

    for (produto, quantidade) in &itens_verificados {

        let novo_item = NovoItemPedido {
            pedido_id: pedido.id,
            produto_id: produto.id,
            quantidade: *quantidade,
            preco_unitario: produto.preco,
        };
        item_pedido_repository::criar(conn, novo_item)?;

        let novo_estoque = produto.estoque - quantidade;
        produto_repository::atualizar_estoque(conn, produto.id, novo_estoque)?;

        itens_response.push(ItemPedidoResponse {
            produto_id: produto.id,
            produto_nome: produto.nome.clone(),
            quantidade: *quantidade,
            preco_unitario: produto.preco,
            subtotal: produto.preco * *quantidade as f64,
        });
    }

    item_carrinho_repository::limpar_carrinho(conn, carrinho.id)?;

    log::info!("Pedido #{} criado com sucesso. Valor total: R${:.2}", pedido.id, valor_total);

    Ok(PedidoDetalheResponse {
        id: pedido.id,
        valor_total: pedido.valor_total,
        status: pedido.status,
        criado_em: pedido.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        itens: itens_response,
    })
}

pub fn listar_pedidos(conn: &mut DbConn, usuario_id: i32) -> Result<Vec<PedidoResumoResponse>, AppError> {
    let pedidos = pedido_repository::listar_por_usuario(conn, usuario_id)?;

    let mut respostas = Vec::with_capacity(pedidos.len());

    for pedido in &pedidos {
        let itens = item_pedido_repository::listar_por_pedido(conn, pedido.id)?;
        let total_itens: i32 = itens.iter().map(|i| i.quantidade).sum();

        respostas.push(PedidoResumoResponse {
            id: pedido.id,
            valor_total: pedido.valor_total,
            status: pedido.status.clone(),
            criado_em: pedido.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            total_itens,
        });
    }

    Ok(respostas)
}

pub fn detalhar_pedido(conn: &mut DbConn, usuario_id: i32, pedido_id: i32) -> Result<PedidoDetalheResponse, AppError> {
    let pedido = pedido_repository::buscar_por_id(conn, pedido_id)?;

    if pedido.usuario_id != usuario_id {
        return Err(AppError::Forbidden("Pedido não pertence ao usuário".to_string()));
    }

    let itens = item_pedido_repository::listar_por_pedido(conn, pedido_id)?;

    let itens_response: Vec<ItemPedidoResponse> = itens
        .iter()
        .map(|item| {

            let produto_nome = produto_repository::buscar_por_id(conn, item.produto_id)
                .map(|p| p.nome)
                .unwrap_or_else(|_| "Produto removido".to_string());

            ItemPedidoResponse {
                produto_id: item.produto_id,
                produto_nome,
                quantidade: item.quantidade,
                preco_unitario: item.preco_unitario,
                subtotal: item.preco_unitario * item.quantidade as f64,
            }
        })
        .collect();

    Ok(PedidoDetalheResponse {
        id: pedido.id,
        valor_total: pedido.valor_total,
        status: pedido.status,
        criado_em: pedido.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        itens: itens_response,
    })
}

pub fn cancelar_pedido(conn: &mut DbConn, usuario_id: i32, pedido_id: i32) -> Result<(), AppError> {
    let pedido = pedido_repository::buscar_por_id(conn, pedido_id)?;

    if pedido.usuario_id != usuario_id {
        return Err(AppError::Forbidden("Pedido não pertence ao usuário".to_string()));
    }

    let status = StatusPedido::from_str(&pedido.status)
        .map_err(|e| AppError::InternalError(e))?;

    if !status.pode_cancelar() {
        return Err(AppError::BadRequest(format!(
            "Pedido com status '{}' não pode ser cancelado",
            pedido.status
        )));
    }

    let itens = item_pedido_repository::listar_por_pedido(conn, pedido_id)?;
    for item in &itens {
        let produto = produto_repository::buscar_por_id(conn, item.produto_id)?;
        let novo_estoque = produto.estoque + item.quantidade;
        produto_repository::atualizar_estoque(conn, produto.id, novo_estoque)?;
    }

    pedido_repository::atualizar_status(conn, pedido_id, StatusPedido::Cancelado.as_str())?;

    log::info!("Pedido #{} cancelado. Estoque devolvido.", pedido_id);

    Ok(())
}
