use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::{env, io};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 {
        println!("Usage: touch [filename]");
    } else {
        for i in 1..argv.len() {
            //check if file exists
            if Path::new(&argv[i]).exists() {
                println!("File with name {} already exists", &argv[i]);
                print!("Continue to overwrite? (y/N) "); // Touch by default change acess time stamp if file already exist
            
                }
            }
            else {
                File::create(&argv[i]).unwrap();
            }
        }
    }
}
