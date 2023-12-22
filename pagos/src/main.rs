pub mod commands;
pub mod server;

use std::io::{Error, self, Write};

use commands::handler::handle_command;


fn main() {
    loop {
        let input = match get_input() {
            Ok(input) => input,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        };

        if input == "q\n" {
            return;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if let Err(e) = handle_command(args) {
            println!("Error: {e}")
        }
    }
}
fn get_input() -> Result<String, Error> {
    print!("\x1b[34minput: $ \x1b[0m");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}