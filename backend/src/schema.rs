
diesel::table! {
    usuarios (id) {
        id -> Integer,
        nome -> Text,
        email -> Text,
        senha_hash -> Text,
        criado_em -> Timestamp,
    }
}

diesel::table! {
    categorias (id) {
        id -> Integer,
        nome -> Text,
    }
}

diesel::table! {
    produtos (id) {
        id -> Integer,
        categoria_id -> Integer,
        nome -> Text,
        descricao -> Text,
        preco -> Double,
        estoque -> Integer,
        imagem -> Nullable<Text>,
    }
}

diesel::table! {
    carrinhos (id) {
        id -> Integer,
        usuario_id -> Integer,
    }
}

diesel::table! {
    itens_carrinho (id) {
        id -> Integer,
        carrinho_id -> Integer,
        produto_id -> Integer,
        quantidade -> Integer,
    }
}

diesel::table! {
    pedidos (id) {
        id -> Integer,
        usuario_id -> Integer,
        valor_total -> Double,
        status -> Text,
        criado_em -> Timestamp,
    }
}

diesel::table! {
    itens_pedido (id) {
        id -> Integer,
        pedido_id -> Integer,
        produto_id -> Integer,
        quantidade -> Integer,
        preco_unitario -> Double,
    }
}

diesel::joinable!(produtos -> categorias (categoria_id));
diesel::joinable!(carrinhos -> usuarios (usuario_id));
diesel::joinable!(itens_carrinho -> carrinhos (carrinho_id));
diesel::joinable!(itens_carrinho -> produtos (produto_id));
diesel::joinable!(pedidos -> usuarios (usuario_id));
diesel::joinable!(itens_pedido -> pedidos (pedido_id));
diesel::joinable!(itens_pedido -> produtos (produto_id));

diesel::allow_tables_to_appear_in_same_query!(
    usuarios,
    categorias,
    produtos,
    carrinhos,
    itens_carrinho,
    pedidos,
    itens_pedido,
);
