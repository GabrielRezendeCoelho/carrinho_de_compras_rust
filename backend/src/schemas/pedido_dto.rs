use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ItemPedidoResponse {
    pub produto_id: i32,
    pub produto_nome: String,
    pub quantidade: i32,
    pub preco_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PedidoResumoResponse {
    pub id: i32,
    pub valor_total: f64,
    pub status: String,
    pub criado_em: String,
    pub total_itens: i32,
}

#[derive(Debug, Serialize)]
pub struct PedidoDetalheResponse {
    pub id: i32,
    pub valor_total: f64,
    pub status: String,
    pub criado_em: String,
    pub itens: Vec<ItemPedidoResponse>,
}
