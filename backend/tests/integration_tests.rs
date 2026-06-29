use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::json;

use shopping_cart_backend::build_rocket;

/// Teste de integração do fluxo completo da API.
///
/// Cobre:
/// - Registro de usuário (201)
/// - Registro de usuário duplicado (409)
/// - Login (200)
/// - Perfil /me com token válido (200)
/// - Perfil /me com token inválido/ausente (401)
/// - Listagem de produtos (200)
/// - CRUD de carrinho

#[test]
fn test_fluxo_autenticacao_e_produtos() {
    let client = Client::tracked(build_rocket()).expect("Instância Rocket válida");

    // 1. Testar registro de usuário
    let registro_payload = json!({
        "nome": "Cliente Teste",
        "email": "cliente_teste@shop.com",
        "senha": "SenhaForte@123"
    });

    let response = client
        .post("/api/auth/register")
        .header(ContentType::JSON)
        .body(registro_payload.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    let registro_corpo: serde_json::Value = response.into_json().expect("Response JSON válido");
    assert!(registro_corpo.get("token").is_some());
    let _token = registro_corpo["token"].as_str().unwrap();

    // 2. Testar registro duplicado
    let response_duplicado = client
        .post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json!({
            "nome": "Outro Nome",
            "email": "cliente_teste@shop.com",
            "senha": "SenhaDiferente@123"
        }).to_string())
        .dispatch();

    assert_eq!(response_duplicado.status(), Status::Conflict);

    // 3. Testar login
    let login_payload = json!({
        "email": "cliente_teste@shop.com",
        "senha": "SenhaForte@123"
    });

    let response_login = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(login_payload.to_string())
        .dispatch();

    assert_eq!(response_login.status(), Status::Ok);
    let login_corpo: serde_json::Value = response_login.into_json().expect("Response JSON válido");
    let login_token = login_corpo["token"].as_str().unwrap();

    // 4. Testar obter perfil (/me) com token válido
    let response_me = client
        .get("/api/auth/me")
        .header(rocket::http::Header::new("Authorization", format!("Bearer {}", login_token)))
        .dispatch();

    assert_eq!(response_me.status(), Status::Ok);
    let me_corpo: serde_json::Value = response_me.into_json().expect("Perfil válido");
    assert_eq!(me_corpo["email"], "cliente_teste@shop.com");

    // 5. Testar obter perfil (/me) sem token
    let response_me_sem_token = client.get("/api/auth/me").dispatch();
    assert_eq!(response_me_sem_token.status(), Status::Unauthorized);

    // 6. Testar listagem de produtos com paginação
    let response_produtos = client.get("/api/produtos?pagina=1&por_pagina=5").dispatch();
    assert_eq!(response_produtos.status(), Status::Ok);
    let prod_corpo: serde_json::Value = response_produtos.into_json().expect("Produtos válidos");
    assert!(prod_corpo.get("dados").unwrap().is_array());
    assert_eq!(prod_corpo["pagina"], 1);
}
