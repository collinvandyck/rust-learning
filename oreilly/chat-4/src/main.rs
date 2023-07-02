use std::error::Error;

use chat_4::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let s = Server::new(3000);
    s.run()?;
    Ok(())
}
