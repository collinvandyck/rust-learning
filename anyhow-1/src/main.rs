use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

fn main() {
    let path = "src/mainf.rs";
    match read_file(path) {
        Ok(content) => println!("{content}"),
        Err(e) => println!("Error: {:?}", e),
    }

    let res = return_err();
    match res {
        Ok(content) => println!("{content}"),
        Err(e) => println!("Error: {}", e),
    }
}

fn return_err() -> Result<String> {
    let err = anyhow!("Error Happened!!");
    let err = Err(err);
    err
}

fn read_file(path: &str) -> Result<String> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file {}", path))?;
    Ok(content)
}
