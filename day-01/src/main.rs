use std::fs;

fn main() {
    let contents = match fs::read_to_string("./src/input.txt") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    println!("{}", contents);
}