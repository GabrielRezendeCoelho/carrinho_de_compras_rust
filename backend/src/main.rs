#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    env_logger::init();
    log::info!("🚀 Iniciando Shopping Cart API...");
    shopping_cart_backend::build_rocket()
}
