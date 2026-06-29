#[macro_use]
extern crate rocket;

pub mod config;
pub mod controllers;
pub mod database;
pub mod entities;
pub mod errors;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;
pub mod schemas;
pub mod services;
pub mod utils;

pub fn build_rocket() -> rocket::Rocket<rocket::Build> {
    dotenvy::dotenv().ok();

    let pool = config::database::inicializar_pool();

    {
        let mut conn = pool.get().expect("Falha ao obter conexão para migrations");
        config::database::executar_migrations(&mut conn);
        database::seed::executar_seed(&mut conn);
    }

    let cors_fairing = config::cors::configurar_cors()
        .to_cors()
        .expect("Falha ao configurar CORS");

    rocket::build()
        .manage(pool)
        .attach(cors_fairing)
        .mount("/api/auth", routes::api::auth_routes())
        .mount("/api/produtos", routes::api::produto_routes())
        .mount("/api/categorias", routes::api::categoria_routes())
        .mount("/api/carrinho", routes::api::carrinho_routes())
        .mount("/api/pedidos", routes::api::pedido_routes())
}
