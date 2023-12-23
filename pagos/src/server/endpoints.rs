use std::fs::create_dir;
use std::io::Write;

use actix_web::{get, post, web, Responder, HttpResponse};
use crate::structs::client::Cliente;
use crate::structs::manager::Manager;
use crate::structs::trans::Transaction;
use chrono::prelude::*;

#[post("/new_client")]
pub async fn post_new_client(client: web::Json<Cliente>, manager: web::Data<Manager>) -> impl Responder {
    let c = client.into_inner();
    let mut users = manager.users.lock().unwrap();
    let id = users.len();
    users.push((c, 0.0));
    HttpResponse::Ok().body(id.to_string())
}

#[post("/new_credit_transaction")]
pub async fn post_new_credit_transaction(trans: web::Json<Transaction>, manager: web::Data<Manager>) -> impl Responder {
    let t = trans.into_inner();
    let mut users = manager.users.lock().unwrap();
    if users.len() > t.client_id {
        users[t.client_id].1 += t.credit_amount;
        HttpResponse::Ok().body(users[t.client_id].1.to_string())
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }
}

#[post("/new_debit_transaction")]
pub async fn post_new_debit_transaction(trans: web::Json<Transaction>, manager: web::Data<Manager>) -> impl Responder {
    let t = trans.into_inner();
    let mut users = manager.users.lock().unwrap();
    if users.len() > t.client_id {
        if users[t.client_id].1 < t.credit_amount {
            return HttpResponse::Ok().body("ERROR: INSUFFICIENT FUNDS!")
        }
        users[t.client_id].1 -= t.credit_amount;
        HttpResponse::Ok().body(users[t.client_id].1.to_string())
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }}

#[post("/store_balance")]
pub async fn post_store_balance(manager: web::Data<Manager>) -> impl Responder {
    let mut users = manager.users.lock().unwrap();
    let mut gen = manager.generated.lock().unwrap();
    let date = Local::now().format("%d%m%Y").to_string();
    let _ = create_dir("reports");
    let mut file = std::fs::File::create(format!("reports/{date}_{gen}.dat")).unwrap();
    for id in 0..users.len() {
        let _ = file.write(format!("{id} {}\n",users[id].1).as_bytes());
        users[id].1 = 0.0;
    }
    *gen += 1;
    HttpResponse::Ok().body("STORE BALANCE!")
}

#[get("/get_client_balance/{id}")]
pub async fn get_client_balance(id: web::Path<usize>, manager: web::Data<Manager>) -> impl Responder {
    let users = manager.users.lock().unwrap();
    let id = id.into_inner();
    if users.len() > id.clone() {
        HttpResponse::Ok().body(format!("{:?}\n\nBalance:{}",users[id].0, users[id].1))
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }

}