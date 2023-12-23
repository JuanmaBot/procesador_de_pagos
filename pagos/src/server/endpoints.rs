use std::fs::create_dir;
use std::io::Write;

use crate::structs::client::Cliente;
use crate::structs::manager::Manager;
use crate::structs::trans::Transaction;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;

/// post_new_client es el servicio que se encarga de crear un nuevo cliente.
/// # Recibe
/// * client: Un Cliente en formato JSON.
/// * manager: Una referencia a un Manager.
/// # Devuelve
/// * Un HttpResponse con el id del cliente creado.
#[post("/new_client")]
pub async fn post_new_client(
    client: web::Json<Cliente>,
    manager: web::Data<Manager>,
) -> impl Responder {
    let c = client.into_inner();
    let mut users = manager.users.lock().unwrap();
    let id = users.len();
    users.push((c, 0.0));
    HttpResponse::Ok().body(id.to_string())
}

/// post_new_credit_transaction es el servicio que se encarga de hacerle un credito a un cliente.
/// # Recibe
/// * trans: Una Transaction en formato JSON.
/// * manager: Una referencia a un Manager.
/// # Devuelve
/// * Un HttpResponse con el nuevo saldo del cliente o un error si el cliente no existe.
#[post("/new_credit_transaction")]
pub async fn post_new_credit_transaction(
    trans: web::Json<Transaction>,
    manager: web::Data<Manager>,
) -> impl Responder {
    let t = trans.into_inner();
    let mut users = manager.users.lock().unwrap();
    if users.len() > t.client_id {
        users[t.client_id].1 += t.credit_amount;
        HttpResponse::Ok().body(users[t.client_id].1.to_string())
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }
}

/// post_new_debit_transaction es el servicio que se encarga de hacerle un debito a un cliente.
/// # Recibe
/// * trans: Una Transaction en formato JSON.
/// * manager: Una referencia a un Manager.
/// # Devuelve
/// * Un HttpResponse con el nuevo saldo del cliente o un error si el cliente no existe o no tiene suficiente saldo.
#[post("/new_debit_transaction")]
pub async fn post_new_debit_transaction(
    trans: web::Json<Transaction>,
    manager: web::Data<Manager>,
) -> impl Responder {
    let t = trans.into_inner();
    let mut users = manager.users.lock().unwrap();
    if users.len() > t.client_id {
        if users[t.client_id].1 < t.credit_amount {
            return HttpResponse::Ok().body("ERROR: INSUFFICIENT FUNDS!"); // Esta condicion es opcional, se puede quitar y que quede saldo negativo
        }
        users[t.client_id].1 -= t.credit_amount;
        HttpResponse::Ok().body(users[t.client_id].1.to_string())
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }
}

/// post_store_balance es el servicio que se encarga de almacenar el balance de los clientes en un archivo.
/// # Recibe
/// * manager: Una referencia a un Manager.
/// # Devuelve
/// * Un HttpResponse con un mensaje de confirmación y el nombre del archivo.
#[post("/store_balance")]
pub async fn post_store_balance(manager: web::Data<Manager>) -> impl Responder {
    let mut users = manager.users.lock().unwrap();
    let mut gen = manager.generated.lock().unwrap();
    let date = Local::now().format("%d%m%Y").to_string();
    let _ = create_dir("reports");
    let mut file = std::fs::File::create(format!("reports/{date}_{gen}.dat")).unwrap();
    for id in 0..users.len() {
        let _ = file.write(format!("{id} {}\n", users[id].1).as_bytes());
        users[id].1 = 0.0;
    }
    *gen += 1;
    HttpResponse::Ok().body(format!(
        "BALANCE STORED! FILE NAME: reports/{}_{}.dat",
        &date,
        *gen - 1
    ))
}

/// get_client_balance es el servicio que se encarga de mostrar el balance de un cliente.
/// # Recibe
/// * id: El id del cliente.
/// * manager: Una referencia a un Manager.
/// # Devuelve
/// * Un HttpResponse con la informacion y el balance del cliente o un error si el cliente no existe.
#[get("/client_balance/{id}")]
pub async fn get_client_balance(
    id: web::Path<usize>,
    manager: web::Data<Manager>,
) -> impl Responder {
    let users = manager.users.lock().unwrap();
    let id = id.into_inner();
    if users.len() > id {
        HttpResponse::Ok().body(format!("{:?}\n\nBalance:{}", users[id].0, users[id].1))
    } else {
        HttpResponse::Ok().body("ERROR: CLIENT ID NOT FOUND!")
    }
}
