CREATE TABLE itens_carrinho (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    carrinho_id INTEGER NOT NULL REFERENCES carrinhos(id),
    produto_id INTEGER NOT NULL REFERENCES produtos(id),
    quantidade INTEGER NOT NULL CHECK(quantidade >= 1)
);
