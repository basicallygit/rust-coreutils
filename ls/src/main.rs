extern crate chrono;
extern crate colored;
use chrono::{DateTime, Local};
use colored::Colorize;

use std::{fs, env};

//TODO: optimize and use CLAP to parse the command line arguments

fn parseflags(path: &str) {
    let argv: Vec<String> = env::args().collect();
    let flag = &argv[1];
    //"all" - shows all files including hidden files (.example.txt)
    if flag == "-a" {
        let contents = fs::read_dir(path).unwrap();
        for entry in contents {
            let entry = entry.unwrap();
            //check if its a directory or not
            if entry.file_type().unwrap().is_dir() {
                println!("{}", entry.file_name().to_str().unwrap().truecolor(166, 77, 255));
            }
            else {
                println!("{}", entry.file_name().to_str().unwrap().truecolor(255, 77, 77));
            }
        }
    }
    else if flag == "-l" { //"long" - shows all non-hidden files and their sizes
        let contents = fs::read_dir(path).unwrap();
        for entry in contents {
            let entry = entry.unwrap();
            if !entry.file_name().to_str().unwrap().starts_with(".") {
                match fs::metadata(entry.path()) {
                    Ok(metadata) => {
                        let time: DateTime<Local> = DateTime::from(metadata.modified()
                            .expect(format!("Error getting modified time for {}", entry.file_name().to_str().unwrap()).as_str()));
                        if metadata.is_dir() {
                            println!("{:>8} {:>13} {}", metadata.len(), time.format("%_d %b %H:%M"), entry.file_name().to_str().unwrap().truecolor(166, 77, 255));

                        }
                        else {
                            println!("{:>8} {:>13} {}", metadata.len(), time.format("%_d %b %H:%M"), entry.file_name().to_str().unwrap().truecolor(255, 77, 77));
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
    else if flag == "-la" {
        let contents = fs::read_dir(path).unwrap();
        for entry in contents {
            let entry = entry.unwrap();
            match fs::metadata(entry.path()) {
                Ok(metadata) => {
                    let time: DateTime<Local> = DateTime::from(metadata.modified()
                        .expect(format!("Error getting modified time for {}", entry.file_name().to_str().unwrap()).as_str()));
                    if metadata.is_dir() {
                        println!("{:>8} {:>13} {}", metadata.len(), time.format("%_d %b %H:%M"), entry.file_name().to_str().unwrap().truecolor(166, 77, 255));
                    }
                    else {
                        println!("{:>8} {:>13} {}", metadata.len(), time.format("%_d %b %H:%M"), entry.file_name().to_str().unwrap().truecolor(255, 77, 77));
                    }
                }
                Err(_) => {}
            }
        }
    }
    else if fs::metadata(flag).unwrap().is_dir() {
        let contents = fs::read_dir(flag).unwrap();
        for entry in contents {
            let entry = entry.unwrap();
            if !entry.file_name().to_str().unwrap().starts_with(".") {
                match fs::metadata(entry.path()) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            println!("{}", entry.file_name().to_str().unwrap().truecolor(166, 77, 255));
                        }
                        else {
                            println!("{}", entry.file_name().to_str().unwrap().truecolor(255, 77, 77));
                        }
                    }
                    Err(_) => {}
                }
            }
        }

    } else {
        println!("{} is not a valid flag or directory", flag);
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();
    match argc {
        1 => {
            let current_dir = env::current_dir().unwrap();
            let contents = fs::read_dir(current_dir).unwrap();
            for entry in contents {
                let entry = entry.unwrap();
                if !entry.file_name().to_str().unwrap().starts_with(".") {
                    match fs::metadata(entry.path()) {
                        Ok(metadata) => {
                            if metadata.is_dir() {
                                println!("{}", entry.file_name().to_str().unwrap().truecolor(166, 77, 255));
                            }
                            else {
                                println!("{}", entry.file_name().to_str().unwrap().truecolor(255, 77, 77));
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
        2 => {
            parseflags(".");
        }
        _ => {
            let dir = &argv[2];
            if fs::metadata(dir).unwrap().is_dir() {
                parseflags(dir);
            }
                
        }
    }

}