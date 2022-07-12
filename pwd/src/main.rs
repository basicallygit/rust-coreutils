use std::env;
fn main() {
    println!("{}", env::current_dir().expect("Failed to get current directory").display());
}
