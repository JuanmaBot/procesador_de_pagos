use pagos::server;
fn main() {
    if let Err(e) = server::initialization::main() {
        println!("Error: {e}")
    }
}