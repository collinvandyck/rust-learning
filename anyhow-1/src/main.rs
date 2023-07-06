use anyhow::Result;

fn main() {
    let path = "src/main.rs";
    match read_file(path) {
        Ok(content) => println!("{content}"),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
