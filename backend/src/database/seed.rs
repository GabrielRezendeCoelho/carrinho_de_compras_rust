use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::models::categoria::NovaCategoria;
use crate::models::produto::NovoProduto;
use crate::models::usuario::NovoUsuario;
use crate::schema::{categorias, produtos, usuarios};
use crate::utils::password;

pub fn executar_seed(conn: &mut SqliteConnection) {
    log::info!("Verificando necessidade de seed...");

    let count: i64 = usuarios::table
        .count()
        .get_result(conn)
        .unwrap_or(0);

    if count > 0 {
        log::info!("Banco já possui dados. Seed ignorado.");
        return;
    }

    log::info!("Executando seed de dados iniciais...");

    let senha_hash = password::hash_senha("admin123")
        .expect("Falha ao gerar hash da senha do admin");

    let admin = NovoUsuario {
        nome: "Administrador".to_string(),
        email: "admin@shop.com".to_string(),
        senha_hash,
    };

    diesel::insert_into(usuarios::table)
        .values(&admin)
        .execute(conn)
        .expect("Falha ao criar usuário admin no seed");

    let senha_hash_teste = password::hash_senha("teste123")
        .expect("Falha ao gerar hash da senha de teste");

    let teste = NovoUsuario {
        nome: "Usuário Teste".to_string(),
        email: "teste@shop.com".to_string(),
        senha_hash: senha_hash_teste,
    };

    diesel::insert_into(usuarios::table)
        .values(&teste)
        .execute(conn)
        .expect("Falha ao criar usuário teste no seed");

    let nomes_categorias = vec![
        "Eletrônicos",
        "Roupas",
        "Livros",
        "Casa e Decoração",
        "Esportes",
    ];

    for nome in &nomes_categorias {
        let nova = NovaCategoria {
            nome: nome.to_string(),
        };
        diesel::insert_into(categorias::table)
            .values(&nova)
            .execute(conn)
            .unwrap_or_else(|_| panic!("Falha ao criar categoria: {}", nome));
    }

    let produtos_seed = vec![

        NovoProduto {
            categoria_id: 1,
            nome: "Notebook Pro 16".to_string(),
            descricao: "Notebook com processador de última geração, 16GB RAM, 512GB SSD".to_string(),
            preco: 4599.90,
            estoque: 15,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 1,
            nome: "Smartphone Galaxy Ultra".to_string(),
            descricao: "Smartphone com câmera de 200MP, tela AMOLED 6.8 polegadas".to_string(),
            preco: 3299.00,
            estoque: 25,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 1,
            nome: "Fone Bluetooth Premium".to_string(),
            descricao: "Fone de ouvido sem fio com cancelamento de ruído ativo".to_string(),
            preco: 499.90,
            estoque: 50,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 1,
            nome: "Tablet 10 polegadas".to_string(),
            descricao: "Tablet com tela IPS de alta resolução, 128GB".to_string(),
            preco: 1899.00,
            estoque: 20,
            imagem: None,
        },

        NovoProduto {
            categoria_id: 2,
            nome: "Camiseta Algodão Premium".to_string(),
            descricao: "Camiseta 100% algodão pima, corte regular".to_string(),
            preco: 89.90,
            estoque: 100,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 2,
            nome: "Calça Jeans Slim".to_string(),
            descricao: "Calça jeans com elastano, lavagem escura".to_string(),
            preco: 159.90,
            estoque: 60,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 2,
            nome: "Jaqueta Corta-Vento".to_string(),
            descricao: "Jaqueta impermeável leve, ideal para atividades ao ar livre".to_string(),
            preco: 249.90,
            estoque: 30,
            imagem: None,
        },

        NovoProduto {
            categoria_id: 3,
            nome: "The Rust Programming Language".to_string(),
            descricao: "O livro oficial da linguagem Rust, 2ª edição".to_string(),
            preco: 129.90,
            estoque: 40,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 3,
            nome: "Clean Code".to_string(),
            descricao: "Código limpo: habilidades práticas do Agile Software".to_string(),
            preco: 99.90,
            estoque: 35,
            imagem: None,
        },

        NovoProduto {
            categoria_id: 4,
            nome: "Luminária LED Moderna".to_string(),
            descricao: "Luminária de mesa com ajuste de intensidade e temperatura de cor".to_string(),
            preco: 189.90,
            estoque: 45,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 4,
            nome: "Conjunto de Canecas Artesanais".to_string(),
            descricao: "Kit com 4 canecas de cerâmica pintadas à mão".to_string(),
            preco: 79.90,
            estoque: 70,
            imagem: None,
        },

        NovoProduto {
            categoria_id: 5,
            nome: "Tênis de Corrida Performance".to_string(),
            descricao: "Tênis com tecnologia de amortecimento, ideal para corridas longas".to_string(),
            preco: 399.90,
            estoque: 40,
            imagem: None,
        },
        NovoProduto {
            categoria_id: 5,
            nome: "Garrafa Térmica 1L".to_string(),
            descricao: "Garrafa de aço inox, mantém temperatura por 12h".to_string(),
            preco: 69.90,
            estoque: 80,
            imagem: None,
        },
    ];

    for produto in &produtos_seed {
        diesel::insert_into(produtos::table)
            .values(produto)
            .execute(conn)
            .unwrap_or_else(|_| panic!("Falha ao criar produto: {}", produto.nome));
    }

    log::info!(
        "Seed concluído: 2 usuários, {} categorias, {} produtos",
        nomes_categorias.len(),
        produtos_seed.len()
    );
}
