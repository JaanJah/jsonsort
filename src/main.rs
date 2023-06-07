use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        if let Ok(line) = line {
            // Process each line of input here
            println!("Received input: {}", line);
        }
    }
}
