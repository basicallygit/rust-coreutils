use std::{fs, env};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();
    match argc {
        2 => {
            let filename = &argv[1];
            match fs::metadata(filename) { 
                Ok(metadata) => {
                    if metadata.is_file() {
                        let contents = fs::read_to_string(filename).unwrap();
                        println!("{}", contents);
                    }
                    else {
                        println!("File not found: {}", filename);
                    }
                }
                Err(error) => {
                    println!("Error opening file: {}", error);
                }
            }
        }
        1 | _ => {
            println!("Usage: cat <file>");
        }
    }
}