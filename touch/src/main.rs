use std::path::Path;
use std::fs::{File, OpenOptions};
use std::env;
use std::io::Write;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 {
        println!("Usage: touch [filename]");    
    } else {
        for i in 1..argv.len() {
            //if file does not already exist, create it, if not the timestamp is updated
            if !Path::new(&argv[i]).exists() {
                File::create(&argv[i]).unwrap();
            }
            else {
                //write a single space to the file to update the modified timestamp
                //TODO: this is a quick fix, need to find a better way to modify it
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&argv[i])
                    .unwrap();
                write!(file, " ").unwrap();
            }
        }
    }
}
