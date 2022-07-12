use std::{fs, env, io};
use std::io::Write;

//TODO: also optimize and use CLAP to parse the command line arguments

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    match argc {
        1 => {
            println!("Usage: rm <flag> [file/dir]");
        }
        2 => {
            let arg = &args[1];
            if arg == "-r" {
                println!("Expected file/dir after flag");
            }
            //check if its a file or directory
            else if fs::metadata(arg).unwrap().is_file() {
                fs::remove_file(arg).unwrap();
            }
            else if fs::metadata(arg).unwrap().is_dir() {
                println!("{} is a directory", arg);
            }
            else {
                println!("{} is not a valid file name, flag or directory", arg);
            }
        }
        3 => {
            let flag = &args[1];
            let dir = &args[2];
            if flag == "-r" {
                if fs::metadata(dir).unwrap().is_dir() {
                    //make sure the user wants to delete the directory
                    print!("Are you sure you want to delete [{}]? (y/N) ", dir);
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim().to_lowercase().starts_with("y") {
                        fs::remove_dir_all(dir).unwrap();
                    }
                }
                else {
                    println!("{}: directory not found", dir);
                }
            }
            else if flag == "-rf" {
                if fs::metadata(dir).unwrap().is_dir() {
                    fs::remove_dir_all(dir).unwrap();
                }
                else {
                    println!("{}: directory not found", dir);
                }
            }
            else {
                println!("Unknown flag: {}", flag);
            }
        }
        _ => {
            println!("Too many arguments");
        }
    }
}