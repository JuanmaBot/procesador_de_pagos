use actix_web::{get, web, App, HttpServer, Responder};

use crate::structs::manager;

use super::endpoints;

/// main inicializa el HttpServer con sus servicios correspondientes.
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let manager = manager::Manager::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(manager.clone()))
            .service(web::scope("/api").configure(config))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// index es el servicio que se encarga de mostrar un mensaje de bienvenida.
#[get("/")]
async fn index() -> impl Responder {
    "Bienvenido a Pagos"
}

/// config es la función que se encarga de configurar los servicios del HttpServer con los endpoints de la API.
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoints::post_new_client);
    cfg.service(endpoints::post_new_credit_transaction);
    cfg.service(endpoints::post_new_debit_transaction);
    cfg.service(endpoints::post_store_balance);
    cfg.service(endpoints::get_client_balance);
}
