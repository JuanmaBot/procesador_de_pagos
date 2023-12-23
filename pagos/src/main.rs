use pagos::server;

/// main es la función principal que se encarga de inicializar el HttpServer.
fn main() {
    if let Err(e) = server::initialization::main() {
        println!("Error: {e}")
    }
}
