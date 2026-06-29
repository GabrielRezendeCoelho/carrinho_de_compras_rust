CREATE TABLE produtos (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    categoria_id INTEGER NOT NULL REFERENCES categorias(id),
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL DEFAULT '',
    preco REAL NOT NULL CHECK(preco >= 0),
    estoque INTEGER NOT NULL DEFAULT 0 CHECK(estoque >= 0),
    imagem TEXT
);
