use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::structs::client::Cliente;

#[post("/new_client")]
pub async fn post_new_client(client: web::Json<Cliente>) -> impl Responder {
    let c = client.into_inner();
    println!("CLIENTE: {:?}", c.client_name);
    web::Json(c)
}

#[post("/new_credit_transaction")]
pub async fn post_new_credit_transaction() -> impl Responder {
    HttpResponse::Ok().body("NEW CREDIT TRANSACTION!")
}

#[post("/new_debit_transaction")]
pub async fn post_new_debit_transaction() -> impl Responder {
    HttpResponse::Ok().body("NEW DEBIT TRANSACTION!")
}

#[post("/store_balance")]
pub async fn post_store_balance() -> impl Responder {
    HttpResponse::Ok().body("STORE BALANCE!")
}

#[get("/get_client_balance/{id}")]
pub async fn get_client_balance(info: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok().body("CLIENT BALANCE! CLIENT ID: ".to_owned() + &info.to_string())
}