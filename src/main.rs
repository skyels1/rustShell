use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                if input.to_lowercase().trim().eq_ignore_ascii_case("q") {
                    print!("closing shell...");
                    break;
                }
            },
            Err(e) => println!("Error reading input: {}", e)
        }
    }
}