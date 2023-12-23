use actix_web::{get, web, App, HttpServer, Responder};

use super::endpoints;

#[get("/")]
async fn index() -> impl Responder {
    "Bienvenido a Pagos"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new().service(web::scope("/api").configure(config)).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoints::post_new_client);
    cfg.service(endpoints::post_new_credit_transaction);
    cfg.service(endpoints::post_new_debit_transaction);
    cfg.service(endpoints::post_store_balance);
    cfg.service(endpoints::get_client_balance);
}