use std::{net::TcpStream, io::{Read, Error}};

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];


    if let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            return Ok(());
        }
        let request = String::from_utf8_lossy(&buffer).to_string();
        let command: &str = &request.split_whitespace().collect::<Vec<&str>>()[0..2].join(" ");
        match command {
            "POST /new_client" => {post_new_client(request)?},
            "POST /new_credit_transaction" => {post_new_credit_transaction(request)?},
            "POST /new_debit_transaction" => {post_new_debit_transaction(request)?},
            "POST /store_balances" => {post_store_balance(request)?},
            "GET /client_balance" => {get_client_balance(request)?},
            _ => return Err(Error::new(std::io::ErrorKind::Unsupported, "Unsuported command"))
        }
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Error: no se pudo leer el stream",
        ))
    }
}

fn post_new_client(request: String) -> Result<(), Error> {
    Ok(())
}

fn post_new_credit_transaction(request: String) -> Result<(), Error> {
    Ok(())
}

fn post_new_debit_transaction(request: String) -> Result<(), Error> {
    Ok(())
}

fn post_store_balance(request: String) -> Result<(), Error> {
    Ok(())
}

fn get_client_balance(request: String) -> Result<(), Error> {
    Ok(())
}