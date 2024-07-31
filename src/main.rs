use std::io::{self, Read};
mod mappings;
mod translate;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }
        let mapped = translate::translate(&line);
        println!("{}", mapped);
        line.clear();
    }
}
