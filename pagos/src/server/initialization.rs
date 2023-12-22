use std::{io::{Error, Write}, net::{TcpListener, TcpStream}, thread};
use super::handler;
const PUERTO: &str = "localhost:9418";

pub fn server_init() {

}

fn initialize() -> Result<(), Error>{
    let listener = TcpListener::bind(PUERTO)?;
    let mut childs = Vec::new();
    let adr2 = PUERTO.to_string();
    childs.push(thread::spawn(move || {get_input(&adr2)}));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut buf: [u8; 1] = [0; 1];
                let n = stream.peek(&mut buf)?;
                if n == 0 || buf[0] == b'q' {
                    break;
                }
                let builder = thread::Builder::new().name("cliente".to_string());
                childs.push(builder.spawn(|| handler::handle_client(stream))?);
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }
    for child in childs {
        match child.join() {
            Ok(result) => match result {
                Ok(_) => {}
                Err(e) => {
                    let err = format!("Error al manejar un cliente: {e}");
                    eprintln!("{err}");
                }
            },
            Err(_e) => {
                println!("Se experimentó un Error con un Cliente");
            }
        }
    }
    Ok(())
}

fn get_input(s_addr: &str) -> std::io::Result<()>{
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_lowercase();
        if trimmed == "q" {
            // Envia un mensaje al hilo principal para indicar que debe salir
            let _ = TcpStream::connect(s_addr)?.write("q".as_bytes())?;
            break;
        }
        input.clear();
    }
    Ok(())
}