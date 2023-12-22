use std::{io::Error, net::TcpStream};

const PUERTO: &str = "localhost:9418";
const USUARIO: &str = "User";

pub fn handle_command(args: Vec<&str>) -> Result<(),Error>{
    match args[0] {
        "new_client" => {new_client(args)},
        "new_credit_transaction" => {new_credit_transaction(args)},
        "new_debit_transaction" => {new_debit_transaction(args)},
        "store_balances" => {store_balances(args)},
        "client_balance" => {client_balance(args)},
        _ => Err(Error::new(std::io::ErrorKind::InvalidInput, "El comando provisto no esta soportado"))
    }
}

// POST /repos/demo/pulls HTTP/1.1
// Host: localhost:9410
// User-Agent: curl/7.68.0
// Accept: */*
// Content-Type: application/json
// Content-Length: 141

// {"id":1,"title":"PR para demo","description":"Mergeamos dos branches triviales con conflict","head":"branch","base":"master","status":"open"}

// === === === === === === === === 

// GET /repos/demo/pulls HTTP/1.1
// Host: localhost:9410
// User-Agent: curl/7.68.0
// Accept: */*

fn new_client(args: Vec<&str>) -> Result<(),Error> {
    let mut stream = TcpStream::connect(PUERTO)?;
    let mut msj = format!("POST /new_client HTTP/1.1\nHost: {PUERTO}\nUser-Agent: {USUARIO}\nACCEPT: */*\nContent-Type: application/json\n");
    
    Ok(())
}

fn new_credit_transaction(args: Vec<&str>) -> Result<(),Error> {
    Ok(())
}

fn new_debit_transaction(args: Vec<&str>) -> Result<(),Error> {
    Ok(())
}

fn store_balances(args: Vec<&str>) -> Result<(),Error> {
    Ok(())
}

fn client_balance(args: Vec<&str>) -> Result<(),Error> {
    Ok(())
}