fn main() {
    std::process::Command::new("cmd")
        .args(["/c", "cls"]).status().unwrap();
}