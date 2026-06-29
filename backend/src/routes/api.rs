use rocket::Route;

use crate::controllers::{
    auth_controller, carrinho_controller, categoria_controller, pedido_controller,
    produto_controller,
};

pub fn auth_routes() -> Vec<Route> {
    routes![
        auth_controller::register,
        auth_controller::login,
        auth_controller::me,
    ]
}

pub fn produto_routes() -> Vec<Route> {
    routes![
        produto_controller::listar,
        produto_controller::buscar,
        produto_controller::criar,
        produto_controller::atualizar,
        produto_controller::remover,
    ]
}

pub fn categoria_routes() -> Vec<Route> {
    routes![
        categoria_controller::listar,
        categoria_controller::buscar,
        categoria_controller::criar,
        categoria_controller::atualizar,
        categoria_controller::remover,
    ]
}

pub fn carrinho_routes() -> Vec<Route> {
    routes![
        carrinho_controller::listar,
        carrinho_controller::adicionar_item,
        carrinho_controller::atualizar_quantidade,
        carrinho_controller::remover_item,
        carrinho_controller::limpar,
    ]
}

pub fn pedido_routes() -> Vec<Route> {
    routes![
        pedido_controller::criar,
        pedido_controller::listar,
        pedido_controller::detalhar,
        pedido_controller::cancelar,
    ]
}
